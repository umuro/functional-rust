(* Example 105: Lifetime Basics — Why OCaml Doesn't Need Them *)

(* OCaml's GC ensures values live as long as they're referenced.
   No dangling pointers possible. *)

(* Approach 1: Returning references to local data — safe in OCaml *)
let make_greeting name =
  let greeting = "Hello, " ^ name ^ "!" in
  greeting  (* returned value kept alive by GC *)

let approach1 () =
  let g = make_greeting "Alice" in
  Printf.printf "%s\n" g;
  assert (g = "Hello, Alice!")

(* Approach 2: Storing references in data structures *)
let longest a b =
  if String.length a >= String.length b then a else b

let approach2 () =
  let result =
    let x = "long string" in
    let y = "short" in
    longest x y
  in
  Printf.printf "Longest: %s\n" result;
  assert (result = "long string")

(* Approach 3: Closures capturing local values *)
let make_adder n =
  fun x -> x + n  (* n is captured, kept alive by GC *)

let approach3 () =
  let add5 = make_adder 5 in
  let result = add5 10 in
  assert (result = 15);
  Printf.printf "add5(10) = %d\n" result

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
