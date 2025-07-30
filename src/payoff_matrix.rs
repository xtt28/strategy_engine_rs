use crate::util_function::PlayerDiffVariable;

pub struct PayoffMatrix {
    pub rows: [PayoffMatrixRow; 2],
}

pub struct PayoffMatrixRow {
    pub entries: [PayoffEntry; 2],
}

pub struct PayoffEntry {
    pub payoff_x: f64,
    pub payoff_y: f64,
}

impl PayoffEntry {
    pub fn get_payoff(&self, player: &PlayerDiffVariable) -> f64 {
        match player {
            PlayerDiffVariable::PlayerX => self.payoff_x,
            PlayerDiffVariable::PlayerY => self.payoff_y,
        }
    }
}