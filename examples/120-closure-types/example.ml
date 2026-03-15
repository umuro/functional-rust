(* Example 120: Fn, FnMut, FnOnce — OCaml Closures vs Rust Closure Traits *)

(* OCaml has one closure type. Rust has three traits based on how
   the closure uses captured variables. *)

(* Approach 1: Pure closure — Fn (read-only capture) *)
let make_greeter prefix =
  fun name -> prefix ^ ", " ^ name ^ "!"

let approach1 () =
  let greet = make_greeter "Hello" in
  let r1 = greet "Alice" in
  let r2 = greet "Bob" in
  assert (r1 = "Hello, Alice!");
  assert (r2 = "Hello, Bob!");
  Printf.printf "%s | %s\n" r1 r2

(* Approach 2: Mutating closure — FnMut (mutable capture) *)
let make_counter () =
  let count = ref 0 in
  fun () ->
    incr count;
    !count

let approach2 () =
  let next = make_counter () in
  assert (next () = 1);
  assert (next () = 2);
  assert (next () = 3);
  Printf.printf "Counter: 1, 2, 3 ✓\n"

(* Approach 3: Consuming closure — FnOnce (consumes captured value) *)
let consume_and_greet name =
  let message = "Goodbye, " ^ name ^ "!" in
  fun () -> message  (* captures message — but in OCaml, no "consumption" *)

let approach3 () =
  let farewell = consume_and_greet "World" in
  let msg = farewell () in
  let msg2 = farewell () in  (* In OCaml, we can call again *)
  assert (msg = msg2);
  Printf.printf "%s\n" msg

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
