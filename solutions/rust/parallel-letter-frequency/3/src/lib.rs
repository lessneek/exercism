use std::collections::HashMap;
use std::thread;

fn count_chars(text: &[&str]) -> HashMap<char, usize> {
    let mut chars_map = HashMap::<char, usize>::new();

    text.iter()
        .flat_map(|&line| line.chars())
        .filter(|ch| ch.is_alphabetic())
        .map(|ch| ch.to_ascii_lowercase())
        .for_each(|ch| *chars_map.entry(ch).or_default() += 1);

    chars_map
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    match input.len() {
        0 => HashMap::new(),
        lines_count if lines_count < 300 => count_chars(input),
        lines_count => {
            let mut chars_map = HashMap::<char, usize>::new();

            thread::scope(|s| {
                let mut threads = Vec::with_capacity(worker_count);
                for chunk in input.chunks(lines_count / worker_count + 1) {
                    threads.push(s.spawn(|| count_chars(chunk)));
                }
                for thread in threads {
                    thread.join().unwrap().iter().for_each(|(&ch, &count)| {
                        chars_map.entry(ch).and_modify(|c| *c += count).or_insert(count);
                    });
                }
            });

            chars_map
        }
    }
}
