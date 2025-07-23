//! This crate contains all shared fullstack server functions.
use std::{
    cell::{RefCell, UnsafeCell},
    fs,
    rc::Rc,
};

use dioxus::prelude::*;
use pulldown_cmark::{
    html, CowStr, Event, HeadingLevel, Options as PulldownOptions, Parser, Tag, TagEnd,
};
use walkdir::WalkDir;

const BLOG_POST_PATH: &str = "./posts";

pub fn build_parser(markdown: &str) -> Parser {
    Parser::new_ext(&markdown, PulldownOptions::all())
}

pub fn parse_blog_markdown(blog_id: &str, markdown: &str, is_preview: bool) -> String {
    let parser = build_parser(markdown);

    let html_content = |content: String| Event::Html(CowStr::from(content));
    let text_content = |content: &'static str| Event::Text(CowStr::from(content));

    let mut heading = String::new();
    let parser = parser.flat_map(|event| match event {
        Event::Start(Tag::Heading {
            level,
            id: Some(heading_id),
            classes: _,
            attrs: _,
        }) => {
            if is_preview {
                vec![html_content(format!(
                    "<a href=\"/blog/{}#{}\"><{} id=\"{}\">",
                    blog_id, heading_id, level, heading_id
                ))]
            } else {
                heading = heading_id.to_string();
                vec![html_content(format!("<{} id=\"{}\">", level, heading_id))]
            }
        }
        Event::End(TagEnd::Heading(level)) => {
            if !is_preview {
                let events = vec![
                    text_content(" ["),
                    html_content(format!(
                        "<a href=\"/blog/{}#{}\" style=\"text-decoration: none;\">#</a>",
                        blog_id, heading
                    )),
                    text_content("]"),
                    html_content(format!("</{}>", level)),
                ];
                heading.clear();
                events
            } else {
                vec![html_content(format!("</{}></a>", level))]
            }
        }
        _ => vec![event],
    });

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

pub async fn get_blog_by_id(blog_id: &str) -> Result<String, ServerFnError> {
    let content = fs::read_to_string(format!("{}/{}.md", BLOG_POST_PATH, blog_id))?;
    Ok(content)
}

#[server(Blog)]
pub async fn blog(blog_id: String) -> Result<String, ServerFnError> {
    Ok(parse_blog_markdown(
        &blog_id,
        &get_blog_by_id(&blog_id).await?,
        false,
    ))
}

#[server(Feed)]
pub async fn get_blog_ids() -> Result<Vec<String>, ServerFnError> {
    Ok(WalkDir::new(BLOG_POST_PATH)
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
    let content = fs::read_to_string(format!("{}/{}.md", BLOG_POST_PATH, blog_id))?;
    let lines: Vec<&str> = content.lines().take(3).collect();
    let preview_md = lines.join("\n");
    let html = parse_blog_markdown(&blog_id, &preview_md, true);
    Ok(html)
}
