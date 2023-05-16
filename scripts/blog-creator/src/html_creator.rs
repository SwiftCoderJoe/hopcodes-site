use std::borrow::Borrow;

use markdown::{Block, Span, ListItem};
use crate::blog::{Blog, Author};

pub fn html_for_author(author_name: &str, author: Author, blogs: &[Blog]) -> String {

    let mut html = String::from("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Hopcodes Blog</title><link rel=\"stylesheet\" href=\"/shared/nav.css\"><link rel=\"stylesheet\" type=\"text/css\" href=\"index.css\"><link rel=\"icon\" type=\"image/x-icon\" href=\"/shared/favicon.ico\"></head><body class=\"bg-zinc-50 text-zinc-900 dark:bg-zinc-900 dark:text-zinc-50\"><!-- Mobile Nav --><nav class=\"flex flex-col md:hidden bg-maroon p-2 text-3xl\"><div class=\"flex justify-between\"><a href=\"/\" class=\"bg-contain bg-home-icon h-10 w-10\"></a><div class=\"flex gap-2\"><button id=\"nav-hamburger-button\" type=\"button\" class=\"inline-flex items-center p-2 rounded-lg\"><svg class=\"w-6 h-6\" fill=\"currentColor\" viewBox=\"0 0 20 20\" xmlns=\"http://www.w3.org/2000/svg\"><path fill-rule=\"evenodd\" d=\"M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z\" clip-rule=\"evenodd\"></path></svg></button><img class=\"lightTheme cursor-pointer h-10\" src=\"/shared/Sun-icon.png\"/><img class=\"darkTheme cursor-pointer h-10\" src=\"/shared/Moon-icon.png\"/></div></div><div class=\"hidden w-full\" id=\"navbar-collapsable\"><div class=\"flex flex-col\"><a href=\"/\" class=\"mobileNavElement\">Home</a><a href=\"#\" class=\"mobileNavElement\">Calendar</a><a href=\"#\" class=\"mobileNavElement\">Robotics</a><a href=\"/blog/\" class=\"mobileNavElement\">Blog</a></div></div></nav><!-- Desktop Nav --><nav class=\"desktopNav\"><!-- Homepage --><a href=\"/\" class=\"navElement bg-home-icon\"><div>Home</div></a><!-- Navigation Icons --><div class=\"flex gap-2\"><!-- Calendar --><a href=\"?\" class=\"navElement bg-calendar-icon\"><div>Calendar</div></a><!-- Robotics --><a href=\"?\" class=\"navElement bg-robotics-icon\"><div>Robotics</div></a><!-- Blog --><a href=\"/blog/\" class=\"navElement bg-robotics-icon\"><div>Blog</div></a><!--Light/Dark Mode--><img class=\"lightTheme cursor-pointer h-16\" src=\"/shared/Sun-icon.png\"/><img class=\"darkTheme cursor-pointer h-16\" src=\"/shared/Moon-icon.png\"/></div></nav><header class=\"flex md:flex-row flex-col justify-center items-stretch gap-4\">");

    html.push_str(&author_name);

    html.push_str("<div class=\"h-0.5 md:w-0.5 md:h-auto dark:bg-zinc-900 bg-zinc-50\"></div><div class=\"flex flex-col text-lg justify-center text-left\"><div>Top Tags</div><div>");
    
    let mut tag_iter = author.top_tags.iter();
    // First tag
    html.push_str(tag_iter.next().expect("This author has no tags."));
    // The rest of the tags
    for tag in tag_iter {
        html.push_str(", ");
        html.push_str(&tag);
    }

    html.push_str("</div></div></header>");

    let filtered_blogs: Vec<&Blog> = blogs.into_iter().filter(|blog| blog.author.eq(author_name)).collect();

    html.push_str(
        &html_for_blog_blurbs(
            &filtered_blogs
        )
    );

    html.push_str("<script src=\"/shared/darkMode.js\"></script><script src=\"/shared/menu.js\"></script></body></html>");

    return html;
}

