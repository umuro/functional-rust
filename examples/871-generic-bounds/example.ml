(* Example 077: Generic Bounds *)
(* OCaml type constraints → Rust <T: Trait> bounds *)

(* Approach 1: Polymorphic functions with module constraints *)
module type Printable = sig
  type t
  val to_string : t -> string
end

module type Comparable = sig
  type t
  val compare : t -> t -> int
end

(* Approach 2: Using built-in polymorphic compare *)
let find_max lst =
  match lst with
  | [] -> None
  | [x] -> Some x
  | x :: rest -> Some (List.fold_left max x rest)

let find_min lst =
  match lst with
  | [] -> None
  | x :: rest -> Some (List.fold_left min x rest)

(* Approach 3: Explicit comparison function parameter *)
let find_max_by (cmp : 'a -> 'a -> int) = function
  | [] -> None
  | x :: rest ->
    Some (List.fold_left (fun acc y -> if cmp acc y >= 0 then acc else y) x rest)

let clamp ~lo ~hi x =
  if x < lo then lo
  else if x > hi then hi
  else x

(* Generic pair operations *)
let pair_map f (a, b) = (f a, f b)

let pair_fold f init (a, b) = f (f init a) b

(* Tests *)
let () =
  assert (find_max [3; 1; 4; 1; 5; 9] = Some 9);
  assert (find_max [] = None);
  assert (find_min [3; 1; 4; 1; 5; 9] = Some 1);

  assert (find_max_by compare [3; 1; 4; 1; 5; 9] = Some 9);
  assert (find_max_by (fun a b -> compare b a) [3; 1; 4; 1; 5; 9] = Some 1);

  assert (clamp ~lo:0 ~hi:10 15 = 10);
  assert (clamp ~lo:0 ~hi:10 (-5) = 0);
  assert (clamp ~lo:0 ~hi:10 5 = 5);

  assert (pair_map (fun x -> x * 2) (3, 4) = (6, 8));
  assert (pair_fold (+) 0 (3, 4) = 7);

  Printf.printf "✓ All tests passed\n"
