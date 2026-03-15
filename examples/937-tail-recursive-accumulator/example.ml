(* 937: Tail-Recursive Accumulator Pattern

   OCaml has guaranteed tail-call optimisation (TCO) — any function call
   in tail position is compiled to a jump, not a stack push.
   This makes the accumulator pattern the primary tool for stack-safe recursion. *)

(* ── Sum ─────────────────────────────────────────────────────────────────── *)

(* Naive: NOT tail recursive — builds up (h + ...) frames *)
let rec sum_naive = function
  | [] -> 0
  | h :: rest -> h + sum_naive rest

(* Tail-recursive with accumulator — OCaml guarantees no stack growth *)
let sum_tr lst =
  let rec go acc = function
    | [] -> acc
    | h :: rest -> go (acc + h) rest
  in
  go 0 lst

(* Idiomatic: List.fold_left is also tail-recursive in OCaml's stdlib *)
let sum_fold lst = List.fold_left ( + ) 0 lst

(* ── Reverse ─────────────────────────────────────────────────────────────── *)

(* Tail-recursive reverse using accumulator *)
let rev_tr lst =
  let rec go acc = function
    | [] -> acc
    | x :: rest -> go (x :: acc) rest   (* prepend to acc reverses order *)
  in
  go [] lst

(* ── Fibonacci ───────────────────────────────────────────────────────────── *)

(* Naive: exponential time, deep stack *)
let rec fib_naive = function
  | 0 -> 0
  | 1 -> 1
  | n -> fib_naive (n - 1) + fib_naive (n - 2)

(* Tail-recursive with two accumulators (a=fib(k), b=fib(k+1)) *)
let fib_tr n =
  let rec go a b = function
    | 0 -> a
    | n -> go b (a + b) (n - 1)
  in
  go 0 1 n

(* ── Length ──────────────────────────────────────────────────────────────── *)

let length_tr lst =
  let rec go acc = function
    | [] -> acc
    | _ :: rest -> go (acc + 1) rest
  in
  go 0 lst

(* ── Map (tail-recursive via fold + reverse) ─────────────────────────────── *)

(* List.map in OCaml's stdlib is NOT tail-recursive in older versions.
   A safe version uses fold_left and reverses the accumulator. *)
let map_tr f lst =
  List.rev (List.fold_left (fun acc x -> f x :: acc) [] lst)

(* ── Flatten ─────────────────────────────────────────────────────────────── *)

let flatten_tr lists =
  List.fold_left (fun acc xs ->
    List.fold_left (fun a x -> x :: a) acc (List.rev xs)
  ) [] (List.rev lists)

let () =
  let big = List.init 100_000 (fun i -> i + 1) in
  let expected = 100_000 * 100_001 / 2 in

  (* sum functions all agree on large list *)
  assert (sum_tr   big = expected);
  assert (sum_fold big = expected);

  (* naive sum works on small lists *)
  assert (sum_naive [1; 2; 3] = 6);

  (* empty *)
  assert (sum_tr   [] = 0);
  assert (sum_fold [] = 0);

  (* rev_tr *)
  assert (rev_tr [1; 2; 3] = [3; 2; 1]);
  assert (rev_tr ([] : int list) = []);

  (* fib *)
  assert (fib_tr 0 = 0);
  assert (fib_tr 1 = 1);
  assert (fib_tr 10 = 55);
  assert (fib_tr 40 = 102334155);
  assert (fib_naive 10 = 55);

  (* length_tr *)
  assert (length_tr [1; 2; 3; 4; 5] = 5);
  assert (length_tr [] = 0);

  (* map_tr *)
  assert (map_tr (fun x -> x * 2) [1; 2; 3] = [2; 4; 6]);
  assert (map_tr (fun x -> x * 2) [] = []);

  (* OCaml's own List.init with fold is stack-safe *)
  let sum_big = List.fold_left ( + ) 0 (List.init 100_000 (fun i -> i + 1)) in
  assert (sum_big = expected);

  print_endline "937-tail-recursive-accumulator: all tests passed"
