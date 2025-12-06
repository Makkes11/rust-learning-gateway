fn main() {
    let mut v = vec![1, 2, 3];

    let s1 = sum_borrowed(&v);
    println!("sum borrowed = {}", s1);

    increment_all(&mut v);

    let s2 = sum_owned(v);
    println!("sum owned   = {}", s2);

    let text = String::from("hello world rustacean");
    let first = first_word(&text);
    println!("{}", first);

    let mut data = vec![10, 20, 30];

    let a = data[0];
    println!("a = {}", a);
    let b = &mut data[1];

    *b += 1;

    println!("b = {}", b);

    let mut values = [1, 2, 3, 4, 5, 6];
    process(&mut values);
    println!("{:?}", values); // erwartet: [2,3,4,8,10,12]
}

fn sum_owned(nums: Vec<i32>) -> i32 {
    nums.iter().sum()
}

fn sum_borrowed(nums: &Vec<i32>) -> i32 {
    nums.iter().sum()
}

fn increment_all(nums: &mut Vec<i32>) {
    for n in nums {
        *n += 1
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn process(nums: &mut [i32]) {
    let (left, right) = nums.split_at_mut(nums.len() / 2);
    for i in left {
        *i += 1;
    }

    for i in right {
        *i *= 2;
    }
}

#[derive(Debug)]
struct Stats {
    values: Vec<i32>,
}

impl Stats {
    fn new() -> Self {
        Stats { values: Vec::new() }
    }

    fn add(&mut self, value: i32) {
        self.values.push(value);
    }

    fn sum(&self) -> i32 {
        self.values.iter().sum()
    }

    fn mean(&self) -> f64 {
        f64::from(self.sum()) / self.values.len() as f64
    }

    fn scale(&mut self, factor: i32) {
        self.values.iter_mut().for_each(|v| *v *= factor)
    }

    fn into_sorted(self) -> Vec<i32> {
        let mut v = self.values; // Besitz übernehmen, self bleibt unverändert
        v.sort();
        v
    }
}
