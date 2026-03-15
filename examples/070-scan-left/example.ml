(* 070: Scan Left — running accumulation
   scan_left produces all intermediate fold results *)

(* --- Approach 1: Manual scan_left --- *)

(* Returns [init; f init x0; f (f init x0) x1; ...] *)
let scan_left f init xs =
  let rec aux acc prev = function
    | [] -> List.rev acc
    | x :: rest ->
      let next = f prev x in
      aux (next :: acc) next rest
  in
  aux [init] init xs

let running_sum xs = scan_left ( + ) 0 xs

let running_product xs = scan_left ( * ) 1 xs

(* --- Approach 2: Running max (same idea, different operator) --- *)

let running_max = function
  | [] -> []
  | first :: rest ->
    scan_left max first rest

(* --- Approach 3: Using Seq for lazy running sums --- *)

let running_sum_seq xs =
  let s = List.to_seq xs in
  Seq.scan ( + ) 0 s |> List.of_seq

let () =
  Printf.printf "running_sum [1;2;3;4] = [%s]\n"
    (String.concat "; " (List.map string_of_int (running_sum [1;2;3;4])));
  Printf.printf "running_product [1;2;3;4] = [%s]\n"
    (String.concat "; " (List.map string_of_int (running_product [1;2;3;4])));
  Printf.printf "running_max [3;1;4;1;5;9] = [%s]\n"
    (String.concat "; " (List.map string_of_int (running_max [3;1;4;1;5;9])));
  Printf.printf "running_sum_seq [1;2;3;4] = [%s]\n"
    (String.concat "; " (List.map string_of_int (running_sum_seq [1;2;3;4])))
