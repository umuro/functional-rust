(* String.sub and String.concat — Substring and Join *)
(* Extract substrings and join strings *)

let s = "Hello, World!"
let hello = String.sub s 0 5
let world = String.sub s 7 5
let () = Printf.printf "'%s' and '%s'\n" hello world

let parts = ["one"; "two"; "three"]
let joined = String.concat " | " parts
let () = Printf.printf "Joined: %s\n" joined
