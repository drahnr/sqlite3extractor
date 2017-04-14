extern crate rusqlite;
extern crate time;

use time::Timespec;
use rusqlite::{Connection,OpenFlags};

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
struct Post {
//
//0|id|INTEGER|1||1
//1|title|VARCHAR(80)|0||0
//2|body|TEXT|0||0
//3|date|DATETIME|0||0
//4|slug|VARCHAR(30)|0||0
//5|author_id|INTEGER|0||0
//6|category_id|INTEGER|0||0

    id: i32,
    title: String,
    body: String,
    date: Timespec,
    slug: String,
}

fn write_out(post : Post) {
	static BASE : &'static str =  "/tmp";
	let date = time::at(post.date);
	let path = format!("{}/{}__{}.md", BASE, date.strftime("%Y%m%d_%H%M%S").unwrap(), post.slug);
	let path = Path::new(path.as_str());
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => {panic!("couldn't create {}: {}",
                           display,
                           why.description()); },
        Ok(file) => file,
    };

	let content : String = format!("#{}\n{}", post.title, post.body);

    match file.write_all(content.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               why.description());
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn main() {
	let path = Path::new("/tmp/bottleship.db");
    let conn = Connection::open_with_flags(path, rusqlite::SQLITE_OPEN_READ_ONLY).unwrap();
    let mut stmt = conn.prepare("SELECT id, title, date, body, slug FROM post").unwrap();
    let post_iter = stmt.query_map(&[], |row| {
        Post {
            id: row.get(0),
            title: row.get(1),
            date: row.get(2),
            body: row.get(3),
            slug: row.get(4)
        }
    }).unwrap();

    for post in post_iter {
		write_out(post.unwrap());
    }
}
