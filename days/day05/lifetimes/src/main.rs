fn main() {
    assert_eq!(longer("kurz", "länger"), "länger");
    assert_eq!(longer("länger", "kurz"), "länger");

    let p = Person { name: "Peter" };
    assert_eq!(first_char(&p), "P");

    assert_eq!(good_ref(), "hi");
}

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

struct Person<'a> {
    name: &'a str,
}

fn first_char<'a>(p: &'a Person) -> &'a str {
    &p.name[0..1]
}

// fn bad_ref<'a>() -> &'a str {
//     let s = String::from("hi");
//     &s
// }

// die referenz endet mit beenden der methode --> dangling reference
//

fn good_ref() -> String {
    String::from("hi")
}
