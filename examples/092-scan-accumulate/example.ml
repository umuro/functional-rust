(* 092: Scan with Accumulator *)

let scan_left f init lst =
  let _, result =
    List.fold_left (fun (acc, res) x ->
      let next = f acc x in (next, next :: res)
    ) (init, [init]) lst
  in
  List.rev result

let running_sum lst = scan_left ( + ) 0 lst
let running_max lst =
  match lst with
  | [] -> []
  | x :: xs -> scan_left max x xs

(* Tests *)
let () =
  assert (running_sum [1; 2; 3; 4] = [0; 1; 3; 6; 10]);
  assert (running_max [3; 1; 4; 1; 5] = [3; 3; 4; 4; 5]);
  Printf.printf "✓ All tests passed\n"
