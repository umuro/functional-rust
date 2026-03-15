(* 097: Flatten *)

let flatten lst = List.concat lst

let flat_map f lst = List.concat_map f lst

(* Approach: flatten nested sequences *)
let flatten_seq ss =
  Seq.flat_map (fun s -> s) ss

(* Tests *)
let () =
  assert (flatten [[1;2]; [3;4]; [5]] = [1;2;3;4;5]);
  assert (flat_map (fun x -> [x; x*10]) [1;2;3] = [1;10;2;20;3;30]);
  assert (flatten [[]; [1]; []; [2;3]] = [1;2;3]);
  Printf.printf "✓ All tests passed\n"
