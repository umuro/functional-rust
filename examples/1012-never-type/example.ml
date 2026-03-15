(* 1012: The Never Type *)
(* OCaml doesn't have a never/bottom type, but we can simulate *)

(* Approach 1: Functions that never return — use 'a return type *)
(* In OCaml, exit/raise have type 'a (universally quantified = bottom) *)

let diverge_exit () : 'a =
  Printf.printf "about to exit\n";
  exit 1

let diverge_raise () : 'a =
  failwith "this never returns"

(* Approach 2: Empty variant type (closest to Rust's !) *)
type never = |  (* empty variant — no constructors, uninhabitable *)

(* A function returning never can never actually return *)
(* let impossible () : never = ??? — can't construct it! *)

(* Using in match for exhaustiveness *)
let handle_result (r : (int, never) result) : int =
  match r with
  | Ok n -> n
  (* Error case is unreachable — never has no constructors *)

(* Approach 3: Diverging in match arms *)
let classify n =
  match n with
  | n when n > 0 -> Printf.sprintf "positive: %d" n
  | n when n < 0 -> Printf.sprintf "negative: %d" n
  | 0 -> "zero"
  | _ -> failwith "unreachable"  (* type 'a unifies with string *)

let test_diverging () =
  (* We can't test that diverge_exit/diverge_raise return,
     but we can test they have the right type *)
  (try let _ = diverge_raise () in assert false
   with Failure _ -> ());
  Printf.printf "  Approach 1 (diverging functions): passed\n"

let test_empty_variant () =
  (* never type makes Error branch unreachable *)
  assert (handle_result (Ok 42) = 42);
  Printf.printf "  Approach 2 (empty variant): passed\n"

let test_match_diverge () =
  assert (classify 5 = "positive: 5");
  assert (classify (-3) = "negative: -3");
  assert (classify 0 = "zero");
  Printf.printf "  Approach 3 (diverging match arms): passed\n"

let () =
  Printf.printf "Testing never type:\n";
  test_diverging ();
  test_empty_variant ();
  test_match_diverge ();
  Printf.printf "✓ All tests passed\n"
