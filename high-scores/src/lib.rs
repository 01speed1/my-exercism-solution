#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        let last = &self.scores.iter().last().copied();
        *last
    }

    pub fn personal_best(&self) -> Option<u32> {
        if &self.scores.len() == &0 {
            return None;
        }

        let best = &self.scores.iter().max().copied();
        *best
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.clone();
        scores.sort();
        scores.reverse();
        scores.truncate(3);
        scores
    }
}
