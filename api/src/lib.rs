//! This crate contains all shared fullstack server functions.
use std::fs;

use dioxus::prelude::*;
use pulldown_cmark::{html, Event, HeadingLevel, Options as PulldownOptions, Parser, Tag};
use walkdir::WalkDir;

pub fn build_parser(markdown: &str) -> Parser {
    Parser::new_ext(
        &markdown,
        PulldownOptions::ENABLE_TABLES
            | PulldownOptions::ENABLE_FOOTNOTES
            | PulldownOptions::ENABLE_TASKLISTS
            | PulldownOptions::ENABLE_SMART_PUNCTUATION
            | PulldownOptions::ENABLE_HEADING_ATTRIBUTES,
    )
}

pub async fn parse_blog_markdown(blog_id: &str, markdown: &str, is_preview: bool) -> String {
    let parser = build_parser(markdown);
    let mut html = String::new();

    for event in parser {
        match event {
            Event::Text(text) => {
                let mut remaining = text.as_ref();
                while let Some(start) = remaining.find("^^") {
                    let before = &remaining[..start];
                    html.push_str(before);

                    if let Some(end) = remaining[start + 2..].find("^^") {
                        let sup_text = &remaining[start + 2..start + 2 + end];
                        html.push_str(&format!("<sup>{}</sup>", sup_text));
                        remaining = &remaining[start + 2 + end + 2..];
                    } else {
                        // No closing ^^ found, treat as normal text
                        html.push_str(&remaining[start..]);
                        break;
                    }
                }
                html.push_str(remaining);
            }
            Event::Start(Tag::Heading(level, Some(heading_id), _)) => {
                if is_preview {
                    html.push_str(&format!(
                        "<a href=\"/blog/{}#{}\"><{} id=\"{}\">",
                        blog_id, heading_id, level, heading_id
                    ));
                } else {
                    html.push_str(&format!("<{} id=\"{}\">", level, heading_id));
                }
            }
            Event::End(Tag::Heading(level, Some(heading_id), _)) => {
                if !is_preview {
                    html.push_str(&format!(
                        " <a href=\"/blog/{}#{}\">#</a></{}>",
                        blog_id, heading_id, level
                    ));
                } else {
                    html.push_str(&format!("</{}></a>", level));
                }
            }
            Event::Start(tag) => {
                pulldown_cmark::html::push_html(
                    &mut html,
                    std::iter::once(Event::Start(tag.clone())),
                );
            }
            Event::End(tag) => {
                pulldown_cmark::html::push_html(
                    &mut html,
                    std::iter::once(Event::End(tag.clone())),
                );
            }
            _ => {}
        }
    }

    html
}

pub async fn get_blog_by_id(blog_id: &str) -> Result<String, ServerFnError> {
    let content = fs::read_to_string(format!("./posts/{}.md", blog_id))?;
    Ok(content)
}

#[server(Blog)]
pub async fn blog(blog_id: String) -> Result<String, ServerFnError> {
    Ok(parse_blog_markdown(&blog_id, &get_blog_by_id(&blog_id).await?, false).await)
}

#[server(Feed)]
pub async fn get_blog_ids() -> Result<Vec<String>, ServerFnError> {
    Ok(WalkDir::new("./posts")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            e.path()
                .file_stem()
                .map(|stem| stem.to_string_lossy().to_string())
        })
        .collect::<Vec<String>>())
}

#[server(BlogPreview)]
pub async fn blog_preview(blog_id: String) -> Result<String, ServerFnError> {
    let content = fs::read_to_string(format!("./posts/{}.md", blog_id))?;
    let lines: Vec<&str> = content.lines().take(3).collect();
    let preview_md = lines.join("\n");
    let html = parse_blog_markdown(&blog_id, &preview_md, true).await;
    Ok(html)
}
