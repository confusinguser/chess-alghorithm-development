use std::time::Instant;

use chess::{Action, Board};

pub(crate) trait Algorithm {
    fn next_action(
        &self,
        board: &Board,
        analyze: bool,
        deadline: Instant,
    ) -> (chess::Action, Vec<String>);
    fn eval(&self, board: &Board) -> f32;
}
