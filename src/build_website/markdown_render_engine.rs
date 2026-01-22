use anyhow::{anyhow, Context, Result};
use askama::Template;
use kuchiki::traits::*;
use serde::Deserialize;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::EmbeddedBuildTimestamp;

use super::collections::Collection;

pub enum BuildMode {
    Dev,
    Prod,
}
pub struct MarkdownRenderEngine<'a, 'b> {
    options: comrak::Options<'b>,
    arena: comrak::Arena<comrak::nodes::AstNode<'a>>,
    pub build_timestamp: SystemTime,
    output_dir: PathBuf,
    build_mode: BuildMode,
}

impl<'a, 'b> MarkdownRenderEngine<'a, 'b> {
    pub fn new(build_mode: BuildMode) -> Result<Self> {
        let new = Self {
            options: comrak::Options {
                extension: comrak::ExtensionOptions::builder()
                    .front_matter_delimiter("---".into())
                    .autolink(true)
                    .footnotes(true)
                    .header_ids("".to_string())
                    // NB: math_dollars only works in combination with the mathjax in the base.html.j2 template
                    .math_dollars(true)
                    .build(),
                parse: comrak::ParseOptions::default(),
                render: comrak::RenderOptions::builder().unsafe_(true).build(),
            },
            arena: comrak::Arena::new(),
            build_timestamp: SystemTime::now(),
            output_dir: PathBuf::from("_site"),
            build_mode,
        };
        std::fs::create_dir_all(&new.output_dir)?;
        Ok(new)
    }

    fn parse_markdown(&'a self, markdown: String) -> Result<ParsedMarkdown> {
        let root = comrak::parse_document(&self.arena, &markdown, &self.options);

        let mut frontmatter = None;
        let mut h1: Option<String> = None;
        for child in root.children() {
            if let comrak::nodes::NodeValue::FrontMatter(ref fm) = child.data.borrow().value {
                let fm = fm.split("---\n").collect::<Vec<&str>>().join("");
                let fm: MdFrontmatter = serde_yaml::from_str(&fm)?;
                frontmatter = Some(fm);
            }
            if let comrak::nodes::NodeValue::Heading(_) = child.data.borrow().value {
                let h1_ref = &child.first_child().unwrap().data.borrow();
                h1 = Some(h1_ref.value.text().clone().unwrap().clone());
                break;
            }
        }

        let mut buf = Vec::new();
        comrak::format_html(root, &self.options, &mut buf)?;
        let html = String::from_utf8(buf)
            .context("Failed to convert markdown to HTML (UTF8 validation failed)")?;
        let html = transform_footnotes_to_sidenotes(&html)?;

        Ok(ParsedMarkdown {
            frontmatter: frontmatter.unwrap_or_default(),
            first_h1: h1.context("Failed to parse title")?,
            html,
        })
    }

    pub fn build_collection<TCollection: Collection>(
        &'a self,
        collection: TCollection,
    ) -> Result<Vec<TCollection::TRenderOutput>> {
        let mut outputs = vec![];

        for input_path in TCollection::input_paths() {
            let output_paths = TCollection::output_paths(&input_path)
                .into_iter()
                .map(|p| self.output_dir.join(p))
                .collect::<Vec<PathBuf>>();
            tracing::info!("Converting {} to {:?}", input_path.display(), output_paths);

            let markdown = fs::read_to_string(&input_path)?;

            let parsed = self.parse_markdown(markdown)?;

            let (html, output) = collection.render(&input_path, &parsed, &self.options)?;
            outputs.push(output);

            let html = BaseTemplate {
                title: parsed.frontmatter.title.unwrap_or(parsed.first_h1),
                css: parsed.frontmatter.css.unwrap_or("".to_string()),
                content: html,
                dev_mode: match self.build_mode {
                    BuildMode::Dev => true,
                    BuildMode::Prod => false,
                },
                build_timestamp: EmbeddedBuildTimestamp(self.build_timestamp),
            }
            .render()
            .context("Failed to render template")?;

            // Write the HTML to the output file
            for output_path in output_paths {
                if let Some(parent) = output_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(output_path, &html)?;
            }
        }

        tracing::debug!("Finished building {}", std::any::type_name::<TCollection>());
        Ok(outputs)
    }
}

