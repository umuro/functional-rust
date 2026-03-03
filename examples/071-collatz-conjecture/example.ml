let rec collatz_steps n =
  if n = 1 then 0
  else if n mod 2 = 0 then 1 + collatz_steps (n / 2)
  else 1 + collatz_steps (3 * n + 1)

let collatz n =
  if n <= 0 then Error "Only positive integers are allowed"
  else Ok (collatz_steps n)

let () =
  assert (collatz 1 = Ok 0);
  assert (collatz 6 = Ok 8);
  assert (collatz 27 = Ok 111);
  assert (collatz (-1) = Error "Only positive integers are allowed");
  print_endline "All assertions passed."
