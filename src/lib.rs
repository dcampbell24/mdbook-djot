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

impl Preprocessor for Djot {
    fn name(&self) -> &str {
        "djot-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, book: Book) -> Result<Book, Error> {
        let mut book_copy = book.clone();
        for item in book.sections {
            match item {
                BookItem::Chapter(mut chapter) => {
                    if OsStr::new("dj")
                        == chapter
                            .source_path
                            .as_ref()
                            .expect("Didn't find a source path!")
                            .extension()
                            .expect("source path doesn't have an extension!")
                    {
                        let events = Parser::new(&chapter.content);
                        let mut content = String::new();
                        self.renderer.push(events, &mut content)?;
                        let content_stripped = content.trim().to_string();
                        chapter.content = content_stripped;
                        book_copy.sections.push(BookItem::Chapter(chapter));
                    }
                }
                item @ (BookItem::Separator | BookItem::PartTitle(_)) => {
                    book_copy.sections.push(item);
                }
            }
        }

        Ok(book_copy)
    }

    fn supports_renderer(&self, _renderer: &str) -> bool {
        true
    }
}

