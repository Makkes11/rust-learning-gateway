fn main() {
    let mut state = Light::Red;

    for _ in 0..4 {
        state = state.next();
        println!("{:?}", state);
    }

    let words = "  this is a word";
    let count = count_words(&words);
    println!("{}", count);

    let mut reader: Reader = Reader {
        state: ReaderState::Closed,
    };

    println!("state: {:?}", reader.state);
    let string_to_read = "hello";
    reader.open();
    println!("state: {:?}", reader.state);
    reader.open();
    println!("state: {:?}", reader.state);

    for _ in string_to_read.chars() {
        let result = reader.read_char(&string_to_read);
        match result {
            Some(r) => {
                println!("state: {:?}", reader.state);
                println!("{}", r);
            }
            None => {}
        }
    }

    reader.close();
    println!("state: {:?}", reader.state);
}

#[derive(Debug)]
enum Light {
    Red,
    Green,
    Yellow,
}

impl Light {
    fn next(self) -> Self {
        match self {
            Light::Red => Light::Green,
            Light::Green => Light::Yellow,
            Light::Yellow => Light::Red,
        }
    }
}

#[derive(Debug)]
enum WordState {
    Start,
    InWord,
    BetweenWords,
}

fn count_words(s: &str) -> usize {
    let mut state = WordState::Start;
    let mut word_count: usize = 0;

    for c in s.chars() {
        let is_whitespace: bool = c.is_whitespace();
        match state {
            WordState::Start | WordState::BetweenWords => {
                if !is_whitespace {
                    state = WordState::InWord;
                    word_count += 1
                }
            }
            WordState::InWord => {
                if is_whitespace {
                    state = WordState::BetweenWords
                }
            }
        }
    }

    word_count
}

#[derive(Debug)]
enum ReaderState {
    Closed,
    Open(usize),
    Error(String),
}

#[derive(Debug)]
struct Reader {
    state: ReaderState,
}

impl Reader {
    fn open(&mut self) {
        match self.state {
            ReaderState::Closed => self.state = ReaderState::Open(0),
            ReaderState::Open(_) => {
                self.state = ReaderState::Error("already open".to_string());
            }
            ReaderState::Error(_) => {}
        }
    }

    fn close(&mut self) {
        self.state = ReaderState::Closed
    }

    fn read_char(&mut self, content: &str) -> Option<char> {
        match self.state {
            ReaderState::Closed => None,
            ReaderState::Open(i) => {
                if i < content.len() {
                    self.state = ReaderState::Open(i + 1);
                    Some(content.chars().nth(i).unwrap())
                } else {
                    None
                }
            }
            ReaderState::Error(_) => None,
        }
    }
}
