(* 086: Custom Iterator with State *)

(* Approach 1: Counter using mutable ref *)
let make_counter start step =
  let n = ref (start - step) in
  fun () -> n := !n + step; Some !n

(* Approach 2: Fibonacci via Seq *)
let fibonacci () =
  let rec aux a b () = Seq.Cons (a, aux b (a + b)) in
  aux 0 1

let take_seq n s =
  let rec aux n s acc =
    if n <= 0 then List.rev acc
    else match s () with
      | Seq.Nil -> List.rev acc
      | Seq.Cons (x, rest) -> aux (n - 1) rest (x :: acc)
  in
  aux n s []

(* Approach 3: Collatz sequence iterator *)
let collatz_seq start =
  let rec aux n () =
    if n = 1 then Seq.Cons (1, fun () -> Seq.Nil)
    else Seq.Cons (n, aux (if n mod 2 = 0 then n / 2 else 3 * n + 1))
  in
  aux start

(* Tests *)
let () =
  let counter = make_counter 0 2 in
  assert (counter () = Some 2);
  assert (counter () = Some 4);
  assert (counter () = Some 6);
  let fibs = take_seq 8 (fibonacci ()) in
  assert (fibs = [0; 1; 1; 2; 3; 5; 8; 13]);
  let collatz = List.of_seq (collatz_seq 6) in
  assert (collatz = [6; 3; 10; 5; 16; 8; 4; 2; 1]);
  Printf.printf "✓ All tests passed\n"
