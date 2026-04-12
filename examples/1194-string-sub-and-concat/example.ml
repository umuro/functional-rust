(* Idiomatic OCaml: String.sub and String.concat *)

let s = "Hello, World!"

(* String.sub s start len — extract substring at position *)
let hello = String.sub s 0 5
let world = String.sub s 7 5

(* String.concat sep list — join list of strings with separator *)
let parts = ["one"; "two"; "three"]
let joined = String.concat " | " parts

(* Safe lookup — raises Invalid_argument on bad range *)
let safe_sub s start len =
  if start < 0 || len < 0 || start + len > String.length s then None
  else Some (String.sub s start len)

let () =
  assert (hello = "Hello");
  assert (world = "World");
  assert (joined = "one | two | three");
  assert (String.concat "" ["hello"; "world"] = "helloworld");
  assert (String.concat ", " [] = "");
  assert (safe_sub "Hello" 3 10 = None);
  assert (safe_sub "Hello" 1 3 = Some "ell");
  Printf.printf "'%s' and '%s'\n" hello world;
  Printf.printf "Joined: %s\n" joined;
  print_endline "ok"
