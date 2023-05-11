use std::ffi::OsString;
use std::fs::{self};
use markdown::{self, Block};

mod html_creator;

fn main() {

    let mut blog_entries: Vec<Blog> = Vec::new();

    for file in fs::read_dir("../../blog/").expect("There was an error reading the folder.") {
        let file = file.expect("There was an error reading a file.");
        blog_entries.push(
            parse_blog(
                fs::read_to_string(file.path()).expect("There was an error reading a file into a string."),
                file.file_name()
            ).unwrap()
        )
    }
    
    for (i, blog) in blog_entries.iter().enumerate() {
        println!("Blog entry {}:", i);
        println!("  title: {}", blog.title);
        println!("  date: {}", blog.date);
        println!("  author: {}", blog.author);
        println!("  content:");
        println!("{}", html_creator::generate_html(blog));
    }
}

pub struct Blog {
    title: String,
    author: String,
    date: String,
    content: Vec<Block>,
}

fn parse_blog(content: String, filename: OsString) -> Option<Blog> {
    let title = &filename.to_str().expect("Couldn't turn filename into a string.")[0..(filename.len() - 3)];
    let mut date: &str = "";
    let mut author: &str = "";

    let (first_paragraph, unparsed_content) = content.split_once("\n\n")?;
    let mut first_paragraph_split = first_paragraph.split("\n");
    loop {
        println!("BRUH!");
        let Some(line) = first_paragraph_split.next() else {
            break;
        };
        if line.starts_with("date: ") {
            date = &line[6..line.len()];
        }
        if line.starts_with("author: ") {
            author = &line[8..line.len()];
        }
        // Break after the first "paragraph"
    }

    let content = markdown::tokenize(unparsed_content);

    return Some(Blog { 
        title: title.to_owned(), 
        author: author.to_owned(), 
        date: date.to_owned(), 
        content: content
    })
}