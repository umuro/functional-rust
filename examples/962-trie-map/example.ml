(* 962: Trie (Prefix Tree) Map
   OCaml: algebraic type with Hashtbl children.
   A trie stores strings by their characters; each node can hold a value
   and a map from the next character to a child node. *)

type 'a trie = {
  mutable value : 'a option;             (* value stored at this prefix *)
  children : (char, 'a trie) Hashtbl.t;
}

let create_node () = { value = None; children = Hashtbl.create 4 }

let create () = create_node ()

(* Insert key→value into the trie *)
let insert trie key value =
  let node = ref trie in
  String.iter (fun c ->
    let child =
      match Hashtbl.find_opt !node.children c with
      | Some ch -> ch
      | None ->
        let ch = create_node () in
        Hashtbl.add !node.children c ch;
        ch
    in
    node := child
  ) key;
  !node.value <- Some value

(* Look up an exact key *)
let find trie key =
  let node = ref (Some trie) in
  let i = ref 0 in
  let n = String.length key in
  while !node <> None && !i < n do
    node := Hashtbl.find_opt (Option.get !node).children key.[!i];
    incr i
  done;
  match !node with
  | Some n -> n.value
  | None -> None

(* Test whether any key with this prefix exists *)
let has_prefix trie prefix =
  let node = ref (Some trie) in
  let i = ref 0 in
  let n = String.length prefix in
  while !node <> None && !i < n do
    node := Hashtbl.find_opt (Option.get !node).children prefix.[!i];
    incr i
  done;
  !node <> None

(* Collect all (key, value) pairs with the given prefix *)
let with_prefix trie prefix =
  (* Walk to the prefix node *)
  let node = ref (Some trie) in
  String.iter (fun c ->
    node := match !node with
      | Some n -> Hashtbl.find_opt n.children c
      | None -> None
  ) prefix;
  (* DFS from there, accumulating the suffix *)
  let results = ref [] in
  let rec dfs n sfx =
    (match n.value with Some v -> results := (prefix ^ sfx, v) :: !results | None -> ());
    Hashtbl.iter (fun c child -> dfs child (sfx ^ String.make 1 c)) n.children
  in
  (match !node with Some n -> dfs n "" | None -> ());
  List.sort compare !results

let delete trie key =
  (* Walk and collect path; mark final node's value as None *)
  let path = ref [] in
  let node = ref (Some trie) in
  String.iter (fun c ->
    match !node with
    | Some n ->
      path := (n, c) :: !path;
      node := Hashtbl.find_opt n.children c
    | None -> ()
  ) key;
  match !node with
  | Some n -> n.value <- None
  | None -> ()

let () =
  let t : int trie = create () in
  insert t "apple" 1;
  insert t "app" 2;
  insert t "application" 3;
  insert t "apply" 4;
  insert t "banana" 5;

  Printf.printf "find apple = %s\n" (match find t "apple" with Some v -> string_of_int v | None -> "None");
  Printf.printf "find app = %s\n" (match find t "app" with Some v -> string_of_int v | None -> "None");
  Printf.printf "find ap = %s\n" (match find t "ap" with Some v -> string_of_int v | None -> "None");

  Printf.printf "has_prefix app = %b\n" (has_prefix t "app");
  Printf.printf "has_prefix xyz = %b\n" (has_prefix t "xyz");

  let results = with_prefix t "app" in
  Printf.printf "with_prefix app:\n";
  List.iter (fun (k, v) -> Printf.printf "  %s -> %d\n" k v) results;

  delete t "apple";
  Printf.printf "find apple after delete = %s\n" (match find t "apple" with Some v -> string_of_int v | None -> "None");
  Printf.printf "has_prefix apple still = %b\n" (has_prefix t "apple")
