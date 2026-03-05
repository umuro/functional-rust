(* OCaml: simple trie *)

module CharMap = Map.Make(Char)

type trie = { is_end: bool; children: trie CharMap.t }
let empty = { is_end=false; children=CharMap.empty }

let insert t word =
  let n = String.length word in
  let rec go node i =
    if i = n then { node with is_end=true }
    else
      let c = word.[i] in
      let child = try CharMap.find c node.children with Not_found -> empty in
      let new_child = go child (i+1) in
      { node with children=CharMap.add c new_child node.children }
  in go t 0

let search t word =
  let n = String.length word in
  let rec go node i =
    if i=n then node.is_end
    else match CharMap.find_opt word.[i] node.children with
    | None -> false | Some child -> go child (i+1)
  in go t 0

let () =
  let t = List.fold_left insert empty ["apple";"app";"application";"apply";"banana"] in
  List.iter (fun w -> Printf.printf "%s: %b\n" w (search t w))
    ["apple";"app";"ap";"application";"ban";"banana";"cat"]
