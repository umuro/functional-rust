(* 468: Lock-Free Queue (Michael-Scott simplified)
   OCaml 5's Atomic gives us compare_and_set for building a
   non-blocking FIFO queue with a sentinel/dummy head node. *)

(* Node: value=None is the sentinel/dummy head *)
type 'a node = {
  value : 'a option;
  next  : 'a node option Atomic.t;
}

type 'a queue = {
  head : 'a node Atomic.t;
  tail : 'a node Atomic.t;
}

let make_node v = { value = v; next = Atomic.make None }

let make () =
  let sentinel = make_node None in
  { head = Atomic.make sentinel; tail = Atomic.make sentinel }

let enqueue q v =
  let node = make_node (Some v) in
  let rec loop () =
    let t = Atomic.get q.tail in
    match Atomic.get t.next with
    | None ->
      (* Try to link node at the tail *)
      if Atomic.compare_and_set t.next None (Some node) then
        (* Advance tail (best-effort; other threads may help) *)
        ignore (Atomic.compare_and_set q.tail t node)
      else
        loop ()  (* retry *)
    | Some next ->
      (* Tail is lagging; help advance it *)
      ignore (Atomic.compare_and_set q.tail t next);
      loop ()
  in
  loop ()

let dequeue q =
  let rec loop () =
    let h = Atomic.get q.head in
    let t = Atomic.get q.tail in
    match Atomic.get h.next with
    | None ->
      (* Queue is empty *)
      if h == t then None else loop ()
    | Some next ->
      (* Try to swing head to next node *)
      if Atomic.compare_and_set q.head h next
      then next.value  (* value stored in the NEW head (was next) *)
      else loop ()
  in
  loop ()

let () =
  (* FIFO order *)
  let q = make () in
  for i = 1 to 5 do enqueue q i done;
  for i = 1 to 5 do
    assert (dequeue q = Some i)
  done;
  assert (dequeue q = None);
  Printf.printf "FIFO 1..5: ok\n%!";

  (* Concurrent enqueue from multiple domains *)
  let q2 = make () in
  let domains = List.init 4 (fun i ->
    Domain.spawn (fun () ->
      for j = 0 to 24 do enqueue q2 (i * 25 + j) done))
  in
  List.iter Domain.join domains;
  let count = ref 0 in
  while dequeue q2 <> None do incr count done;
  assert (!count = 100);
  Printf.printf "concurrent enqueue: %d items dequeued\n%!" !count
