(* 471: String vs &str — Ownership Semantics
   Rust distinguishes owned String from borrowed &str.
   OCaml has one string type; all strings are immutable values.
   The analogous distinction is between:
   - string  (owned, heap-allocated, immutable value)
   - Bytes.t (mutable buffer — like String in a builder context)
   Functions accepting read-only data take string; builders use Buffer. *)

(* Accept any string — no ownership cost (OCaml strings are immutable) *)
let greet name = Printf.printf "Hello, %s!\n%!" name

(* Return an owned string — allocated on the heap *)
let make_greeting name = "Hello, " ^ name ^ "!"

(* Extract the first word without allocating (substring sharing) *)
let first_word s =
  match String.index_opt s ' ' with
  | Some i -> String.sub s 0 i
  | None   -> s

(* Demonstrate that string literals and String.t are the same type *)
let () =
  (* Works with a string literal *)
  greet "world";

  (* Works with a heap-allocated string *)
  let name = String.make 4 'x' in    (* "xxxx" — like String::from(...) *)
  let g = make_greeting name in
  assert (g = "Hello, xxxx!");
  Printf.printf "make_greeting: %s\n%!" g;

  (* Works with a &str-like string literal *)
  let g2 = make_greeting "hi" in
  assert (g2 = "Hello, hi!");
  Printf.printf "make_greeting literal: %s\n%!" g2;

  (* first_word — pure substring, no extra allocation *)
  assert (first_word "hello world" = "hello");
  assert (first_word "single"      = "single");
  Printf.printf "first_word: \"%s\", \"%s\"\n%!"
    (first_word "hello world") (first_word "single");

  (* Bytes.t = mutable buffer (like Rust's String when used as builder) *)
  let buf = Bytes.make 5 '_' in
  Bytes.set buf 0 'R'; Bytes.set buf 1 'u'; Bytes.set buf 2 's';
  Bytes.set buf 3 't'; Bytes.set buf 4 '!';
  let owned = Bytes.to_string buf in
  assert (owned = "Rust!");
  Printf.printf "Bytes→string: %s\n%!" owned
