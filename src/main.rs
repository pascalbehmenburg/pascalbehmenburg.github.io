//! This crate contains all shared fullstack server functions.
use jiff::civil::DateTime;
use minify_html_onepass::{Cfg, in_place_str};
use pulldown_cmark::{
    CodeBlockKind, CowStr, Event, MetadataBlockKind, Options as PulldownOptions, Parser, Tag,
    TagEnd, html,
};
use sailfish::TemplateSimple;
use std::collections::HashMap;
use std::{fs, path::Path};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use walkdir::{DirEntry, WalkDir};

const STATIC_PATH: &str = "./static";
const PUBLIC_SRV_PATH: &str = "./public";

type Frontmatter = HashMap<String, String>;

fn parse_frontmatter(input: &str) -> Frontmatter {
    input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            if parts.len() == 2 {
                Some((parts[0].to_string(), parts[1].to_string()))
            } else {
                None
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BlogMetadata {
    title: String,
    author: String,
    date: String,               // original string for display
    datetime: Option<DateTime>, // parsed date for sorting
}

impl Default for BlogMetadata {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            author: "".to_string(),
            date: "".to_string(),
            datetime: None,
        }
    }
}

impl BlogMetadata {
    fn new_from_frontmatter(frontmatter: &Frontmatter) -> Self {
        let date_str = frontmatter
            .get("date")
            .unwrap_or(&"".to_string())
            .to_string();
        let datetime = date_str.parse().ok();
        Self {
            title: frontmatter
                .get("title")
                .unwrap_or(&"".to_string())
                .to_string(),
            author: frontmatter
                .get("author")
                .unwrap_or(&"".to_string())
                .to_string(),
            date: date_str,
            datetime,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BlogPost {
    id: String,
    content: String,
    metadata: BlogMetadata,
}

impl From<DirEntry> for BlogPost {
    fn from(file: DirEntry) -> Self {
        println!("File name: {}", file.file_name().to_string_lossy());
        let file_name = file
            .file_name()
            .to_string_lossy()
            .strip_suffix(".md")
            .expect("File name ends with .md")
            .to_string();
        let content = fs::read_to_string(file.path()).unwrap();

        let parser = Parser::new_ext(&content, PulldownOptions::all()).into_offset_iter();

        let mut heading = String::new();

        let mut scanning_frontmatter = false;
        let mut is_scanning_code = false;
        let mut frontmatter = String::new();
        let mut code = String::new();
        let mut code_lang = String::new();
        let mut code_start_line: usize = 0;
        let mut metadata: BlogMetadata = BlogMetadata::default();

        // Helper to calculate line number from byte offset
        let line_number = |offset: usize| content[..offset].matches('\n').count() + 1;

        let html_content = |content: String| Event::Html(CowStr::from(content));
        let text_content = |content: &'static str| Event::Text(CowStr::from(content));
        let parser = parser.flat_map(|(event, range)| match event {
            Event::Start(Tag::Heading {
                level,
                id: Some(heading_id),
                classes: _,
                attrs: _,
            }) => {
                heading = heading_id.to_string();
                vec![html_content(format!("<{} id=\"{}\">", level, heading_id))]
            }
            Event::End(TagEnd::Heading(level)) => {
                let events = vec![
                    text_content(" ["),
                    html_content(format!(
                        "<a href=\"/{}.html#{}\" style=\"text-decoration: none;\">#</a>",
                        file_name, heading
                    )),
                    text_content("]"),
                    html_content(format!("</{}>", level)),
                ];
                heading.clear();
                events
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                is_scanning_code = true;
                code_lang = lang.to_string();
                code_start_line = line_number(range.start);
                vec![]
            }
            Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle)) => {
                scanning_frontmatter = true;
                vec![]
            }
            Event::Text(ref text) => {
                if scanning_frontmatter {
                    frontmatter.push_str(text);
                    return vec![];
                }
                if is_scanning_code {
                    code.push_str(text);
                    return vec![];
                }
                vec![event]
            }
            Event::End(TagEnd::CodeBlock) => {
                let ss = SyntaxSet::load_defaults_newlines();
                let ts = ThemeSet::load_defaults();
                let theme = &ts.themes["base16-ocean.dark"];

                // Try to find syntax, with fallback for common aliases
                let syntax = ss
                    .find_syntax_by_token(&code_lang)
                    .or_else(|| match code_lang.to_lowercase().as_str() {
                        "md" => ss.find_syntax_by_token("markdown"),
                        _ => None,
                    })
                    .unwrap_or_else(|| {
                        eprintln!(
                            "Warning: static/blog/{}.md:{}: Unknown syntax '{}', falling back to plain text",
                            file_name,
                            code_start_line,
                            code_lang
                        );
                        ss.find_syntax_plain_text()
                    });

                let highlighted_code =
                    highlighted_html_for_string(&code, &ss, syntax, theme).unwrap();
                is_scanning_code = false;
                code.clear();
                code_lang.clear();
                vec![html_content(highlighted_code)]
            }
            Event::End(TagEnd::MetadataBlock(MetadataBlockKind::YamlStyle)) => {
                metadata = BlogMetadata::new_from_frontmatter(&parse_frontmatter(&frontmatter));
                scanning_frontmatter = false;
                frontmatter.clear();
                vec![]
            }
            _ => vec![event],
        });

        let mut html_content = String::new();
        html::push_html(&mut html_content, parser);
        let html_file_path = format!("{}/{}.html", PUBLIC_SRV_PATH, &file_name);
        let blog_post = BlogPost {
            id: file_name,
            content: html_content.clone(),
            metadata,
        };

        if let Ok(minified_html_content) = in_place_str(
            &mut BlogPostTemplate {
                post: blog_post.clone(),
            }
            .render_once()
            .unwrap(),
            &Cfg {
                minify_js: true,
                minify_css: true,
            },
        ) {
            fs::write(&html_file_path, minified_html_content).unwrap();
        }

        blog_post
    }
}

#[derive(TemplateSimple)]
#[template(path = "blog.stpl", escape = false)]
struct BlogTemplate {
    posts: Vec<BlogPost>,
}

#[derive(TemplateSimple)]
#[template(path = "post.stpl", escape = false)]
struct BlogPostTemplate {
    post: BlogPost,
}

pub fn main() {
    let blog_path = format!("{}{}", STATIC_PATH, "/blog");

    let minify_cfg = &Cfg {
        minify_js: true,
        minify_css: true,
    };
    if Path::new(PUBLIC_SRV_PATH).exists() {
        // empty the public directory if exists
        fs::remove_dir_all(PUBLIC_SRV_PATH).expect("Failed to empty public directory");
        fs::create_dir_all(PUBLIC_SRV_PATH).expect("Failed to recreate public directory");
    } else {
        // create the public directory if it doesn't exist
        fs::create_dir_all(PUBLIC_SRV_PATH).expect("Failed to create public directory");
    }

    // copy css files to public dir
    let css_file_name = "main.css";
    let css_source_path = format!("{}/css/{}", STATIC_PATH, &css_file_name);
    let css_dest_path = format!("{}/{}", PUBLIC_SRV_PATH, &css_file_name);
    let mut css_content = fs::read_to_string(&css_source_path).expect("Failed to read css file");
    if let Ok(minified_css_content) = in_place_str(&mut css_content, minify_cfg) {
        fs::write(&css_dest_path, minified_css_content).unwrap();
    }

    // copy images folder to public dir
    let images_source_path = format!("{}/img", STATIC_PATH);
    let images_dest_path = format!("{}/img", PUBLIC_SRV_PATH);
    fs::create_dir(&images_dest_path).expect("Failed to create images directory");
    WalkDir::new(&images_source_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .for_each(|e| {
            let src = e.path().to_string_lossy().to_string();
            let dest = format!("{}/{}", images_dest_path, e.file_name().to_string_lossy());
            fs::copy(&src, &dest).expect("Failed to copy image");
        });

    // convert markdown blog files to html
    let mut posts: Vec<BlogPost> = WalkDir::new(&blog_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(BlogPost::from)
        .collect();

    // Sort posts by datetime descending (newest first)
    posts.sort_by(|a, b| match (&a.metadata.datetime, &b.metadata.datetime) {
        (Some(ad), Some(bd)) => bd.cmp(ad),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    });

    let blog_template = BlogTemplate { posts };

    let mut html_content = blog_template.render_once().unwrap();
    if let Ok(minified_html_content) = in_place_str(&mut html_content, minify_cfg) {
        fs::write(
            format!("{}/index.html", PUBLIC_SRV_PATH),
            minified_html_content,
        )
        .unwrap();
    }
}
