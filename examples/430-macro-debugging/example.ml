(* Debugging macros in OCaml *)

(* OCaml macro debugging via ppx dump flags *)
(* cargo-like: ocamlfind ocamlopt -package ppx_deriving -ppx ... *)

(* Debugging technique: add Printf to macro output *)
let debug_macro_expand name value =
  Printf.printf "[MACRO EXPAND] %s => %s\n" name value

(* Verify expansion produces expected results *)
let test_macro_output name actual expected =
  if actual = expected then
    Printf.printf "[OK] %s\n" name
  else
    Printf.printf "[FAIL] %s: expected %s, got %s\n" name expected actual

let () =
  (* Simulate macro that we're debugging *)
  debug_macro_expand "stringify!(x + y)" ""x + y"";
  debug_macro_expand "concat!("a", "b")" ""ab"";

  test_macro_output "simple add" (string_of_int (1 + 2)) "3";
  test_macro_output "concat" ("foo" ^ "bar") "foobar"
