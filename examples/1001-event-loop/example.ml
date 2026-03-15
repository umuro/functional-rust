(* 1001: Event Loop
   Pure functional event dispatch using pattern matching and records.
   State is immutable — each event produces a new state value.
   The loop is a fold over the event list, stopping at Quit. *)

type event =
  | Click of { x: int; y: int }
  | KeyPress of char
  | Timer of string
  | NetworkData of string
  | Quit

type app_state = {
  clicks: int;
  keys: string;
  timers: int;
  network_msgs: string list;
}

let initial_state = {
  clicks = 0;
  keys = "";
  timers = 0;
  network_msgs = [];
}

(* Pure dispatch: event → next state *)
let dispatch state = function
  | Click _       -> { state with clicks = state.clicks + 1 }
  | KeyPress c    -> { state with keys = state.keys ^ String.make 1 c }
  | Timer _       -> { state with timers = state.timers + 1 }
  | NetworkData m -> { state with network_msgs = state.network_msgs @ [m] }
  | Quit          -> state  (* handled by loop *)

(* Functional event loop — stops at Quit *)
let run_until_quit events init =
  let rec loop state = function
    | [] -> state
    | Quit :: _ -> state
    | ev :: rest -> loop (dispatch state ev) rest
  in
  loop init events

(* Also show fold-based variant (does not short-circuit) *)
let run_event_loop events init =
  List.fold_left (fun state ev ->
    if ev = Quit then state else dispatch state ev
  ) init events

let test_events = [
  Click { x = 10; y = 20 };
  KeyPress 'h';
  KeyPress 'i';
  Timer "heartbeat";
  NetworkData "hello";
  Click { x = 5; y = 5 };
  NetworkData "world";
  Timer "refresh";
  Quit;
  Click { x = 0; y = 0 };  (* ignored — after Quit *)
]

let () =
  let s = run_until_quit test_events initial_state in
  assert (s.clicks = 2);
  assert (s.keys = "hi");
  assert (s.timers = 2);
  assert (List.length s.network_msgs = 2);

  (* Quit stops processing *)
  let s2 = run_until_quit [Click { x = 0; y = 0 }; Quit; Click { x = 0; y = 0 }] initial_state in
  assert (s2.clicks = 1);

  let s3 = run_until_quit [] initial_state in
  assert (s3 = initial_state);

  Printf.printf "clicks=%d  keys=%s  timers=%d  network_msgs=%d\n"
    s.clicks s.keys s.timers (List.length s.network_msgs)
