(* 1002: Backpressure
   OCaml (stdlib) has no native bounded channels, but we can model
   backpressure with a simple bounded queue and Thread/Mutex/Condition.

   The key concept: a bounded buffer forces the producer to wait when
   the consumer is slow — coordinating producer/consumer speeds.

   We demonstrate the concept with a functional bounded queue module,
   then show blocking push semantics with Mutex/Condition. *)

module BoundedQueue = struct
  type 'a t = {
    mutable buf: 'a Queue.t;
    capacity: int;
    mutex: Mutex.t;
    not_full: Condition.t;   (* signalled when space becomes available *)
    not_empty: Condition.t;  (* signalled when item becomes available *)
  }

  let create capacity = {
    buf = Queue.create ();
    capacity;
    mutex = Mutex.create ();
    not_full = Condition.create ();
    not_empty = Condition.create ();
  }

  (* Blocking push — waits if full (backpressure) *)
  let push q item =
    Mutex.lock q.mutex;
    while Queue.length q.buf >= q.capacity do
      Condition.wait q.not_full q.mutex
    done;
    Queue.push item q.buf;
    Condition.signal q.not_empty;
    Mutex.unlock q.mutex

  (* Try push — returns false when full (non-blocking backpressure) *)
  let try_push q item =
    Mutex.lock q.mutex;
    let accepted = Queue.length q.buf < q.capacity in
    if accepted then begin
      Queue.push item q.buf;
      Condition.signal q.not_empty
    end;
    Mutex.unlock q.mutex;
    accepted

  (* Blocking pop *)
  let pop q =
    Mutex.lock q.mutex;
    while Queue.is_empty q.buf do
      Condition.wait q.not_empty q.mutex
    done;
    let item = Queue.pop q.buf in
    Condition.signal q.not_full;
    Mutex.unlock q.mutex;
    item

  let length q =
    Mutex.lock q.mutex;
    let n = Queue.length q.buf in
    Mutex.unlock q.mutex;
    n
end

(* Demo: bounded pipeline — producer blocks when buffer is full *)
let bounded_backpressure () =
  let q = BoundedQueue.create 3 in
  let produced = ref 0 in
  let consumed = ref 0 in

  let producer = Thread.create (fun () ->
    for i = 1 to 9 do
      BoundedQueue.push q i;
      incr produced
    done
  ) () in

  let consumer = Thread.create (fun () ->
    for _ = 1 to 9 do
      let _item = BoundedQueue.pop q in
      incr consumed
    done
  ) () in

  Thread.join producer;
  Thread.join consumer;
  (!produced, !consumed)

(* Demo: try_push drops items when full *)
let try_send_demo () =
  let q = BoundedQueue.create 2 in
  let accepted = ref 0 in
  let dropped = ref 0 in
  for i = 1 to 10 do
    ignore i;
    if BoundedQueue.try_push q i then incr accepted
    else incr dropped
  done;
  (!accepted, !dropped)

let () =
  let (prod, cons) = bounded_backpressure () in
  assert (prod = 9);
  assert (cons = 9);

  let (acc, dropped) = try_send_demo () in
  assert (acc + dropped = 10);
  assert (acc <= 2);  (* buffer capacity = 2 *)

  Printf.printf "bounded: produced=%d consumed=%d\n" prod cons;
  Printf.printf "try_push: accepted=%d dropped=%d\n" acc dropped
