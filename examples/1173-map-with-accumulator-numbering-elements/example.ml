(* Map with Accumulator — Numbering Elements *)
(* Stateful mapping patterns *)

let number_list lst =
  let rec aux n = function
    | [] -> []
    | x :: xs -> (n, x) :: aux (n + 1) xs
  in aux 1 lst

let indexed = number_list ["alpha"; "beta"; "gamma"; "delta"]
let () = List.iter (fun (i, s) -> Printf.printf "%d. %s\n" i s) indexed

(* Using fold for running stats *)
let running_avg lst =
  let (_, avgs) = List.fold_left (fun (sum, acc) x ->
    let sum' = sum +. x in
    let n = float_of_int (List.length acc + 1) in
    (sum', acc @ [sum' /. n])
  ) (0.0, []) lst
  in avgs

let avgs = running_avg [10.0; 20.0; 30.0; 40.0]
let () = List.iter (fun x -> Printf.printf "%.1f " x) avgs
