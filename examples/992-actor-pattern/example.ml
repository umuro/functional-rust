(* 992: Actor Pattern *)
(* Actor = thread + channel mailbox. Messages are enum variants *)

(* --- Message type for a counter actor --- *)

type message =
  | Increment of int
  | Decrement of int
  | GetValue of int ref * Mutex.t * Condition.t
  | Shutdown

(* --- Channel helpers --- *)

type 'a chan = { q: 'a Queue.t; m: Mutex.t; cond: Condition.t }

let make_chan () = { q = Queue.create (); m = Mutex.create (); cond = Condition.create () }

let send c v =
  Mutex.lock c.m;
  Queue.push v c.q;
  Condition.signal c.cond;
  Mutex.unlock c.m

let recv c =
  Mutex.lock c.m;
  while Queue.is_empty c.q do Condition.wait c.cond c.m done;
  let v = Queue.pop c.q in
  Mutex.unlock c.m;
  v

(* --- Actor: runs in its own thread, processes messages --- *)

let make_counter_actor () =
  let mailbox = make_chan () in

  let _actor_thread = Thread.create (fun () ->
    let state = ref 0 in
    let running = ref true in
    while !running do
      match recv mailbox with
      | Increment n -> state := !state + n
      | Decrement n -> state := !state - n
      | GetValue (result, m, cond) ->
        Mutex.lock m;
        result := !state;
        Condition.signal cond;
        Mutex.unlock m
      | Shutdown -> running := false
    done
  ) () in

  mailbox

(* --- Approach 1: Send messages to actor --- *)

let () =
  let actor = make_counter_actor () in

  send actor (Increment 10);
  send actor (Increment 5);
  send actor (Decrement 3);

  (* Synchronous get: send a "reply channel" in message *)
  let result = ref 0 in
  let reply_m = Mutex.create () in
  let reply_cond = Condition.create () in
  Mutex.lock reply_m;
  send actor (GetValue (result, reply_m, reply_cond));
  Condition.wait reply_cond reply_m;
  Mutex.unlock reply_m;

  assert (!result = 12); (* 10+5-3 *)
  Printf.printf "Approach 1 (counter actor): %d\n" !result;

  send actor Shutdown

(* --- Approach 2: Multiple actors collaborating --- *)

type adder_msg = Add of int * int * (int -> unit) | Stop

let make_adder_actor () =
  let mailbox = make_chan () in
  let _ = Thread.create (fun () ->
    let rec loop () =
      match recv mailbox with
      | Add (a, b, reply) -> reply (a + b); loop ()
      | Stop -> ()
    in
    loop ()
  ) () in
  mailbox

let () =
  let adder = make_adder_actor () in
  let result = ref 0 in
  let m = Mutex.create () in
  let cond = Condition.create () in
  Mutex.lock m;
  send adder (Add (17, 25, fun v ->
    Mutex.lock m;
    result := v;
    Condition.signal cond;
    Mutex.unlock m
  ));
  Condition.wait cond m;
  Mutex.unlock m;
  assert (!result = 42);
  Printf.printf "Approach 2 (adder actor): %d\n" !result;
  send adder Stop

let () = Printf.printf "✓ All tests passed\n"
