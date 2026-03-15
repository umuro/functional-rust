(* Unfold — Generating Sequences from Seeds *)
(* The dual of fold: produces a list from a seed value *)

let rec unfold f seed = match f seed with
  | None -> []
  | Some (value, next_seed) -> value :: unfold f next_seed

let range a b =
  unfold (fun i -> if i > b then None else Some (i, i + 1)) a

let countdown n =
  unfold (fun i -> if i < 0 then None else Some (i, i - 1)) n

let collatz n =
  unfold (fun x ->
    if x = 1 then Some (1, 0)
    else if x = 0 then None
    else Some (x, if x mod 2 = 0 then x / 2 else 3 * x + 1)
  ) n

let () =
  List.iter (Printf.printf "%d ") (range 1 5);
  print_newline ();
  List.iter (Printf.printf "%d ") (collatz 6);
  print_newline ();
  assert (range 1 5 = [1;2;3;4;5]);
  assert (countdown 3 = [3;2;1;0]);
  Printf.printf "All unfold tests passed!\n"
