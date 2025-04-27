use anyhow::{Context, Result};
use askama::Template;
use chrono::NaiveDate;
use serde::Deserialize;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;

use crate::EmbeddedBuildTimestamp;

fn build_assets() -> Result<()> {
    let output_dir = Path::new("_site");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    let root_assets_dir = Path::new("root-assets");
    if !root_assets_dir.exists() {
        anyhow::bail!("Root assets directory not found");
    }
    for entry in WalkDir::new(root_assets_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let relative_path = path.strip_prefix(root_assets_dir)?;
        let output_path = output_dir.join(relative_path);
        fs::copy(path, output_path)?;
    }

    // Copy assets directory instead of creating symlink
    let assets_dir = Path::new("assets");
    if !assets_dir.exists() {
        anyhow::bail!("Assets directory not found");
    }
    let output_assets_dir = output_dir.join("assets");
    for entry in WalkDir::new(assets_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let relative_path = path.strip_prefix(assets_dir)?;
        let output_path = output_assets_dir.join(relative_path);
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(path, output_path)?;
    }

    Ok(())
}

fn build_scss() -> Result<()> {
    // Ensure the output directory exists
    let output_dir = Path::new("_site/css");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    // Check if the scss directory exists
    let scss_dir = Path::new("scss");
    if !scss_dir.exists() {
        anyhow::bail!("SCSS directory not found");
    }

    // Use walkdir to iterate through all files in the scss directory
    for entry in WalkDir::new(scss_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().extension() == Some(OsStr::new("scss")))
    {
        let path = entry.path();

        // Get the relative path from the scss directory
        let relative_path = path.strip_prefix(scss_dir)?;
        let output_path = output_dir.join(relative_path.with_extension("css"));

        // Ensure the parent directory exists
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        tracing::debug!("Compiling {} to {}", path.display(), output_path.display());

        // Compile the SCSS file
        let css = grass::from_path(path, &grass::Options::default())?;

        // Write the compiled CSS to the output file
        fs::write(output_path, css)?;
    }

    tracing::debug!("SCSS compilation completed successfully");
    Ok(())
}

struct MarkdownRenderEngine<'a, 'b> {
    options: comrak::Options<'b>,
    arena: comrak::Arena<comrak::nodes::AstNode<'a>>,
    build_timestamp: SystemTime,
    #[allow(dead_code)]
    output_dir: PathBuf,
}

struct ParsedMarkdown {
    frontmatter: MdFrontmatter,
    first_h1: String,
    html: String,
}

trait Collection {
    type TRenderOutput;

    fn input_paths() -> impl Iterator<Item = PathBuf>;

    fn output_paths(input_path: &Path) -> Vec<PathBuf>;

    fn render(
        &self,
        input_path: &Path,
        parsed: &ParsedMarkdown,
        options: &comrak::Options,
    ) -> Result<(String, Self::TRenderOutput)>;
}

struct PostCollection;
impl Collection for PostCollection {
    type TRenderOutput = PostMetadata;

    fn input_paths() -> impl Iterator<Item = PathBuf> {
        WalkDir::new("posts")
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().to_path_buf())
    }

    fn output_paths(input_path: &Path) -> Vec<PathBuf> {
        let input_path = input_path
            .strip_prefix("posts")
            .expect("Failed to strip 'posts' prefix from path in 'posts/'");
        vec![
            input_path.with_extension("html"),
            input_path.with_extension("").join("index.html"),
        ]
    }

    fn render(
        &self,
        input_path: &Path,
        parsed: &ParsedMarkdown,
        _: &comrak::Options,
    ) -> Result<(String, PostMetadata)> {
        let post_metadata = PostMetadata {
            timestamp: NaiveDate::parse_from_str(
                &input_path.file_name().unwrap().to_str().unwrap()[..10],
                "%Y-%m-%d",
            )?,
            title: parsed.first_h1.clone(),
            path: input_path
                .file_stem()
                .expect("Failed to get file stem")
                .to_str()
                .unwrap()
                .to_string(),
        };
        let mut html = parsed.html.lines().collect::<Vec<&str>>();
        let formatted_date = &format!("<p>{}</p>", post_metadata.timestamp.format("%Y %b %d"));
        html.insert(1, formatted_date);
        html.insert(2, "<hr/>");

        Ok((html.join("\n"), post_metadata))
    }
}

