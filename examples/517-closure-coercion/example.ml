(* OCaml: no distinction between function pointers and closures *)
(* All functions are values with the same representation *)

let double x = x * 2
let triple x = x * 3
let negate x = -x

(* Array of functions (all same type) *)
let transforms = [| double; triple; negate; (fun x -> x + 10) |]

(* Higher-order that takes a raw function *)
let apply_all fns x =
  Array.map (fun f -> f x) fns

let () =
  let results = apply_all transforms 5 in
  Array.iter (fun r -> Printf.printf "%d " r) results;
  print_newline ();

  (* Closures and named functions are the same type *)
  let add5 = fun x -> x + 5 in  (* closure, same type as double *)
  Printf.printf "add5(10) = %d, double(10) = %d\n" (add5 10) (double 10)
