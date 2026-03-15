(* 464: Actor Pattern
   An actor owns private state and processes messages sequentially.
   In OCaml we use a Domain running a message loop over a channel
   (Mutex + Queue + Condition). *)

type msg =
  | Inc  of int64
  | Dec  of int64
  | Get  of int64 ref * Mutex.t * Condition.t
  | Reset
  | Stop

type actor = { send : msg -> unit }

let make_actor () =
  let mu   = Mutex.create () in
  let cond = Condition.create () in
  let q    = Queue.create () in

  let send msg =
    Mutex.lock mu;
    Queue.push msg q;
    Condition.signal cond;
    Mutex.unlock mu
  in

  (* Actor domain: event loop *)
  let _domain = Domain.spawn (fun () ->
    let state = ref 0L in
    let running = ref true in
    while !running do
      Mutex.lock mu;
      while Queue.is_empty q do Condition.wait cond mu done;
      let msg = Queue.pop q in
      Mutex.unlock mu;
      match msg with
      | Inc n             -> state := Int64.add !state n
      | Dec n             -> state := Int64.sub !state n
      | Reset             -> state := 0L
      | Stop              -> running := false
      | Get (cell, m, c)  ->
        Mutex.lock m;
        cell := !state;
        Condition.signal c;
        Mutex.unlock m
    done)
  in
  { send }

(* Synchronous get: sends a Get message and blocks until reply *)
let actor_get actor =
  let cell = ref 0L in
  let m    = Mutex.create () in
  let c    = Condition.create () in
  Mutex.lock m;
  actor.send (Get (cell, m, c));
  Condition.wait c m;
  let v = !cell in
  Mutex.unlock m;
  v

let () =
  let a = make_actor () in
  a.send (Inc 7L);
  a.send (Inc 3L);
  assert (actor_get a = 10L);
  Printf.printf "after inc 7+3: %Ld\n%!" (actor_get a);

  a.send (Dec 4L);
  assert (actor_get a = 6L);
  Printf.printf "after dec 4: %Ld\n%!" (actor_get a);

  a.send Reset;
  assert (actor_get a = 0L);
  Printf.printf "after reset: %Ld\n%!" (actor_get a);

  a.send Stop
