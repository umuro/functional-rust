(* List Operations: fundamental recursive patterns in OCaml *)

(* ── Basic recursive versions ─────────────────────────────────── *)

let rec length = function
  | [] -> 0
  | _ :: tail -> 1 + length tail

let rec sum = function
  | [] -> 0
  | head :: tail -> head + sum tail

let rec append l1 l2 =
  match l1 with
  | [] -> l2
  | head :: tail -> head :: append tail l2

let rec reverse = function
  | [] -> []
  | head :: tail -> reverse tail @ [head]

(* ── Tail-recursive versions (efficient, no stack overflow) ──── *)

let length_tr lst =
  let rec aux acc = function
    | [] -> acc
    | _ :: tail -> aux (acc + 1) tail
  in aux 0 lst

let sum_tr lst =
  let rec aux acc = function
    | [] -> acc
    | h :: t -> aux (acc + h) t
  in aux 0 lst

let reverse_tr lst =
  let rec aux acc = function
    | [] -> acc
    | h :: t -> aux (h :: acc) t
  in aux [] lst

(* ── Higher-order: map, filter ───────────────────────────────── *)

let rec map f = function
  | [] -> []
  | h :: t -> f h :: map f t

let rec filter pred = function
  | [] -> []
  | h :: t -> if pred h then h :: filter pred t else filter pred t

(* ── Tests ────────────────────────────────────────────────────── *)
let () =
  assert (length [] = 0);
  assert (length [1;2;3] = 3);
  assert (sum [1;2;3;4;5] = 15);
  assert (append [1;2] [3;4] = [1;2;3;4]);
  assert (reverse [1;2;3] = [3;2;1]);
  assert (length_tr [1;2;3] = 3);
  assert (sum_tr [1;2;3;4;5] = 15);
  assert (reverse_tr [1;2;3] = [3;2;1]);
  assert (map (fun x -> x * 2) [1;2;3] = [2;4;6]);
  assert (filter (fun x -> x mod 2 = 0) [1;2;3;4;5] = [2;4]);
  print_endline "✓ All list operations tests passed"
