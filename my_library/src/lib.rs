mod library {
    mod book {
        pub struct Page {
            pub content: String,
        }
    }
    mod bookshelf {}
    pub mod magazine {
        pub struct Page {
            pub content: String,
        }
    }
}

mod book {
    pub struct Book {
        title: String,
        author: String,
    }
    impl Book {
        fn new(title: &str, author: &str) -> Self {
            Self {
                title: title.to_string(),
                author: author.to_string(),
            }
        }
    }
}

mod bookshelf {
    use crate::library::magazine;

    use super::book::Book;
    use super::library::magazine::Page;

    fn some_function() {
        Page {
            content: "Hello".to_string(),
        }
    }

    pub struct Bookshelf {
        books: Vec<Book>,
    }
    impl Bookshelf {
        fn new() -> Self {
            Self { books: Vec::new() }
        }

        // 本を追加するメソッド
        pub fn add_book(&mut self, book: Book) {
            self.books.push(book);
        }

        // タイトルで本を検索するメソッド
        pub fn search_books(&self, title_query: &str) -> Vec<&Book> {
            todo!("Implement `Bookshelf::search_books`");
        }

        // 本を本棚から取り出すメソッド
        pub fn remove_book(&mut self, book: &Book) -> Option<Book> {
            todo!("Implement `Bookshelf::remove_book`");
        }

        // 本棚の本をすべて取り出すメソッド
        pub fn take_all_books(&mut self) -> Vec<Book> {
            todo!("Implement `Bookshelf::take_all_books`");
        }
    }
}
