(* 1057: Matrix Chain Multiplication — Optimal Parenthesization *)

(* Approach 1: Bottom-up DP *)
let matrix_chain_dp dims =
  let n = Array.length dims - 1 in
  let dp = Array.init n (fun _ -> Array.make n 0) in
  (* chain length l = 2..n *)
  for l = 2 to n do
    for i = 0 to n - l do
      let j = i + l - 1 in
      dp.(i).(j) <- max_int;
      for k = i to j - 1 do
        let cost = dp.(i).(k) + dp.(k + 1).(j)
                   + dims.(i) * dims.(k + 1) * dims.(j + 1) in
        if cost < dp.(i).(j) then
          dp.(i).(j) <- cost
      done
    done
  done;
  dp.(0).(n - 1)

(* Approach 2: With parenthesization tracking *)
let matrix_chain_parens dims =
  let n = Array.length dims - 1 in
  let dp = Array.init n (fun _ -> Array.make n 0) in
  let split = Array.init n (fun _ -> Array.make n 0) in
  for l = 2 to n do
    for i = 0 to n - l do
      let j = i + l - 1 in
      dp.(i).(j) <- max_int;
      for k = i to j - 1 do
        let cost = dp.(i).(k) + dp.(k + 1).(j)
                   + dims.(i) * dims.(k + 1) * dims.(j + 1) in
        if cost < dp.(i).(j) then begin
          dp.(i).(j) <- cost;
          split.(i).(j) <- k
        end
      done
    done
  done;
  let buf = Buffer.create 32 in
  let rec build i j =
    if i = j then
      Buffer.add_string buf (Printf.sprintf "A%d" (i + 1))
    else begin
      Buffer.add_char buf '(';
      build i split.(i).(j);
      Buffer.add_char buf '*';
      build (split.(i).(j) + 1) j;
      Buffer.add_char buf ')'
    end
  in
  build 0 (n - 1);
  (dp.(0).(n - 1), Buffer.contents buf)

(* Approach 3: Recursive with memoization *)
let matrix_chain_memo dims =
  let n = Array.length dims - 1 in
  let cache = Hashtbl.create 64 in
  let rec solve i j =
    if i = j then 0
    else
      match Hashtbl.find_opt cache (i, j) with
      | Some v -> v
      | None ->
        let best = ref max_int in
        for k = i to j - 1 do
          let cost = solve i k + solve (k + 1) j
                     + dims.(i) * dims.(k + 1) * dims.(j + 1) in
          if cost < !best then best := cost
        done;
        Hashtbl.add cache (i, j) !best;
        !best
  in
  solve 0 (n - 1)

let () =
  (* dims: A1=30x35, A2=35x15, A3=15x5, A4=5x10, A5=10x20, A6=20x25 *)
  let dims = [|30; 35; 15; 5; 10; 20; 25|] in
  assert (matrix_chain_dp dims = 15125);
  assert (matrix_chain_memo dims = 15125);
  let (cost, parens) = matrix_chain_parens dims in
  assert (cost = 15125);
  assert (String.length parens > 0);

  let dims2 = [|10; 20; 30; 40|] in
  assert (matrix_chain_dp dims2 = 18000);

  Printf.printf "✓ All tests passed\n"
