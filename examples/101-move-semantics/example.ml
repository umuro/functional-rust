(* 101: Move Semantics *)
(* OCaml: GC handles everything. No move semantics. *)

(* Approach 1: OCaml — values are freely shared via GC *)
let use_string s = String.length s
let share_freely () =
  let s = "hello" in
  let a = use_string s in   (* s is still valid *)
  let b = use_string s in   (* s is still valid *)
  a + b

(* Approach 2: Simulating ownership with linear types *)
(* OCaml doesn't enforce this, but we can model it *)
type 'a owned = Owned of 'a | Moved

let take (r : 'a owned ref) =
  match !r with
  | Owned v -> r := Moved; v
  | Moved -> failwith "value already moved"

let demo_simulated_move () =
  let r = ref (Owned "hello") in
  let s = take r in
  (* take r again would fail *)
  String.length s

(* Tests *)
let () =
  assert (share_freely () = 10);
  assert (demo_simulated_move () = 5);
  Printf.printf "✓ All tests passed\n"
