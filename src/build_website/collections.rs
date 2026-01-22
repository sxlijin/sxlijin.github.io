use anyhow::Result;
use chrono::NaiveDate;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use super::markdown_render_engine::ParsedMarkdown;

pub trait Collection {
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

#[derive(Debug, Clone)]
/// When rendering index.html, we want to show the list of blog posts,
/// so we need to collect metadata from each post.
pub struct PostMetadata {
    pub timestamp: NaiveDate,
    title: String,
    path: String,
}

pub struct PostCollection;
impl Collection for PostCollection {
    type TRenderOutput = PostMetadata;

    fn input_paths() -> impl Iterator<Item = PathBuf> {
        WalkDir::new("posts")
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                e.path()
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| !name.starts_with("draft-"))
                    .unwrap_or(true)
            })
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

pub struct IndexHtmlCollection {
    pub posts: Vec<PostMetadata>,
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

pub struct PageCollection;
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
