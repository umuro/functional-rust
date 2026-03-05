(* Matrix Chain Multiplication — top-down memoised DP
   dims.(i) = rows of matrix i, dims.(i+1) = cols of matrix i
   e.g., for 3 matrices: dims = [|p0; p1; p2; p3|] *)

let matrix_chain dims =
  let n = Array.length dims - 1 in
  (* dp.(i).(j) = min cost to multiply matrices i..j (0-indexed) *)
  let dp    = Array.make_matrix n n 0 in
  let split = Array.make_matrix n n 0 in
  (* chain length l from 2 to n *)
  for l = 2 to n do
    for i = 0 to n - l do
      let j = i + l - 1 in
      dp.(i).(j) <- max_int;
      for k = i to j - 1 do
        let cost = dp.(i).(k) + dp.(k+1).(j)
                   + dims.(i) * dims.(k+1) * dims.(j+1) in
        if cost < dp.(i).(j) then begin
          dp.(i).(j)    <- cost;
          split.(i).(j) <- k
        end
      done
    done
  done;
  (dp.(0).(n-1), split)

(* Reconstruct optimal parenthesization as a string *)
let rec parenthesize split i j =
  if i = j then
    Printf.sprintf "M%d" (i + 1)
  else
    let k = split.(i).(j) in
    Printf.sprintf "(%s × %s)"
      (parenthesize split i k)
      (parenthesize split (k+1) j)

let () =
  (* Example: 4 matrices with dims 30×35, 35×15, 15×5, 5×10 *)
  let dims = [| 30; 35; 15; 5; 10; 20; 25 |] in
  let n    = Array.length dims - 1 in
  Printf.printf "Number of matrices: %d\n" n;
  let (cost, split) = matrix_chain dims in
  Printf.printf "Minimum multiplications: %d\n" cost;
  Printf.printf "Optimal order: %s\n" (parenthesize split 0 (n-1));

  (* Small example *)
  let dims2  = [| 10; 30; 5; 60 |] in
  let (c2, s2) = matrix_chain dims2 in
  Printf.printf "\n3-matrix example (10×30, 30×5, 5×60):\n";
  Printf.printf "Min cost: %d, Order: %s\n" c2 (parenthesize s2 0 2)
