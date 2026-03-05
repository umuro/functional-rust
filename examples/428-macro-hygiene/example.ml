(* Macro hygiene in OCaml *)
(* ppx macros are NOT hygienic by default — must use gensym *)

(* Simulate hygiene problem in a "macro" *)
let x = 10

(* Non-hygienic: would capture outer x if expanded naively *)
let expand_macro_bad outer =
  (* This "macro expansion" reuses name 'x' — captured! *)
  let x = outer * 2 in
  x + x  (* uses inner x, not outer *)

(* Hygienic approach: use fresh names *)
let expand_macro_good outer =
  let inner_result = outer * 2 in
  inner_result + inner_result

let () =
  Printf.printf "outer x = %d\n" x;
  Printf.printf "macro result = %d\n" (expand_macro_good 5);
  Printf.printf "outer x unchanged = %d\n" x
