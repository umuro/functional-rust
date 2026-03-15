(* 730: Small String Optimization — OCaml doesn't have true SSO,
   but we can model the concept with a variant type. *)

(* Simulate SSO: inline up to 15 bytes, otherwise heap string *)
type sso_string =
  | Inline of bytes   (* short: stored in OCaml boxed bytes (not true SSO, but conceptual) *)
  | Heap of string    (* long: regular OCaml string *)

let sso_threshold = 15

let sso_of_string s =
  if String.length s <= sso_threshold
  then Inline (Bytes.of_string s)
  else Heap s

let sso_to_string = function
  | Inline b -> Bytes.to_string b
  | Heap s   -> s

let sso_len = function
  | Inline b -> Bytes.length b
  | Heap s   -> String.length s

let is_inline = function
  | Inline _ -> true
  | Heap _   -> false

let () =
  let short = sso_of_string "hello" in
  let long  = sso_of_string "this is a very long string indeed" in
  Printf.printf "Short '%s' inline=%b len=%d\n"
    (sso_to_string short) (is_inline short) (sso_len short);
  Printf.printf "Long  '%.20s...' inline=%b len=%d\n"
    (sso_to_string long) (is_inline long) (sso_len long)