pub fn html_for_main_page(blog_entries: &Vec<Blog>) -> String {
    // Head, nav, and header
    let mut html = String::from("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Hopcodes Blog</title><link rel=\"stylesheet\" href=\"/shared/nav.css\"><link rel=\"stylesheet\" type=\"text/css\" href=\"index.css\"><link rel=\"icon\" type=\"image/x-icon\" href=\"/shared/favicon.ico\"></head><body class=\"bg-zinc-50 text-zinc-900 dark:bg-zinc-900 dark:text-zinc-50\"><!-- Mobile Nav --><nav class=\"flex flex-col md:hidden bg-maroon p-2 lg:text-3xl\"><div class=\"flex justify-between\"><a href=\"/\" class=\"bg-contain bg-home-icon h-10 w-10\"></a><div class=\"flex gap-2\"><button id=\"nav-hamburger-button\" type=\"button\" class=\"inline-flex items-center p-2 rounded-lg\"><svg class=\"w-6 h-6\" fill=\"currentColor\" viewBox=\"0 0 20 20\" xmlns=\"http://www.w3.org/2000/svg\"><path fill-rule=\"evenodd\" d=\"M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z\" clip-rule=\"evenodd\"></path></svg></button><img class=\"lightTheme cursor-pointer h-10\" src=\"/shared/Sun-icon.png\"/><img class=\"darkTheme cursor-pointer h-10\" src=\"/shared/Moon-icon.png\"/></div></div><div class=\"hidden w-full\" id=\"navbar-collapsable\"><div class=\"flex flex-col\"><a href=\"/\" class=\"mobileNavElement\">Home</a><a href=\"#\" class=\"mobileNavElement\">Calendar</a><a href=\"#\" class=\"mobileNavElement\">Robotics</a><a href=\"/blog/\" class=\"mobileNavElement\">Blog</a></div></div></nav><!-- Desktop Nav --><nav class=\"desktopNav\"><!-- Homepage --><a href=\"/\" class=\"navElement bg-home-icon\"><div>Home</div></a><!-- Navigation Icons --><div class=\"flex gap-2\"><!-- Calendar --><a href=\"?\" class=\"navElement bg-calendar-icon\"><div>Calendar</div></a><!-- Robotics --><a href=\"?\" class=\"navElement bg-robotics-icon\"><div>Robotics</div></a><!-- Blog --><a href=\"/blog/\" class=\"navElement bg-robotics-icon\"><div>Blog</div></a><!--Light/Dark Mode--><img class=\"lightTheme cursor-pointer h-16\" src=\"/shared/Sun-icon.png\"/><img class=\"darkTheme cursor-pointer h-16\" src=\"/shared/Moon-icon.png\"/></div></nav><header>Hopcodes Blog</header>");

    html.push_str(&html_for_blog_blurbs(blog_entries));

    html.push_str("<script src=\"/shared/darkMode.js\"></script><script src=\"/shared/menu.js\"></script></body></html>");

    return html
}

fn html_for_blog_blurbs<T: Borrow<Blog>>(blog_entries: &[T]) -> String {
    let mut html = String::from("<div class=\"flex flex-col items-center gap-5\">");

    // Add blurbs for each blog
    for blog in blog_entries {

        let blog = blog.borrow();

        // Since blog entries are already sorted, we can use their enumerated position for URLs
        html.push_str("<article class=\"w-3/4 border-t-2\"><div class=\"flex flex-col md:flex-row md:items-end py-2 gap-2\"><h2 class=\"text-2xl md:text-4xl\"><a href=\"/blog/");
        html.push_str(&blog.url);
        html.push_str("/\" class=\"hover:underline\">");
        html.push_str(&blog.title);
        html.push_str("</a></h2><p>");
        html.push_str(&blog.date_descriptor);
        html.push_str(" - <a href=\"/blog/authors/");
        html.push_str(&blog.author.replace(" ", "_"));
        html.push_str("/\" class=\"hover:underline text-sky-700\">");
        html.push_str(&blog.author);
        html.push_str("</a></p></div><p>");
        match blog.content.first().expect("There wasn't a first paragraph.") {
            Block::Paragraph(vec) => html.push_str(&parse_paragraph(&vec)),
            _ => { }
        }
        html.push_str("</p><a href=\"/blog/");
        html.push_str(&blog.url);
        html.push_str("/\" class=\"hover:underline text-sky-700\">Read more...</a></article>");
    }

    html.push_str("</div>");

    return html
}

