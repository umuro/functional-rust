(* Example 098: Partition Iterator *)
(* Split into two collections by predicate *)

(* Approach 1: List.partition *)
let split_even_odd lst =
  List.partition (fun x -> x mod 2 = 0) lst

let split_positive lst =
  List.partition (fun x -> x > 0) lst

(* Approach 2: Multi-way partition *)
let partition3 p1 p2 lst =
  List.fold_right (fun x (a, b, c) ->
    if p1 x then (x :: a, b, c)
    else if p2 x then (a, x :: b, c)
    else (a, b, x :: c)
  ) lst ([], [], [])

let classify_numbers lst =
  partition3 (fun x -> x < 0) (fun x -> x = 0) lst

(* Approach 3: Partition with transformation *)
let partition_map f lst =
  List.fold_right (fun x (lefts, rights) ->
    match f x with
    | Either.Left l -> (l :: lefts, rights)
    | Either.Right r -> (lefts, r :: rights)
  ) lst ([], [])

let validate_ages ages =
  let valid, invalid = List.partition (fun a -> a >= 0 && a <= 150) ages in
  (valid, invalid)

let split_at_first pred lst =
  let rec aux acc = function
    | [] -> (List.rev acc, [])
    | x :: rest when pred x -> (List.rev acc, x :: rest)
    | x :: rest -> aux (x :: acc) rest
  in
  aux [] lst

(* Tests *)
let () =
  let (evens, odds) = split_even_odd [1;2;3;4;5;6] in
  assert (evens = [2;4;6]);
  assert (odds = [1;3;5]);

  let (pos, neg) = split_positive [3; -1; 4; -5; 0] in
  assert (pos = [3; 4]);
  assert (neg = [-1; -5; 0]);

  let (neg, zero, pos) = classify_numbers [-2; 0; 3; -1; 0; 5] in
  assert (neg = [-2; -1]);
  assert (zero = [0; 0]);
  assert (pos = [3; 5]);

  let (valid, invalid) = validate_ages [25; -5; 200; 30; 0] in
  assert (valid = [25; 30; 0]);
  assert (invalid = [-5; 200]);

  let (before, after) = split_at_first (fun x -> x > 3) [1;2;3;4;5] in
  assert (before = [1;2;3]);
  assert (after = [4;5]);

  Printf.printf "✓ All tests passed\n"
