(* Recursive closures and Y combinator in OCaml *)

(* Natural recursion with let rec *)
let rec factorial n = if n <= 1 then 1 else n * factorial (n - 1)

(* Open recursion: pass self as argument *)
let factorial_open self n = if n <= 1 then 1 else n * self self (n - 1)
let factorial_via_open n = factorial_open factorial_open n

(* Y combinator in OCaml *)
(* Works in OCaml because it has lazy evaluation option or recursive types *)
(* Using explicit recursion trick: *)
let y f =
  let rec x a = f (fun v -> x a v) a in
  x

(* Usage of Y combinator *)
let fact_step self n = if n <= 1 then 1 else n * self (n - 1)
let factorial_y = y fact_step

let () =
  Printf.printf "factorial(5) = %d\n" (factorial 5);
  Printf.printf "factorial_via_open(6) = %d\n" (factorial_via_open 6);
  Printf.printf "factorial_y(7) = %d\n" (factorial_y 7);

  (* Anonymous recursive via Y *)
  let fib = y (fun self n ->
    if n <= 1 then n
    else self (n-1) + self (n-2))
  in
  Printf.printf "fib(10) = %d\n" (fib 10)
