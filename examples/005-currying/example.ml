(* Currying and Partial Application *)
(* In OCaml, ALL multi-argument functions are curried by default *)

(* ── Basic currying: every function is unary ─────────────── *)

(* `let add a b = a + b` is sugar for: *)
let add = fun a -> fun b -> a + b

(* Partial application is free *)
let add5 = add 5
let double = ( * ) 2  (* partially apply multiplication *)

(* ── Multi-argument currying ─────────────────────────────── *)

let add3 a b c = a + b + c
(* add3 1       → fun b c -> 1 + b + c *)
(* add3 1 2     → fun c -> 1 + 2 + c   *)
(* add3 1 2 3   → 6                     *)

(* ── Building predicates via partial application ─────────── *)

let greater_than threshold x = x > threshold
let between low high x = x >= low && x <= high

let filter_between lst low high =
  List.filter (between low high) lst

(* ── Practical: apply a pipeline of transforms ───────────── *)

let apply_all value transforms =
  List.fold_left (fun acc f -> f acc) value transforms

(* ── Generic partial application (explicit) ──────────────── *)

let partial f a = fun b -> f a b

(* ── Tests ────────────────────────────────────────────────── *)
let () =
  assert (add5 3 = 8);
  assert (double 7 = 14);
  assert (add3 1 2 3 = 6);
  assert ((add3 10 20) 30 = 60);
  assert (greater_than 4 5 = true);
  assert (greater_than 4 3 = false);
  assert (filter_between [1;2;3;4;5;6] 2 5 = [2;3;4;5]);
  assert (apply_all 5 [add 10; ( * ) 2; add (-1)] = 29);
  let square = partial (fun base exp -> int_of_float (Float.pow (float_of_int base) (float_of_int exp))) 2 in
  assert (square 10 = 1024);
  print_endline "✓ All currying tests passed"
