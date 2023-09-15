// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::io;
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::{anyhow, Result};
use clap::{App, Arg, SubCommand};
use log::*;
use mdbook::book::BookItem;
use mdbook::preprocess::CmdPreprocessor;
use pulldown_cmark::{Event, Parser, Tag};
use pulldown_cmark_to_cmark::cmark;

/// The Vulkan API Registry index.
const INDEX: &str = include_str!("../../../index.txt");
/// The version of `vulkanalia` used by the tutorial.
const VERSION: &str = "0.22.0";

/// The number of documentation link replacements made.
static REPLACEMENTS: AtomicUsize = AtomicUsize::new(0);

#[rustfmt::skip]
pub fn app() -> App<'static, 'static> {
    App::new("vk-preprocessor")
        .about("An mdbook preprocessor for the tutorial book.")
        .subcommand(SubCommand::with_name("supports")
            .arg(Arg::with_name("renderer").required(true))
            .about("Check whether a renderer is supported by this preprocessor"))
}

fn main() -> Result<()> {
    pretty_env_logger::init();

    // Check renderer support.
    if let Some(args) = app().get_matches().subcommand_matches("supports") {
        let renderer = args.value_of("renderer").unwrap();
        process::exit(if renderer == "html" { 0 } else { 1 });
    }

    // Parse the book and check version compatibility.
    let (context, mut book) = CmdPreprocessor::parse_input(io::stdin())?;
    if context.mdbook_version != mdbook::MDBOOK_VERSION {
        return Err(anyhow!(
            "Preprocessor build with mdbook {}, called with mdbook {}.",
            mdbook::MDBOOK_VERSION,
            context.mdbook_version
        ));
    }

    // Preprocess the book.
    let index = load_index();
    book.for_each_mut(|i| preprocess_item(i, &index).unwrap());
    serde_json::to_writer(io::stdout(), &book)?;

    let replacements = REPLACEMENTS.load(Ordering::Relaxed);
    info!("Made {} documentation link replacements.", replacements);

    Ok(())
}

#[rustfmt::skip]
fn load_index() -> HashMap<&'static str, &'static str> {
    let mut index = INDEX
        .lines()
        .map(|l| {
            let mut tokens = l.split('\t');
            let name = tokens.next().unwrap();
            let path = tokens.next().unwrap();
            (name, path)
        })
        .collect::<HashMap<_, _>>();

    info!("Loaded index has {} entries.", index.len());

    // Add entries for non-generated items.
    index.insert("Bytecode", "https://docs.rs/vulkanalia/%VERSION%/vulkanalia/bytecode/struct.Bytecode.html");
    index.insert("Device", "https://docs.rs/vulkanalia/%VERSION%/vulkanalia/struct.Device.html");
    index.insert("Entry", "https://docs.rs/vulkanalia/%VERSION%/vulkanalia/struct.Entry.html");
    index.insert("Instance", "https://docs.rs/vulkanalia/%VERSION%/vulkanalia/struct.Instance.html");
    index.insert("vk_window::create_surface", "https://docs.rs/vulkanalia/%VERSION%/vulkanalia/window/fn.create_surface.html");
    index.insert("vk_window::get_required_instance_extensions", "https://docs.rs/vulkanalia/%VERSION%/vulkanalia/window/fn.get_required_instance_extensions.html");

    index
}

fn preprocess_item(item: &mut BookItem, index: &HashMap<&str, &str>) -> Result<()> {
    if let BookItem::Chapter(ref mut chapter) = item {
        let parser = Parser::new(&chapter.content);
        let events = parser.into_iter().map(|e| map_event(e, index));
        let mut buffer = String::with_capacity(chapter.content.len() * 2);
        cmark(events, &mut buffer, None)?;
        chapter.content = buffer;
    }

    Ok(())
}

fn map_event<'e>(event: Event<'e>, index: &HashMap<&str, &str>) -> Event<'e> {
    if let Event::Code(code) = &event {
        if let Some(url) = index.get(&code[..]) {
            REPLACEMENTS.fetch_add(1, Ordering::Relaxed);
            let url = url.replace("%VERSION%", VERSION);
            Event::Html(format!("<a href=\"{}\"><code class=\"hljs\">{}</code></a>", url, code).into())
        } else if let Some(code) = code.strip_prefix('^') {
            Event::Code(code.to_string().into())
        } else {
            event
        }
    } else if let Event::End(Tag::Link(ltype, url, title)) = &event {
        let url = url.replace("%VERSION%", VERSION).into();
        Event::End(Tag::Link(*ltype, url, title.clone()))
    } else {
        event
    }
}
