date: May 16, 2023
author: Joe Cardenas
tags: Hopcodes Website, Rust

When creating the HopCodes website, everyone agreed we needed a blog. Given that the site is written in plain HTML/CSS, I thought it would help with consistency to be able to write blog posts in pure markdown and translate them to custom HTML pages live. So I wrote a Rust program to translate markdown files into HTML.

# Why Rust?

Rust is a high-level language, but it can be much harder to work in than other high-level languages like JavaScript/TypeScript, Swift, or Java. Since Rust does not automatic reference counting or garbage collection and also does not require manual memory management like C, you must manage memory differently. This is done through a set of rules called "Ownership" that ensures memory is always available when needed and freed when it is no longer in use. Ownership can be difficult. It requires that only one part of a program can edit a piece of memory at a time, and it can limit the flexibility of what you do.

So Rust doesn't sound so great. I assure you, though, it has its upsides. For example, it's almost impossible to have a memory leak. It's also extremely performant -- in some cases even outperforming C and other system languages. Of course, this program doesn't need to be particularly performant. It also has an easy-to-use packaging and dependency system, along with a compiler that effortlessly leads you to *real* solutions to your problems. Ultimately, I chose Rust simply because I thought it would be a good learning experience. 

# How does it work?

Essentially, the program only needs to read a list of markdown files. The files are named with their title, and the date, author, and any tags are labeled at the top of the file. After that, you can write markdown like normal. *Anyone* can edit and post on the blog. This is important -- writing blog posts could be a long and arduous process involving creating the post page, adding the post to the main blog page, updating any tag pages or author pages that also need to be updated, and, most importantly, not messing anything else up in the process. With this new program, you only need to know how to make a markdown file and run a program. Everything else is automatic.

The program first reads in the markdown files and parses them into a generic markdown tree using Rust's expressive type system. This tree can be pretty easily turned into HTML through string manipulation. Each blog gets its own instance of a rust type, `Blog`. `Blog` contains the title, author name, date, URL, tags, and content of the blog post. It then sorts them by date and puts blurbs of each post on the main page. 

Next, the program iterates through the blogs, creating a page for each one. Our site also has pages for each author, so it must also iterate through each blog and synthesize a list of authors, each with their own list of top tags. After each author's information is encoded into HTML and given their own pages, the program exits. 

It's pretty simple!

# Issues

Programming it was not as simple. Going from my last project's extensive use of JavaScript to Rust was difficult. At first, I found myself unsure of when I should be borrowing, or owning parameters to functions. But Rust's compiler helped me out, and after a lot of unneccessary memory reallocations and copies, I made it to my first working version of the program. 

Later, as I sought to extend my project with author pages, tags, and more automation, I began to become far more familiar with Rust's ownership system. I avoided unneccessary copies, ensured I borrowed when I didn't need ownership of a value, and became much more familiar with Rust's type system. 

The program is still not perfect, and it almost definitely is not as performant or clean as it could be. However, it is still a working solution, and I'm fairly proud of it.

# Next steps

In the future, I hope to ensure that the program isn't making any unnecessary copies and clones. These can make the program much slower, so avoiding them when they are unnecessary is ideal. I'd also like to implement pages for each tag, allowing users to quickly search for topics they're interested in. I'd also like to make sure I have an easy-to-use config system that allows people who *aren't me* to make changes to the HTML templates that I use for each page. This way, this website can outlast me and iterate through the many design changes that it might go through!