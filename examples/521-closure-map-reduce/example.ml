(* Map-reduce patterns in OCaml *)

(* Basic map-reduce *)
let map_reduce mapper reducer init xs =
  List.fold_left reducer init (List.map mapper xs)

(* Word frequency count *)
let word_count words =
  List.fold_left (fun acc word ->
    let count = try Hashtbl.find acc word with Not_found -> 0 in
    Hashtbl.replace acc word (count + 1);
    acc
  ) (Hashtbl.create 16) words

let () =
  let nums = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10] in

  (* Sum of squares *)
  let sum_sq = map_reduce (fun x -> x * x) (+) 0 nums in
  Printf.printf "sum of squares: %d\n" sum_sq;

  (* Max element after transform *)
  let max_val = List.fold_left max min_int (List.map (fun x -> x * 3 - 7) nums) in
  Printf.printf "max(3x-7): %d\n" max_val;

  (* String reduction *)
  let words = ["hello"; "world"; "foo"; "bar"; "hello"; "foo"; "foo"] in
  let freq = word_count words in
  Hashtbl.iter (fun w c -> Printf.printf "%s: %d\n" w c) freq
