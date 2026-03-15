(* 092: Scan with Accumulator
   Running prefix computations — scan_left produces all intermediate fold results *)

(* --- Approach 1: Running sum using Seq.scan --- *)

let running_sum xs =
  xs
  |> List.to_seq
  |> Seq.scan ( + ) 0
  |> List.of_seq

(* --- Approach 2: Running max --- *)

let running_max = function
  | [] -> []
  | first :: rest ->
    List.fold_left
      (fun (max_so_far, acc) x ->
        let new_max = max max_so_far x in
        (new_max, new_max :: acc))
      (first, [first])
      rest
    |> (fun (_, acc) -> List.rev acc)

(* --- Approach 3: Generic scan_left --- *)

let scan_left f init xs =
  xs
  |> List.to_seq
  |> Seq.scan f init
  |> List.of_seq

let () =
  Printf.printf "running_sum [1;2;3;4] = [%s]\n"
    (String.concat "; " (List.map string_of_int (running_sum [1;2;3;4])));

  Printf.printf "running_max [3;1;4;1;5] = [%s]\n"
    (String.concat "; " (List.map string_of_int (running_max [3;1;4;1;5])));

  Printf.printf "scan_left (*) 1 [1;2;3;4] = [%s]\n"
    (String.concat "; " (List.map string_of_int (scan_left ( * ) 1 [1;2;3;4])));

  Printf.printf "running_sum [] = [%s]\n"
    (String.concat "; " (List.map string_of_int (running_sum [])))
