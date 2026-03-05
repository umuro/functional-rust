(* Iterator combinators in OCaml using Seq *)

let range a b = Seq.init (b - a) (fun i -> a + i)

let () =
  (* Pipeline: range -> filter -> map -> fold *)
  let sum_of_squares_of_evens =
    range 1 11
    |> Seq.filter (fun x -> x mod 2 = 0)
    |> Seq.map (fun x -> x * x)
    |> Seq.fold_left (+) 0
  in
  Printf.printf "Sum of squares of evens 1..10: %d\n" sum_of_squares_of_evens;

  (* flat_map / concat_map *)
  let pairs =
    range 1 4
    |> Seq.flat_map (fun x -> Seq.map (fun y -> (x, y)) (range 1 4))
    |> Seq.filter (fun (x, y) -> x < y)
    |> List.of_seq
  in
  Printf.printf "Pairs (x<y) from 1..3: %d pairs\n" (List.length pairs);

  (* zip *)
  let names = List.to_seq ["Alice"; "Bob"; "Carol"] in
  let scores = List.to_seq [95; 87; 91] in
  Seq.zip names scores
  |> Seq.iter (fun (n, s) -> Printf.printf "  %s: %d\n" n s)
