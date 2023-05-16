use std::ffi::OsString;
use std::fs;
use markdown;

mod blog;
use blog::{Blog, build_blog, create_author_list};

mod html_creator;
use html_creator::{html_for_article, html_for_main_page, html_for_author};

const TAILWIND_CSS: &str = "@tailwind base;@tailwind components;@tailwind utilities;";

fn main() {

    let mut blog_entries: Vec<Blog> = Vec::new();

    for file in fs::read_dir("../../blog/").expect("There was an error reading the folder.") {
        let file = file.expect("There was an error reading a file.");
        let path = file.path();
        if path.is_dir() { return }
        blog_entries.push(
            parse_blog(
                fs::read_to_string(path).expect("There was an error reading a file into a string."),
                file.file_name()
            ).unwrap()
        )
    }

    blog_entries.sort_by(|a, b| b.date.cmp(&a.date));

    // Remove the existing blog dir
    // This not only removes an entire directory but it also fails silently. EXTREMELY DANGEROUS!
    let _ = fs::remove_dir_all("../../src/blog/");
    fs::create_dir_all("../../src/blog").expect("Couldn't create blog folder.");

    // Create main page
    fs::write("../../src/blog/index.html", html_for_main_page(&blog_entries))
        .expect("Couldn't write blog homepage.");
    fs::write("../../src/blog/index.css", TAILWIND_CSS)
        .expect("Couldn't write main page CSS.");
    
    // Create blog pages
    for blog in blog_entries.iter() {
        fs::create_dir_all(format!("../../src/blog/{}", blog.url)).expect("Couldn't create article folder.");
        fs::write(format!("../../src/blog/{}/index.html", blog.url), html_for_article(blog))
            .expect("Couldn't write blog file.");
        fs::write(format!("../../src/blog/{}/index.css", blog.url), TAILWIND_CSS)
            .expect("Couldn't write article CSS.");
    }

    // Create author pages
    let authors = create_author_list(&blog_entries);

    for (author_name, author) in authors {
        let author_url = author_name.replace(" ", "_");
        fs::create_dir_all(format!("../../src/blog/authors/{}", author_url)).expect("Couldn't create author foler.");
        fs::write(format!("../../src/blog/authors/{}/index.html", author_url), html_for_author(&author_name, author, &blog_entries))
            .expect("Couldn't write blog file.");
        fs::write(format!("../../src/blog/authors/{}/index.css", author_url), TAILWIND_CSS)
            .expect("Couldn't write article CSS.");
    }
}

fn parse_blog(content: String, filename: OsString) -> Option<Blog> {
    let title = &filename.to_str().expect("Couldn't turn filename into a string.")[0..(filename.len() - 3)];
    let mut date: &str = "";
    let mut author: &str = "";
    let mut tags: Vec<String> = Vec::new();

    let (first_paragraph, unparsed_content) = content.split_once("\n\n")?;
    let mut first_paragraph_split = first_paragraph.split("\n");
    loop {
        // Break after the first "paragraph"
        let Some(line) = first_paragraph_split.next() else {
            break;
        };
        if line.starts_with("date: ") {
            date = &line[6..line.len()];
        } else if line.starts_with("author: ") {
            author = &line[8..line.len()];
        } else if line.starts_with("tags: ") {
            tags = line[6..line.len()].split(", ").map(String::from).collect();
        }
    }

    let content = markdown::tokenize(unparsed_content);

    return Some(build_blog(
        title.to_owned(),
        author.to_owned(),
        date.to_owned(),
        tags,
        content
    ))
}