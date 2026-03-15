(* 087: Iterator Adapters — custom map/filter/take *)

(* Custom Seq-based adapters *)
let rec my_map f s () =
  match s () with
  | Seq.Nil -> Seq.Nil
  | Seq.Cons (x, rest) -> Seq.Cons (f x, my_map f rest)

let rec my_filter p s () =
  match s () with
  | Seq.Nil -> Seq.Nil
  | Seq.Cons (x, rest) ->
    if p x then Seq.Cons (x, my_filter p rest)
    else my_filter p rest ()

let rec my_take n s () =
  if n <= 0 then Seq.Nil
  else match s () with
    | Seq.Nil -> Seq.Nil
    | Seq.Cons (x, rest) -> Seq.Cons (x, my_take (n - 1) rest)

let rec my_skip n s () =
  if n <= 0 then s ()
  else match s () with
    | Seq.Nil -> Seq.Nil
    | Seq.Cons (_, rest) -> my_skip (n - 1) rest ()

(* Compose adapters *)
let naturals =
  let rec aux n () = Seq.Cons (n, aux (n + 1)) in
  aux 0

let even_squares =
  naturals
  |> my_filter (fun x -> x mod 2 = 0)
  |> my_map (fun x -> x * x)
  |> my_take 5

(* Tests *)
let () =
  let to_list s = List.of_seq s in
  let range = Seq.init 5 (fun i -> i) in
  assert (to_list (my_map (fun x -> x * 2) range) = [0; 2; 4; 6; 8]);
  assert (to_list (my_filter (fun x -> x > 2) range) = [3; 4]);
  assert (to_list (my_take 3 range) = [0; 1; 2]);
  assert (to_list (my_skip 3 range) = [3; 4]);
  assert (to_list even_squares = [0; 4; 16; 36; 64]);
  Printf.printf "✓ All tests passed\n"
