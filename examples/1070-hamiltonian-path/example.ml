(* 1070: Hamiltonian Path — Backtracking *)

(* Approach 1: Adjacency matrix backtracking *)
let hamiltonian_path adj =
  let n = Array.length adj in
  let path = Array.make n (-1) in
  let visited = Array.make n false in
  path.(0) <- 0;
  visited.(0) <- true;
  let rec solve pos =
    if pos = n then true
    else begin
      let found = ref false in
      for v = 0 to n - 1 do
        if not !found && not visited.(v) && adj.(path.(pos - 1)).(v) = 1 then begin
          path.(pos) <- v;
          visited.(v) <- true;
          if solve (pos + 1) then found := true
          else begin
            path.(pos) <- -1;
            visited.(v) <- false
          end
        end
      done;
      !found
    end
  in
  if solve 1 then Some (Array.to_list path) else None

(* Approach 2: Adjacency list *)
let hamiltonian_path_adj adj n =
  let path = Array.make n (-1) in
  let visited = Array.make n false in
  path.(0) <- 0;
  visited.(0) <- true;
  let rec solve pos =
    if pos = n then true
    else begin
      let found = ref false in
      List.iter (fun v ->
        if not !found && not visited.(v) then begin
          path.(pos) <- v;
          visited.(v) <- true;
          if solve (pos + 1) then found := true
          else begin
            path.(pos) <- -1;
            visited.(v) <- false
          end
        end
      ) adj.(path.(pos - 1));
      !found
    end
  in
  if solve 1 then Some (Array.to_list path) else None

(* Approach 3: Try all starting vertices *)
let hamiltonian_path_any adj =
  let n = Array.length adj in
  let path = Array.make n (-1) in
  let visited = Array.make n false in
  let rec solve pos =
    if pos = n then true
    else begin
      let found = ref false in
      for v = 0 to n - 1 do
        if not !found && not visited.(v) && adj.(path.(pos - 1)).(v) = 1 then begin
          path.(pos) <- v;
          visited.(v) <- true;
          if solve (pos + 1) then found := true
          else begin
            path.(pos) <- -1;
            visited.(v) <- false
          end
        end
      done;
      !found
    end
  in
  let result = ref None in
  for start = 0 to n - 1 do
    if !result = None then begin
      Array.fill path 0 n (-1);
      Array.fill visited 0 n false;
      path.(0) <- start;
      visited.(start) <- true;
      if solve 1 then result := Some (Array.to_list path)
    end
  done;
  !result

let () =
  (* Complete graph K4 — trivially has Hamiltonian path *)
  let adj = [|
    [|0;1;1;1|];
    [|1;0;1;1|];
    [|1;1;0;1|];
    [|1;1;1;0|]
  |] in
  (match hamiltonian_path adj with
   | Some path -> assert (List.length path = 4)
   | None -> assert false);

  (* Path graph: 0-1-2-3 *)
  let adj2 = [|
    [|0;1;0;0|];
    [|1;0;1;0|];
    [|0;1;0;1|];
    [|0;0;1;0|]
  |] in
  (match hamiltonian_path adj2 with
   | Some path -> assert (List.length path = 4)
   | None -> assert false);

  (match hamiltonian_path_any adj with
   | Some path -> assert (List.length path = 4)
   | None -> assert false);

  Printf.printf "✓ All tests passed\n"
