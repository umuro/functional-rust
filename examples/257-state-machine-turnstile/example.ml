(* Example 257: State Machine — Turnstile *)
(* Encoding a finite state machine with algebraic types and pattern matching *)

(* Idiomatic OCaml — types and a transition function *)
type state = Locked | Unlocked
type event = Coin | Push

let transition state event = match state, event with
  | Locked, Coin -> Unlocked
  | Unlocked, Push -> Locked
  | Locked, Push -> Locked
  | Unlocked, Coin -> Unlocked

let state_name = function Locked -> "Locked" | Unlocked -> "Unlocked"

(* Recursive style — fold the event list, accumulating state *)
let rec run_machine state = function
  | [] -> state
  | event :: rest ->
    let next = transition state event in
    run_machine next rest

let () =
  (* Basic transition assertions *)
  assert (transition Locked Coin = Unlocked);
  assert (transition Unlocked Push = Locked);
  assert (transition Locked Push = Locked);
  assert (transition Unlocked Coin = Unlocked);

  (* OCaml sequence: Coin Push Push Coin Coin Push starting from Locked *)
  let events = [Coin; Push; Push; Coin; Coin; Push] in

  (* Print each step — mirrors the original snippet *)
  let _final = List.fold_left (fun s e ->
    let s' = transition s e in
    Printf.printf "%s -> %s\n" (state_name s) (state_name s');
    s'
  ) Locked events in

  (* Recursive version gives the same final state *)
  assert (run_machine Locked events = Locked);
  print_endline "ok"
