(* Finite automaton in OCaml *)
type state = Idle | Running of int | Paused of int | Done of int

type event = Start | Tick | Pause | Resume | Stop

let transition state event =
  match (state, event) with
  | (Idle,       Start)  -> Running 0
  | (Running n,  Tick)   -> Running (n+1)
  | (Running n,  Pause)  -> Paused n
  | (Running n,  Stop)   -> Done n
  | (Paused n,   Resume) -> Running n
  | (Paused n,   Stop)   -> Done n
  | (s, _)               -> s

let describe = function
  | Idle      -> "idle"
  | Running n -> Printf.sprintf "running (tick %d)" n
  | Paused n  -> Printf.sprintf "paused at %d" n
  | Done n    -> Printf.sprintf "done after %d ticks" n

let () =
  let events = [Start;Tick;Tick;Pause;Resume;Tick;Stop] in
  let state = List.fold_left (fun s e ->
    let s' = transition s e in
    Printf.printf "-> %s\n" (describe s'); s'
  ) Idle events in
  ignore state
