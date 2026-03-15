module IS = Set.Make(Int)

let sum_of_multiples factors limit =
  List.fold_left (fun s factor ->
    if factor = 0 then s
    else
      let multiples = List.init ((limit - 1) / factor) (fun i -> factor * (i + 1)) in
      List.fold_left (fun s m -> IS.add m s) s multiples
  ) IS.empty factors
  |> IS.fold (+) |> fun f -> f 0

let () =
  assert (sum_of_multiples [3; 5] 1000 = 233168);
  assert (sum_of_multiples [2; 3; 5; 7; 11] 10000 = 23331668);
  assert (sum_of_multiples [] 1000 = 0);
  assert (sum_of_multiples [0; 3] 10 = 18);
  Printf.printf "%d\n" (sum_of_multiples [3; 5] 1000);
  Printf.printf "%d\n" (sum_of_multiples [2; 3; 5; 7; 11] 10000);
  print_endline "ok"
