(* Accumulate — Custom Map *)

(* Version 1: Simple recursive *)
let rec accumulate f = function
  | [] -> []
  | h :: t -> f h :: accumulate f t

(* Version 2: Tail-recursive with accumulator *)
let accumulate_tr f lst =
  let rec go acc = function
    | [] -> List.rev acc
    | h :: t -> go (f h :: acc) t
  in go [] lst

let () =
  assert (accumulate (fun x -> x * x) [1;2;3;4;5] = [1;4;9;16;25]);
  assert (accumulate_tr String.uppercase_ascii ["hello";"world"] = ["HELLO";"WORLD"])
