(* Tarjan's SCC — O(V+E) recursive DFS *)

let tarjan_scc adj n =
  let disc     = Array.make n (-1) in
  let low      = Array.make n 0 in
  let on_stack = Array.make n false in
  let stack    = ref [] in
  let timer    = ref 0 in
  let sccs     = ref [] in

  let rec dfs u =
    disc.(u) <- !timer;
    low.(u)  <- !timer;
    incr timer;
    stack    := u :: !stack;
    on_stack.(u) <- true;
    List.iter (fun v ->
      if disc.(v) = -1 then begin
        dfs v;
        low.(u) <- min low.(u) low.(v)
      end else if on_stack.(v) then
        low.(u) <- min low.(u) disc.(v)
    ) adj.(u);
    (* u is root of SCC *)
    if low.(u) = disc.(u) then begin
      let scc = ref [] in
      let cont = ref true in
      while !cont do
        match !stack with
        | [] -> cont := false
        | w :: rest ->
          stack := rest;
          on_stack.(w) <- false;
          scc := w :: !scc;
          if w = u then cont := false
      done;
      sccs := !scc :: !sccs
    end
  in
  for v = 0 to n - 1 do
    if disc.(v) = -1 then dfs v
  done;
  !sccs

let () =
  let n   = 8 in
  let adj = Array.make n [] in
  let add u v = adj.(u) <- v :: adj.(u) in
  add 0 1; add 1 2; add 2 0; add 2 3;
  add 3 4; add 4 5; add 5 3;
  add 6 5; add 6 7; add 7 6;
  let sccs = tarjan_scc adj n in
  Printf.printf "Number of SCCs: %d\n" (List.length sccs);
  List.iteri (fun i scc ->
    Printf.printf "  SCC %d: [%s]\n" (i+1)
      (String.concat "," (List.map string_of_int scc))
  ) sccs
