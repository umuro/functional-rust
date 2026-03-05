(* String — Trim, Uppercase, Contains *)
(* Common string operations *)

let s = "  Hello, World!  "
let trimmed = String.trim s
let upper = String.uppercase_ascii trimmed
let lower = String.lowercase_ascii trimmed
let has_world = String.length s > 0 &&
  let rec find i =
    if i > String.length s - 5 then false
    else if String.sub s i 5 = "World" then true
    else find (i + 1)
  in find 0
let () = Printf.printf "Trimmed: '%s'\nUpper: '%s'\n" trimmed upper
