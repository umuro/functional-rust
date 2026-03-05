(* Counting Paths in Grid with Obstacles — DP O(m×n) *)
(* grid.(i).(j) = 0 means open, 1 means obstacle *)

let count_paths grid =
  let m = Array.length grid in
  if m = 0 then 0
  else begin
    let n  = Array.length grid.(0) in
    let dp = Array.make_matrix m n 0 in
    (* Start must be open *)
    if grid.(0).(0) = 1 then 0
    else begin
      dp.(0).(0) <- 1;
      (* First row *)
      for j = 1 to n - 1 do
        dp.(0).(j) <- if grid.(0).(j) = 0 then dp.(0).(j-1) else 0
      done;
      (* First col *)
      for i = 1 to m - 1 do
        dp.(i).(0) <- if grid.(i).(0) = 0 then dp.(i-1).(0) else 0
      done;
      (* Rest *)
      for i = 1 to m - 1 do
        for j = 1 to n - 1 do
          dp.(i).(j) <- if grid.(i).(j) = 1 then 0
                        else dp.(i-1).(j) + dp.(i).(j-1)
        done
      done;
      dp.(m-1).(n-1)
    end
  end

let () =
  let grid1 = [| [|0;0;0|]; [|0;1;0|]; [|0;0;0|] |] in
  Printf.printf "3×3 grid with center obstacle: %d paths\n" (count_paths grid1);

  let grid2 = [| [|0;0;0|]; [|0;0;0|]; [|0;0;0|] |] in
  Printf.printf "3×3 open grid: %d paths\n" (count_paths grid2);

  let grid3 = [| [|0;1|]; [|0;0|] |] in
  Printf.printf "2×2 grid [[0,1],[0,0]]: %d paths\n" (count_paths grid3)
