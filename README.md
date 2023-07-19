# Connect4 Monte-Carlo Tree Search

Play against an MCTS connect4 agent in your terminal.

<img width="290" alt="CleanShot 2023-07-18 at 19 45 18@2x" src="https://github.com/saolsen/connect4/assets/508702/e2356bb4-a8d1-4ab1-a54c-d161e2f8e2ef">


Includes two agents that play connect4.
* Random agent that makes random moves.
* MCTS agent that simulates thousands of games to pick the best move.

## Dependencies

- Rust (see [rustup](https://rustup.rs/) for installation)

## Usage

```bash
cargo run --release
```

Run it in release mode. More games are simulated so the agent is smarter.
