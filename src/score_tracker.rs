// use std::collections::{BinaryHeap};

#[derive(Copy, Clone, Debug)]
pub struct ScoreTracker {
    // best_scores: VecDeque<usize>,
    // best_scores: BinaryHeap<usize>,
    score: usize,
}

impl ScoreTracker {
    pub fn new() -> Self {
        ScoreTracker {
            // best_scores: BinaryHeap::with_capacity(3),
            score: 0
        }
    }

    // pub fn update_best_scores(&mut self) {
    //     self.best_scores.push(self.score);
    // }

    pub fn update_last_score_by(&mut self, score: usize) {
        self.score += score;
    }

    pub fn get_last_score(&self) -> &usize {
        &self.score
    }
}

impl Default for ScoreTracker {
    fn default() -> Self {
        Self::new()
    }
}