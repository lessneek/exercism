pub fn nth(n: u32) -> u32 {
    let n = n as usize;
    let mut len = n * 10 + 3;
    let mut primegen = vec![true; len];

    loop {
        for i in 2..(len as f64).sqrt() as usize {
            if !primegen[i] {
                continue;
            }
            let mut j = i * i;
            while j < len {
                primegen[j] = false;
                j += i;
            }
        }

        if let Some(result) = primegen
            .iter()
            .enumerate()
            .skip(2)
            .filter(|(_, &f)| f)
            .nth(n)
            .map(|(i, _)| i as u32)
        {
            return result;
        }

        len *= 2;
        primegen.resize(len, true);
    }
}
