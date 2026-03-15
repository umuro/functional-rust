(* Example 106: Lifetime Elision — When You Don't Need to Annotate *)

(* OCaml never needs lifetime annotations. This example shows the
   patterns where Rust also skips them thanks to elision rules. *)

(* Approach 1: Single input reference → output borrows from it *)
let first_word s =
  match String.index_opt s ' ' with
  | Some i -> String.sub s 0 i
  | None -> s

let approach1 () =
  let word = first_word "hello world" in
  assert (word = "hello");
  Printf.printf "First word: %s\n" word

(* Approach 2: Method-style — self is the obvious source *)
type text_buffer = { content : string }

let get_content buf = buf.content
let get_length buf = String.length buf.content

let approach2 () =
  let buf = { content = "Hello, World!" } in
  let c = get_content buf in
  let l = get_length buf in
  assert (l = 13);
  Printf.printf "Content: %s, Length: %d\n" c l

(* Approach 3: Multiple inputs — OCaml doesn't care *)
let pick_longer a b =
  if String.length a >= String.length b then a else b

let approach3 () =
  let result = pick_longer "hello" "hi" in
  assert (result = "hello");
  Printf.printf "Longer: %s\n" result

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
