(* Split borrows are automatic in OCaml *)
type point = { mutable x: float; mutable y: float }
type game_state = {
  mutable player: point;
  mutable enemies: int list;
  mutable score: int;
}

let () =
  let state = {
    player = { x = 0.0; y = 0.0 };
    enemies = [1;2;3];
    score = 0;
  } in
  (* Can read player.x and mutate score simultaneously — no issue *)
  let px = state.player.x in
  state.score <- state.score + 10;
  Printf.printf "player.x=%.1f, score=%d\n" px state.score;
  Printf.printf "enemies: %d\n" (List.length state.enemies)
