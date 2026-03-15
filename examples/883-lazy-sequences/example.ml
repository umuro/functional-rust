(* Example 089: Lazy Sequences *)
(* OCaml Seq → Rust Iterator + take *)

(* Approach 1: Lazy natural numbers *)
let naturals () =
  let rec aux n () = Seq.Cons (n, aux (n + 1)) in
  aux 0

let squares () =
  Seq.map (fun n -> n * n) (naturals ())

let primes () =
  let is_prime n =
    if n < 2 then false
    else
      let rec check d = d * d > n || (n mod d <> 0 && check (d + 1)) in
      check 2
  in
  Seq.filter is_prime (naturals ())

(* Approach 2: Recursive lazy generation *)
let powers_of base =
  let rec aux p () = Seq.Cons (p, aux (p * base)) in
  aux 1

let triangle_numbers () =
  let rec aux n sum () =
    let new_sum = sum + n in
    Seq.Cons (new_sum, aux (n + 1) new_sum)
  in
  aux 1 0

(* Approach 3: Seq operations on infinite sequences *)
let seq_take n seq =
  let rec aux n seq acc =
    if n = 0 then List.rev acc
    else match seq () with
    | Seq.Nil -> List.rev acc
    | Seq.Cons (x, rest) -> aux (n - 1) rest (x :: acc)
  in
  aux n seq []

let seq_drop_while pred seq =
  let rec aux seq () =
    match seq () with
    | Seq.Nil -> Seq.Nil
    | Seq.Cons (x, rest) ->
      if pred x then aux rest ()
      else Seq.Cons (x, rest)
  in
  aux seq

let seq_take_while pred seq =
  let rec aux seq () =
    match seq () with
    | Seq.Nil -> Seq.Nil
    | Seq.Cons (x, rest) ->
      if pred x then Seq.Cons (x, aux rest)
      else Seq.Nil
  in
  aux seq

(* Tests *)
let () =
  assert (seq_take 5 (naturals ()) = [0; 1; 2; 3; 4]);
  assert (seq_take 5 (squares ()) = [0; 1; 4; 9; 16]);
  assert (seq_take 5 (primes ()) = [2; 3; 5; 7; 11]);
  assert (seq_take 5 (powers_of 2) = [1; 2; 4; 8; 16]);
  assert (seq_take 5 (triangle_numbers ()) = [1; 3; 6; 10; 15]);

  let first_prime_over_100 = seq_take 1 (seq_drop_while (fun x -> x <= 100) (primes ())) in
  assert (first_prime_over_100 = [101]);

  let small_primes = seq_take 100 (seq_take_while (fun x -> x < 20) (primes ())) in
  assert (small_primes = [2; 3; 5; 7; 11; 13; 17; 19]);

  Printf.printf "✓ All tests passed\n"
