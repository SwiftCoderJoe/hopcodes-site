use chrono::NaiveDate;
use markdown::Block;
use std::{collections::HashMap, cmp::min};

const NUMBER_OF_TAGS: usize = 3;

pub fn build_blog(title: String, author: String, date: String, tags: Vec<String>, content: Vec<Block>) -> Blog {
    let mut url = date.to_owned().replace(",", "").replace(" ", "-");

    url.push('-');
    url.push_str(
        &author.replace(" ", "_")
    );
    
    Blog {
        title,
        author,
        date_descriptor: date.to_owned(),
        date: NaiveDate::parse_from_str(&date, "%B %d, %Y").unwrap(),
        tags,
        url,
        content
    }
}

pub fn create_author_list(blogs: &Vec<Blog>) -> HashMap<String, Author> {
    let mut tagless_author_map: HashMap<String, TaglessAuthor> = HashMap::new();

    for blog in blogs {
        // Create an Author if it doesn't already exist
        if !tagless_author_map.contains_key(&blog.author) {
            tagless_author_map.insert(blog.author.to_owned(), TaglessAuthor {
                name: blog.author.to_owned(),
                tags: HashMap::new()
            });
        }
        
        // TODO: Refactor into result and get rid of this expect, OR consider the [] getter, idk if its mutable
        iterate_tags(tagless_author_map.get_mut(&blog.author).expect("FIX!"), &blog.tags)
    }

    let mut author_map: HashMap<String, Author> = HashMap::new();

    for (key, author) in tagless_author_map {
        author_map.insert(key, author_from_tagless_author(author));
    }

    return author_map;

}


/// MUTATES the Author struct that is passed in
fn iterate_tags(author: &mut TaglessAuthor, tags: &Vec<String>) {
    for tag in tags {
        // Create a spot for the tag if it doesn't already exist
        // Refactor to .entry()
        if !author.tags.contains_key(tag) {
            author.tags.insert(tag.to_owned(), 1);
        } else {
            let tag_count_ref = author.tags.get_mut(tag).expect("impossible");
            *tag_count_ref = *tag_count_ref + 1
        }
    }
}

// LOTS of string copies happening here... fix later?
fn author_from_tagless_author(author: TaglessAuthor) -> Author {
    let binding = author.tags.to_owned();
    let mut raw_tags: Vec<(&String, &u16)> = binding.iter().map(|(id, count)| (id, count)).collect();
    raw_tags.sort_by(|(_, a), (_, b)| b.cmp(a));

    return Author { 
        name: author.name, 
        tags: author.tags, 
        top_tags: raw_tags[0..min(raw_tags.len(), 3)].iter().map(|(tag, _)| tag.to_owned().to_owned()).collect()
    };
}

pub struct Blog {
    pub title: String,
    pub author: String,
    pub date_descriptor: String,
    pub date: NaiveDate,
    pub tags: Vec<String>,
    pub url: String,
    pub content: Vec<Block>,
}

pub struct Author {
    name: String,
    tags: HashMap<String, u16>,
    top_tags: Vec<String>
}
struct TaglessAuthor {
    name: String,
    tags: HashMap<String, u16>
}