(* 962: Trie Map *)
(* Prefix tree for string keys with associated values *)

module CharMap = Map.Make(Char)

type 'a trie = {
  mutable value: 'a option;
  mutable children: 'a trie CharMap.t;
}

let create_node () = { value = None; children = CharMap.empty }

let insert trie key value =
  let node = ref trie in
  String.iter (fun c ->
    let child =
      match CharMap.find_opt c (!node).children with
      | Some n -> n
      | None ->
        let n = create_node () in
        (!node).children <- CharMap.add c n (!node).children;
        n
    in
    node := child
  ) key;
  (!node).value <- Some value

let search trie key =
  let node = ref (Some trie) in
  let i = ref 0 in
  let n = String.length key in
  while !i < n && !node <> None do
    (match !node with
     | Some nd ->
       node := CharMap.find_opt key.[!i] nd.children
     | None -> ());
    incr i
  done;
  match !node with
  | Some nd -> nd.value
  | None -> None

let starts_with trie prefix =
  let node = ref (Some trie) in
  let i = ref 0 in
  let n = String.length prefix in
  while !i < n && !node <> None do
    (match !node with
     | Some nd ->
       node := CharMap.find_opt prefix.[!i] nd.children
     | None -> ());
    incr i
  done;
  !node <> None

let () =
  let t = create_node () in
  insert t "apple" 1;
  insert t "app" 2;
  insert t "application" 3;
  insert t "banana" 4;
  insert t "band" 5;

  assert (search t "apple" = Some 1);
  assert (search t "app" = Some 2);
  assert (search t "application" = Some 3);
  assert (search t "banana" = Some 4);
  assert (search t "band" = Some 5);
  assert (search t "ap" = None);      (* prefix, not full word *)
  assert (search t "apricot" = None);
  assert (search t "" = None);

  assert (starts_with t "app");
  assert (starts_with t "ban");
  assert (starts_with t "apple");
  assert (not (starts_with t "xyz"));
  assert (not (starts_with t "apricot"));

  (* Update existing key *)
  insert t "apple" 99;
  assert (search t "apple" = Some 99);

  Printf.printf "✓ All tests passed\n"
