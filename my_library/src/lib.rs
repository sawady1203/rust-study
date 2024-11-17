mod library {
    mod book{}
    mod bookshelf{}
}

mod book{
    pub struct Book{
        title: String,
        author: String,
    }
    impl Book {
        fn new(title: &str, author: &str) -> Self {
            Self {
                title: title.to_string(),
                author: author.to_string()
            }
        }
    }
}

mod Bookshelf {
    pub struct Bookshelf{
        books: Vec<super::book::Book>,
    }
    impl Bookshelf {
        fn new() -> Self {

            Self {books: Vec::new()}
        }

        // 本を追加するメソッド

        // タイトルで本を検索するメソッド

        // 本を本棚から取り出すメソッド

        // 本棚の本をすべて取り出すメソッド
    }
}