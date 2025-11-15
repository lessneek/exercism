use std::rc::Rc;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Default)]
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

#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
    user_defined_words: Vec<Rc<Word>>,
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
        self.eval_internal(input.as_str())
    }

    fn try_eval_value(&mut self, param: &str) -> bool {
        if let Ok(num) = param.parse::<Value>() {
            self.stack.push(num);
            return true;
        }
        false
    }

    fn try_eval_word(&mut self, param: &str) -> std::result::Result<bool, Error> {
        if let Some((i, word)) = self
            .user_defined_words
            .iter()
            .rev()
            .enumerate()
            .skip(self.words_head)
            .find(|(_, x)| x.name == param)
            .map(|(i, x)| (i, x.clone()))
        {
            let words_head = self.words_head;
            self.words_head = i + 1;
            self.eval_internal(word.body.as_str())?;
            self.words_head = words_head;
            return Ok(true);
        }
        Ok(false)
    }

    /// Evaluates a string for internal usage.
    /// Note: the `input` must be always lowercase.
    fn eval_internal(&mut self, input: &str) -> Result {
        let mut iiter = input.split_whitespace();

        while let Some(param) = iiter.next() {
            if self.try_eval_value(param) {
                continue;
            }

            if self.try_eval_word(param)? {
                continue;
            }

            match param {
                "+" => {
                    let (b, a) = self.pop2()?;
                    self.push(a + b);
                }
                "-" => {
                    let (b, a) = self.pop2()?;
                    self.push(a - b);
                }
                "*" => {
                    let (b, a) = self.pop2()?;
                    self.push(a * b);
                }
                "/" => {
                    let (b, a) = self.pop2()?;
                    if b == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    self.push(a / b);
                }
                "dup" => {
                    let a = *self.stack.last().ok_or(Error::StackUnderflow)?;
                    self.push(a);
                }
                "drop" => {
                    self.pop()?;
                }
                "swap" => {
                    let (b, a) = self.pop2()?;
                    self.push(b);
                    self.push(a);
                }
                "over" => {
                    let (b, a) = self.pop2()?;
                    self.push(a);
                    self.push(b);
                    self.push(a);
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
                            w => {
                                word.body.push_str(w);
                                word.body.push(' ');
                            }
                        }
                    }
                    if !is_correct_end || word.body.is_empty() { return Err(Error::InvalidWord); };
                    self.user_defined_words.push(Rc::new(word));
                }
                _ => {
                    return Err(Error::UnknownWord);
                }
            }
        }

        Ok(())
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> std::result::Result<Value, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn pop2(&mut self) -> std::result::Result<(Value, Value), Error> {
        Ok((self.pop()?, self.pop()?))
    }
}
