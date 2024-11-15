mod library{
    mod book{}
    mod bookshelf{}
}

mod book {
    pub struct Book{
        title: String,
        author: String,
    }
    impl Book {
        fn new(title: &str, author: &str) -> Self {
            Self { title: title.to_string(), author: author.to_string() }
        }
    }
}

mod bookshelf{
    struct Bookshelf{
        books: Vec<super::book::Book>,
    }
    impl Bookshelf {
        pub fn new() -> Self {
            Self {books: Vec::new()}
        }

        // 本を追加するメソッド
        pub fn add_book(&mut self, book:super::book::Book) {
            self.books.push(book)
        }

        // タイトルで本を検索するメソッド
        pub fn search_book(
            &self,
            title_query: &str
        ) -> Vec<&super::book::Book>{
            todo!("Implement `Bookshelf::search_books`");
        }

        // 本を本棚から取り出すメソッド
        pub fn remove_book(
            &mut self, 
            book: super::book::Book
        ) -> Option<super::book::Book>{
            todo!("Implement `Bookshelf::remove_book`");
        }

        // 本棚の本をすべて取り出すメソッド
        pub fn take_all_books(&mut self) -> Vec<super::book::Book> {
            todo!("Implement `Bookshelf::take_all_books`");            
        }
    }
}
