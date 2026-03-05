(* Range Minimum Query — Sparse Table O(n log n) build, O(1) query *)

let build_sparse arr =
  let n = Array.length arr in
  let log2 = if n > 1 then int_of_float (log (float_of_int n) /. log 2.0) + 1 else 1 in
  let table = Array.init log2 (fun k ->
    Array.init n (fun i ->
      if k = 0 then arr.(i)
      else min max_int max_int (* filled below *)
    )
  ) in
  table.(0) <- Array.copy arr;
  for k = 1 to log2 - 1 do
    for i = 0 to n - (1 lsl k) do
      table.(k).(i) <- min table.(k-1).(i) table.(k-1).(i + (1 lsl (k-1)))
    done
  done;
  table

let query table l r =
  let len = r - l + 1 in
  let k   = int_of_float (log (float_of_int len) /. log 2.0) in
  min table.(k).(l) table.(k).(r - (1 lsl k) + 1)

let () =
  let arr   = [| 2; 4; 3; 1; 6; 7; 8; 9; 1; 7 |] in
  let table = build_sparse arr in
  Printf.printf "Array: ";
  Array.iter (fun x -> Printf.printf "%d " x) arr;
  print_newline ();
  Printf.printf "RMQ(0,9) = %d  (expect 1)\n" (query table 0 9);
  Printf.printf "RMQ(1,5) = %d  (expect 1)\n" (query table 1 5);
  Printf.printf "RMQ(2,4) = %d  (expect 1)\n" (query table 2 4);
  Printf.printf "RMQ(6,9) = %d  (expect 1)\n" (query table 6 9)
