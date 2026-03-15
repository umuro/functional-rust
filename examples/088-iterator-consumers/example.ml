(* 088: Iterator Consumers — terminal operations that drive a lazy sequence
   OCaml stdlib: List.fold_left, List.for_all, List.exists, List.length, etc.
   For Seq: Seq.fold_left, Seq.find, Seq.iter *)

let range a b = List.init (b - a) (fun i -> i + a)

let () =
  let xs = range 1 6 in   (* [1;2;3;4;5] *)

  (* sum *)
  Printf.printf "sum [1..5] = %d\n" (List.fold_left ( + ) 0 xs);

  (* product *)
  Printf.printf "product [1..5] = %d\n" (List.fold_left ( * ) 1 xs);

  (* count / length *)
  Printf.printf "count [0..9] = %d\n" (List.length (range 0 10));

  (* collect — List.map is lazy → eager, already a "collect" *)
  let collected = List.map (fun x -> x) (range 0 3) in
  Printf.printf "collected [0;1;2] = [%s]\n"
    (String.concat "; " (List.map string_of_int collected));

  (* fold *)
  Printf.printf "fold (+) 0 [1..5] = %d\n" (List.fold_left ( + ) 0 xs);

  (* min / max *)
  Printf.printf "min [3;1;4;1;5] = %d\n" (List.fold_left min max_int [3;1;4;1;5]);
  Printf.printf "max [3;1;4;1;5] = %d\n" (List.fold_left max min_int [3;1;4;1;5]);

  (* any (exists) / all (for_all) *)
  Printf.printf "any (>3) [1;2;3;4] = %b\n" (List.exists  (fun x -> x > 3) [1;2;3;4]);
  Printf.printf "any (>10) [1;2;3] = %b\n"  (List.exists  (fun x -> x > 10) [1;2;3]);
  Printf.printf "all (>0) [1;2;3] = %b\n"   (List.for_all (fun x -> x > 0) [1;2;3]);
  Printf.printf "all (>2) [1;2;3] = %b\n"   (List.for_all (fun x -> x > 2) [1;2;3]);

  (* collect to string — concat strings *)
  let s = String.concat "" ["a"; "b"; "c"] in
  Printf.printf "concat strings = %s\n" s
