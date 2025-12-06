use std::borrow::Cow;

fn main() {
    let message = Message::Borrowed("abc");
    assert_eq!(msg_len(&message), 3);

    let message_1 = Message::Owned("abcde".to_string());
    assert_eq!(msg_len(&message_1), 5);

    let h = emphasize("Hello");
    assert_eq!(h.text, "Hello");
    assert_eq!(h.level, 1);

    assert!(matches!(maybe_uppercase("HELLO"), Cow::Borrowed(_)));
    assert!(matches!(maybe_uppercase("Hello"), Cow::Owned(_)));
    assert_eq!(maybe_uppercase("Hello"), "HELLO");

    print_it(&"hello");
    print_it(&"test".to_string());
    print_it(&maybe_uppercase("Hello"));
}

// 8.1
enum Message<'a> {
    Borrowed(&'a str),
    Owned(String),
}

fn msg_len<'a>(msg: &'a Message<'a>) -> usize {
    match msg {
        Message::Borrowed(b) => (*b).len(), //nur zur verdeutlichung das &&str hier gilt
        Message::Owned(o) => o.len(),
    }
}

//8.2
struct Highlight<'a> {
    text: &'a str,
    level: u8,
}

fn emphasize<'a>(text: &'a str) -> Highlight<'a> {
    Highlight {
        text: (text),
        level: (1),
    }
}

//8.3
fn maybe_uppercase<'a>(input: &'a str) -> Cow<'a, str> {
    if input.chars().all(|c| c.is_uppercase()) {
        Cow::Borrowed(input)
    } else {
        Cow::Owned(input.to_uppercase())
    }
}

//8.4
trait AsStr {
    fn as_str(&self) -> &str;
}

impl AsStr for String {
    fn as_str(&self) -> &str {
        &self
    }
}

impl AsStr for &str {
    fn as_str(&self) -> &str {
        &self
    }
}

impl<'a> AsStr for Cow<'a, str> {
    fn as_str(&self) -> &str {
        match self {
            Cow::Borrowed(b) => b,
            Cow::Owned(o) => o.as_str(),
        }
    }
}

fn print_it(item: &impl AsStr) {
    println!("{}", item.as_str());
}
