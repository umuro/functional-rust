(* Bob — Response Patterns *)
(* String classification with boolean predicates and pattern matching *)

let is_empty s = String.trim s = ""

let is_shouting s =
  let has_alpha = ref false in
  String.iter (fun c ->
    if c >= 'a' && c <= 'z' then has_alpha := true) s;
  not !has_alpha &&
  String.iter (fun c ->
    if c >= 'A' && c <= 'Z' then has_alpha := true) s;
  !has_alpha

let is_question s =
  String.length s > 0 && s.[String.length s - 1] = '?'

let response_for s =
  let s = String.trim s in
  if is_empty s then "Fine. Be that way!"
  else if is_shouting s && is_question s then "Calm down, I know what I'm doing!"
  else if is_shouting s then "Whoa, chill out!"
  else if is_question s then "Sure."
  else "Whatever."
