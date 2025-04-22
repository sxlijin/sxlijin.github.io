use anyhow::{Context, Result};
use askama::Template;
use comrak::Arena;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use tracing_subscriber::{self, EnvFilter};
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

pub fn build_posts() -> Result<()> {
    // Ensure the output directory exists
    let output_dir = Path::new("_site/posts");
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

            // Ensure the parent directory exists
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }

            tracing::info!("Converting {} to {}", path.display(), output_path.display());

            // Read the markdown file
            let markdown = fs::read_to_string(path)?;

            // Convert markdown to HTML using comrak
            let options = comrak::Options {
                extension: comrak::ExtensionOptions::builder()
                    .front_matter_delimiter("---".into())
                    .build(),
                parse: comrak::ParseOptions::default(),
                render: comrak::RenderOptions::default(),
            };
            let html = comrak::markdown_to_html(&markdown, &options);

            // Write the HTML to the output file
            fs::write(output_path, html)?;
        }
    }

    tracing::info!("Markdown conversion completed successfully");
    Ok(())
}

#[derive(Debug, Deserialize, askama::Template)]
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

pub fn build_pages() -> Result<()> {
    // Ensure the output directory exists
    let output_dir = Path::new("_site/");
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

            // Read the markdown file
            let markdown = fs::read_to_string(path)?;

            // Convert markdown to HTML using comrak
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
            let mut frontmatter = None;
            for child in root.children() {
                if let comrak::nodes::NodeValue::FrontMatter(ref fm) = child.data.borrow().value {
                    let fm = fm.split("---\n").collect::<Vec<&str>>().join("");
                    let fm: PageFrontmatter = serde_yaml::from_str(&fm)?;
                    println!("frontmatter: {:?}", fm);
                    frontmatter = Some(fm);
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

fn main() -> Result<()> {
    // Initialize tracing with default configuration and RUST_LOG environment variable support
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting build process");
    build_scss()?;
    build_posts()?;
    build_pages()?;
    tracing::info!("Build process completed");
    Ok(())
}
