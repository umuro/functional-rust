(* Example 127: Const Functions — Compile-Time Computation *)
(* OCaml evaluates module-level expressions at program start *)

(* Approach 1: Module-level computation *)
let fibonacci n =
  let rec fib a b count =
    if count = 0 then a
    else fib b (a + b) (count - 1)
  in
  fib 0 1 n

(* These are computed when the module loads *)
let fib_10 = fibonacci 10
let fib_20 = fibonacci 20

(* Approach 2: Compile-time via PPX (conceptual) *)
(* In real OCaml, you'd use ppx_blob or ppx_optcomp for true compile-time *)
let lookup_table = Array.init 256 (fun i -> i * i)

(* Approach 3: Recursive pure computation *)
let pow_int base exp =
  let rec go acc b e =
    if e = 0 then acc
    else if e mod 2 = 1 then go (acc * b) (b * b) (e / 2)
    else go acc (b * b) (e / 2)
  in
  go 1 base exp

let two_to_10 = pow_int 2 10
let three_to_5 = pow_int 3 5

(* Factorial *)
let rec factorial n = if n <= 1 then 1 else n * factorial (n - 1)
let fact_10 = factorial 10

(* Tests *)
let () =
  assert (fib_10 = 55);
  assert (fib_20 = 6765);
  assert (lookup_table.(16) = 256);
  assert (two_to_10 = 1024);
  assert (three_to_5 = 243);
  assert (fact_10 = 3628800);
  Printf.printf "✓ All tests passed\n"
