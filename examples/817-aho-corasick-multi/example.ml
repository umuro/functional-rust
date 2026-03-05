(* Aho-Corasick Multi-Pattern Matching — O(Σ|patterns| + n + matches) *)

let alpha = 256

type node = {
  children : int array;
  mutable fail : int;
  mutable output : int list;  (* pattern indices ending at this node *)
}

let make_node () = { children = Array.make alpha (-1); fail = 0; output = [] }

let build_automaton patterns =
  let nodes = ref [| make_node () |] in  (* root at index 0 *)

  (* Insert all patterns into trie *)
  List.iteri (fun pid pat ->
    let cur = ref 0 in
    String.iter (fun c ->
      let ci = Char.code c in
      if !nodes.(!cur).children.(ci) = -1 then begin
        !nodes.(!cur).children.(ci) <- Array.length !nodes;
        nodes := Array.append !nodes [| make_node () |]
      end;
      cur := !nodes.(!cur).children.(ci)
    ) pat;
    !nodes.(!cur).output <- pid :: !nodes.(!cur).output
  ) patterns;

  (* BFS to compute failure links *)
  let q = Queue.create () in
  for c = 0 to alpha - 1 do
    let ch = !nodes.(0).children.(c) in
    if ch = -1 then !nodes.(0).children.(c) <- 0
    else begin !nodes.(ch).fail <- 0; Queue.push ch q end
  done;
  while not (Queue.is_empty q) do
    let u = Queue.pop q in
    !nodes.(u).output <- !nodes.(u).output @
                         !nodes.(!nodes.(u).fail).output;
    for c = 0 to alpha - 1 do
      let v = !nodes.(u).children.(c) in
      if v = -1 then
        !nodes.(u).children.(c) <- !nodes.(!nodes.(u).fail).children.(c)
      else begin
        !nodes.(v).fail <- !nodes.(!nodes.(u).fail).children.(c);
        Queue.push v q
      end
    done
  done;
  !nodes

let search_ac nodes text patterns =
  let state   = ref 0 in
  let matches = ref [] in
  String.iteri (fun i c ->
    let ci = Char.code c in
    state := nodes.(!state).children.(ci);
    List.iter (fun pid ->
      let pat = List.nth patterns pid in
      matches := (i - String.length pat + 1, pid) :: !matches
    ) nodes.(!state).output
  ) text;
  List.rev !matches

let () =
  let patterns = ["he"; "she"; "his"; "hers"] in
  let text     = "ushers" in
  let nodes    = build_automaton patterns in
  let matches  = search_ac nodes text patterns in
  Printf.printf "Text: %S\n" text;
  Printf.printf "Patterns: [%s]\n" (String.concat "; " patterns);
  List.iter (fun (pos, pid) ->
    Printf.printf "  Found %S at position %d\n" (List.nth patterns pid) pos
  ) matches
