use std::ffi::OsStr;

use anyhow::Error;
use jotdown::{
    html::{Indentation, Renderer},
    Parser, Render,
};
use mdbook::{
    book::Book,
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
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
        Self {
            renderer,
        }
    }
}

impl Default for Djot {
    fn default() -> Self {
        Self::new()
    }
}

impl Preprocessor for Djot {
    fn name(&self) -> &str {
        "djot-preprocessor"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        if let Some(_cfg) = ctx.config.get_preprocessor(self.name()) {
            //
        }

        book.for_each_mut(|item| {
            match item {
                BookItem::Chapter(chapter) => {
                    let path = match chapter.source_path.as_ref() {
                        Some(path) => path,
                        None => return,
                    };
                    let extension = match path.extension() {
                        Some(extension) => extension,
                        None => return,
                    };
                    if OsStr::new("dj") == extension {
                        let events = Parser::new(&chapter.content);
                        let mut content = String::new();
                        self.renderer.push(events, &mut content).unwrap();
                        let content_stripped = content.trim().to_string();
                        chapter.content = content_stripped;
                    }
                }
                BookItem::Separator | BookItem::PartTitle(_) => return,
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "markdown"
    }
}

