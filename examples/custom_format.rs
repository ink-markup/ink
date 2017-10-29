extern crate squid;

use squid::BlockParser;
use squid::ast::{Heading, HeadingLevel};
use squid::html::{Generator, Format};
use squid::html::builders::Builder;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct CustomFormat;

impl Format for CustomFormat {
    fn heading(&self, builder: &mut Builder, heading: Heading) {
        let level = match heading.level() {
            HeadingLevel::Level2 => "2",
            HeadingLevel::Level3 => "3",
            _ => "1",
        };

        builder
            .tag_start("div")
            .add_attr("class", format!("heading-level-{}", level))
            .finish()
            .text(heading.content())
            .tag_end("div");
    }
}

fn main() {
    let file = File::open("examples/demo.sq").unwrap();
    let reader = BufReader::new(&file);
    let parser = BlockParser::new(reader.lines());
    let generator = Generator::with_format(&CustomFormat, parser);

    for block in generator.take(3) {
        println!("{}", block.unwrap());
    }
}