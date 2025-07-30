use std::fmt;

use crate::payoff_matrix::PayoffMatrix;

#[derive(PartialEq)]
pub enum PlayerDiffVariable {
    PlayerX,
    PlayerY,
}

pub struct UtilFunction {
    xy_coeff: f64,
    x_coeff: f64,
    y_coeff: f64,
    const_term: f64,
}

impl fmt::Display for UtilFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}xy + {}x + {}y + {}",
            self.xy_coeff, self.x_coeff, self.y_coeff, self.const_term
        )
    }
}

impl UtilFunction {
    pub fn new(matrix: &PayoffMatrix, player: &PlayerDiffVariable) -> Self {
        let top_left_payoff = matrix.rows[0].entries[0].get_payoff(player);
        let top_right_payoff = matrix.rows[0].entries[1].get_payoff(player);
        let bottom_left_payoff = matrix.rows[1].entries[0].get_payoff(player);
        let bottom_right_payoff = matrix.rows[1].entries[1].get_payoff(player);

        let xy_coeff =
            top_left_payoff - top_right_payoff - bottom_left_payoff + bottom_right_payoff;
        let x_coeff = top_right_payoff - bottom_right_payoff;
        let y_coeff = bottom_left_payoff - bottom_right_payoff;
        let const_term = bottom_right_payoff;

        Self {
            xy_coeff,
            x_coeff,
            y_coeff,
            const_term,
        }
    }

    pub fn differentiate(&self, wrt: &PlayerDiffVariable) -> Self {
        match wrt {
            PlayerDiffVariable::PlayerX => UtilFunction {
                xy_coeff: 0.0,
                x_coeff: 0.0,
                y_coeff: self.xy_coeff,
                const_term: self.x_coeff,
            },

            PlayerDiffVariable::PlayerY => UtilFunction {
                xy_coeff: 0.0,
                x_coeff: self.xy_coeff,
                y_coeff: 0.0,
                const_term: self.y_coeff,
            },
        }
    }

    pub fn find_critical(&self, wrt: &PlayerDiffVariable) -> f64 {
        let diff = self.differentiate(wrt);
        let num = -(diff.const_term);
        let denom = if *wrt == PlayerDiffVariable::PlayerX {
            diff.y_coeff
        } else {
            diff.x_coeff
        };

        num / denom
    }

    pub fn eval(&self, x: f64, y: f64) -> f64 {
        return self.xy_coeff * x * y + self.x_coeff * x + self.y_coeff * y + self.const_term;
    }
}
