(* Trie in OCaml — functional-style with records and Map *)

module CharMap = Map.Make(Char)

type trie = {
  is_end  : bool;
  children: trie CharMap.t;
}

let empty_trie = { is_end = false; children = CharMap.empty }

(* Insert a word into the trie — pure functional (returns new trie) *)
let rec insert (word : string) (pos : int) (t : trie) : trie =
  if pos = String.length word then
    { t with is_end = true }
  else
    let c = word.[pos] in
    let child =
      match CharMap.find_opt c t.children with
      | None -> empty_trie
      | Some ct -> ct
    in
    let child' = insert word (pos + 1) child in
    { t with children = CharMap.add c child' t.children }

let insert_word word t = insert word 0 t

(* Find the node for a given prefix, if it exists *)
let rec find_prefix (prefix : string) (pos : int) (t : trie) : trie option =
  if pos = String.length prefix then Some t
  else
    let c = prefix.[pos] in
    match CharMap.find_opt c t.children with
    | None -> None
    | Some child -> find_prefix prefix (pos + 1) child

(* Collect all words in a subtrie, prepending the given prefix *)
let rec collect_words (prefix : string) (t : trie) : string list =
  let from_children =
    CharMap.fold
      (fun c child acc ->
        (collect_words (prefix ^ String.make 1 c) child) @ acc)
      t.children []
  in
  if t.is_end then prefix :: from_children
  else from_children

(* Autocomplete: return all words with the given prefix *)
let autocomplete (prefix : string) (t : trie) : string list =
  match find_prefix prefix 0 t with
  | None -> []
  | Some subtrie -> List.sort String.compare (collect_words prefix subtrie)

let () =
  let words = ["apple"; "app"; "application"; "apply"; "apt"; "bat"; "ball"; "band"] in
  let trie = List.fold_right insert_word words empty_trie in

  let test prefix =
    let results = autocomplete prefix trie in
    Printf.printf "autocomplete(%S): [%s]\n" prefix (String.concat ", " results)
  in

  test "app";
  test "ba";
  test "apt";
  test "xyz";
  test ""
