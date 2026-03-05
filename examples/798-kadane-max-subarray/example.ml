(* Kadane's Algorithm — functional fold, O(n) *)

let max_subarray arr =
  let n = Array.length arr in
  if n = 0 then failwith "empty array"
  else begin
    (* State: (curr_sum, best_sum, curr_start, best_start, best_end) *)
    let init = (arr.(0), arr.(0), 0, 0, 0) in
    let (_, best, _, bs, be) =
      Array.fold_left (fun (curr, best, cs, bs, be) x ->
        let i = be + 1 in (* current index in fold — we track via be *)
        let (curr', cs') =
          if x > curr + x then (x, i)
          else (curr + x, cs)
        in
        if curr' > best then (curr', curr', cs', cs', i)
        else (curr', best, cs', bs, be)
      ) init (Array.sub arr 1 (n - 1))
    in
    (* Simpler version with index tracking *)
    ignore (best, bs, be);

    (* Clean re-implementation with proper index tracking *)
    let best_sum   = ref arr.(0) in
    let best_start = ref 0 in
    let best_end   = ref 0 in
    let curr_sum   = ref arr.(0) in
    let curr_start = ref 0 in
    for i = 1 to n - 1 do
      if arr.(i) > !curr_sum + arr.(i) then begin
        curr_sum   := arr.(i);
        curr_start := i
      end else
        curr_sum := !curr_sum + arr.(i);
      if !curr_sum > !best_sum then begin
        best_sum   := !curr_sum;
        best_start := !curr_start;
        best_end   := i
      end
    done;
    (!best_sum, !best_start, !best_end)
  end

let () =
  let arr = [| -2; 1; -3; 4; -1; 2; 1; -5; 4 |] in
  let (sum, s, e) = max_subarray arr in
  Printf.printf "Array: ";
  Array.iter (fun x -> Printf.printf "%d " x) arr;
  print_newline ();
  Printf.printf "Max subarray sum: %d  (indices %d..%d)\n" sum s e;
  Printf.printf "Subarray: ";
  for i = s to e do Printf.printf "%d " arr.(i) done;
  print_newline ();

  let all_neg = [| -5; -3; -1; -2; -4 |] in
  let (s2,_,_) = max_subarray all_neg in
  Printf.printf "All-negative max sum: %d\n" s2
