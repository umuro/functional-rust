(* List — Scan (Running Accumulation) *)
(* Compute running totals with scan *)

(* scan_left: like fold but keeps all intermediate results *)
let scan_left f init lst =
  let rec aux acc last = function
    | [] -> List.rev acc
    | x :: xs ->
      let next = f last x in
      aux (next :: acc) next xs
  in List.rev (init :: List.rev (aux [] init lst))

let running_sum = scan_left ( + ) 0 [1; 2; 3; 4; 5]
let running_max = scan_left max min_int [3; 1; 4; 1; 5; 9; 2; 6]

let () =
  Printf.printf "Running sum: %s\n"
    (String.concat " " (List.map string_of_int running_sum));
  Printf.printf "Running max: %s\n"
    (String.concat " " (List.map string_of_int running_max))
