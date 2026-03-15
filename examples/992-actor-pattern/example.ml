(* 992: Actor Pattern
   Each actor is a thread with a mailbox (channel).
   Actors communicate only by sending messages; no shared mutable state.
   OCaml: Thread + Queue + Mutex + Condition as mailbox. *)

(* Generic actor mailbox *)
type 'msg mailbox = {
  q     : 'msg Queue.t;
  mutex : Mutex.t;
  cond  : Condition.t;
  mutable alive : bool;
}

let make_mailbox () =
  { q = Queue.create (); mutex = Mutex.create ();
    cond = Condition.create (); alive = true }

let send_msg mb msg =
  Mutex.lock mb.mutex;
  if mb.alive then begin
    Queue.push msg mb.q;
    Condition.signal mb.cond
  end;
  Mutex.unlock mb.mutex

let recv_msg mb =
  Mutex.lock mb.mutex;
  while Queue.is_empty mb.q && mb.alive do
    Condition.wait mb.cond mb.mutex
  done;
  let r = if Queue.is_empty mb.q then None else Some (Queue.pop mb.q) in
  Mutex.unlock mb.mutex;
  r

let stop_actor mb =
  Mutex.lock mb.mutex;
  mb.alive <- false;
  Condition.broadcast mb.cond;
  Mutex.unlock mb.mutex

(* Spawn an actor: run loop f until mailbox is stopped *)
let spawn_actor handle =
  let mb = make_mailbox () in
  let thread = Thread.create (fun () ->
    let running = ref true in
    while !running do
      match recv_msg mb with
      | None -> running := false
      | Some msg -> handle msg
    done
  ) () in
  (mb, thread)

(* --- Example: counter actor --- *)
type counter_msg =
  | Increment
  | Add of int
  | Reset
  | Get of int ref * Mutex.t * Condition.t

let make_counter_actor () =
  let count = ref 0 in
  spawn_actor (fun msg ->
    match msg with
    | Increment -> count := !count + 1
    | Add n     -> count := !count + n
    | Reset     -> count := 0
    | Get (r, m, c) ->
      Mutex.lock m;
      r := !count;
      Condition.broadcast c;
      Mutex.unlock m
  )

let get_counter mb =
  let r = ref 0 in
  let m = Mutex.create () and c = Condition.create () in
  Mutex.lock m;
  send_msg mb (Get (r, m, c));
  Condition.wait c m;
  let v = !r in
  Mutex.unlock m;
  v

(* --- Example: logger actor (serialises log writes) --- *)
type log_msg = Log of string | Flush of unit ref * Mutex.t * Condition.t

let make_logger_actor () =
  let buf = Buffer.create 256 in
  let (mb, t) = spawn_actor (fun msg ->
    match msg with
    | Log s -> Buffer.add_string buf s; Buffer.add_char buf '\n'
    | Flush (done_, m, c) ->
      Mutex.lock m;
      done_ := ();
      Condition.broadcast c;
      Mutex.unlock m
  ) in
  let flush () =
    let done_ = ref () in
    let m = Mutex.create () and c = Condition.create () in
    Mutex.lock m;
    send_msg mb (Flush (done_, m, c));
    Condition.wait c m;
    Mutex.unlock m;
    Buffer.contents buf
  in
  (mb, t, flush)

let () =
  Printf.printf "=== Counter actor ===\n";
  let (mb, thread) = make_counter_actor () in

  (* Multiple threads send messages concurrently *)
  let senders = List.init 4 (fun _ ->
    Thread.create (fun () ->
      for _ = 1 to 250 do send_msg mb Increment done
    ) ()
  ) in
  List.iter Thread.join senders;
  send_msg mb (Add 100);

  let v = get_counter mb in
  Printf.printf "counter = %d (expected %d)\n" v (4 * 250 + 100);

  stop_actor mb;
  Thread.join thread;

  Printf.printf "\n=== Logger actor (serialised I/O) ===\n";
  let (log_mb, log_t, flush) = make_logger_actor () in

  let writers = List.init 3 (fun i ->
    Thread.create (fun () ->
      for j = 1 to 3 do
        send_msg log_mb (Log (Printf.sprintf "thread-%d msg-%d" i j))
      done
    ) ()
  ) in
  List.iter Thread.join writers;

  let contents = flush () in
  let lines = String.split_on_char '\n' contents |> List.filter (fun s -> s <> "") in
  Printf.printf "logged %d lines (expected 9)\n" (List.length lines);

  stop_actor log_mb;
  Thread.join log_t;

  Printf.printf "\n=== Ping-pong between two actors ===\n";
  let count = ref 0 in
  let done_mutex = Mutex.create () and done_cond = Condition.create () in
  let done_ = ref false in

  let mb_a = make_mailbox () in
  let mb_b = make_mailbox () in

  let _ta = Thread.create (fun () ->
    let running = ref true in
    while !running do
      match recv_msg mb_a with
      | None -> running := false
      | Some n ->
        if n >= 10 then begin
          Mutex.lock done_mutex;
          count := n;
          done_ := true;
          Condition.broadcast done_cond;
          Mutex.unlock done_mutex
        end else
          send_msg mb_b (n + 1)
    done
  ) () in

  let _tb = Thread.create (fun () ->
    let running = ref true in
    while !running do
      match recv_msg mb_b with
      | None -> running := false
      | Some n -> send_msg mb_a (n + 1)
    done
  ) () in

  send_msg mb_a 0;  (* start the ball rolling *)

  Mutex.lock done_mutex;
  while not !done_ do Condition.wait done_cond done_mutex done;
  Mutex.unlock done_mutex;
  Printf.printf "ping-pong terminated at count=%d\n" !count;

  stop_actor mb_a; stop_actor mb_b
