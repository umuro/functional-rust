module CMap = Map.Make(Char)

type trie = { is_word: bool; children: trie CMap.t }

let empty = { is_word = false; children = CMap.empty }

let insert word trie =
  let rec go i node =
    if i = String.length word then { node with is_word = true }
    else
      let c = word.[i] in
      let child = try CMap.find c node.children with Not_found -> empty in
      { node with children = CMap.add c (go (i+1) child) node.children }
  in go 0 trie

let mem word trie =
  let rec go i node =
    if i = String.length word then node.is_word
    else match CMap.find_opt word.[i] node.children with
    | None -> false | Some child -> go (i+1) child
  in go 0 trie

let () =
  let t = List.fold_left (fun t w -> insert w t)
    empty ["cat";"car";"card";"care";"dare"] in
  List.iter (fun w ->
    Printf.printf "%s: %b\n" w (mem w t)
  ) ["cat";"ca";"card";"dare";"dog"]
