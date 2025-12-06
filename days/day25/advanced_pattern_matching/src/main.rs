fn main() {
    println!("{}", classify(-10));
    println!("{}", classify(0));
    println!("{}", classify(2));
    println!("{}", classify(15));
    println!("{}", classify(100));
}

fn classify(n: i32) -> &'static str {
    match n {
        i32::MIN..=-1 => "negative",
        0 => "zero",
        1..=10 => "small",
        _ => match n % 2 {
            0 => "big even",
            _ => "big odd",
        },
    }
}

enum Role {
    Admin { level: u8 },
    User,
}

enum Event {
    Login { name: String, role: Role },
    Logout(String),
}

// fn describe(ev: Event) -> String {
//     if let Event::Login { role, name } = ev {
//         if let Role::Admin { level } = role {
//             match level {
//                 0..=4 => format!("Admin login: {name} (low)"),
//                 _ => format!("Admin login: {name} (high)"),
//             }
//         } else {
//             format!("User login: {name}")
//         }
//     } else if let Event::Logout(name) = ev {
//         format!("Goodbye {name}")
//     } else {
//         unreachable!("All Event variants handled");
//     }
// }

fn describe(ev: Event) -> String {
    match ev {
        Event::Login {
            name,
            role: Role::Admin { level },
        } if (0..=4).contains(&level) => format!("Admin login: {name} (low)"),

        Event::Login {
            name,
            role: Role::Admin { .. },
        } => format!("Admin login: {name} (high)"),

        Event::Login {
            name,
            role: Role::User,
        } => format!("User login: {name}"),

        Event::Logout(name) => format!("Goodbye {name}"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Shape {
    Circle(Point, u32),
    Rect {
        top_left: Point,
        bottom_right: Point,
    },
}

fn area(shape: Shape) -> u32 {
    match shape {
        Shape::Circle(Point { x: _, y: _ }, r) => 3 * r * r,
        Shape::Rect {
            top_left: Point { x: x1, y: y1 },
            bottom_right: Point { x: x2, y: y2 },
        } => {
            let width = (x2 - x1).abs() as u32;
            let height = (y2 - y1).abs() as u32;
            width * height
        }
    }
}
