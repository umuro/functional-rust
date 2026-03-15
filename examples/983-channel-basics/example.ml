(* 983: Channel Basics
   Message passing between threads using a bounded/unbounded channel.
   OCaml: Queue + Mutex + Condition variable (the standard pattern).
   For synchronous rendezvous, OCaml's Event module provides typed channels. *)

(* --- Unbounded MPSC channel (Multi-Producer, Single-Consumer) --- *)
type 'a chan = {
  q     : 'a Queue.t;
  mutex : Mutex.t;
  not_empty : Condition.t;
  mutable closed : bool;
}

let create_chan () =
  { q = Queue.create (); mutex = Mutex.create ();
    not_empty = Condition.create (); closed = false }

let send ch x =
  Mutex.lock ch.mutex;
  if ch.closed then (Mutex.unlock ch.mutex; failwith "send on closed channel");
  Queue.push x ch.q;
  Condition.signal ch.not_empty;
  Mutex.unlock ch.mutex

(* recv: blocks until a message is available or the channel is closed *)
let recv ch =
  Mutex.lock ch.mutex;
  while Queue.is_empty ch.q && not ch.closed do
    Condition.wait ch.not_empty ch.mutex
  done;
  let r = if Queue.is_empty ch.q then None else Some (Queue.pop ch.q) in
  Mutex.unlock ch.mutex;
  r

(* try_recv: non-blocking *)
let try_recv ch =
  Mutex.lock ch.mutex;
  let r = if Queue.is_empty ch.q then None else Some (Queue.pop ch.q) in
  Mutex.unlock ch.mutex;
  r

let close_chan ch =
  Mutex.lock ch.mutex;
  ch.closed <- true;
  Condition.broadcast ch.not_empty;  (* wake any blocked receivers *)
  Mutex.unlock ch.mutex

(* --- Bounded channel (blocks producer when full) --- *)
type 'a bounded_chan = {
  buf     : 'a array;
  mutable head : int;
  mutable len  : int;
  cap     : int;
  mutex   : Mutex.t;
  not_empty : Condition.t;
  not_full  : Condition.t;
  mutable closed : bool;
  dummy : 'a;
}

let create_bounded cap dummy =
  { buf = Array.make cap dummy; head = 0; len = 0; cap;
    mutex = Mutex.create ();
    not_empty = Condition.create (); not_full = Condition.create ();
    closed = false; dummy }

let send_bounded ch x =
  Mutex.lock ch.mutex;
  while ch.len = ch.cap && not ch.closed do
    Condition.wait ch.not_full ch.mutex
  done;
  if ch.closed then (Mutex.unlock ch.mutex; failwith "closed");
  ch.buf.((ch.head + ch.len) mod ch.cap) <- x;
  ch.len <- ch.len + 1;
  Condition.signal ch.not_empty;
  Mutex.unlock ch.mutex

let recv_bounded ch =
  Mutex.lock ch.mutex;
  while ch.len = 0 && not ch.closed do
    Condition.wait ch.not_empty ch.mutex
  done;
  if ch.len = 0 then (Mutex.unlock ch.mutex; None)
  else begin
    let x = ch.buf.(ch.head) in
    ch.head <- (ch.head + 1) mod ch.cap;
    ch.len <- ch.len - 1;
    Condition.signal ch.not_full;
    Mutex.unlock ch.mutex;
    Some x
  end

let () =
  Printf.printf "=== Unbounded channel (producer → consumer) ===\n";
  let ch : int chan = create_chan () in
  let producer = Thread.create (fun () ->
    for i = 1 to 5 do
      send ch i;
      Printf.printf "sent %d\n%!" i
    done;
    close_chan ch
  ) () in
  let consumer = Thread.create (fun () ->
    let sum = ref 0 in
    let running = ref true in
    while !running do
      match recv ch with
      | Some v -> sum := !sum + v; Printf.printf "recv %d\n%!" v
      | None   -> running := false
    done;
    Printf.printf "consumer total = %d\n%!" !sum
  ) () in
  Thread.join producer;
  Thread.join consumer;

  Printf.printf "\n=== Bounded channel (capacity=2) ===\n";
  let bch : int bounded_chan = create_bounded 2 0 in
  let prod2 = Thread.create (fun () ->
    for i = 1 to 4 do
      Printf.printf "sending %d...\n%!" i;
      send_bounded bch i;
      Printf.printf "sent %d\n%!" i
    done;
    Mutex.lock bch.mutex; bch.closed <- true;
    Condition.broadcast bch.not_empty;
    Mutex.unlock bch.mutex
  ) () in
  Thread.delay 0.01;  (* let producer fill the buffer first *)
  let cons2 = Thread.create (fun () ->
    let running = ref true in
    while !running do
      match recv_bounded bch with
      | Some v -> Printf.printf "received %d\n%!" v; Thread.delay 0.005
      | None   -> running := false
    done
  ) () in
  Thread.join prod2;
  Thread.join cons2;

  Printf.printf "\n=== Synchronous Event channel ===\n";
  (* OCaml's Event module provides CML-style synchronous channels *)
  let ech = Event.new_channel () in
  let _t1 = Thread.create (fun () ->
    let v = Event.sync (Event.receive ech) in
    Printf.printf "received: %d\n" v
  ) () in
  Event.sync (Event.send ech 42);
  Thread.delay 0.01
