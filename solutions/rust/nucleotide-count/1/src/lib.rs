use std::collections::HashMap;

fn is_nucleotide(nuc: char) -> bool {
    matches!(nuc, 'A' | 'C' | 'G' | 'T')
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_nucleotide(nucleotide) {
        return Err(nucleotide);
    }

    let mut count = 0;
    for ch in dna.chars() {
        if !is_nucleotide(ch) {
            return Err(ch);
        }
        if ch == nucleotide {
            count += 1;
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result: HashMap<char, usize> = [('A', 0), ('C', 0), ('G', 0), ('T', 0)]
        .into_iter()
        .collect();

    for ch in dna.chars() {
        if !is_nucleotide(ch) {
            return Err(ch);
        }
        result
            .entry(ch)
            .and_modify(|count| *count += 1)
            .or_default();
    }

    Ok(result)
}
