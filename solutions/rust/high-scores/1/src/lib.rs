#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.into(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().map(|&x| x)
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().map(|&x| x)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three = self.scores.clone();
        top_three.sort_unstable_by(|a, b| b.cmp(a));
        top_three.truncate(3);
        top_three
    }
}
