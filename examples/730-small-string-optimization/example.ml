(* 730: Small String Optimization (SSO) — inline vs heap strings in OCaml *)
(* Rust's SsoString stores ≤23 bytes inline in the enum variant.
   OCaml note: OCaml strings are always heap-allocated (managed by the GC),
   so true SSO (avoiding heap allocation) is not the idiomatic approach.
   However, we can model the SAME data structure — an enum that chooses
   between an inline buffer and a heap-backed string — to demonstrate
   the concept and show OCaml's equivalent representation. *)

let inline_cap = 23

type sso_string =
  | Inline of { buf : bytes; len : int }
  | Heap   of string

let sso_new s =
  let n = String.length s in
  if n <= inline_cap then begin
    let buf = Bytes.make inline_cap '\000' in
    Bytes.blit_string s 0 buf 0 n;
    Inline { buf; len = n }
  end else
    Heap s

let as_str = function
  | Inline { buf; len } -> Bytes.sub_string buf 0 len
  | Heap s              -> s

let sso_length = function
  | Inline { len; _ } -> len
  | Heap s            -> String.length s

let is_empty sso = sso_length sso = 0

let is_inline = function
  | Inline _ -> true
  | Heap _   -> false

let () =
  (* empty is inline *)
  let s = sso_new "" in
  assert (is_inline s);
  assert (sso_length s = 0);
  assert (as_str s = "");
  print_endline "empty is_inline: ok";

  (* short string inline *)
  let s2 = sso_new "hello" in
  assert (is_inline s2);
  assert (as_str s2 = "hello");
  print_endline "short inline: ok";

  (* exactly 23 bytes — still inline *)
  let s23 = String.make inline_cap 'a' in
  let sso23 = sso_new s23 in
  assert (is_inline sso23);
  assert (as_str sso23 = s23);
  print_endline "boundary 23 inline: ok";

  (* 24 bytes — heap *)
  let s24 = String.make (inline_cap + 1) 'a' in
  let sso24 = sso_new s24 in
  assert (not (is_inline sso24));
  assert (as_str sso24 = s24);
  print_endline "boundary 24 heap: ok";

  (* long string — heap *)
  let long = "this is a long string that exceeds the inline capacity" in
  let ssol = sso_new long in
  assert (not (is_inline ssol));
  assert (as_str ssol = long);
  print_endline "long heap: ok";

  (* is_empty *)
  assert (is_empty (sso_new ""));
  assert (not (is_empty (sso_new "x")));
  print_endline "is_empty: ok";

  print_endline "All assertions passed."
