(* String — Regular Expression-like Matching *)
(* Simple glob-style pattern matching *)

(* Simple glob matching: * matches any substring, ? matches one char *)
let rec glob_match pattern str =
  match (pattern, str) with
  | ("", "") -> true
  | ("", _) -> false
  | ("*", _) -> true
  | _ when String.length pattern > 0 && String.length str = 0 ->
    pattern = "*"
  | _ ->
    let pc = pattern.[0] and sc = str.[0] in
    let prest = String.sub pattern 1 (String.length pattern - 1) in
    let srest = String.sub str 1 (String.length str - 1) in
    if pc = '*' then
      glob_match prest str || glob_match pattern srest
    else if pc = '?' || pc = sc then
      glob_match prest srest
    else false

let tests = [("*.ml", "hello.ml"); ("test_?", "test_a"); ("foo*", "bar")]
let () = List.iter (fun (p, s) ->
  Printf.printf "glob(%s, %s) = %b\n" p s (glob_match p s)
) tests
