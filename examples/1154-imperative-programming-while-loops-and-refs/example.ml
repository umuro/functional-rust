(* Imperative Programming — While Loops and Refs *)
(* Imperative constructs: while, for, refs *)

(* GCD using while loop *)
let gcd a b =
  let a = ref (abs a) and b = ref (abs b) in
  while !b <> 0 do
    let t = !b in
    b := !a mod !b;
    a := t
  done;
  !a

(* Collatz sequence length *)
let collatz_length n =
  let n = ref n and steps = ref 0 in
  while !n <> 1 do
    if !n mod 2 = 0 then n := !n / 2
    else n := 3 * !n + 1;
    incr steps
  done;
  !steps

let () =
  Printf.printf "gcd(48, 36) = %d\n" (gcd 48 36);
  Printf.printf "collatz(27) = %d steps\n" (collatz_length 27)
