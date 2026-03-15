(* 455: Lock-Free Stack
   Treiber stack: each push/pop uses a CAS loop on the head pointer.
   OCaml 5 Atomic provides compare_and_set for this purpose. *)

type 'a node = Nil | Cons of 'a * 'a node

type 'a stack = { head : 'a node Atomic.t }

let make () = { head = Atomic.make Nil }

(* Push: link a new node at the head using a CAS loop *)
let push stack value =
  let rec loop () =
    let old_head = Atomic.get stack.head in
    let new_head = Cons (value, old_head) in
    if not (Atomic.compare_and_set stack.head old_head new_head)
    then loop ()
  in
  loop ()

(* Pop: detach the head using a CAS loop; returns None when empty *)
let pop stack =
  let rec loop () =
    match Atomic.get stack.head with
    | Nil -> None
    | Cons (value, next) as old_head ->
      if Atomic.compare_and_set stack.head old_head next
      then Some value
      else loop ()
  in
  loop ()

let () =
  (* LIFO order *)
  let s = make () in
  push s 1; push s 2; push s 3;
  assert (pop s = Some 3);
  assert (pop s = Some 2);
  assert (pop s = Some 1);
  assert (pop s = None);
  Printf.printf "LIFO: push 1,2,3 → pop 3,2,1,None: ok\n%!";

  (* Concurrent pushes from multiple domains *)
  let s2 = make () in
  let domains = List.init 4 (fun _ ->
    Domain.spawn (fun () ->
      for i = 0 to 99 do push s2 i done))
  in
  List.iter Domain.join domains;
  let count = ref 0 in
  while pop s2 <> None do incr count done;
  assert (!count = 400);
  Printf.printf "concurrent push from 4 domains × 100 items = %d\n%!" !count
