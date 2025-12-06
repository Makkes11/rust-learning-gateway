fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
enum Event {
    Click { x: i32, y: i32 },
    Key(char),
    Resize(u32, u32),
    Quit,
}

fn process(ev: Event) -> String {
    match ev {
        Event::Click { x, y } => format!("Click at ({x},{y})"),
        Event::Key(k) => match k {
            'q' => "Quit key".into(),
            'c' => format!("Key {k}"),
            other => format!("Other key: {other}"),
        },
        Event::Resize(w, h) => format!("Resized to {w}x{h}"),
        Event::Quit => format!("Program ended"),
    }
}

fn only_click(ev: &Event) -> Option<(i32, i32)> {
    let Event::Click { x, y } = ev else {
        return None;
    };

    Some((*x, *y))
}

fn filter_clicks(events: Vec<Event>) -> Vec<(i32, i32)> {
    let mut result = Vec::new();

    for ev in events {
        if let Event::Click { x, y } = ev {
            result.push((x, y));
        }
    }

    result
}

fn dispatch(ev: Event) {
    let Event::Key(c) = ev else {
        println!("not a key");
        return;
    };

    println!("Pressed key: {c}");
}
