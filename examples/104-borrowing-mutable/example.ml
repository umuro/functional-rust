(* 104: Mutable Borrowing *)
(* OCaml: mutable fields, but no borrow checker *)

(* Approach 1: Mutable ref *)
let increment r = r := !r + 1

let demo_mut () =
  let x = ref 0 in
  increment x;
  increment x;
  !x

(* Approach 2: Mutable array — aliasing is possible! *)
let demo_alias () =
  let arr = [|1; 2; 3|] in
  let alias = arr in  (* same array! *)
  alias.(0) <- 99;
  arr.(0)  (* returns 99 — aliased mutation! *)

(* In Rust, the borrow checker prevents this aliasing *)

(* Tests *)
let () =
  assert (demo_mut () = 2);
  assert (demo_alias () = 99);  (* OCaml allows this! *)
  Printf.printf "✓ All tests passed\n"
