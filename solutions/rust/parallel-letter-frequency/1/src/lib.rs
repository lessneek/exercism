use std::collections::HashMap;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut freq_map = HashMap::<char, usize>::new();
    if input.is_empty() {
        return freq_map;
    }
    let work_chunks = Arc::new(Mutex::new(input.to_vec()));
    let (tx, rx) = channel::<HashMap<char, usize>>();

    thread::scope(|s| for _ in 0..worker_count {
        let (tx, work_chunks) = (tx.clone(), work_chunks.clone());
        s.spawn(move || {
            let mut fmap = HashMap::<char, usize>::new();
            while let Some(work_chunk) = work_chunks.lock().unwrap().pop() {
                for ch in work_chunk.chars().filter(|ch| ch.is_alphabetic()).map(|ch| ch.to_ascii_lowercase()) {
                    fmap.entry(ch).and_modify(|count| *count += 1).or_insert(1);
                }
            }
            tx.send(fmap).unwrap();
        });
    });

    drop(tx);

    for x in rx {
        x.iter().for_each(|(&ch, &count)| {
            freq_map.entry(ch).and_modify(|c| *c += count).or_insert(count);
        });
    }

    freq_map
}
