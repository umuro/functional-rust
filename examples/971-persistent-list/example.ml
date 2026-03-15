(* 971: Persistent (Immutable) Linked List
   OCaml's native list IS a persistent linked list with structural sharing.
   Prepend (cons) is O(1); the tail is shared with all other lists that use it.
   No Rc/Arc needed — the GC handles shared references automatically. *)

(* OCaml's built-in list type is exactly:
     type 'a list = Nil | Cons of 'a * 'a list
   We re-expose it under cleaner names for clarity. *)

let nil = []
let push x tail = x :: tail          (* O(1), tail is shared *)

let pop = function
  | [] -> None
  | x :: rest -> Some (x, rest)

let peek = function [] -> None | x :: _ -> Some x

(* Structural sharing example: two lists share the same tail *)
let shared_tail () =
  let shared = [3; 4; 5] in
  let list_a = push 1 (push 2 shared) in  (* [1;2;3;4;5] *)
  let list_b = push 10 shared in          (* [10;3;4;5] *)
  (* shared, list_a, list_b all live concurrently; no copying *)
  (list_a, list_b, shared)

(* Functional stack built on lists *)
module Stack = struct
  type 'a t = 'a list
  let empty    = []
  let push x s = x :: s
  let pop      = function [] -> None | x :: r -> Some (x, r)
  let peek     = function [] -> None | x :: _  -> Some x
  let is_empty = function [] -> true | _ -> false
  let to_list s = s
end

(* Persistent queue (two-list Banker's queue) *)
module PQueue = struct
  (* front @ List.rev rear — keep |front| >= |rear| *)
  type 'a t = { front : 'a list; rear : 'a list; flen : int; rlen : int }

  let empty = { front = []; rear = []; flen = 0; rlen = 0 }

  let rebalance q =
    if q.rlen <= q.flen then q
    else { front = q.front @ List.rev q.rear; rear = []; flen = q.flen + q.rlen; rlen = 0 }

  let enqueue x q = rebalance { q with rear = x :: q.rear; rlen = q.rlen + 1 }

  let dequeue q =
    let q = rebalance q in
    match q.front with
    | []     -> None
    | x :: f -> Some (x, rebalance { q with front = f; flen = q.flen - 1 })

  let is_empty q = q.front = [] && q.rear = []
  let to_list  q = q.front @ List.rev q.rear
end

let () =
  (* --- Persistent stack via plain lists --- *)
  Printf.printf "=== Persistent stack (OCaml native lists) ===\n";
  let s0 = Stack.empty in
  let s1 = Stack.push 1 s0 in
  let s2 = Stack.push 2 s1 in
  let s3 = Stack.push 3 s2 in
  Printf.printf "s3 = [%s]\n" (String.concat "; " (List.map string_of_int (Stack.to_list s3)));
  (match Stack.pop s3 with
   | Some (x, s3') ->
     Printf.printf "pop s3 = %d; s3' = [%s]; s3 unchanged = [%s]\n" x
       (String.concat "; " (List.map string_of_int (Stack.to_list s3')))
       (String.concat "; " (List.map string_of_int (Stack.to_list s3)))
   | None -> ());

  (* --- Structural sharing --- *)
  Printf.printf "\n=== Structural sharing ===\n";
  let (a, b, shared) = shared_tail () in
  Printf.printf "shared: [%s]\n" (String.concat "; " (List.map string_of_int shared));
  Printf.printf "list_a: [%s]\n" (String.concat "; " (List.map string_of_int a));
  Printf.printf "list_b: [%s]\n" (String.concat "; " (List.map string_of_int b));
  Printf.printf "All three alive simultaneously — GC handles sharing\n";

  (* --- Persistent queue --- *)
  Printf.printf "\n=== Persistent queue ===\n";
  let q0 = PQueue.empty in
  let q1 = PQueue.enqueue "a" q0 in
  let q2 = PQueue.enqueue "b" q1 in
  let q3 = PQueue.enqueue "c" q2 in
  Printf.printf "queue: [%s]\n" (String.concat "; " (PQueue.to_list q3));
  (match PQueue.dequeue q3 with
   | Some (x, q3') ->
     Printf.printf "dequeue = %s; old q3 still has: [%s]\n" x
       (String.concat "; " (PQueue.to_list q3))  (* q3 unchanged *)
   | None -> ());

  (* --- Branching versions (Git-like history) --- *)
  Printf.printf "\n=== Branching versions ===\n";
  let base    = [1; 2; 3] in
  let branch1 = push 10 base in   (* version A *)
  let branch2 = push 20 base in   (* version B, shares base *)
  Printf.printf "base    = [%s]\n" (String.concat "; " (List.map string_of_int base));
  Printf.printf "branch1 = [%s]\n" (String.concat "; " (List.map string_of_int branch1));
  Printf.printf "branch2 = [%s]\n" (String.concat "; " (List.map string_of_int branch2))
