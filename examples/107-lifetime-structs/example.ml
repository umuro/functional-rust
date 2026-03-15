(* Example 107: Lifetimes in Structs — Storing References Safely *)

(* Approach 1: Struct holding a reference to external data *)
type excerpt = { text : string; page : int }

let make_excerpt text page = { text; page }

let approach1 () =
  let book = "Call me Ishmael. Some years ago..." in
  let exc = make_excerpt (String.sub book 0 16) 1 in
  assert (exc.text = "Call me Ishmael.");
  Printf.printf "Excerpt p.%d: %s\n" exc.page exc.text

(* Approach 2: Struct with multiple string fields *)
type article = { title : string; author : string; body : string }

let summarize a =
  Printf.sprintf "%s by %s (%d chars)" a.title a.author (String.length a.body)

let approach2 () =
  let a = { title = "Rust Ownership"; author = "Alice"; body = "Ownership is..." } in
  let s = summarize a in
  Printf.printf "%s\n" s

(* Approach 3: Nested structs with shared data *)
type highlight = { excerpt : excerpt; note : string }

let approach3 () =
  let exc = { text = "important passage"; page = 42 } in
  let h = { excerpt = exc; note = "Remember this!" } in
  assert (h.excerpt.page = 42);
  Printf.printf "Highlight p.%d: %s — %s\n" h.excerpt.page h.excerpt.text h.note

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
