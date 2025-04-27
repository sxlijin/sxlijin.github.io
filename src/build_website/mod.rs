mod collections;
mod markdown_render_engine;

use anyhow::Result;
use collections::{IndexHtmlCollection, PageCollection, PostCollection};
use markdown_render_engine::MarkdownRenderEngine;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use walkdir::WalkDir;

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

#[derive(Debug, Clone)]
pub struct BuildSummary {
    pub build_timestamp: SystemTime,
}

pub fn build_website() -> Result<BuildSummary> {
    build_assets()?;
    build_scss()?;

    let engine = MarkdownRenderEngine::new()?;
    let mut posts = engine.build_collection(PostCollection)?;
    posts.sort_by_key(|p| std::cmp::Reverse(p.timestamp));
    engine.build_collection(IndexHtmlCollection { posts })?;
    engine.build_collection(PageCollection)?;

    Ok(BuildSummary {
        build_timestamp: engine.build_timestamp,
    })
}
