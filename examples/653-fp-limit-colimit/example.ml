(* Limits and Colimits in OCaml *)

(* Product = Limit *)
type ('a, 'b) product = 'a * 'b

(* Coproduct = Colimit *)
type ('a, 'b) coproduct = Left of 'a | Right of 'b

(* Terminal object *)
type terminal = unit

(* Initial object *)
type initial = |  (* empty type *)

(* Equalizer *)
let equalizer f g xs =
  List.filter (fun x -> f x = g x) xs

(* Pullback *)
let pullback f g xs ys =
  List.concat_map (fun a ->
    List.filter_map (fun b ->
      if f a = g b then Some (a, b) else None
    ) ys
  ) xs

let () =
  (* Equalizer example *)
  let eq = equalizer (fun x -> x mod 2) (fun x -> x mod 4) [0;1;2;3;4;5;6;7;8] in
  Printf.printf "Equalizer: [%s]\n" (String.concat "; " (List.map string_of_int eq));
  
  (* Pullback as JOIN *)
  let users = [(1, "Alice"); (2, "Bob")] in
  let orders = [(1, "Widget"); (1, "Gadget")] in
  let joined = pullback fst fst users orders in
  Printf.printf "Joined: %d pairs\n" (List.length joined)
