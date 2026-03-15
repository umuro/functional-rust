(* Example 123: impl Trait — Argument and Return Position *)

(* OCaml uses parametric polymorphism and modules for similar patterns. *)

(* Approach 1: Polymorphic function arguments *)
let stringify_all to_s items =
  List.map to_s items

let approach1 () =
  let ints = stringify_all string_of_int [1; 2; 3] in
  let floats = stringify_all string_of_float [1.0; 2.0; 3.0] in
  assert (ints = ["1"; "2"; "3"]);
  Printf.printf "Ints: %s | Floats: %s\n"
    (String.concat ", " ints) (String.concat ", " floats)

(* Approach 2: Returning a function (existential type) *)
let make_formatter uppercase =
  if uppercase then String.uppercase_ascii
  else String.lowercase_ascii

let approach2 () =
  let fmt = make_formatter true in
  assert (fmt "hello" = "HELLO");
  let fmt2 = make_formatter false in
  assert (fmt2 "HELLO" = "hello");
  Printf.printf "Upper: %s, Lower: %s\n" (fmt "hello") (fmt2 "HELLO")

(* Approach 3: First-class modules as trait-like abstraction *)
module type Summarizable = sig
  type t
  val summarize : t -> string
end

let print_summary (type a) (module M : Summarizable with type t = a) x =
  Printf.printf "Summary: %s\n" (M.summarize x)

let approach3 () =
  let module IntSum = struct
    type t = int
    let summarize x = Printf.sprintf "Integer: %d" x
  end in
  print_summary (module IntSum) 42

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
