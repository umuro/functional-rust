(* 1002: Backpressure — Bounded Channel Blocks Producer *)
(* When consumer is slow, bounded buffer fills up and blocks the producer *)

(* --- Bounded queue (simulates sync_channel) --- *)

type 'a bounded_chan = {
  q: 'a Queue.t;
  capacity: int;
  m: Mutex.t;
  not_full: Condition.t;
  not_empty: Condition.t;
  mutable closed: bool;
}

let make_bounded_chan capacity = {
  q = Queue.create ();
  capacity;
  m = Mutex.create ();
  not_full = Condition.create ();
  not_empty = Condition.create ();
  closed = false;
}

let send_bounded c v =
  Mutex.lock c.m;
  while Queue.length c.q >= c.capacity && not c.closed do
    Condition.wait c.not_full c.m  (* BLOCK when full — backpressure! *)
  done;
  if not c.closed then begin
    Queue.push v c.q;
    Condition.signal c.not_empty
  end;
  Mutex.unlock c.m

let recv_bounded c =
  Mutex.lock c.m;
  while Queue.is_empty c.q && not c.closed do
    Condition.wait c.not_empty c.m
  done;
  let v = if Queue.is_empty c.q then None else Some (Queue.pop c.q) in
  Condition.signal c.not_full;
  Mutex.unlock c.m;
  v

let close_bounded c =
  Mutex.lock c.m;
  c.closed <- true;
  Condition.broadcast c.not_full;
  Condition.broadcast c.not_empty;
  Mutex.unlock c.m

(* --- Approach 1: Slow consumer applies backpressure --- *)

let () =
  let chan = make_bounded_chan 3 in  (* buffer of 3 *)
  let sent_times = ref [] in
  let recv_times = ref [] in
  let m = Mutex.create () in

  let producer = Thread.create (fun () ->
    for i = 1 to 9 do
      send_bounded chan i;
      Mutex.lock m;
      sent_times := Unix.gettimeofday () :: !sent_times;
      Mutex.unlock m
    done;
    close_bounded chan
  ) () in

  let consumer = Thread.create (fun () ->
    let rec loop () =
      match recv_bounded chan with
      | None -> ()
      | Some _ ->
        Unix.sleepf 0.005;  (* slow consumer *)
        Mutex.lock m;
        recv_times := Unix.gettimeofday () :: !recv_times;
        Mutex.unlock m;
        loop ()
    in loop ()
  ) () in

  Thread.join producer;
  Thread.join consumer;

  assert (List.length !sent_times = 9);
  assert (List.length !recv_times = 9);
  Printf.printf "Approach 1 (backpressure): sent=%d recv=%d (producer was blocked by slow consumer)\n"
    (List.length !sent_times) (List.length !recv_times)

(* --- Approach 2: Producer detects backpressure (try_send) --- *)

let try_send c v =
  Mutex.lock c.m;
  let ok = Queue.length c.q < c.capacity in
  if ok then begin
    Queue.push v c.q;
    Condition.signal c.not_empty
  end;
  Mutex.unlock c.m;
  ok

let () =
  let chan = make_bounded_chan 2 in
  let accepted = ref 0 in
  let dropped = ref 0 in

  for i = 1 to 10 do
    if try_send chan i then incr accepted
    else incr dropped
  done;

  assert (!accepted = 2);
  assert (!dropped = 8);
  Printf.printf "Approach 2 (try_send): accepted=%d dropped=%d\n" !accepted !dropped

let () = Printf.printf "✓ All tests passed\n"
