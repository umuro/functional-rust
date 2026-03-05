(* OCaml: array optimization via unboxed arrays *)

(* OCaml arrays of floats/ints are already unboxed *)
let small_array = [|1;2;3;4|]  (* stack-like, unboxed *)

(* For dynamic small collections, use list or array *)
let push_small arr x = Array.append arr [|x|]

let () =
  let a = ref small_array in
  a := push_small !a 5;
  a := push_small !a 6;
  Array.iter (Printf.printf "%d ") !a; print_newline ();
  Printf.printf "Length: %d\n" (Array.length !a)
