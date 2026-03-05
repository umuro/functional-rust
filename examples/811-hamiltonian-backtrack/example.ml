(* Hamiltonian Cycle — backtracking O(n!) worst case *)

let hamiltonian_cycle adj n =
  let path    = Array.make n (-1) in
  let visited = Array.make n false in
  path.(0)    <- 0;
  visited.(0) <- true;

  let rec backtrack pos =
    if pos = n then
      (* Check if last vertex connects to start *)
      List.mem 0 adj.(path.(n-1))
    else
      List.exists (fun v ->
        if not visited.(v) && List.mem v adj.(path.(pos-1)) then begin
          path.(pos)    <- v;
          visited.(v)   <- true;
          let result = backtrack (pos + 1) in
          if not result then begin
            path.(pos)  <- -1;
            visited.(v) <- false
          end;
          result
        end else false
      ) (List.init n (fun i -> i))
  in
  if backtrack 1 then
    Some (Array.to_list path @ [0])
  else
    None

let () =
  (* Petersen graph — known to have Hamiltonian cycle *)
  let n   = 5 in
  let adj = Array.make n [] in
  let add u v = adj.(u) <- v :: adj.(u); adj.(v) <- u :: adj.(v) in
  add 0 1; add 1 2; add 2 3; add 3 4; add 4 0;  (* outer 5-cycle *)
  (match hamiltonian_cycle adj n with
   | None   -> Printf.printf "No Hamiltonian cycle\n"
   | Some p -> Printf.printf "Hamiltonian cycle: [%s]\n"
       (String.concat " -> " (List.map string_of_int p)));

  (* Complete graph K4 *)
  let n2  = 4 in
  let adj2 = Array.make n2 [] in
  for u = 0 to n2-1 do
    for v = 0 to n2-1 do
      if u <> v then adj2.(u) <- v :: adj2.(u)
    done
  done;
  (match hamiltonian_cycle adj2 n2 with
   | None   -> Printf.printf "No Hamiltonian cycle in K4\n"
   | Some p -> Printf.printf "K4 Hamiltonian cycle: [%s]\n"
       (String.concat " -> " (List.map string_of_int p)))
