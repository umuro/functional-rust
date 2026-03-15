(* 005: Reverse a List
   Multiple idiomatic OCaml approaches *)

(* --- Approach 1: Stdlib List.rev --- *)
let reverse_stdlib xs = List.rev xs

(* --- Approach 2: Fold-based — fold_left builds reversed list naturally --- *)
let reverse_fold xs =
  (* fold_left processes left-to-right; consing onto acc reverses *)
  List.fold_left (fun acc x -> x :: acc) [] xs

(* --- Approach 3: Explicit tail-recursive accumulator --- *)
let reverse_acc xs =
  let rec aux acc = function
    | [] -> acc
    | x :: rest -> aux (x :: acc) rest
  in
  aux [] xs

(* --- Approach 4: Naive recursive (not tail-recursive, for illustration) --- *)
let rec reverse_naive = function
  | [] -> []
  | x :: xs -> reverse_naive xs @ [x]

let () =
  let v = [1; 2; 3; 4; 5] in
  let show xs = "[" ^ String.concat "; " (List.map string_of_int xs) ^ "]" in
  Printf.printf "stdlib:   %s\n" (show (reverse_stdlib v));
  Printf.printf "fold:     %s\n" (show (reverse_fold v));
  Printf.printf "acc:      %s\n" (show (reverse_acc v));
  Printf.printf "naive:    %s\n" (show (reverse_naive v));
  Printf.printf "empty:    %s\n" (show (reverse_stdlib []))
