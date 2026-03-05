(* Recursive Data — Trie (Prefix Tree) *)
(* Store strings in a prefix tree *)

module CharMap = Map.Make(Char)

type trie = { is_end: bool; children: trie CharMap.t }
let empty = { is_end = false; children = CharMap.empty }

let insert word t =
  let rec aux i t =
    if i = String.length word then { t with is_end = true }
    else
      let c = word.[i] in
      let child = try CharMap.find c t.children with Not_found -> empty in
      { t with children = CharMap.add c (aux (i+1) child) t.children }
  in aux 0 t

let mem word t =
  let rec aux i t =
    if i = String.length word then t.is_end
    else match CharMap.find_opt word.[i] t.children with
      | None -> false | Some child -> aux (i+1) child
  in aux 0 t

let t = List.fold_left (fun t w -> insert w t) empty
  ["cat"; "car"; "card"; "care"; "bat"]
let () = List.iter (fun w ->
  Printf.printf "%s: %b\n" w (mem w t)
) ["cat"; "ca"; "car"; "care"; "dog"]
