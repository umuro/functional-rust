(* Bipartite Check — BFS 2-colouring O(V+E) *)

let is_bipartite adj n =
  let color = Array.make n (-1) in
  let result = ref true in
  let q = Queue.create () in
  for start = 0 to n - 1 do
    if color.(start) = -1 && !result then begin
      color.(start) <- 0;
      Queue.push start q;
      while not (Queue.is_empty q) && !result do
        let u = Queue.pop q in
        List.iter (fun v ->
          if color.(v) = -1 then begin
            color.(v) <- 1 - color.(u);
            Queue.push v q
          end else if color.(v) = color.(u) then
            result := false
        ) adj.(u)
      done
    end
  done;
  if !result then Some color else None

let () =
  let n1  = 4 in
  let adj1 = Array.make n1 [] in
  let add1 u v = adj1.(u) <- v :: adj1.(u); adj1.(v) <- u :: adj1.(v) in
  add1 0 1; add1 1 2; add1 2 3; add1 3 0;  (* 4-cycle — bipartite *)
  (match is_bipartite adj1 n1 with
   | None   -> Printf.printf "4-cycle: NOT bipartite\n"
   | Some c -> Printf.printf "4-cycle: bipartite, colors=[%s]\n"
       (String.concat "," (Array.to_list (Array.map string_of_int c))));

  let n2   = 3 in
  let adj2 = Array.make n2 [] in
  let add2 u v = adj2.(u) <- v :: adj2.(u); adj2.(v) <- u :: adj2.(v) in
  add2 0 1; add2 1 2; add2 2 0;  (* triangle — not bipartite *)
  (match is_bipartite adj2 n2 with
   | None   -> Printf.printf "Triangle: NOT bipartite\n"
   | Some c -> Printf.printf "Triangle: bipartite, colors=[%s]\n"
       (String.concat "," (Array.to_list (Array.map string_of_int c))))
