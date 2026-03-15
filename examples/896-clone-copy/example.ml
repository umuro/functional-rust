(* Example 102: Clone vs Copy — OCaml Implicit Sharing → Rust Explicit Cloning *)

(* In OCaml, all values are implicitly shared via GC.
   There's no distinction between "shallow copy" and "deep copy" at the language level. *)

(* Approach 1: Sharing structured data — always implicit *)
type point = { x : float; y : float }

let translate p dx dy =
  { x = p.x +. dx; y = p.y +. dy }

let approach1 () =
  let origin = { x = 0.0; y = 0.0 } in
  let moved = translate origin 1.0 2.0 in
  (* origin is still valid — OCaml creates new record, GC manages old *)
  assert (origin.x = 0.0);
  assert (moved.x = 1.0);
  Printf.printf "Origin: (%.1f, %.1f), Moved: (%.1f, %.1f)\n"
    origin.x origin.y moved.x moved.y

(* Approach 2: List sharing — structural sharing *)
let approach2 () =
  let xs = [1; 2; 3] in
  let ys = 0 :: xs in  (* ys shares the tail with xs *)
  assert (List.length xs = 3);
  assert (List.length ys = 4);
  Printf.printf "xs = %s, ys = %s\n"
    (String.concat "; " (List.map string_of_int xs))
    (String.concat "; " (List.map string_of_int ys))

(* Approach 3: Deep copy via Marshal (rarely needed) *)
let deep_copy x =
  Marshal.from_string (Marshal.to_string x [Marshal.Closures]) 0

let approach3 () =
  let data = [| [1; 2]; [3; 4] |] in
  let copy = deep_copy data in
  assert (data = copy);
  Printf.printf "Deep copied array of lists\n"

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
