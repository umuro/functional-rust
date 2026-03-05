(* Anamorphism in OCaml *)

(* unfold: generate a list from a seed *)
let unfold f seed =
  let rec go acc s =
    match f s with
    | None        -> List.rev acc
    | Some (x, s') -> go (x :: acc) s'
  in go [] seed

let range lo hi    = unfold (fun i -> if i >= hi then None else Some (i, i+1)) lo
let fibs max_n     = unfold (fun (a,b) -> if a > max_n then None else Some (a, (b, a+b))) (0,1)
let to_digits n    = unfold (fun n -> if n=0 then None else Some (n mod 10, n/10)) n

let () =
  Printf.printf "range 1..5: %s\n" (String.concat "," (List.map string_of_int (range 1 6)));
  Printf.printf "fibs<=100: %s\n" (String.concat "," (List.map string_of_int (fibs 100)));
  Printf.printf "digits 1234: %s\n" (String.concat "," (List.map string_of_int (to_digits 1234)))
