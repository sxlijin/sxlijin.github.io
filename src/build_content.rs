use anyhow::{Context, Result};
use askama::Template;
use chrono::NaiveDate;
use comrak::Arena;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn build_scss() -> Result<()> {
    // Ensure the output directory exists
    let output_dir = Path::new("_site/scss");
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
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        // Only process .scss files
        if path.extension().and_then(|s| s.to_str()) == Some("scss") {
            // Get the relative path from the scss directory
            let relative_path = path.strip_prefix(scss_dir)?;
            let output_path = output_dir.join(relative_path.with_extension("css"));

            // Ensure the parent directory exists
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }

            tracing::info!("Compiling {} to {}", path.display(), output_path.display());

            // Compile the SCSS file
            let css = grass::from_path(path, &grass::Options::default())?;

            // Write the compiled CSS to the output file
            fs::write(output_path, css)?;
        }
    }

    tracing::info!("SCSS compilation completed successfully");
    Ok(())
}

struct PostMetadata {
    timestamp: NaiveDate,
    title: String,
    path: String,
}

fn build_posts() -> Result<Vec<PostMetadata>> {
    let mut posts = Vec::new();

    // Ensure the output directory exists
    let output_dir = Path::new("_site");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    // Check if the posts directory exists
    let posts_dir = Path::new("posts");
    if !posts_dir.exists() {
        anyhow::bail!("Posts directory not found");
    }

    // Use walkdir to iterate through all files in the posts directory
    for entry in WalkDir::new(posts_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        // Only process .md files
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            // Get the relative path from the posts directory
            let relative_path = path.strip_prefix(posts_dir)?;
            let output_path = output_dir.join(relative_path.with_extension("html"));
            let output_path2 = output_dir
                .join(relative_path.with_extension(""))
                .join("index.html");

            // Ensure the parent directory exists
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }

            tracing::info!("Converting {} to {}", path.display(), output_path.display());

            // Read the markdown file
            let markdown = fs::read_to_string(path)?;

            // Convert markdown to HTML using comrak
            // let options = comrak::Options {
            //     extension: comrak::ExtensionOptions::builder()
            //         .front_matter_delimiter("---".into())
            //         .build(),
            //     parse: comrak::ParseOptions::default(),
            //     render: comrak::RenderOptions::default(),
            // };
            // let html = comrak::markdown_to_html(&markdown, &options);
            let options = comrak::Options {
                extension: comrak::ExtensionOptions::builder()
                    .front_matter_delimiter("---".into())
                    .autolink(true)
                    .build(),
                parse: comrak::ParseOptions::default(),
                render: comrak::RenderOptions::default(),
            };

            let arena = Arena::new();
            let root = comrak::parse_document(&arena, &markdown, &options);
            for child in root.children() {
                if let comrak::nodes::NodeValue::Heading(_) = child.data.borrow().value {
                    let h1_ref = &child.first_child().unwrap().data.borrow();
                    let h1 = h1_ref.value.text().clone().unwrap();
                    tracing::info!("heading: {:?}", h1);

                    posts.push(PostMetadata {
                        timestamp: NaiveDate::parse_from_str(
                            &path.file_name().unwrap().to_str().unwrap()[..10],
                            "%Y-%m-%d",
                        )?,
                        title: h1.clone(),
                        path: path
                            .file_stem()
                            .expect("Failed to get file stem")
                            .to_str()
                            .unwrap()
                            .to_string(),
                    });
                    break;
                }
            }
            let mut buf = Vec::new();
            comrak::format_html(&root, &options, &mut buf)?;
            let html = String::from_utf8(buf)
                .context("Failed to convert markdown to HTML (UTF8 validation failed)")?;
            let mut html = html.lines().collect::<Vec<&str>>();
            let formatted_date = &format!(
                "<p>{}</p>",
                posts.last().unwrap().timestamp.format("%Y %b %d")
            );
            html.insert(1, formatted_date);
            html.insert(2, "<hr/>");
            let html = Page {
                title: "placeholder-title".to_string(),
                css: "bizzzzzr.css".to_string(),
                content: html.join("\n"),
            }
            .render()
            .context("Failed to render template")?;

            // Write the HTML to the output file
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }
            if let Some(parent) = output_path2.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(output_path, html.clone())?;
            fs::write(output_path2, html)?;
        }
    }

    tracing::info!("Markdown conversion completed successfully");
    Ok(posts)
}

