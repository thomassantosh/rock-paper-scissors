# rock-paper-scissors
Comparing time to run 1 million iterations of the game "Rock, Paper, Scissors" between Python and Rust
implementations. Can do this by running `make compare` at the command line. Interestingly, using the `time`
function, the results are fairly similar.

## Results Comparison
| Language | Real | User | Sys  |
| --       | --   | --   | --   |
| Python   | 1.48 | 1.40 | 0.03 |
| Rust     | 1.41 | 1.40 | 0.01 |