pub struct ParsedMarkdown {
    frontmatter: MdFrontmatter,
    pub first_h1: String,
    pub html: String,
}

#[derive(Debug, askama::Template)]
#[template(path = "base.html.j2")]
/// Common template for posts/, pages/, and index.html.
struct BaseTemplate {
    title: String,
    css: String,
    content: String,
    dev_mode: bool,
    build_timestamp: EmbeddedBuildTimestamp,
}

#[derive(Debug, Deserialize, Default)]
/// Frontmatter from any page.
struct MdFrontmatter {
    title: Option<String>,
    css: Option<String>,
}

fn transform_footnotes_to_sidenotes(html: &str) -> Result<String> {
    let wrapped_html = format!("<div id=\"__content\">{}</div>", html);
    let document = kuchiki::parse_html().one(wrapped_html);

    let mut footnote_map = HashMap::new();
    if let Ok(footnotes) = document.select("section.footnotes li[id]") {
        for li in footnotes {
            let li_node = li.as_node().clone();
            let Some(li_element) = li_node.as_element() else { continue };
            let Some(id) = li_element.attributes.borrow().get("id").map(|s| s.to_string()) else {
                continue;
            };

            if let Ok(backrefs) = li_node.select("a.footnote-backref") {
                for backref in backrefs {
                    backref.as_node().detach();
                }
            }

            let content = if let Some(p_node) = only_p_child(&li_node) {
                serialize_children(&p_node)?
            } else {
                serialize_children(&li_node)?
            };
            footnote_map.insert(id, content);
        }
    }

    if footnote_map.is_empty() {
        return Ok(html.to_string());
    }

    let footnote_refs: Vec<_> = match document.select("sup.footnote-ref") {
        Ok(selection) => selection.map(|sup| sup.as_node().clone()).collect(),
        Err(_) => Vec::new(),
    };

    for sup_node in footnote_refs {
        let href = sup_node
            .select_first("a")
            .ok()
            .and_then(|anchor| anchor.attributes.borrow().get("href").map(|s| s.to_string()));
        let Some(href) = href else { continue };
        let footnote_id = href.trim_start_matches('#');
        let Some(content) = footnote_map.get(footnote_id) else { continue };

        let sidenote_id = format!("sidenote-{}", footnote_id);
        let replacement_html = format!(
            "<span data-sidenote-wrapper>\
                <label for=\"{id}\" class=\"margin-toggle sidenote-number\"></label>\
                <input type=\"checkbox\" id=\"{id}\" class=\"margin-toggle\">\
                <span class=\"sidenote\">{content}</span>\
            </span>",
            id = sidenote_id,
            content = content
        );
        let replacement_doc = kuchiki::parse_html().one(replacement_html);
        let Ok(wrapper) = replacement_doc.select_first("span[data-sidenote-wrapper]") else {
            continue;
        };
        let wrapper_node = wrapper.as_node().clone();
        let children: Vec<_> = wrapper_node.children().collect();
        for child in children {
            sup_node.insert_before(child);
        }
        sup_node.detach();
    }

    if let Ok(sections) = document.select("section.footnotes") {
        for section in sections {
            section.as_node().detach();
        }
    }

    let content_node = document
        .select_first("#__content")
        .map_err(|_| anyhow!("Failed to locate HTML wrapper for sidenote transform"))?;
    serialize_children(content_node.as_node())
}

fn only_p_child(node: &kuchiki::NodeRef) -> Option<kuchiki::NodeRef> {
    let mut element_children = node.children().filter(|child| child.as_element().is_some());
    let first = element_children.next()?;
    if element_children.next().is_some() {
        return None;
    }
    let element = first.as_element()?;
    if element.name.local.as_ref() == "p" {
        Some(first)
    } else {
        None
    }
}

fn serialize_children(node: &kuchiki::NodeRef) -> Result<String> {
    let mut buf = Vec::new();
    for child in node.children() {
        child.serialize(&mut buf)?;
    }
    String::from_utf8(buf).context("Failed to serialize HTML")
}
