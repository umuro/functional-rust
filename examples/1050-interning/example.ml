(* 1050: String Interning — Dedup Strings to IDs *)
(* Map strings to unique integer IDs for efficient comparison/storage *)

(* Approach 1: Basic string interner using Hashtbl *)
module StringInterner = struct
  type t = {
    to_id: (string, int) Hashtbl.t;
    to_str: (int, string) Hashtbl.t;
    mutable next_id: int;
  }

  let create () = {
    to_id = Hashtbl.create 64;
    to_str = Hashtbl.create 64;
    next_id = 0;
  }

  let intern interner s =
    match Hashtbl.find_opt interner.to_id s with
    | Some id -> id
    | None ->
      let id = interner.next_id in
      interner.next_id <- id + 1;
      Hashtbl.add interner.to_id s id;
      Hashtbl.add interner.to_str id s;
      id

  let resolve interner id =
    Hashtbl.find_opt interner.to_str id

  let len interner = interner.next_id
end

let basic_interning () =
  let interner = StringInterner.create () in
  let id1 = StringInterner.intern interner "hello" in
  let id2 = StringInterner.intern interner "world" in
  let id3 = StringInterner.intern interner "hello" in  (* same as id1 *)
  assert (id1 = id3);  (* Same string → same ID *)
  assert (id1 <> id2); (* Different strings → different IDs *)
  assert (StringInterner.resolve interner id1 = Some "hello");
  assert (StringInterner.resolve interner id2 = Some "world");
  assert (StringInterner.len interner = 2)

(* Approach 2: Interned comparison is O(1) *)
let fast_comparison () =
  let interner = StringInterner.create () in
  let words = ["the"; "cat"; "sat"; "on"; "the"; "mat"; "the"; "cat"] in
  let ids = List.map (StringInterner.intern interner) words in
  (* Compare IDs instead of strings: integer comparison is O(1) *)
  let the_id = List.hd ids in
  let count = List.length (List.filter (fun id -> id = the_id) ids) in
  assert (count = 3)  (* "the" appears 3 times *)

(* Approach 3: Interned symbol table *)
let symbol_table () =
  let interner = StringInterner.create () in
  let vars = ["x"; "y"; "x"; "z"; "y"; "x"] in
  let interned = List.map (StringInterner.intern interner) vars in
  (* Only 3 unique symbols *)
  assert (StringInterner.len interner = 3);
  (* Dedup by comparing IDs *)
  let unique = List.sort_uniq compare interned in
  assert (List.length unique = 3);
  (* Resolve back *)
  let names = List.filter_map (StringInterner.resolve interner) unique in
  assert (List.length names = 3)

let () =
  basic_interning ();
  fast_comparison ();
  symbol_table ();
  Printf.printf "✓ All tests passed\n"
