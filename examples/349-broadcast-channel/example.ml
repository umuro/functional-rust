(* 349: Broadcast Channel
   One sender, many receivers — every subscriber gets every message.
   Implemented with a list of per-subscriber queues protected by a mutex. *)

type 'a broadcast_sender = {
  mutex       : Mutex.t;
  subscribers : 'a Queue.t list ref;
}

(* A receiver is just a queue + condition for blocking recv *)
type 'a broadcast_receiver = {
  queue     : 'a Queue.t;
  cond      : Condition.t;
  recv_lock : Mutex.t;
}

let make_sender () = { mutex = Mutex.create (); subscribers = ref [] }

(* Subscribe: creates a new receiver and registers its queue *)
let subscribe sender =
  let q   = Queue.create () in
  let r   = { queue = q; cond = Condition.create (); recv_lock = Mutex.create () } in
  Mutex.lock sender.mutex;
  sender.subscribers := q :: !(sender.subscribers);
  Mutex.unlock sender.mutex;
  r

(* Send a copy of msg to every subscriber *)
let broadcast sender msg =
  Mutex.lock sender.mutex;
  List.iter (fun q ->
    Queue.push msg q
  ) !(sender.subscribers);
  Mutex.unlock sender.mutex

(* Non-blocking receive *)
let try_recv receiver =
  Mutex.lock receiver.recv_lock;
  let v = if Queue.is_empty receiver.queue then None
          else Some (Queue.pop receiver.queue) in
  Mutex.unlock receiver.recv_lock;
  v

let () =
  let sender = make_sender () in
  let r1 = subscribe sender in
  let r2 = subscribe sender in

  broadcast sender 42;

  (* Both receivers get the message *)
  assert (try_recv r1 = Some 42);
  assert (try_recv r2 = Some 42);
  Printf.printf "Broadcast to r1 and r2: ok\n%!";

  (* Late subscriber misses earlier messages *)
  broadcast sender 1;
  let r3 = subscribe sender in
  broadcast sender 2;
  assert (try_recv r3 = Some 2);
  assert (try_recv r3 = None);
  Printf.printf "Late subscriber missed first message: ok\n%!"