struct IndexHtmlCollection {
    posts: Vec<PostMetadata>,
}
impl Collection for IndexHtmlCollection {
    type TRenderOutput = ();
    fn input_paths() -> impl Iterator<Item = PathBuf> {
        std::iter::once(PathBuf::from("pages/index.md"))
    }

    fn output_paths(_: &Path) -> Vec<PathBuf> {
        vec![PathBuf::from("index.html")]
    }

    fn render(
        &self,
        _: &Path,
        parsed: &ParsedMarkdown,
        options: &comrak::Options,
    ) -> Result<(String, Self::TRenderOutput)> {
        let post_list = format!(
            "<div class=\"post-list\">\n{}</div>",
            comrak::markdown_to_html(
                &self
                    .posts
                    .iter()
                    .map(|p| format!(
                        "* <div class=\"post-date\">{}</div> <a href=\"{}\">{}</a>",
                        p.timestamp.format("%b %Y"),
                        p.path,
                        p.title
                    ))
                    .collect::<Vec<String>>()
                    .join("\n"),
                options
            )
        );

        Ok((format!("{}\n{}", parsed.html, post_list), ()))
    }
}

struct PageCollection;
impl Collection for PageCollection {
    type TRenderOutput = ();

    fn input_paths() -> impl Iterator<Item = PathBuf> {
        WalkDir::new("pages")
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && e.path() != Path::new("pages/index.md"))
            .map(|e| e.path().to_path_buf())
    }

    fn output_paths(input_path: &Path) -> Vec<PathBuf> {
        let input_path = input_path
            .strip_prefix("pages")
            .expect("Failed to strip 'pages' prefix from path in 'pages/'");
        vec![
            input_path.with_extension("html"),
            input_path.with_extension("").join("index.html"),
        ]
    }

    fn render(
        &self,
        _: &Path,
        parsed: &ParsedMarkdown,
        _: &comrak::Options,
    ) -> Result<(String, Self::TRenderOutput)> {
        Ok((parsed.html.clone(), ()))
    }
}

impl<'a, 'b> MarkdownRenderEngine<'a, 'b> {
    fn new() -> Self {
        Self {
            options: comrak::Options {
                extension: comrak::ExtensionOptions::builder()
                    .front_matter_delimiter("---".into())
                    .autolink(true)
                    .footnotes(true)
                    // TODO: mathjax rendering
                    .math_dollars(true)
                    .build(),
                parse: comrak::ParseOptions::default(),
                render: comrak::RenderOptions::builder().unsafe_(true).build(),
            },
            arena: comrak::Arena::new(),
            build_timestamp: SystemTime::now(),
            output_dir: PathBuf::from("_site"),
        }
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

        Ok(ParsedMarkdown {
            frontmatter: frontmatter.unwrap_or_default(),
            first_h1: h1.context("Failed to parse title")?,
            html,
        })
    }

    fn build_collection<TCollection: Collection>(
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
#[derive(Debug, Clone)]
struct PostMetadata {
    timestamp: NaiveDate,
    title: String,
    path: String,
}

#[derive(Debug, askama::Template)]
#[template(path = "base.html.j2")]
/// Common template for posts/, pages/, and index.html.
struct BaseTemplate {
    title: String,
    css: String,
    content: String,
    build_timestamp: EmbeddedBuildTimestamp,
}

#[derive(Debug, Deserialize, Default)]
/// Frontmatter from any page.
struct MdFrontmatter {
    title: Option<String>,
    css: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BuildSummary {
    pub build_timestamp: SystemTime,
}

pub fn build_website() -> Result<BuildSummary> {
    std::fs::create_dir_all("_site")?;

    build_assets()?;
    build_scss()?;

    let engine = MarkdownRenderEngine::new();
    let mut posts = engine.build_collection(PostCollection)?;
    posts.sort_by_key(|p| std::cmp::Reverse(p.timestamp));
    engine.build_collection(IndexHtmlCollection { posts })?;
    engine.build_collection(PageCollection)?;

    Ok(BuildSummary {
        build_timestamp: engine.build_timestamp,
    })
}
