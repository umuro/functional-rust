(* 700: Unsafe blocks — OCaml's safe equivalent *)
(* Rust uses unsafe { } blocks to access global mutable state, raw pointers,
   and other invariants the type system cannot verify.

   In OCaml:
   - There are no "unsafe blocks" — the language is uniformly memory-safe.
   - Global mutable state uses refs or module-level mutable values.
   - The OCaml equivalent of "minimal unsafe footprint" is keeping
     mutation scoped to the smallest possible functions and hiding
     mutable state behind a safe interface.
   - For true thread-safe global state, use Atomic (OCaml 5) or Mutex. *)

(* Thread-safe global counter using Atomic — the recommended OCaml equivalent
   of Rust's static mut GLOBAL_COUNTER: u64 guarded by unsafe.
   Atomic.t requires no explicit locking for single operations. *)
let global_counter : int Atomic.t = Atomic.make 0

let increment () = Atomic.incr global_counter

let get () = Atomic.get global_counter

let reset () =
  Atomic.set global_counter 0;
  (* Safe operations after the state mutation *)
  print_endline "Counter reset to 0."

let () =
  reset ();
  assert (get () = 0);
  increment ();
  increment ();
  assert (get () = 2);
  Printf.printf "counter after 2 increments: %d\n" (get ());
  reset ();
  assert (get () = 0);
  print_endline "counter lifecycle: ok";

  (* Demonstrate safe code that compiles without any special annotation *)
  let v = [| 1; 2; 3 |] in
  let sum = Array.fold_left (+) 0 v in
  assert (sum = 6);
  print_endline "safe sum: ok";

  print_endline "All assertions passed."
