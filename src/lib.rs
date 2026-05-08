// SPDX-FileCopyrightText: 2025 David Lawrence Campbell
// SPDX-License-Identifier: MPL-2.0

use std::ffi::OsStr;

use anyhow::Error;
use jotdown::{
    Parser, Render,
    html::{Indentation, Renderer},
};
use log::debug;

use mdbook_preprocessor::{Preprocessor, PreprocessorContext, book::Book};

/// A Djot preprocessor.
pub struct Djot;

impl Preprocessor for Djot {
    fn name(&self) -> &'static str {
        "djot-preprocessor"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        if ctx.renderer != "html" {
            return Ok(book);
        }

        book.for_each_chapter_mut(|chapter| {
            let Some(path) = chapter.source_path.as_ref() else {
                return;
            };

            let Some(extension) = path.extension() else {
                return;
            };

            if OsStr::new("dj") == extension {
                debug!("Preprocessing {chapter}");

                let mut renderer = Renderer::indented(Indentation {
                    string: " ".repeat(4),
                    initial_level: 6,
                });

                let events = Parser::new(&chapter.content);
                let mut content = String::new();

                renderer.push_events(events, &mut content).unwrap();
                chapter.content = content.trim().to_string();
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> anyhow::Result<bool> {
        Ok(renderer == "html")
    }
}
