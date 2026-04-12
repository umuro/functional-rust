(* Idiomatic OCaml: List.partition splits a list in one pass *)
let numbers = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10]
let (small, big) = List.partition (fun x -> x <= 5) numbers

(* Partition strings by length *)
let words = ["hi"; "hello"; "ok"; "world"; "rust"]
let (short, long) = List.partition (fun s -> String.length s <= 3) words

let () =
  assert (small = [1; 2; 3; 4; 5]);
  assert (big = [6; 7; 8; 9; 10]);
  assert (List.partition (fun _ -> true) [] = ([], []));
  assert (short = ["hi"; "ok"]);
  assert (long = ["hello"; "world"; "rust"]);
  print_endline "ok"
