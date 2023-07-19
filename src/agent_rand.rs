use crate::connect4::{Action, Match, COLS};

pub fn agent(mat: &Match) -> Action {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    loop {
        // Generate random actions until one is valid.
        let action = Action {
            column: rng.gen_range(0..COLS),
        };
        if mat.valid_action(&action) {
            return action;
        }
    }
}
