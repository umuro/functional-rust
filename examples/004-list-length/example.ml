(* Find the length of a list *)

(* Naive recursion (stack overflow risk) *)
let rec length_naive = function
  | [] -> 0
  | _ :: t -> 1 + length_naive t

(* Tail-recursive (production-ready) *)
let length list =
  let rec aux n = function
    | [] -> n
    | _ :: t -> aux (n + 1) t
  in
  aux 0 list

(* Tests *)
let () =
  assert (length [] = 0);
  assert (length [1; 2; 3; 4] = 4);
  assert (length (List.init 10000 (fun x -> x)) = 10000);
  print_endline "✓ OCaml tests passed"
