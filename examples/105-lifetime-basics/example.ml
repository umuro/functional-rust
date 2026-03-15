(* 105: Lifetimes in Rust vs OCaml
   Rust uses explicit lifetime annotations to enforce reference validity.
   OCaml's GC manages memory automatically — no lifetimes needed.
   This shows the OCaml equivalents of the Rust patterns. *)

(* Rust's longest<'a>(s1: &'a str, s2: &'a str) -> &'a str
   In OCaml, strings are values; returning the longer one is trivial. *)
let longest s1 s2 =
  if String.length s1 >= String.length s2 then s1 else s2

(* Rust's first_element<'a>(v: &'a [i32]) -> Option<&'a i32>
   OCaml: lists/arrays own their values; we just return an option. *)
let first_element = function
  | [||] -> None
  | arr  -> Some arr.(0)

(* Rust's struct Important<'a> { content: &'a str }
   In OCaml, a record holding a string is fine — no lifetime annotation. *)
type important = { content : string }

let make_important content = { content }
let get_content imp = imp.content

(* Rust's first_word<'a>(s: &'a str) -> &'a str
   In OCaml, String.split_on_char or a manual scan — returns a new string. *)
let first_word s =
  match String.split_on_char ' ' s with
  | word :: _ -> word
  | []        -> s

(* OCaml has no dangling references — the GC ensures values live
   as long as any reference to them exists. Rust prevents this at
   compile time via lifetimes; OCaml prevents it at runtime via GC. *)

let () =
  assert (longest "hello" "hi" = "hello");
  assert (longest "a" "bb" = "bb");

  assert (first_element [||] = None);
  assert (first_element [|1; 2; 3|] = Some 1);

  let msg = make_important "test" in
  assert (get_content msg = "test");

  assert (first_word "hello world" = "hello");
  assert (first_word "single" = "single");

  Printf.printf "All lifetime-basics demos passed.\n"
