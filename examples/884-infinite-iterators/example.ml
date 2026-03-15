(* Example 090: Infinite Iterators *)
(* iterate, repeat, cycle, unfold *)

(* Approach 1: repeat and cycle *)
let repeat x () = Seq.Cons (x, repeat x)

let rec cycle lst () =
  let rec from = function
    | [] -> cycle lst ()
    | x :: rest -> Seq.Cons (x, fun () -> from rest)
  in
  from lst

let seq_take n seq =
  let rec aux n seq acc =
    if n = 0 then List.rev acc
    else match seq () with
    | Seq.Nil -> List.rev acc
    | Seq.Cons (x, rest) -> aux (n - 1) rest (x :: acc)
  in
  aux n seq []

(* Approach 2: iterate — apply function repeatedly *)
let iterate f x =
  let rec aux v () = Seq.Cons (v, aux (f v)) in
  aux x

let doubles_from n = iterate (fun x -> x * 2) n
let collatz_seq n = iterate (fun x -> if x mod 2 = 0 then x / 2 else 3 * x + 1) n

(* Approach 3: unfold — generalized sequence generation *)
let unfold f init =
  let rec aux state () =
    match f state with
    | None -> Seq.Nil
    | Some (value, next_state) -> Seq.Cons (value, aux next_state)
  in
  aux init

let fibonacci_unfold () =
  unfold (fun (a, b) -> Some (a, (b, a + b))) (0, 1)

let range_unfold start stop =
  unfold (fun n -> if n >= stop then None else Some (n, n + 1)) start

let digits n =
  if n = 0 then [0]
  else
    unfold (fun n -> if n = 0 then None else Some (n mod 10, n / 10)) n
    |> seq_take 100
    |> List.rev

(* Tests *)
let () =
  assert (seq_take 5 (repeat 42) = [42; 42; 42; 42; 42]);
  assert (seq_take 7 (cycle [1; 2; 3]) = [1; 2; 3; 1; 2; 3; 1]);

  assert (seq_take 5 (doubles_from 1) = [1; 2; 4; 8; 16]);
  assert (seq_take 5 (iterate (( + ) 3) 0) = [0; 3; 6; 9; 12]);

  assert (seq_take 8 (fibonacci_unfold ()) = [0; 1; 1; 2; 3; 5; 8; 13]);
  assert (seq_take 5 (range_unfold 3 8) = [3; 4; 5; 6; 7]);
  assert (digits 12345 = [1; 2; 3; 4; 5]);

  Printf.printf "✓ All tests passed\n"
