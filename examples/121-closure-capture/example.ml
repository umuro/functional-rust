(* Example 121: Closure Capture Modes *)

(* OCaml closures always capture by reference (GC-managed).
   Rust closures capture by reference, mutable reference, or move. *)

(* Approach 1: Capture by reference (immutable) *)
let approach1 () =
  let x = 42 in
  let f = fun () -> x + 1 in  (* captures x by reference *)
  assert (f () = 43);
  assert (x = 42);  (* x unchanged *)
  Printf.printf "f() = %d, x = %d\n" (f ()) x

(* Approach 2: Capture mutable state *)
let approach2 () =
  let total = ref 0 in
  let add x = total := !total + x in
  add 10; add 20; add 30;
  assert (!total = 60);
  Printf.printf "Total: %d\n" !total

(* Approach 3: Closure outliving local scope *)
let make_multiplier factor =
  fun x -> x * factor  (* factor captured, GC keeps it alive *)

let approach3 () =
  let double = make_multiplier 2 in
  let triple = make_multiplier 3 in
  assert (double 5 = 10);
  assert (triple 5 = 15);
  Printf.printf "double(5)=%d, triple(5)=%d\n" (double 5) (triple 5)

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
