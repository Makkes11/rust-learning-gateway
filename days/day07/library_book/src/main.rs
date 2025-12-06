fn main() {
    let b = Book {
        title: "Rust".to_string(),
        pages: 500,
    };
    let p1 = borrow_book(&b);
    assert_eq!(p1, 500);
    println!("{}", b.title);
    let p = take_ownership(b);
    assert_eq!(p, 500);
    // let p1 = take_ownership(b);

    let b1 = Book {
        title: "A".into(),
        pages: 10,
    };
    let b2 = Book {
        title: "B".into(),
        pages: 20,
    };
    let lib = make_library(&b1, &b2);
    assert_eq!(lib.book1.pages, 10);
    assert_eq!(lib.book2.title, "B");

    let mut b3 = Book {
        title: "Test".into(),
        pages: 100,
    };

    add_pages_existing_book(&mut b3, 50);
    assert_eq!(b3.pages, 150);
    let b4 = add_pages(b3, 50);
    assert_eq!(b4.pages, 200);
}

struct Library<'a> {
    book1: &'a Book,
    book2: &'a Book,
}

struct Book {
    title: String,
    pages: usize,
}

fn take_ownership(book: Book) -> usize {
    book.pages
}

fn borrow_book(book: &Book) -> usize {
    book.pages
}

fn make_library<'a>(b1: &'a Book, b2: &'a Book) -> Library<'a> {
    Library {
        book1: b1,
        book2: b2,
    }
}

fn add_pages(mut book: Book, extra: usize) -> Book {
    book.pages += extra;
    book
}

fn add_pages_existing_book(book: &mut Book, extra: usize) {
    book.pages += extra;
}
