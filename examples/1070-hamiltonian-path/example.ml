(* 1070: Hamiltonian Path — Backtracking and Bitmask DP *)

(* Approach 1: Backtracking from vertex 0 *)
let hamiltonian_path adj =
  let n = Array.length adj in
  let path = Array.make n 0 in
  let visited = Array.make n false in
  visited.(0) <- true;
  let rec solve pos =
    if pos = n then true
    else begin
      let prev = path.(pos - 1) in
      let found = ref false in
      let v = ref 0 in
      while !v < n && not !found do
        if not visited.(!v) && adj.(prev).(!v) = 1 then begin
          path.(pos) <- !v;
          visited.(!v) <- true;
          if solve (pos + 1) then found := true
          else visited.(!v) <- false
        end;
        incr v
      done;
      !found
    end
  in
  if solve 1 then Some (Array.to_list path) else None

(* Approach 2: Try all starting vertices *)
let hamiltonian_path_any adj =
  let n = Array.length adj in
  let try_start start =
    let path = Array.make n 0 in
    let visited = Array.make n false in
    path.(0) <- start;
    visited.(start) <- true;
    let rec solve pos =
      if pos = n then true
      else begin
        let prev = path.(pos - 1) in
        let found = ref false in
        let v = ref 0 in
        while !v < n && not !found do
          if not visited.(!v) && adj.(prev).(!v) = 1 then begin
            path.(pos) <- !v;
            visited.(!v) <- true;
            if solve (pos + 1) then found := true
            else visited.(!v) <- false
          end;
          incr v
        done;
        !found
      end
    in
    if solve 1 then Some (Array.to_list path) else None
  in
  let result = ref None in
  let s = ref 0 in
  while !s < n && !result = None do
    result := try_start !s;
    incr s
  done;
  !result

(* Approach 3: Bitmask DP — O(2^n * n^2) existence check *)
let hamiltonian_exists_dp adj =
  let n = Array.length adj in
  if n = 0 then true
  else begin
    (* dp.(mask).(i) = can we reach node i having visited exactly nodes in mask? *)
    let dp = Array.init (1 lsl n) (fun _ -> Array.make n false) in
    for i = 0 to n - 1 do
      dp.(1 lsl i).(i) <- true
    done;
    for mask = 1 to (1 lsl n) - 1 do
      for u = 0 to n - 1 do
        if dp.(mask).(u) && mask land (1 lsl u) <> 0 then
          for v = 0 to n - 1 do
            if mask land (1 lsl v) = 0 && adj.(u).(v) = 1 then
              dp.(mask lor (1 lsl v)).(v) <- true
          done
      done
    done;
    let full = (1 lsl n) - 1 in
    Array.exists (fun x -> x) dp.(full)
  end

let () =
  (* Complete graph on 4 vertices *)
  let adj = [|
    [|0;1;1;1|]; [|1;0;1;1|]; [|1;1;0;1|]; [|1;1;1;0|]
  |] in
  (match hamiltonian_path adj with
   | Some path -> Printf.printf "Path from 0: %s\n"
       (String.concat " -> " (List.map string_of_int path))
   | None -> print_endline "No path found");

  (* Disconnected graph *)
  let disc = [|[|0;0;0|];[|0;0;0|];[|0;0;0|]|] in
  Printf.printf "Disconnected exists: %b\n" (hamiltonian_exists_dp disc);
  Printf.printf "Complete exists: %b\n" (hamiltonian_exists_dp adj)
