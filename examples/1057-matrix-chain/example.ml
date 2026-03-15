(* 1057: Matrix Chain Multiplication — Optimal Parenthesization
   Bottom-up DP, parenthesization reconstruction, and memoization. *)

(* Approach 1: Bottom-up DP — compute minimum cost *)
let matrix_chain_dp dims =
  let n = Array.length dims - 1 in  (* number of matrices *)
  let dp = Array.init n (fun _ -> Array.make n 0) in
  (* l = chain length *)
  for l = 2 to n do
    for i = 0 to n - l do
      let j = i + l - 1 in
      dp.(i).(j) <- max_int;
      for k = i to j - 1 do
        let cost = dp.(i).(k) + dp.(k+1).(j)
                   + dims.(i) * dims.(k+1) * dims.(j+1) in
        if cost < dp.(i).(j) then dp.(i).(j) <- cost
      done
    done
  done;
  dp.(0).(n - 1)

(* Approach 2: With parenthesization string *)
let matrix_chain_parens dims =
  let n = Array.length dims - 1 in
  let dp    = Array.init n (fun _ -> Array.make n 0) in
  let split = Array.init n (fun _ -> Array.make n 0) in
  for l = 2 to n do
    for i = 0 to n - l do
      let j = i + l - 1 in
      dp.(i).(j) <- max_int;
      for k = i to j - 1 do
        let cost = dp.(i).(k) + dp.(k+1).(j)
                   + dims.(i) * dims.(k+1) * dims.(j+1) in
        if cost < dp.(i).(j) then begin
          dp.(i).(j) <- cost;
          split.(i).(j) <- k
        end
      done
    done
  done;
  let rec build i j =
    if i = j then Printf.sprintf "A%d" (i + 1)
    else
      Printf.sprintf "(%s*%s)"
        (build i split.(i).(j))
        (build (split.(i).(j) + 1) j)
  in
  (dp.(0).(n - 1), build 0 (n - 1))

(* Approach 3: Top-down memoization *)
let matrix_chain_memo dims =
  let n = Array.length dims - 1 in
  let cache = Hashtbl.create 64 in
  let rec solve i j =
    if i = j then 0
    else match Hashtbl.find_opt cache (i, j) with
    | Some v -> v
    | None ->
      let best = ref max_int in
      for k = i to j - 1 do
        let cost = solve i k + solve (k+1) j
                   + dims.(i) * dims.(k+1) * dims.(j+1) in
        if cost < !best then best := cost
      done;
      Hashtbl.add cache (i, j) !best; !best
  in
  solve 0 (n - 1)

let () =
  assert (matrix_chain_dp   [|30;35;15;5;10;20;25|] = 15125);
  assert (matrix_chain_dp   [|10;20;30;40|]          = 18000);
  assert (matrix_chain_memo [|30;35;15;5;10;20;25|] = 15125);
  assert (matrix_chain_memo [|10;20;30;40|]          = 18000);

  let (cost, parens) = matrix_chain_parens [|30;35;15;5;10;20;25|] in
  assert (cost = 15125);
  assert (String.length parens > 0);
  Printf.printf "Parenthesization: %s\n" parens;

  Printf.printf "All matrix-chain tests passed.\n"
