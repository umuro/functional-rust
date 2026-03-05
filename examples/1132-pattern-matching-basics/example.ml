(* Pattern Matching Basics *)
(* Match expressions for structured data decomposition *)

let describe_list = function
  | [] -> "empty"
  | [x] -> Printf.sprintf "singleton: %d" x
  | [x; y] -> Printf.sprintf "pair: %d, %d" x y
  | x :: _ -> Printf.sprintf "starts with %d" x

let () =
  List.iter (fun lst ->
    Printf.printf "%s\n" (describe_list lst)
  ) [[]; [1]; [2;3]; [4;5;6]]
