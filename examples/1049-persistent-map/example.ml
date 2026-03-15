(* 1049: Persistent Map — Functional Updates
   OCaml's Map module IS persistent by design: every operation returns a new
   version sharing structure with the old one (HAMT-like balanced BST).
   No cloning required — old versions coexist automatically. *)

module StringMap = Map.Make(String)

(* Persistent map with undo/redo history *)
type 'a history = {
  current    : 'a StringMap.t;
  undo_stack : 'a StringMap.t list;
  redo_stack : 'a StringMap.t list;
}

let empty_history = { current = StringMap.empty; undo_stack = []; redo_stack = [] }

let apply f h =
  { current    = f h.current;
    undo_stack = h.current :: h.undo_stack;
    redo_stack = [] }

let undo h =
  match h.undo_stack with
  | [] -> (false, h)
  | prev :: rest ->
    (true, { current = prev; undo_stack = rest; redo_stack = h.current :: h.redo_stack })

let redo h =
  match h.redo_stack with
  | [] -> (false, h)
  | next :: rest ->
    (true, { current = next; undo_stack = h.current :: h.undo_stack; redo_stack = rest })

let () =
  (* Persistence demo — all versions coexist *)
  let v1 = StringMap.(empty |> add "a" 1 |> add "b" 2 |> add "c" 3) in
  let v2 = StringMap.add "d" 4 v1 in       (* v2 has a,b,c,d *)
  let v3 = StringMap.add "b" 99 v1 in      (* v3 updates b in v1 *)

  assert (StringMap.find "b" v1 = 2);       (* v1 unchanged *)
  assert (StringMap.find "b" v3 = 99);      (* v3 has updated b *)
  assert (StringMap.cardinal v2 = 4);
  assert (StringMap.cardinal v1 = 3);
  assert (not (StringMap.mem "d" v1));

  (* Version history with list of versions *)
  let versions = ref [StringMap.empty] in
  let update f =
    let cur = List.hd !versions in
    versions := f cur :: !versions
  in
  update (StringMap.add "x" 10);
  update (StringMap.add "y" 20);
  update (StringMap.add "z" 30);
  update (StringMap.remove "y");

  let current = List.hd !versions in
  assert (StringMap.find "x" current = 10);
  assert (not (StringMap.mem "y" current));
  assert (StringMap.find "z" current = 30);

  (* Access version after adding y: versions are stored newest-first *)
  let v_with_y = List.nth !versions 2 in   (* 3 updates ago: had x, y *)
  assert (StringMap.mem "y" v_with_y);
  assert (StringMap.find "y" v_with_y = 20);

  (* Undo/redo *)
  let h = ref empty_history in
  h := apply (StringMap.add "name" "Alice") !h;
  h := apply (StringMap.add "age" "30") !h;
  assert (StringMap.find "name" !h.current = "Alice");
  assert (StringMap.find "age" !h.current = "30");

  let (ok, h') = undo !h in
  h := h';
  assert ok;
  assert (not (StringMap.mem "age" !h.current));
  assert (StringMap.mem "name" !h.current);

  let (ok, h') = redo !h in
  h := h';
  assert ok;
  assert (StringMap.find "age" !h.current = "30");

  (* Empty undo/redo *)
  let (ok, _) = undo empty_history in
  assert (not ok);
  let (ok, _) = redo empty_history in
  assert (not ok);

  Printf.printf "All persistent-map tests passed.\n"
