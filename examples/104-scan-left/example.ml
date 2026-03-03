let scan_left f init lst =
  let _, result = List.fold_left (fun (acc, res) x ->
    let acc' = f acc x in
    (acc', acc' :: res)
  ) (init, [init]) lst in
  List.rev result

let running_sum = scan_left (+) 0
let running_max = scan_left max min_int

let () =
  List.iter (Printf.printf "%d ") (running_sum [1;2;3;4;5]);
  print_newline ();
  List.iter (Printf.printf "%d ") (running_max [3;1;4;1;5;9;2;6])
