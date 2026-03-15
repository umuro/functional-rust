(* Example 087: Iterator Adapters *)
(* Chaining map/filter/flat_map/take/skip *)

(* Approach 1: List pipeline *)
let pipeline data =
  data
  |> List.filter (fun x -> x > 0)
  |> List.map (fun x -> x * x)
  |> List.map string_of_int

let flat_map_example data =
  data
  |> List.map (fun s -> String.split_on_char ' ' s)
  |> List.flatten

(* Approach 2: Seq-based lazy pipeline *)
let seq_pipeline data =
  data
  |> List.to_seq
  |> Seq.filter (fun x -> x mod 2 = 0)
  |> Seq.map (fun x -> x * 3)
  |> Seq.take 5
  |> List.of_seq

let seq_skip n seq =
  let rec aux n seq =
    if n = 0 then seq
    else match seq () with
    | Seq.Nil -> fun () -> Seq.Nil
    | Seq.Cons (_, rest) -> aux (n - 1) rest
  in
  aux n seq

let seq_flat_map f seq =
  Seq.flat_map f seq

(* Approach 3: Complex chained pipeline *)
let word_lengths text =
  text
  |> String.split_on_char ' '
  |> List.filter (fun s -> String.length s > 0)
  |> List.map String.length
  |> List.sort compare

let top_n n transform data =
  data
  |> List.map transform
  |> List.sort (fun a b -> compare b a)
  |> List.filteri (fun i _ -> i < n)

(* Tests *)
let () =
  assert (pipeline [3; -1; 4; -5; 2] = ["9"; "16"; "4"]);

  assert (flat_map_example ["hello world"; "foo bar"] = ["hello"; "world"; "foo"; "bar"]);

  let result = seq_pipeline [1;2;3;4;5;6;7;8;9;10;11;12;13;14] in
  assert (result = [6; 12; 18; 24; 30]);

  let skipped = List.of_seq (seq_skip 3 (List.to_seq [1;2;3;4;5])) in
  assert (skipped = [4; 5]);

  assert (word_lengths "the quick brown fox" = [3; 3; 4; 5]);

  let top3 = top_n 3 (fun x -> x * x) [1; 5; 3; 2; 4] in
  assert (top3 = [25; 16; 9]);

  Printf.printf "✓ All tests passed\n"
