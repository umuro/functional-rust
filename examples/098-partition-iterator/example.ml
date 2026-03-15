(* 098: Partition *)

let partition pred lst = List.partition pred lst

(* Custom partition_map: either left or right *)
let partition_map f lst =
  List.fold_right (fun x (l, r) ->
    match f x with
    | Either.Left v -> (v :: l, r)
    | Either.Right v -> (l, v :: r)
  ) lst ([], [])

(* Tests *)
let () =
  let (evens, odds) = partition (fun x -> x mod 2 = 0) [1;2;3;4;5;6] in
  assert (evens = [2;4;6]);
  assert (odds = [1;3;5]);
  Printf.printf "✓ All tests passed\n"
