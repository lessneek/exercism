pub fn nth(n: u32) -> u32 {
    let n = n as usize;
    const GROW_STEP: usize = 10_000;
    let mut upper_bound = GROW_STEP;
    let mut flags = vec![true; upper_bound + 1];

    loop {
        for i in 2..=(upper_bound as f64).sqrt() as usize {
            if !flags[i] {
                continue;
            }
            let mut j = i * i;
            while j <= upper_bound {
                flags[j] = false;
                j += i;
            }
        }

        if let Some(result) = flags
            .iter()
            .enumerate()
            .skip(2)
            .filter(|(_, &f)| f)
            .nth(n)
            .map(|(i, _)| i as u32)
        {
            return result;
        }

        for _ in 0..GROW_STEP {
            flags.push(true);
        }
        upper_bound += GROW_STEP;
    }
}
