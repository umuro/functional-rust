(* Natural Transformations in OCaml *)

(* Option => List *)
let option_to_list = function
  | None -> []
  | Some x -> [x]

(* List => Option (head) *)
let list_to_option = function
  | [] -> None
  | x :: _ -> Some x

(* Naturality: η_B ∘ F(f) = G(f) ∘ η_A *)
let verify_naturality () =
  let f x = x * 2 in
  let a = Some 10 in
  
  (* Route 1: map then transform *)
  let r1 = option_to_list (Option.map f a) in
  
  (* Route 2: transform then map *)
  let r2 = List.map f (option_to_list a) in
  
  r1 = r2

let () =
  Printf.printf "option_to_list (Some 42): [%s]\n"
    (String.concat "; " (List.map string_of_int (option_to_list (Some 42))));
  Printf.printf "Naturality: %b\n" (verify_naturality ())
