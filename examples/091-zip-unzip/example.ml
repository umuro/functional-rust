(* 091: Zip and Unzip
   OCaml: List.combine / List.split, Seq.zip *)

(* --- Approach 1: zip (List.combine) --- *)

let zip xs ys = List.combine xs ys   (* pairs truncated to shorter length via List.map2 *)

(* zip that silently truncates to shorter list length *)
let zip_truncate xs ys =
  let rec aux = function
    | [], _ | _, [] -> []
    | x :: xs, y :: ys -> (x, y) :: aux (xs, ys)
  in
  aux (xs, ys)

(* --- Approach 2: unzip (List.split) --- *)

let unzip pairs = List.split pairs

(* --- Approach 3: zip_with — zip and apply a function in one pass --- *)

let zip_with f xs ys =
  List.map2 f xs ys

let () =
  (* zip equal-length lists *)
  let pairs = zip_truncate [1;2;3] ["a";"b";"c"] in
  List.iter (fun (n, s) -> Printf.printf "(%d, %s) " n s) pairs;
  print_newline ();

  (* zip unequal lengths — truncates *)
  let short = zip_truncate [1;2] [10;20;30] in
  Printf.printf "zip unequal length = %d\n" (List.length short);

  (* unzip *)
  let (a, b) = unzip [(1,"a"); (2,"b")] in
  Printf.printf "unzip fst = [%s]\n" (String.concat "; " (List.map string_of_int a));
  Printf.printf "unzip snd = [%s]\n" (String.concat "; " b);

  (* zip_with (+) *)
  let sums = zip_with ( + ) [1;2;3] [10;20;30] in
  Printf.printf "zip_with (+) = [%s]\n"
    (String.concat "; " (List.map string_of_int sums));

  (* Seq.zip in OCaml 4.14+ *)
  let zipped = Seq.zip (List.to_seq [1;2;3]) (List.to_seq ["x";"y";"z"]) in
  Seq.iter (fun (n, s) -> Printf.printf "%d-%s " n s) zipped;
  print_newline ()
