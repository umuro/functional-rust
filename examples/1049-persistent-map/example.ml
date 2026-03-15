(* 1049: Persistent HashMap — Functional Update *)
(* OCaml's Map IS persistent — every operation returns a new map
   while sharing structure with the old one *)

module StringMap = Map.Make(String)

(* Approach 1: Persistence via structural sharing *)
let persistence_demo () =
  let v1 = StringMap.empty
    |> StringMap.add "a" 1
    |> StringMap.add "b" 2
    |> StringMap.add "c" 3
  in
  (* v2 is a "new version" sharing structure with v1 *)
  let v2 = StringMap.add "d" 4 v1 in
  let v3 = StringMap.add "b" 99 v1 in  (* update in v1, not v2 *)
  (* All versions coexist *)
  assert (StringMap.find "b" v1 = 2);
  assert (StringMap.find "b" v3 = 99);
  assert (StringMap.cardinal v2 = 4);
  assert (StringMap.cardinal v1 = 3);
  (* v1 is completely unaffected *)
  assert (not (StringMap.mem "d" v1))

(* Approach 2: Version history *)
let version_history () =
  let versions = ref [StringMap.empty] in
  let current () = List.hd !versions in
  let update f =
    versions := f (current ()) :: !versions
  in
  update (StringMap.add "x" 10);
  update (StringMap.add "y" 20);
  update (StringMap.add "z" 30);
  update (StringMap.remove "y");
  (* Current state *)
  assert (StringMap.find "x" (current ()) = 10);
  assert (not (StringMap.mem "y" (current ())));
  assert (StringMap.find "z" (current ()) = 30);
  (* Can access any past version *)
  let v2 = List.nth !versions 2 in  (* after adding y *)
  assert (StringMap.mem "y" v2);
  assert (StringMap.find "y" v2 = 20)

(* Approach 3: Undo/redo with persistent maps *)
type 'a state = {
  data: 'a StringMap.t;
  undo_stack: 'a StringMap.t list;
  redo_stack: 'a StringMap.t list;
}

let empty_state = { data = StringMap.empty; undo_stack = []; redo_stack = [] }

let apply f state =
  { data = f state.data;
    undo_stack = state.data :: state.undo_stack;
    redo_stack = [] }

let undo state =
  match state.undo_stack with
  | [] -> state
  | prev :: rest ->
    { data = prev; undo_stack = rest;
      redo_stack = state.data :: state.redo_stack }

let redo state =
  match state.redo_stack with
  | [] -> state
  | next :: rest ->
    { data = next; undo_stack = state.data :: state.undo_stack;
      redo_stack = rest }

let undo_redo_test () =
  let s = empty_state in
  let s = apply (StringMap.add "name" "Alice") s in
  let s = apply (StringMap.add "age" "30") s in
  assert (StringMap.find "name" s.data = "Alice");
  assert (StringMap.find "age" s.data = "30");
  let s = undo s in
  assert (not (StringMap.mem "age" s.data));
  assert (StringMap.find "name" s.data = "Alice");
  let s = redo s in
  assert (StringMap.find "age" s.data = "30")

let () =
  persistence_demo ();
  version_history ();
  undo_redo_test ();
  Printf.printf "✓ All tests passed\n"
