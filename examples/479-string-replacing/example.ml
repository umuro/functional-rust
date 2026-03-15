(* 479: String replacing — replace, replacen, retain equivalents in OCaml *)

(* OCaml strings are immutable byte sequences; replacements return new strings *)

(* Replace all occurrences of a substring *)
let replace_all ~needle ~replacement s =
  let buf = Buffer.create (String.length s) in
  let n = String.length needle in
  let len = String.length s in
  let i = ref 0 in
  while !i <= len - n do
    if String.sub s !i n = needle then begin
      Buffer.add_string buf replacement;
      i := !i + n
    end else begin
      Buffer.add_char buf s.[!i];
      i := !i + 1
    end
  done;
  (* add any remaining tail shorter than needle *)
  Buffer.add_string buf (String.sub s !i (len - !i));
  Buffer.contents buf

(* Replace at most [count] occurrences *)
let replacen ~needle ~replacement ~count s =
  let buf = Buffer.create (String.length s) in
  let n = String.length needle in
  let len = String.length s in
  let i = ref 0 in
  let replaced = ref 0 in
  while !i <= len - n do
    if !replaced < count && String.sub s !i n = needle then begin
      Buffer.add_string buf replacement;
      i := !i + n;
      incr replaced
    end else begin
      Buffer.add_char buf s.[!i];
      i := !i + 1
    end
  done;
  Buffer.add_string buf (String.sub s !i (len - !i));
  Buffer.contents buf

(* Retain only characters satisfying a predicate — like Rust's String::retain *)
let retain pred s =
  let buf = Buffer.create (String.length s) in
  String.iter (fun c -> if pred c then Buffer.add_char buf c) s;
  Buffer.contents buf

let () =
  (* replace_all *)
  let r = replace_all ~needle:"a" ~replacement:"x" "aabaa" in
  assert (r = "xxbxx");
  Printf.printf "replace_all: %s\n" r;

  (* replacen *)
  let r2 = replacen ~needle:"a" ~replacement:"x" ~count:2 "aabaa" in
  assert (r2 = "xxbaa");
  Printf.printf "replacen 2: %s\n" r2;

  (* no match *)
  let r3 = replace_all ~needle:"xyz" ~replacement:"abc" "hello" in
  assert (r3 = "hello");

  (* retain alphabetic characters *)
  let r4 = retain (fun c -> (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) "h3llo" in
  assert (r4 = "hllo");
  Printf.printf "retain alpha: %s\n" r4;

  print_endline "All assertions passed."
