(* 006: Palindrome Check
   OCaml works on char lists or strings *)

(* --- Approach 1: Convert to char list, reverse and compare --- *)
let is_palindrome_rev s =
  let chars = List.of_seq (String.to_seq s) in
  chars = List.rev chars

(* --- Approach 2: Compare string with its reverse directly --- *)
let is_palindrome_iter s =
  let chars = List.of_seq (String.to_seq s) in
  chars = List.rev chars  (* same as above; OCaml has no "rev iterator" *)

(* --- Approach 3: Case-insensitive, alphanumeric only --- *)
let is_palindrome_clean s =
  let clean =
    s
    |> String.to_seq
    |> Seq.filter (fun c -> Char.code c |> (fun n ->
        (n >= Char.code '0' && n <= Char.code '9') ||
        (n >= Char.code 'a' && n <= Char.code 'z') ||
        (n >= Char.code 'A' && n <= Char.code 'Z')))
    |> Seq.map Char.lowercase_ascii
    |> List.of_seq
  in
  clean = List.rev clean

let () =
  let check label f s =
    Printf.printf "%s %S = %b\n" label s (f s)
  in
  check "rev" is_palindrome_rev "racecar";
  check "rev" is_palindrome_rev "abba";
  check "rev" is_palindrome_rev "hello";
  check "rev" is_palindrome_rev "";
  check "iter" is_palindrome_iter "racecar";
  check "clean" is_palindrome_clean "A man, a plan, a canal: Panama";
  check "clean" is_palindrome_clean "Race Car";
  check "clean" is_palindrome_clean "hello world"
