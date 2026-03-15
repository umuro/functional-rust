(* Example 179: GADT Preventing Runtime Errors — Safe Head *)
(* A list type where head is guaranteed safe at compile time *)

(* Approach 1: Non-empty list GADT *)
type empty = Empty_tag
type nonempty = Nonempty_tag

type ('a, _) safe_list =
  | SNil  : ('a, empty) safe_list
  | SCons : 'a * ('a, _) safe_list -> ('a, nonempty) safe_list

let safe_head : type a. (a, nonempty) safe_list -> a = function
  | SCons (x, _) -> x

let safe_tail_list : type a s. (a, nonempty) safe_list -> a list = function
  | SCons (_, rest) ->
    let rec to_list : type s. (a, s) safe_list -> a list = function
      | SNil -> []
      | SCons (x, xs) -> x :: to_list xs
    in to_list rest

(* Approach 2: NonEmpty module *)
module NonEmpty = struct
  type 'a t = { head : 'a; tail : 'a list }

  let create head tail = { head; tail }
  let head ne = ne.head
  let tail ne = ne.tail
  let to_list ne = ne.head :: ne.tail
  let of_list = function
    | [] -> None
    | x :: xs -> Some { head = x; tail = xs }

  let map f ne = { head = f ne.head; tail = List.map f ne.tail }
  let fold_left f init ne = List.fold_left f (f init ne.head) ne.tail
end

(* Approach 3: Indexed by a boolean *)
type _ is_empty = Yes : empty is_empty | No : nonempty is_empty

type ('a, 's) ilist =
  | INil  : ('a, empty) ilist
  | ICons : 'a * ('a, _) ilist -> ('a, nonempty) ilist

let ihead : type a. (a, nonempty) ilist -> a = function
  | ICons (x, _) -> x

let () =
  (* Test Approach 1 *)
  let l = SCons (1, SCons (2, SCons (3, SNil))) in
  assert (safe_head l = 1);
  assert (safe_tail_list l = [2; 3]);

  (* This would not compile:
     let _ = safe_head SNil
  *)

  (* Test Approach 2 *)
  let ne = NonEmpty.create 1 [2; 3] in
  assert (NonEmpty.head ne = 1);
  assert (NonEmpty.tail ne = [2; 3]);
  assert (NonEmpty.to_list ne = [1; 2; 3]);
  let doubled = NonEmpty.map (fun x -> x * 2) ne in
  assert (NonEmpty.head doubled = 2);
  assert (NonEmpty.of_list [] = None);
  assert (NonEmpty.of_list [42] = Some { head = 42; tail = [] });
  let sum = NonEmpty.fold_left (+) 0 ne in
  assert (sum = 6);

  (* Test Approach 3 *)
  let il = ICons (10, ICons (20, INil)) in
  assert (ihead il = 10);

  print_endline "✓ All tests passed"
