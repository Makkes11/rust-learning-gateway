fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn handle(msg: Message) -> String {
    match msg {
        Message::Quit => "Bye".into(),

        Message::Move { x: 0, y: 0 } => "No movement".into(),
        Message::Move { x, y } => format!("Moving to ({x},{y})"),

        Message::Write(ref s) if s.is_empty() => "Empty".into(),
        Message::Write(s) => format!("Text: {s}"),

        Message::ChangeColor(r, g, b) if r <= 20 && g <= 20 && b <= 20 => "Dark color".into(),
        Message::ChangeColor(r, g, b) => format!("Color RGB({r},{g},{b})"),
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area(Rectangle { width, height }: Rectangle) -> u32 {
    width * height
}

enum Action {
    Mouse(Event),
    Keyboard(Event),
}

enum Event {
    Click { x: i32, y: i32 },
    Key(char),
}

fn extract_click(act: Action) -> Option<(i32, i32)> {
    if let Action::Mouse(Event::Click { x, y }) = act {
        Some((x, y))
    } else {
        None
    }
}
