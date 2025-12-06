fn main() {
    assert_eq!(sum_lengths(&"abc".to_string(), &"zz".to_string()), 5);

    assert_eq!(
        describe(&SensorValue::Temperature(20.0)),
        "Temperature: 20°C"
    );
    assert_eq!(
        describe(&SensorValue::Pressure(1013.2)),
        "Pressure: 1013.2hPa"
    );
    assert_eq!(
        describe(&SensorValue::Status("OK".to_string())),
        "Status: OK"
    );

    assert_eq!(consume(Data::Number(-10)), 10);
    assert_eq!(consume(Data::Text("abc".to_string())), 3);

    assert_eq!(store_refs("a", "b"), ("a", "b"))
}

fn sum_lengths<'a>(a: &'a str, b: &'a str) -> usize {
    a.len() + b.len()
}

enum SensorValue {
    Temperature(f64),
    Pressure(f64),
    Status(String),
}

fn describe(value: &SensorValue) -> String {
    match value {
        SensorValue::Temperature(v) => format!("Temperature: {v}°C"),
        SensorValue::Pressure(p) => format!("Pressure: {p}hPa"),
        SensorValue::Status(s) => format!("Status: {s}"),
    }
}

enum Data {
    Number(i32),
    Text(String),
}

fn consume(data: Data) -> usize {
    match data {
        Data::Number(n) => n.abs() as usize,
        Data::Text(s) => s.len(),
    }
}

fn store_refs<'a, 'b>(a: &'a str, b: &'b str) -> (&'a str, &'b str) {
    (a, b)
}
