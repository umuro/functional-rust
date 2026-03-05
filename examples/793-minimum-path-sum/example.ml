(* Minimum Path Sum — in-place DP, O(m×n) time, O(1) extra space *)

let min_path_sum grid =
  let m = Array.length grid in
  if m = 0 then 0
  else begin
    let n = Array.length grid.(0) in
    let g = Array.init m (fun i -> Array.copy grid.(i)) in
    (* Fill first row *)
    for j = 1 to n - 1 do
      g.(0).(j) <- g.(0).(j) + g.(0).(j-1)
    done;
    (* Fill first col *)
    for i = 1 to m - 1 do
      g.(i).(0) <- g.(i).(0) + g.(i-1).(0)
    done;
    (* Fill rest *)
    for i = 1 to m - 1 do
      for j = 1 to n - 1 do
        g.(i).(j) <- g.(i).(j) + min g.(i-1).(j) g.(i).(j-1)
      done
    done;
    g.(m-1).(n-1)
  end

(* Reconstruct the actual path *)
let min_path_reconstruct grid =
  let m = Array.length grid in
  let n = Array.length grid.(0) in
  let g = Array.init m (fun i -> Array.copy grid.(i)) in
  for j = 1 to n - 1 do g.(0).(j) <- g.(0).(j) + g.(0).(j-1) done;
  for i = 1 to m - 1 do g.(i).(0) <- g.(i).(0) + g.(i-1).(0) done;
  for i = 1 to m - 1 do
    for j = 1 to n - 1 do
      g.(i).(j) <- g.(i).(j) + min g.(i-1).(j) g.(i).(j-1)
    done
  done;
  let path = ref [] in
  let i = ref (m-1) and j = ref (n-1) in
  while !i > 0 || !j > 0 do
    path := (!i, !j) :: !path;
    if !i = 0 then decr j
    else if !j = 0 then decr i
    else if g.(!i-1).(!j) < g.(!i).(!j-1) then decr i
    else decr j
  done;
  path := (0,0) :: !path;
  (!path, g.(m-1).(n-1))

let () =
  let grid = [| [|1;3;1|]; [|1;5;1|]; [|4;2;1|] |] in
  Printf.printf "Min path sum: %d\n" (min_path_sum grid);
  let (path, cost) = min_path_reconstruct grid in
  Printf.printf "Path (cost=%d): " cost;
  List.iter (fun (r,c) -> Printf.printf "(%d,%d) " r c) path;
  print_newline ()
