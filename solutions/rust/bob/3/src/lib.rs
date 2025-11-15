pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = message.ends_with('?');
    let is_all_caps = message.chars().any(|c| c.is_ascii_alphabetic())
        && !message
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .any(|c| c.is_ascii_lowercase());

    if is_question {
        if is_all_caps {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else {
        if is_all_caps {
            "Whoa, chill out!"
        } else {
            "Whatever."
        }
    }
}
