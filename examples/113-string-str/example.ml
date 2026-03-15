(* Example 113: String vs &str — OCaml string → Rust Two String Types *)

(* OCaml has one string type. Rust has two: String (owned, heap) and &str (borrowed slice). *)

(* Approach 1: String creation and manipulation *)
let approach1 () =
  let s = "hello" in
  let s2 = s ^ " world" in
  let upper = String.uppercase_ascii s2 in
  assert (upper = "HELLO WORLD");
  Printf.printf "Original: %s, Upper: %s\n" s2 upper

(* Approach 2: Substring operations *)
let approach2 () =
  let s = "hello, world!" in
  let sub = String.sub s 7 5 in
  let first_word = match String.index_opt s ',' with
    | Some i -> String.sub s 0 i
    | None -> s in
  assert (sub = "world");
  assert (first_word = "hello");
  Printf.printf "Sub: %s, First word: %s\n" sub first_word

(* Approach 3: String as bytes *)
let approach3 () =
  let s = "Rust" in
  let bytes = Bytes.of_string s in
  Bytes.set bytes 0 'r';
  let modified = Bytes.to_string bytes in
  assert (modified = "rust");
  Printf.printf "Modified: %s\n" modified

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
