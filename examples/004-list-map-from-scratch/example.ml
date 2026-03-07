(* Build map by abstracting over the per-element operation *)
let rec map f = function
  | []     -> []
  | h :: t ->
    let h' = f h in          (* evaluate before recursing for left-to-right order *)
    h' :: map f t

(* Partial application creates specialised transformers *)
let add1      = map (fun x -> x + 1)
let to_string = map string_of_int
let double    = map (fun x -> x * 2)

let () =
  let nums = [1; 2; 3; 4; 5] in
  List.iter (Printf.printf "%d ") (add1   nums); print_newline ();
  List.iter (Printf.printf "%s ") (to_string nums); print_newline ();
  List.iter (Printf.printf "%d ") (double  nums); print_newline ()