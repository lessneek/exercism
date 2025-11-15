const WHATEVER: &str = "Whatever.";
const SURE: &str = "Sure.";
const CHILL: &str = "Whoa, chill out!";
const CALM: &str = "Calm down, I know what I'm doing!";
const FINE: &str = "Fine. Be that way!";

pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message.is_empty() {
        return FINE;
    }

    let is_question = message.chars().last() == Some('?');
    let is_all_caps = message.chars().any(|c| c.is_ascii_alphabetic())
        && !message
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .any(|c| c.is_ascii_lowercase());

    match (is_question, is_all_caps) {
        (true, true) => CALM,
        (true, false) => SURE,
        (false, true) => CHILL,
        (false, false) => WHATEVER,
    }
}
