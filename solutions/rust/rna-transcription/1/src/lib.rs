#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    value: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    value: String,
}

const DNA_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];
const RNA_NUCLEOTIDES: [char; 4] = ['U', 'G', 'C', 'A'];

fn dna_nuc_to_rna(nuc: char) -> Option<char> {
    Some(RNA_NUCLEOTIDES[DNA_NUCLEOTIDES.iter().position(|&x| nuc == x)?])
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, ch) in dna.char_indices() {
            if !DNA_NUCLEOTIDES.contains(&ch) {
                return Err(i);
            }
        }
        Ok(Self {
            value: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            value: self
                .value
                .chars()
                .map(|nuc| dna_nuc_to_rna(nuc).unwrap())
                .collect::<String>(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, ch) in rna.char_indices() {
            if !RNA_NUCLEOTIDES.contains(&ch) {
                return Err(i);
            }
        }
        Ok(Self {
            value: rna.to_string(),
        })
    }
}
