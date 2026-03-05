(* Functor Composition in OCaml *)

(* Option (List a) functor *)
let map_option_list f ol =
  Option.map (List.map f) ol

(* List (Option a) functor *)
let map_list_option f lo =
  List.map (Option.map f) lo

(* Composition law: map (f . g) = map f . map g *)
let verify_composition_law () =
  let f x = x + 1 in
  let g x = x * 2 in
  let data = Some [1; 2; 3] in
  
  let r1 = map_option_list (fun x -> g (f x)) data in
  let r2 = map_option_list g (map_option_list f data) in
  r1 = r2

let () =
  let ov = Some [1; 2; 3] in
  let result = map_option_list (fun x -> x * 2) ov in
  (match result with
   | None -> print_endline "None"
   | Some xs -> Printf.printf "Some [%s]\n" 
       (String.concat "; " (List.map string_of_int xs)));
  Printf.printf "Composition law: %b\n" (verify_composition_law ())
