pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::new();

    if list.is_empty() {
        return result;
    }

    let first_word = list[0];

    for word in list.windows(2) {
        result.push_str(format!("For want of a {0} the {1} was lost.\n", word[0], word[1]).as_str());
    }

    result.push_str(format!("And all for the want of a {first_word}.").as_str());

    result
}
