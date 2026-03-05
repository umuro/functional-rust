(* Topos concepts in OCaml *)

(* Subobject classifier *)
type omega = True | False

(* Characteristic function *)
let char_fn subset x =
  if List.mem x subset then True else False

(* Power object operations *)
module Power = struct
  let union a b = 
    List.sort_uniq compare (a @ b)
  
  let intersection a b =
    List.filter (fun x -> List.mem x b) a
  
  let to_char_fn set =
    fun x -> if List.mem x set then True else False
end

(* Internal logic *)
let omega_and a b = match a, b with
  | True, True -> True
  | _ -> False

let omega_or a b = match a, b with
  | False, False -> False
  | _ -> True

let omega_implies a b = match a, b with
  | True, False -> False
  | _ -> True

let () =
  let subset = [1; 3; 5] in
  let chi = char_fn subset in
  List.iter (fun x ->
    let v = match chi x with True -> "True" | False -> "False" in
    Printf.printf "χ(%d) = %s\n" x v
  ) [1; 2; 3; 4; 5]
