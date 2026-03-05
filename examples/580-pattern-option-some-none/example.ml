(* Option idioms in OCaml *)
let (let*) = Option.bind

let safe_div a b = if b=0 then None else Some(a/b)
let safe_sqrt x = if x < 0.0 then None else Some(sqrt x)

let compute a b =
  let* q = safe_div a b in
  let* r = safe_sqrt (float_of_int q) in
  Some (r *. 2.0)

let () =
  let show label = function None->Printf.printf "%s: None\n" label
    | Some v -> Printf.printf "%s: %.2f\n" label v in
  show "10/2" (compute 10 2);
  show "10/0" (compute 10 0);
  show "-4/2" (compute (-4) 2);
  let names = [Some"alice";None;Some"bob"] in
  let upper = List.filter_map (Option.map String.capitalize_ascii) names in
  List.iter print_endline upper
