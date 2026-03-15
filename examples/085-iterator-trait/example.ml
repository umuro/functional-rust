(* 085: Iterator — implementing Seq from scratch *)

(* Approach 1: Simple sequence type *)
type 'a my_seq = unit -> 'a my_node
and 'a my_node = Nil | Cons of 'a * 'a my_seq

let empty () = Nil

let rec range_seq a b () =
  if a >= b then Nil
  else Cons (a, range_seq (a + 1) b)

(* Approach 2: Sequence operations *)
let rec seq_map f s () =
  match s () with
  | Nil -> Nil
  | Cons (x, rest) -> Cons (f x, seq_map f rest)

let rec seq_filter p s () =
  match s () with
  | Nil -> Nil
  | Cons (x, rest) ->
    if p x then Cons (x, seq_filter p rest)
    else seq_filter p rest ()

let seq_fold f init s =
  let rec aux acc s =
    match s () with
    | Nil -> acc
    | Cons (x, rest) -> aux (f acc x) rest
  in
  aux init s

let seq_to_list s =
  let rec aux acc s =
    match s () with
    | Nil -> List.rev acc
    | Cons (x, rest) -> aux (x :: acc) rest
  in
  aux [] s

(* Approach 3: Take and collect *)
let rec seq_take n s () =
  if n <= 0 then Nil
  else match s () with
    | Nil -> Nil
    | Cons (x, rest) -> Cons (x, seq_take (n - 1) rest)

(* Tests *)
let () =
  let r = range_seq 0 5 in
  assert (seq_to_list r = [0; 1; 2; 3; 4]);
  let doubled = seq_map (fun x -> x * 2) (range_seq 1 4) in
  assert (seq_to_list doubled = [2; 4; 6]);
  let evens = seq_filter (fun x -> x mod 2 = 0) (range_seq 0 10) in
  assert (seq_to_list evens = [0; 2; 4; 6; 8]);
  let sum = seq_fold ( + ) 0 (range_seq 1 6) in
  assert (sum = 15);
  let first3 = seq_take 3 (range_seq 0 100) in
  assert (seq_to_list first3 = [0; 1; 2]);
  Printf.printf "✓ All tests passed\n"
