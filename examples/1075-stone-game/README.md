📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1075-stone-game)**

---

# 1075-stone-game — Stone Game (Minimax DP)

## Problem Statement

Two players alternate taking stones from either end of a row. Each player plays optimally to maximize their own stones. Does the first player always win when the number of piles is even?

The stone game is a minimax problem — both players are rational and play to their best advantage. Interval DP captures the optimal play for any sub-interval. Interestingly, when piles.len() is even, the first player can always win by a parity argument (but proving it requires the DP to show the margin, not just the winner).

## Learning Outcomes

- Model two-player optimal play as interval DP
- Use `dp[i][j]` = score difference (current player minus opponent) for piles i..j
- Understand the minimax recurrence: take max of (pile[i] - dp[i+1][j]) and (pile[j] - dp[i][j-1])
- Recognize that the "difference" state avoids tracking both players' scores separately
- Connect to game theory and alpha-beta pruning

## Rust Application

`src/lib.rs` implements `stone_game_dp` where `dp[i][j]` is the score advantage of the current player for piles in range `[i, j]`. The recurrence: the current player can take `piles[i]` (gaining `piles[i]` and leaving the opponent with `dp[i+1][j]` advantage) or take `piles[j]`. Taking the max gives the optimal play.

The "score difference" trick encodes minimax in one array: if `dp[0][n-1] > 0`, the first player wins.

## OCaml Approach

```ocaml
let stone_game piles =
  let n = Array.length piles in
  let dp = Array.make_matrix n n 0 in
  for i = 0 to n - 1 do dp.(i).(i) <- piles.(i) done;
  for len = 2 to n do
    for i = 0 to n - len do
      let j = i + len - 1 in
      dp.(i).(j) <- max
        (piles.(i) - dp.(i+1).(j))
        (piles.(j) - dp.(i).(j-1))
    done
  done;
  dp.(0).(n-1) > 0
```

Identical structure. The score-difference encoding is a mathematical insight independent of language.

## Key Differences

1. **Score difference encoding**: Both encode `current_player_score - opponent_score` in one cell; this is the key insight that simplifies the state.
2. **Interval filling order**: Both fill by increasing `len` — the same constraint as all interval DP problems.
3. **Parity argument**: For even n, the first player can always guarantee a win by parity (take all even-indexed or all odd-indexed piles); the DP proves this by showing the margin.
4. **Alpha-beta pruning**: For deeper game trees, alpha-beta pruning reduces the branching factor; the stone game is small enough that full minimax DP is practical.

## Exercises

1. Return the first player's actual score (not just win/lose) from `stone_game_dp`.
2. Implement the greedy strategy (always take the larger of the two ends) and demonstrate it is NOT optimal by constructing a counterexample.
3. Extend to k players (not just 2) taking turns — generalize the DP to track the score difference relative to the leader.
