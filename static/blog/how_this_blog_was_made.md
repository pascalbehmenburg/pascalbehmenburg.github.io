---
title: How This Blog Was Made
author: Pascal Behmenburg
date: 2025-08-02
---
## How This Blog Comes to Life { #how-this-blog-was-made }

~written on July 21, 2025 by Pascal Behmenburg~

---

### What’s the Big Idea? { #whats-the-big-idea }

I wanted a live Markdown-to-HTML server that updates on the fly, no rebuilds, no headaches. My must-haves:

- **Instant Rendering**
  : Markdown files served as HTML with handy, shareable header links
- **Dynamic Feed**
  : Auto-generated list of posts from a folder
- **Hot Reload**
  : Edit a post, refresh the page, see changes immediately

Static site generators are cool, but I craved real-time feedback. So I choose Dioxus (Rust’s React sibling) to glue it all together.
I may later on toggle on static-site generation for production, but for writing, I prefer the flexibility of a live server.

### Markdown Parsing with pulldown-cmark { #markdown-parsing-with-pulldown-cmark }
---

I chose [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) for its speed and extensibility. [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) makes it possible to intercept the parsing via an Event-driven API and modify the HTML output for headers to our liking.

```rs
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
```

### The API { #api }
---

Three server functions fetch IDs, full posts, and 3-line previews straight from a directory of markdown files:

```rs
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
```

### Dioxus Components { #dioxus-components }
---

I built three components mirroring React patterns:

- **Feed**
  : Grabs IDs, loops through previews
- **BlogPreview**
  : Shows the 3-line teaser
- **BlogPost**
  : Renders full post with dangerous_inner_html

```rs
#[component]
pub fn Feed() -> Element {
    let blog_ids = use_server_future(api::get_blog_ids)?()
        .unwrap()
        .unwrap_or_default();

    rsx! {
        div {
            {blog_ids.iter().map(|id| {
                rsx! {
                    BlogPreview { id: id },
                    hr { width: "100%", size: "10"  }
                }
            })}
        }
    }
}

#[component]
pub fn BlogPreview(id: String) -> Element {
    let div_id = format!("blog-preview-{}", id);
    let html = use_server_future(move || api::blog_preview(id.clone()))?()
        .unwrap()
        .unwrap_or_default();

    rsx! {
        div {
            class: "{div_id}",
            dangerous_inner_html: "{html}",
        },
    }
}

#[component]
pub fn Blog(id: String) -> Element {
    let div_id = format!("blog-{}", id);
    let html = use_server_future(move || api::blog(String::from(&id)))?()
        .unwrap()
        .unwrap_or_default();

    rsx! {
        div {
            id: "{div_id}",
            dangerous_inner_html: "{html}"
        }
    }
}
```

### Styling & Syntax { #styling-syntax }
---

* **No-Class CSS**: I tweaked [concrete.css](https://concrete.style/) for responsive layouts and punchy link colors
* **Syntax Highlighting**: Plugged in a subset of languages supported by [highlight.js](https://highlightjs.org/) with two Base16 themes—light or dark, your choice

That is all the magic there is to building a blog, as the one you're reading right now!
