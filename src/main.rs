use crate::{
    payoff_matrix::{PayoffEntry, PayoffMatrix, PayoffMatrixRow},
    util_function::UtilFunction,
};

mod payoff_matrix;
mod util_function;

fn main() {
    let top_left = PayoffEntry {
        payoff_x: 4.0,
        payoff_y: 7.0,
    };
    let top_right = PayoffEntry {
        payoff_x: 2.0,
        payoff_y: 10.0,
    };
    let bottom_left = PayoffEntry {
        payoff_x: 0.0,
        payoff_y: 20.0,
    };
    let bottom_right = PayoffEntry {
        payoff_x: 3.0,
        payoff_y: 3.0,
    };

    let top_row = PayoffMatrixRow {
        entries: [top_left, top_right],
    };
    let bottom_row = PayoffMatrixRow {
        entries: [bottom_left, bottom_right],
    };

    let matrix = PayoffMatrix {
        rows: [top_row, bottom_row],
    };
    let util_x = UtilFunction::new(&matrix, &util_function::PlayerDiffVariable::PlayerX);
    let util_y = UtilFunction::new(&matrix, &util_function::PlayerDiffVariable::PlayerY);
    println!("f_1(x,y) = {util_x}\nf_2(x,y) = {util_y}");

    let y = util_x.find_critical(&util_function::PlayerDiffVariable::PlayerX);
    let x = util_y.find_critical(&util_function::PlayerDiffVariable::PlayerY);

    println!("{x} {y}")
}
