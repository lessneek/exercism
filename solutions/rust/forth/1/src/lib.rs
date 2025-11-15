pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Word {
    name: String,
    body: String,
}

impl Word {
    pub fn new() -> Word {
        Word {
            name: String::new(),
            body: String::new(),
        }
    }
}

pub struct Forth {
    stack: Vec<Value>,
    user_defined_words: Vec<Word>,
    words_head: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: vec![],
            user_defined_words: vec![],
            words_head: 0,
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let input = input.to_lowercase();
        let mut iiter = input.split_whitespace();

        while let Some(param) = iiter.next() {
            if let Ok(num) = param.parse::<i32>() {
                self.stack.push(num);
                continue;
            }

            if let Some((i, word)) = self
                .user_defined_words
                .iter()
                .rev()
                .enumerate()
                .skip(self.words_head)
                .find(|(_, x)| x.name == param)
                .map(|(i, x)| (i, x.body.clone()))
            {
                let words_head = self.words_head;
                self.words_head = i + 1;
                self.eval(word.as_str())?;
                self.words_head = words_head;
                continue;
            }

            match param {
                "+" => {
                    let (b, a) = self.pop_two()?;
                    self.stack.push(a + b);
                }
                "-" => {
                    let (b, a) = self.pop_two()?;
                    self.stack.push(a - b);
                }
                "*" => {
                    let (b, a) = self.pop_two()?;
                    self.stack.push(a * b);
                }
                "/" => {
                    let (b, a) = self.pop_two()?;
                    if b == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    self.stack.push(a / b);
                }
                "dup" => {
                    let a = *self.stack.last().ok_or(Error::StackUnderflow)?;
                    self.stack.push(a);
                }
                "drop" => {
                    self.stack.pop().ok_or(Error::StackUnderflow)?;
                }
                "swap" => {
                    let (b, a) = self.pop_two()?;
                    self.stack.push(b);
                    self.stack.push(a);
                }
                "over" => {
                    let (b, a) = self.pop_two()?;
                    self.stack.push(a);
                    self.stack.push(b);
                    self.stack.push(a);
                }
                ":" => {
                    let mut word = Word::new();

                    word.name = iiter
                        .next()
                        .filter(|&x| {
                            if let Some(c) = x.chars().next() {
                                c.is_ascii_alphabetic() || !c.is_ascii_digit()
                            } else {
                                false
                            }
                        })
                        .map(|w| w.to_owned())
                        .ok_or(Error::InvalidWord)?;

                    let mut is_correct_end = false;
                    for w in iiter.by_ref() {
                        match w {
                            ";" => {
                                is_correct_end = true;
                                break;
                            }
                            w => word.body.push_str(&format!("{w} ")),
                        }
                    }
                    if !is_correct_end || word.body.is_empty() { return Err(Error::InvalidWord); };
                    self.user_defined_words.push(word);
                }
                _ => {
                    return Err(Error::UnknownWord);
                }
            }
        }

        Ok(())
    }

    fn pop_two(&mut self) -> std::result::Result<(Value, Value), Error> {
        if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
            return Ok((a, b));
        }
        Err(Error::StackUnderflow)
    }
}
