use std::ffi::OsStr;

use anyhow::Error;
use jotdown::{
    Parser, Render,
    html::{Indentation, Renderer},
};
use log::debug;

use mdbook_preprocessor::{
    Preprocessor, PreprocessorContext,
    book::{Book, BookItem},
};

/// A Djot preprocessor.
pub struct Djot {
    renderer: Renderer,
}

impl Djot {
    #[must_use]
    pub fn new() -> Djot {
        let renderer = Renderer::indented(Indentation {
            string: " ".repeat(4),
            initial_level: 6,
        });
        Self { renderer }
    }
}

impl Default for Djot {
    fn default() -> Self {
        Self::new()
    }
}

impl Preprocessor for Djot {
    fn name(&self) -> &'static str {
        "djot-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        /* Fixme:
        if let Ok(cfg) = ctx.config.get(self.name()) {
            //
        }
        */

        book.for_each_chapter_mut(|chapter| {
            let Some(path) = chapter.source_path.as_ref() else {
                return;
            };

            let Some(extension) = path.extension() else {
                return;
            };

            if OsStr::new("dj") == extension {
                debug!("Preprocessing {chapter}");
                let events = Parser::new(&chapter.content);
                let mut content = String::new();
                self.renderer.push(events, &mut content).unwrap();
                let content_stripped = content.trim().to_string();
                chapter.content = content_stripped;
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> anyhow::Result<bool> {
        Ok(renderer != "markdown")
    }
}
