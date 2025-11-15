#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

const MAX_PINS: u16 = 10;
const MAX_FRAMES: u8 = 10;

pub struct Frame {
    number: u8,
    rolls: Vec<u16>,
    score: u16,
}

impl Frame {
    pub fn new(number: u8) -> Self {
        Self {
            number,
            rolls: vec![],
            score: 0,
        }
    }

    pub fn is_first_strike(&self) -> bool {
        !self.rolls.is_empty() && self.rolls[0] == MAX_PINS
    }

    pub fn is_second_strike(&self) -> bool {
        self.rolls.len() == 2 && self.rolls[1] == MAX_PINS
    }

    pub fn strikes_count(&self) -> u16 {
        self.rolls.iter().filter(|&&r| r == MAX_PINS).sum()
    }

    pub fn is_spare(&self) -> bool {
        self.rolls.len() >= 2 && self.rolls[0] + self.rolls[1] == MAX_PINS
    }

    pub fn rolls_count(&self) -> usize {
        self.rolls.len()
    }

    pub fn roll(&mut self, mut pins: u16, bonuses: u16) {
        self.rolls.push(pins);
        pins *= bonuses;
        self.score += pins;
    }

    pub fn is_last(&self) -> bool {
        self.number == MAX_FRAMES
    }
}

pub struct BowlingGame {
    frames: Vec<Frame>,
    cur_frame: Option<Frame>,
    pins_left: u16,
    bonuses: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: vec![],
            cur_frame: Some(Frame::new(1)),
            pins_left: 10,
            bonuses: vec![],
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let cur_frame = self.cur_frame.as_mut().ok_or(Error::GameComplete)?;
        let is_last_frame = cur_frame.is_last();

        if pins > self.pins_left {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.pins_left -= pins;

        let bonuses = self.bonuses.pop().unwrap_or(1);

        cur_frame.roll(pins, bonuses);

        let is_spare = cur_frame.is_spare();
        let strikes_count = cur_frame.strikes_count();
        let rolls_count = cur_frame.rolls_count();

        println!(
            "Frame: {} | rolls: {:?} | score: +{:?} ({:?})",
            cur_frame.number,
            cur_frame.rolls,
            cur_frame.score,
            self.frames.iter().map(|f| f.score).sum::<u16>() + cur_frame.score
        );

        'framing: {
            if is_last_frame {
                if strikes_count > 1 {
                    if rolls_count == 3 {
                        self.next_frame_or_end();
                    }
                } else if is_spare && rolls_count == 2 {
                    // One roll bonus.
                    break 'framing;
                } else if matches!(rolls_count, 2 | 3) {
                    self.next_frame_or_end();
                }
            } else {
                if strikes_count > 1 {
                    self.add_roll_bonuses(2);
                }
                if is_spare {
                    self.add_roll_bonuses(1);
                }
                if (strikes_count > 1 && rolls_count == 1) || rolls_count == 2 {
                    self.next_frame_or_end();
                }
            }
        }

        if self.pins_left == 0 {
            self.pins_left = MAX_PINS;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete() {
            return None;
        }
        Some(self.frames.iter().map(|f| f.score).sum())
    }

    fn is_complete(&self) -> bool {
        self.cur_frame.is_none()
    }

    fn next_frame_or_end(&mut self) {
        if let Some(cur_frame) = self.cur_frame.take() {
            println!("Frame {} ended.", cur_frame.number);

            self.pins_left = MAX_PINS;

            let next_num = cur_frame.number + 1;
            self.frames.push(cur_frame);

            if next_num > MAX_FRAMES {
                println!("Game ended with score: {:?}.", self.score());
                return;
            }

            self.cur_frame = Some(Frame::new(next_num));
        }
    }

    fn add_roll_bonuses(&mut self, count: usize) {
        while self.bonuses.len() < count {
            self.bonuses.insert(0, 1);
        }
        for bonus in self.bonuses.iter_mut().rev().take(count) {
            *bonus += 1;
        }
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}