#[derive(Debug, askama::Template)]
#[template(path = "base.html.j2")]
struct Page {
    title: String,
    css: String,
    // header: String,
    content: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PageFrontmatter {
    title: String,
    css: Option<String>,
    layout: Option<String>,
}

pub fn build_pages(posts: Vec<PostMetadata>) -> Result<()> {
    // Ensure the output directory exists
    let output_dir = Path::new("_site");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    // Check if the posts directory exists
    let pages_dir = Path::new("pages");
    if !pages_dir.exists() {
        anyhow::bail!("Pages directory not found");
    }

    // Use walkdir to iterate through all files in the posts directory
    for entry in WalkDir::new(pages_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        // Only process .md files
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            // Get the relative path from the posts directory
            let relative_path = path.strip_prefix(pages_dir)?;
            let output_path = output_dir.join(relative_path.with_extension("html"));
            let output_path2 = output_dir
                .join(relative_path.with_extension(""))
                .join("index.html");

            // Ensure the parent directory exists
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }
            if let Some(parent) = output_path2.parent() {
                fs::create_dir_all(parent)?;
            }

            tracing::info!("Converting {} to {}", path.display(), output_path.display());
            tracing::info!(
                "Converting {} to {}",
                path.display(),
                output_path2.display()
            );

            // Convert markdown to HTML using comrak
            let options = comrak::Options {
                extension: comrak::ExtensionOptions::builder()
                    .front_matter_delimiter("---".into())
                    .autolink(true)
                    .build(),
                parse: comrak::ParseOptions::default(),
                render: comrak::RenderOptions::builder().unsafe_(true).build(),
            };
            // Read the markdown file
            let mut markdown = fs::read_to_string(path)?;
            if path == Path::new("pages/index.md") {
                markdown = format!(
                    "{}\n<div class=\"post-list\">\n{}</div>",
                    markdown,
                    comrak::markdown_to_html(
                        &posts
                            .iter()
                            .map(|p| format!(
                                "* <div class=\"post-date\">{}</div> <a href=\"{}\">{}</a>",
                                p.timestamp.format("%b %Y"),
                                p.path,
                                p.title
                            ))
                            .collect::<Vec<String>>()
                            .join("\n"),
                        &options
                    )
                );
            }

            // let _rebuild = {
            //     use pulldown_cmark::{html, Options, Parser};

            //     let mut options = Options::empty();
            //     options.insert(Options::ENABLE_STRIKETHROUGH);
            //     let parser = Parser::new_ext(&markdown, options);

            //     // Write to String buffer.
            //     let mut html_output = String::new();
            //     pulldown_cmark::html::push_html(&mut html_output, parser);

            //     Page {
            //         title: "placeholder-title".to_string(),
            //         css: "".to_string(),
            //         content: html_output,
            //     }
            //     .render()
            //     .context("Failed to render template")?
            // };

            let arena = Arena::new();
            let root = comrak::parse_document(&arena, &markdown, &options);
            let mut frontmatter = None;
            for child in root.children() {
                if let comrak::nodes::NodeValue::FrontMatter(ref fm) = child.data.borrow().value {
                    let fm = fm.split("---\n").collect::<Vec<&str>>().join("");
                    let fm: PageFrontmatter = serde_yaml::from_str(&fm)?;
                    println!("frontmatter: {:?}", fm);
                    frontmatter = Some(fm);
                }
                if let comrak::nodes::NodeValue::Heading(_) = child.data.borrow().value {
                    let h1_ref = &child.first_child().unwrap().data.borrow();
                    let h1 = h1_ref.value.text().clone().unwrap();
                    tracing::info!("heading: {:?}", h1);
                    break;
                }
            }
            let mut buf = Vec::new();
            comrak::format_html(&root, &options, &mut buf)?;
            let html = String::from_utf8(buf)
                .context("Failed to convert markdown to HTML (UTF8 validation failed)")?;

            let html = Page {
                title: "placeholder-title".to_string(),
                css: match frontmatter {
                    Some(fm) => fm.css.unwrap_or("".to_string()),
                    None => "".to_string(),
                },
                // header: "placeholder-header".to_string(),
                content: html,
            }
            .render()
            .context("Failed to render template")?;

            // Write the HTML to the output file
            fs::write(output_path, &html)?;
            fs::write(output_path2, &html)?;
        }
    }

    tracing::info!("Markdown conversion completed successfully");
    Ok(())
}

// fn main() -> Result<()> {
//     // Initialize tracing with default configuration and RUST_LOG environment variable support
//     tracing_subscriber::fmt()
//         .with_env_filter(EnvFilter::from_default_env())
//         .init();

//     tracing::info!("Starting build process");
//     build_scss()?;
//     build_posts()?;
//     build_pages()?;
//     tracing::info!("Build process completed");
//     Ok(())
// }

pub fn build_all() -> Result<()> {
    build_scss()?;
    let mut posts = build_posts()?;
    posts.sort_by_key(|p| std::cmp::Reverse(p.timestamp));
    build_pages(posts)?;
    Ok(())
}
