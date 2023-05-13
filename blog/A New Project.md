date: May 10, 2023
author: Joe Cardenas
tags: hopcodes website, tailwind, web

Ever since I originally joined HopCodes, I have wanted to create a real, collaborative project. This year, this project comes in the form of a website. I hope that this website will be easily kept up-to-date and it can host lots of interesting information for all HopCodes members, and it also serves as a learning experience for self-taught programmers. Self-taught programmers often have trouble working with other programmers in larger projects, so I've decided to run this project's repository in the same way larger projects are often run. This way, club members can get experience with reading others' code, pull requests, and code review. 

# The Website's Architecture

The HopCodes website was intended as both a learning experience and as something that any programmer, regardless of experience, can contribute to. To balance these two goals, we decided to go with a mostly plain HTML/CSS technology stack with two additions: Tailwind (for easy styling) and Vite (for easy packaging and live server). Adding Tailwind and Vite should not only make development easier, but it also acts as a great experience to learn new technologies.

When we're ready to release the website, it will likely be hosted on my personal server along with my other websites. We will probably package it as a docker container using Caddy.

# Functionality

Here are the things we're hoping to get in the website:
 - Notifications
 - Calendar
 - News & Blog
 - Language guides

The most important things for us to build are the calendar, blog, and some simple guides. Once those things are completed to an acceptable level, we'll be ready to make it publicly available.

# Current Progress

As of May 10, 2023, progress has been steady. I have been focusing on the navbar, mobile support, and the blog. Sophia has been working on implementing a dark mode and the landing page, while Nick has been working on the calendar page. 