# Welcome to strategy_engine's source repository.

strategy_engine is a Rust command-line program that computes the mixed strategy
Nash equilibrium for a given payoff matrix. This program was originally
implemented [in some rather bad C](https://github.com/xtt28/strategy_engine).

Given a payoff matrix, strategy_engine computes a payoff function for each
player. It then takes the partial derivative of each payoff function and finds
the critical points of each function to find the MSNE.

## Usage

### Compiling/running

You will need the Rust toolchain.

```shell
# Clone the Git repository to local machine.
git clone https://github.com/xtt28/strategy_engine_rs.git
cd strategy_engine

# Run the program.
cargo run
```

### Using the CLI

Enter the payoff values for each cell in the payoff matrix when prompted.

## License

This software is licensed under:

    SPDX-License-Identifier: GPL-3.0-or-later

being in concordance with the terms in the `LICENSE` file found in the root of
this repository.