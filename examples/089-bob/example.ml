(* Bob — String Pattern Matching *)

let is_question s = String.length (String.trim s) > 0 &&
  String.get (String.trim s) (String.length (String.trim s) - 1) = '?'

let is_yelling s =
  let has_letter = String.to_seq s |> Seq.exists (fun c ->
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) in
  has_letter && String.uppercase_ascii s = s

let is_silence s = String.trim s = ""

let response_for s =
  match is_silence s, is_yelling s, is_question s with
  | true, _, _ -> "Fine. Be that way!"
  | _, true, true -> "Calm down, I know what I'm doing!"
  | _, true, false -> "Whoa, chill out!"
  | _, false, true -> "Sure."
  | _ -> "Whatever."

let () =
  assert (response_for "WATCH OUT!" = "Whoa, chill out!");
  assert (response_for "Does this work?" = "Sure.");
  assert (response_for "   " = "Fine. Be that way!")
