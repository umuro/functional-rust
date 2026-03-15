(* 006: Palindrome Check *)

(* Approach 1: Reverse and compare *)
let is_palindrome_rev s =
  let chars = List.of_seq (String.to_seq s) in
  chars = List.rev chars

(* Approach 2: Index-based comparison *)
let is_palindrome_idx s =
  let n = String.length s in
  let rec check i =
    if i >= n / 2 then true
    else if s.[i] <> s.[n - 1 - i] then false
    else check (i + 1)
  in
  check 0

(* Approach 3: Case-insensitive, alphanumeric only *)
let is_palindrome_clean s =
  let clean =
    s
    |> String.lowercase_ascii
    |> String.to_seq
    |> Seq.filter (fun c ->
      (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9'))
    |> List.of_seq
  in
  clean = List.rev clean

(* Tests *)
let () =
  assert (is_palindrome_rev "racecar");
  assert (is_palindrome_rev "abba");
  assert (not (is_palindrome_rev "hello"));
  assert (is_palindrome_rev "");
  assert (is_palindrome_rev "a");
  assert (is_palindrome_idx "racecar");
  assert (not (is_palindrome_idx "abc"));
  assert (is_palindrome_clean "A man, a plan, a canal: Panama");
  assert (is_palindrome_clean "Race Car");
  assert (not (is_palindrome_clean "hello world"));
  Printf.printf "✓ All tests passed\n"
