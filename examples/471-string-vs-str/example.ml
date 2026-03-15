(* 471. String vs &str – OCaml *)
(* OCaml has one string type; Bytes.t for mutable buffers *)

let greet name = Printf.printf "Hello, %s!\n" name
let make_greeting name = Printf.sprintf "Hello, %s!" name
let first_word s =
  match String.index_opt s ' ' with
  | None -> s
  | Some i -> String.sub s 0 i

let () =
  greet "World";
  let owned = "Alice" in greet owned;
  let g = make_greeting "Bob" in Printf.printf "%s\n" g;
  let s = Bytes.of_string "hello" in Bytes.set s 0 'H';
  Printf.printf "Mutable: %s\n" (Bytes.to_string s);
  Printf.printf "first_word: %s\n" (first_word "Hello World")
