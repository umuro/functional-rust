(* LIS — patience sorting O(n log n)
   OCaml style: functional binary search, Array for tails *)

(* Binary search: find leftmost position in [0, hi) where tails.(pos) >= x *)
let lower_bound tails hi x =
  let rec go lo hi =
    if lo >= hi then lo
    else
      let mid = lo + (hi - lo) / 2 in
      if tails.(mid) < x then go (mid + 1) hi
      else go lo mid
  in
  go 0 hi

let lis arr =
  let n = Array.length arr in
  if n = 0 then 0
  else begin
    let tails = Array.make n 0 in
    let len = ref 0 in
    Array.iter (fun x ->
      let pos = lower_bound tails !len x in
      tails.(pos) <- x;
      if pos = !len then incr len
    ) arr;
    !len
  end

(* Reconstruct the actual LIS (not just length) *)
let lis_reconstruct arr =
  let n = Array.length arr in
  if n = 0 then [||]
  else begin
    let tails = Array.make n 0 in
    let pred  = Array.make n (-1) in  (* predecessor index *)
    let idx   = Array.make n 0 in     (* index in arr for each tails slot *)
    let len   = ref 0 in
    for i = 0 to n - 1 do
      let x   = arr.(i) in
      let pos = lower_bound tails !len x in
      tails.(pos) <- x;
      idx.(pos)   <- i;
      pred.(i)    <- if pos > 0 then idx.(pos - 1) else -1;
      if pos = !len then incr len
    done;
    (* Walk back via pred *)
    let result = Array.make !len 0 in
    let k = ref (idx.(!len - 1)) in
    for j = !len - 1 downto 0 do
      result.(j) <- arr.(!k);
      k := pred.(!k)
    done;
    result
  end

let () =
  let arr = [| 10; 9; 2; 5; 3; 7; 101; 18 |] in
  Printf.printf "Array: ";
  Array.iter (fun x -> Printf.printf "%d " x) arr;
  print_newline ();
  Printf.printf "LIS length: %d\n" (lis arr);
  let seq = lis_reconstruct arr in
  Printf.printf "LIS sequence: ";
  Array.iter (fun x -> Printf.printf "%d " x) seq;
  print_newline ();

  (* Edge cases *)
  Printf.printf "Empty LIS: %d\n" (lis [||]);
  Printf.printf "All same [3;3;3]: %d\n" (lis [|3;3;3|]);
  Printf.printf "Sorted [1;2;3;4;5]: %d\n" (lis [|1;2;3;4;5|]);
  Printf.printf "Reverse [5;4;3;2;1]: %d\n" (lis [|5;4;3;2;1|])
