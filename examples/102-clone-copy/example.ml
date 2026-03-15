(* 102: Clone vs Copy *)
(* OCaml: GC + structural sharing. No explicit clone/copy needed. *)

(* Approach 1: Everything is "shared" in OCaml *)
let copy_int x = x       (* trivial — same as identity *)
let copy_list lst = lst   (* structural sharing via GC *)

(* Approach 2: Deep copy when needed *)
let deep_copy_list lst = List.map Fun.id lst
let deep_copy_array arr = Array.copy arr

(* Approach 3: Mutable values need explicit copy *)
let demo_array_copy () =
  let a = [|1; 2; 3|] in
  let b = Array.copy a in
  b.(0) <- 99;
  (* a is unchanged *)
  assert (a.(0) = 1);
  assert (b.(0) = 99)

(* Tests *)
let () =
  assert (copy_int 42 = 42);
  let lst = [1; 2; 3] in
  assert (copy_list lst = [1; 2; 3]);
  assert (deep_copy_list lst = [1; 2; 3]);
  demo_array_copy ();
  Printf.printf "✓ All tests passed\n"
