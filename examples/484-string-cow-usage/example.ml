(* 484: Cow<str> — Copy-on-Write strings in OCaml *)
(* Rust's Cow<'a, str> holds either a &str borrow or an owned String.
   In OCaml we model this with a variant type.
   OCaml strings are immutable, so "no allocation on the borrowed path" maps to
   returning the original string unchanged. *)

(* A value is either a reference to an existing string or a freshly allocated one *)
type 'a cow =
  | Borrowed of 'a   (* no copy needed *)
  | Owned    of 'a   (* copy / transformation was required *)

(* Deref: get the underlying value regardless of ownership *)
let cow_get = function
  | Borrowed x -> x
  | Owned    x -> x

(* ensure_no_spaces: if no spaces, return Borrowed (no allocation);
   otherwise replace spaces with '_' and return Owned. *)
let ensure_no_spaces s =
  if not (String.contains s ' ') then
    Borrowed s          (* no allocation — same string *)
  else
    Owned (String.concat "_" (String.split_on_char ' ' s))

(* to_uppercase_if_needed: only allocate when lowercase chars present *)
let to_uppercase_if_needed cow =
  let s = cow_get cow in
  let has_lower = String.exists (fun c -> c >= 'a' && c <= 'z') s in
  if has_lower then Owned (String.uppercase_ascii s)
  else cow   (* pass through unchanged *)

(* process: apply trim and wrap in a formatted result *)
let process cow =
  Printf.sprintf "processed: %s" (String.trim (cow_get cow))

let () =
  (* no spaces → Borrowed (no copy) *)
  let r1 = ensure_no_spaces "nospace" in
  assert (match r1 with Borrowed _ -> true | Owned _ -> false);
  Printf.printf "no spaces: Borrowed ok\n";

  (* has space → Owned *)
  let r2 = ensure_no_spaces "has space" in
  assert (match r2 with Owned _ -> true | Borrowed _ -> false);
  assert (cow_get r2 = "has_space");
  Printf.printf "has space: Owned, value=%s\n" (cow_get r2);

  (* content check *)
  assert (cow_get (ensure_no_spaces "a b") = "a_b");

  (* to_uppercase_if_needed *)
  let up = to_uppercase_if_needed (Borrowed "hello") in
  assert (cow_get up = "HELLO");
  let already = to_uppercase_if_needed (Borrowed "ALREADY") in
  assert (match already with Borrowed _ -> true | Owned _ -> false);

  (* process *)
  let p = process (Borrowed "  hello  ") in
  assert (p = "processed: hello");
  Printf.printf "process: %s\n" p;

  print_endline "All assertions passed."
