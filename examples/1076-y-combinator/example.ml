(* Y Combinator — Anonymous Recursion *)

(* OCaml needs a recursive type wrapper *)
type 'a fix = Fix of ('a fix -> 'a)

let y f =
  let g (Fix x as w) = f (fun a -> x w a) in
  g (Fix g)

let factorial = y (fun self n ->
  if n = 0 then 1 else n * self (n - 1))

let fibonacci = y (fun self n ->
  if n <= 1 then n else self (n - 1) + self (n - 2))

let () =
  assert (factorial 0 = 1);
  assert (factorial 10 = 3628800);
  assert (fibonacci 0 = 0);
  assert (fibonacci 1 = 1);
  assert (fibonacci 10 = 55);
  Printf.printf "10! = %d\n" (factorial 10);
  Printf.printf "fib(10) = %d\n" (fibonacci 10);
  print_endline "ok"
