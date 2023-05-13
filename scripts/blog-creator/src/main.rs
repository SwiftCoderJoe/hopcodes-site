use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::{self};
use std::str::FromStr;
use html_creator::html_for_main_page;
use markdown::{self, Block};
use chrono::{NaiveDate};

use crate::html_creator::html_for_blog;

mod html_creator;

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

    blog_entries.sort_by(|a, b| a.date.cmp(&b.date));

    // Create main page
    println!("{}", html_for_main_page(&blog_entries));
    
    for (i, blog) in blog_entries.iter().enumerate() {
        // Create blog pages
        fs::write(format!("../../src/blog/post{}/index.html", i), html_for_blog(blog))
            .expect("Couldn't write blog file.");


        // print!("{}th blog, date: {}, with tags: ", i, blog.date_descriptor);
        // for tag in &blog.tags {
        //     print!("{}, ", tag);
        // }
        // print!("\n");
    }
}

pub struct Blog {
    title: String,
    author: String,
    date_descriptor: String,
    date: NaiveDate,
    tags: Vec<String>,
    content: Vec<Block>,
}

pub struct Author {
    name: String,
    tags: HashMap<String, u16>
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
    let date_time = NaiveDate::parse_from_str(date, "%B %d, %Y").unwrap();

    return Some(Blog { 
        title: title.to_owned(), 
        author: author.to_owned(), 
        date_descriptor: date.to_owned(), 
        date: date_time,
        tags: tags,
        content: content
    })
}