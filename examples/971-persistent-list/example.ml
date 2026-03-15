(* 971: Persistent/Immutable Linked List *)
(* OCaml lists are already persistent with structural sharing *)
(* Each cons cell is shared between versions *)

(* Approach 1: OCaml's built-in list IS persistent *)

let () =
  let list1 = [1; 2; 3] in
  let list2 = 0 :: list1 in  (* O(1), shares tail with list1 *)
  let list3 = (-1) :: list2 in

  (* All three lists exist simultaneously *)
  assert (list1 = [1; 2; 3]);
  assert (list2 = [0; 1; 2; 3]);
  assert (list3 = [-1; 0; 1; 2; 3]);

  (* Structural sharing: list2 and list1 share the [1;2;3] tail *)
  (* This is automatic — no explicit copy *)
  ()

(* Approach 2: Explicit persistent stack *)

type 'a pstack =
  | Empty
  | Cons of 'a * 'a pstack

let empty = Empty

let push x s = Cons (x, s)

let pop = function
  | Empty -> None
  | Cons (x, rest) -> Some (x, rest)

let peek = function
  | Empty -> None
  | Cons (x, _) -> Some x

let rec length = function
  | Empty -> 0
  | Cons (_, rest) -> 1 + length rest

let rec to_list = function
  | Empty -> []
  | Cons (x, rest) -> x :: to_list rest

(* Approach 3: Persistent operations *)

let rec append s1 s2 =
  match s1 with
  | Empty -> s2
  | Cons (x, rest) -> Cons (x, append rest s2)

let () =
  let s0 = empty in
  let s1 = push 1 s0 in
  let s2 = push 2 s1 in
  let s3 = push 3 s2 in

  assert (length s0 = 0);
  assert (length s1 = 1);
  assert (length s2 = 2);
  assert (length s3 = 3);

  assert (peek s3 = Some 3);

  (* s2 unchanged after creating s3 *)
  assert (peek s2 = Some 2);

  let (v, s2') = Option.get (pop s3) in
  assert (v = 3);
  assert (peek s2' = Some 2);
  assert (s2' = s2);  (* structurally equal *)

  assert (to_list s3 = [3; 2; 1]);

  let combined = append s3 s1 in
  assert (to_list combined = [3; 2; 1; 1]);

  Printf.printf "✓ All tests passed\n"
