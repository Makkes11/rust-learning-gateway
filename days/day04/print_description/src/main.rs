fn main() {
    let user = User {
        name: String::from("TestUser"),
    };

    let system = System { id: 12345 };

    print_description(&user);
    print_description(&system);

    let mut vector: Vec<Box<dyn Describe>> = Vec::new();
    vector.push(Box::new(user));
    vector.push(Box::new(system));

    // OK
    assert_eq!(parse_and_double::<i32>("5"), Ok(10));
}

trait Describe {
    fn describe(&self) -> String;
}

struct User {
    name: String,
}

impl Describe for User {
    fn describe(&self) -> String {
        format!("User: {}", self.name)
    }
}

struct System {
    id: u32,
}

impl Describe for System {
    fn describe(&self) -> String {
        format!("System ID: {}", self.id)
    }
}

fn print_description(item: &dyn Describe) {
    println!("{}", item.describe());
}

trait Double {
    fn double(self) -> Self;
}

impl Double for i32 {
    fn double(self) -> Self {
        self + self
    }
}

impl Double for u32 {
    fn double(self) -> Self {
        self * 2
    }
}

impl Double for f64 {
    fn double(self) -> Self {
        self + self
    }
}

fn parse_and_double<T>(input: &str) -> Result<T, String>
where
    T: std::str::FromStr + Double,
{
    let parsed = input.trim().parse::<T>();

    match parsed {
        Ok(v) => Ok(v.double()),
        Err(_) => Err("Env error".to_string()),
    }
}