pub fn html_for_article(blog: &Blog) -> String {
    // Head and nav
    let mut html = String::from("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Hopcodes Blog</title><link rel=\"stylesheet\" href=\"/shared/nav.css\"><link rel=\"stylesheet\" type=\"text/css\" href=\"index.css\"><link rel=\"icon\" type=\"image/x-icon\" href=\"/shared/favicon.ico\"></head><body class=\"bg-zinc-50 text-zinc-900 dark:bg-zinc-900 dark:text-zinc-50\"><!-- Mobile Nav --><nav class=\"flex flex-col md:hidden bg-maroon p-2 text-3xl\"><div class=\"flex justify-between\"><a href=\"/\" class=\"bg-contain bg-home-icon h-10 w-10\"></a><div class=\"flex gap-2\"><button id=\"nav-hamburger-button\" type=\"button\" class=\"inline-flex items-center p-2 rounded-lg\"><svg class=\"w-6 h-6\" fill=\"currentColor\" viewBox=\"0 0 20 20\" xmlns=\"http://www.w3.org/2000/svg\"><path fill-rule=\"evenodd\" d=\"M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z\" clip-rule=\"evenodd\"></path></svg></button><img class=\"lightTheme cursor-pointer h-10\" src=\"/shared/Sun-icon.png\"/><img class=\"darkTheme cursor-pointer h-10\" src=\"/shared/Moon-icon.png\"/></div></div><div class=\"hidden w-full\" id=\"navbar-collapsable\"><div class=\"flex flex-col\"><a href=\"/\" class=\"mobileNavElement\">Home</a><a href=\"#\" class=\"mobileNavElement\">Calendar</a><a href=\"#\" class=\"mobileNavElement\">Robotics</a><a href=\"/blog/\" class=\"mobileNavElement\">Blog</a></div></div></nav><!-- Desktop Nav --><nav class=\"desktopNav\"><!-- Homepage --><a href=\"/\" class=\"navElement bg-home-icon\"><div>Home</div></a><!-- Navigation Icons --><div class=\"flex gap-2\"><!-- Calendar --><a href=\"?\" class=\"navElement bg-calendar-icon\"><div>Calendar</div></a><!-- Robotics --><a href=\"?\" class=\"navElement bg-robotics-icon\"><div>Robotics</div></a><!-- Blog --><a href=\"/blog/\" class=\"navElement bg-robotics-icon\"><div>Blog</div></a><!--Light/Dark Mode--><img class=\"lightTheme cursor-pointer h-16\" src=\"/shared/Sun-icon.png\"/><img class=\"darkTheme cursor-pointer h-16\" src=\"/shared/Moon-icon.png\"/></div></nav>");
    
    // Title bar
    html.push_str("<header class=\"flex md:flex-row flex-col justify-center items-stretch gap-4\">");
    html.push_str(&blog.title);
    html.push_str("<div class=\"h-0.5 md:w-0.5 md:h-auto dark:bg-zinc-900 bg-zinc-50\"></div><div class=\"flex flex-col text-lg justify-center text-left\"><a href=\"/blog/authors/joecardenas/\" class=\"hover:underline text-sky-700\">");
    html.push_str(&blog.author);
    html.push_str("</a><div>");
    html.push_str(&blog.date_descriptor);
    html.push_str("</div></div></header>");

    // Content
    html.push_str("<article class=\"w-3/4 mx-auto\">");
    for item in blog.content.iter() {
        match item {
            Block::Header(vec, size) => html.push_str(&parse_header(vec, *size)),
            Block::Paragraph(vec) => html.push_str(&parse_paragraph(vec)),
            Block::Blockquote(_vec) => todo!(),
            Block::CodeBlock(_option, _string) => todo!(),
            Block::OrderedList(_vec, _list_type) => todo!(),
            Block::UnorderedList(vec) => html.push_str(&parse_unordered_list(vec)),
            Block::Raw(_string) => todo!(),
            Block::Hr => todo!(),
        }
    }
    html.push_str("</article><script src=\"/shared/darkMode.js\"></script><script src=\"/shared/menu.js\"></script></body></html>");
    return html
}

fn parse_header(vec: &Vec<Span>, _size: usize) -> String {
    let mut header = String::from("<h2 class=\"text-2xl mb-2\">");
    header.push_str(&parse_bare_span_list(vec));
    header.push_str("</h2>");
    return header
}

fn parse_paragraph(vec: &Vec<Span>) -> String {
    let mut paragraph = String::from("<p class=\"mb-2\">");
    paragraph.push_str(&parse_bare_span_list(vec));
    paragraph.push_str("</p>");
    return paragraph
}

fn parse_unordered_list(vec: &Vec<ListItem>) -> String {
    let mut list = String::from("<ul class=\"mb-2 list-disc list-inside\">");
    for list_item in vec {
        match list_item {
            ListItem::Simple(vec) => {
                for item in vec {
                    list.push_str("<li>");
                    list.push_str(&parse_span(item));
                    list.push_str("</li>");
                }
            },
            ListItem::Paragraph(_) => todo!(),
        }
    }
    list.push_str("</ul>");
    return list
}

/// Starts with a space
fn parse_bare_span_list(vec: &Vec<Span>) -> String {
    let mut html = String::new();
    for span in vec {
        html.push_str(&parse_span(span));
    }
    return html
}

fn parse_span(span: &Span) -> String {
    match span {
        Span::Break => String::from("<br>"),
        Span::Text(str) => str.to_owned(),
        Span::Code(str) => {
            let mut html = String::from("<span class=\"rounded p-1 bg-zinc-600 text-zinc-50\">");
            html.push_str(str);
            html.push_str("</span>");
            return html;
        },
        Span::Link(_, _, _) => todo!(),
        Span::Image(_, _, _) => todo!(),
        Span::Emphasis(spans) => {
            let mut html = String::from("<em>");
            html.push_str(&parse_bare_span_list(spans));
            html.push_str("</em>");
            return html
        },
        Span::Strong(_) => todo!(),
    }
}