(* 101: Move Semantics
   OCaml uses garbage collection — there is no ownership or move semantics.
   All values are freely shared; strings and lists are heap-allocated but
   immutable by default. This example contrasts OCaml's model with Rust's:
   - In OCaml every binding is a reference to a heap value
   - "Moving" simply means re-binding the name; both names are valid
   - There is no concept of "invalidating" a binding after assignment

   The closest OCaml concept: passing a value to a function does not
   invalidate the caller's reference — sharing is always safe. *)

(* In OCaml, functions receive a reference — the original is still valid *)
let take_ownership s =
  String.length s  (* "consuming" the string — but original still alive *)

(* "Copy" types — in OCaml all primitive values are value-copied *)
let demonstrate_value_sharing () =
  let x = 42 in
  let y = x in  (* x and y are independent copies *)
  assert (x = 42);
  assert (y = 42)

(* Strings are heap-allocated but immutable — sharing is safe *)
let demonstrate_string_sharing () =
  let s1 = "hello" in
  let s2 = s1 in  (* both names point to same string — no "move" needed *)
  assert (String.length s1 = String.length s2);
  assert (s1 = s2)

(* Lists are persistent data structures — sharing sub-lists is safe *)
let demonstrate_list_sharing () =
  let v1 = [1; 2; 3] in
  let v2 = v1 in  (* sharing the same list — perfectly safe *)
  assert (v1 = v2);
  let v3 = 0 :: v1 in  (* prepend creates new list; v1 unchanged *)
  assert (List.hd v3 = 0);
  assert (v1 = [1; 2; 3])

let create_string () = "created"

let () =
  assert (take_ownership "hello" = 5);
  demonstrate_value_sharing ();
  demonstrate_string_sharing ();
  demonstrate_list_sharing ();
  assert (create_string () = "created");
  Printf.printf "No ownership transfers needed — OCaml uses GC\n"
