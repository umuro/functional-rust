(* 1050: String Interning — Deduplicate Strings to Integer IDs
   Map strings to unique ints for O(1) comparison.
   OCaml: Hashtbl for both directions, int as symbol. *)

module StringTbl = Hashtbl.Make(struct
  type t = string
  let equal = String.equal
  let hash = Hashtbl.hash
end)

type symbol = Symbol of int

type interner = {
  to_id  : int StringTbl.t;
  to_str : string array ref;  (* grows as needed *)
  mutable count : int;
}

let make_interner () =
  { to_id = StringTbl.create 16; to_str = ref (Array.make 16 ""); count = 0 }

let intern t s =
  match StringTbl.find_opt t.to_id s with
  | Some id -> Symbol id
  | None ->
    let id = t.count in
    (* Grow the array if needed *)
    if id >= Array.length !(t.to_str) then begin
      let new_arr = Array.make (id * 2 + 1) "" in
      Array.blit !(t.to_str) 0 new_arr 0 id;
      t.to_str := new_arr
    end;
    !(t.to_str).(id) <- s;
    StringTbl.add t.to_id s id;
    t.count <- id + 1;
    Symbol id

let resolve t (Symbol id) =
  if id < t.count then Some !(t.to_str).(id) else None

let len t = t.count

let () =
  let t = make_interner () in
  let id1 = intern t "hello" in
  let id2 = intern t "world" in
  let id3 = intern t "hello" in  (* same as id1 *)

  assert (id1 = id3);            (* same string → same symbol *)
  assert (id1 <> id2);           (* different strings → different symbols *)
  assert (resolve t id1 = Some "hello");
  assert (resolve t id2 = Some "world");
  assert (len t = 2);

  (* Fast comparison: count occurrences using symbols *)
  let words = ["the";"cat";"sat";"on";"the";"mat";"the";"cat"] in
  let ids = List.map (intern t) words in
  let the_id = List.hd ids in
  let count = List.length (List.filter (fun id -> id = the_id) ids) in
  assert (count = 3);

  (* Frequency map using symbols *)
  let freq = Hashtbl.create 8 in
  List.iter (fun id ->
    let c = try Hashtbl.find freq id with Not_found -> 0 in
    Hashtbl.replace freq id (c + 1)
  ) ids;
  assert (Hashtbl.find freq the_id = 3);

  (* Symbol table: unique variable names *)
  let t2 = make_interner () in
  let vars = ["x";"y";"x";"z";"y";"x"] in
  let interned = List.map (intern t2) vars in
  assert (len t2 = 3);

  (* Dedup using sort + dedup on int values *)
  let ids_sorted = List.sort_uniq compare
    (List.map (fun (Symbol id) -> id) interned) in
  assert (List.length ids_sorted = 3);

  (* Resolve back *)
  let names = List.filter_map (fun id -> resolve t2 (Symbol id)) ids_sorted in
  assert (List.length names = 3);

  (* Empty string interning *)
  let t3 = make_interner () in
  let empty_sym = intern t3 "" in
  assert (resolve t3 empty_sym = Some "");

  Printf.printf "All interning tests passed.\n"
