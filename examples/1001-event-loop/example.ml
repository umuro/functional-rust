(* 1001: Simple Event Loop *)
(* Poll events, dispatch enum handlers, process until Quit *)

(* --- Event types --- *)

type event =
  | Click of int * int         (* x, y *)
  | KeyPress of char
  | Timer of string            (* timer name *)
  | NetworkData of string      (* payload *)
  | Quit

type 'state handler = {
  on_click: int -> int -> 'state -> 'state;
  on_key: char -> 'state -> 'state;
  on_timer: string -> 'state -> 'state;
  on_network: string -> 'state -> 'state;
}

(* --- Event loop: dispatch events to handlers, accumulate state --- *)

let run_event_loop ~handler ~init events =
  let rec loop state = function
    | [] -> state
    | Quit :: _ -> state
    | Click (x, y) :: rest -> loop (handler.on_click x y state) rest
    | KeyPress c :: rest -> loop (handler.on_key c state) rest
    | Timer name :: rest -> loop (handler.on_timer name state) rest
    | NetworkData s :: rest -> loop (handler.on_network s state) rest
  in
  loop init events

(* --- Approach 1: Pure functional event loop --- *)

type app_state = {
  clicks: int;
  keys: string;
  timers: int;
  network_msgs: string list;
}

let initial_state = { clicks = 0; keys = ""; timers = 0; network_msgs = [] }

let app_handler = {
  on_click = (fun _x _y s -> { s with clicks = s.clicks + 1 });
  on_key = (fun c s -> { s with keys = s.keys ^ String.make 1 c });
  on_timer = (fun _name s -> { s with timers = s.timers + 1 });
  on_network = (fun msg s -> { s with network_msgs = msg :: s.network_msgs });
}

let () =
  let events = [
    Click (10, 20);
    KeyPress 'h';
    KeyPress 'i';
    Timer "heartbeat";
    NetworkData "hello";
    Click (5, 5);
    NetworkData "world";
    Timer "refresh";
    Quit;
    Click (0, 0);  (* ignored after Quit *)
  ] in
  let final_state = run_event_loop ~handler:app_handler ~init:initial_state events in
  assert (final_state.clicks = 2);
  assert (final_state.keys = "hi");
  assert (final_state.timers = 2);
  assert (List.length final_state.network_msgs = 2);
  Printf.printf "Approach 1: clicks=%d keys=%s timers=%d msgs=%d\n"
    final_state.clicks final_state.keys final_state.timers
    (List.length final_state.network_msgs)

(* --- Approach 2: Stateful event loop with mutation --- *)

let () =
  let q = Queue.create () in
  List.iter (Queue.push q) [Click(1,1); KeyPress 'x'; Timer "t1"; Quit];
  (* Queue is FIFO — push order is preserved when popping *)
  let event_count = ref 0 in
  let rec loop () =
    if Queue.is_empty q then ()
    else match Queue.pop q with
      | Quit -> ()
      | _ -> incr event_count; loop ()
  in
  loop ();
  assert (!event_count = 3);
  Printf.printf "Approach 2 (stateful loop): %d events processed\n" !event_count

let () = Printf.printf "✓ All tests passed\n"
