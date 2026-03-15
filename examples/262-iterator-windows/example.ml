(* 262: Sliding windows over a sequence.
   OCaml standard library has no built-in windows, but it's easy to build.
   windows n yields overlapping sub-lists of length n. *)

(* Sliding windows over a list *)
let windows n lst =
  if n <= 0 then []
  else begin
    let arr = Array.of_list lst in
    let len = Array.length arr in
    if len < n then []
    else
      List.init (len - n + 1) (fun i ->
        Array.to_list (Array.sub arr i n))
  end

let () =
  (* Count of windows *)
  let data = [1; 2; 3; 4; 5] in
  let wins3 = windows 3 data in
  Printf.printf "windows(3) count = %d\n" (List.length wins3);

  (* Moving average over windows of 2 *)
  let avgs = windows 2 data
    |> List.map (fun w ->
      let s = List.fold_left ( + ) 0 w in
      float_of_int s /. float_of_int (List.length w)) in
  Printf.printf "moving avg (n=2) = [%s]\n"
    (avgs |> List.map (Printf.sprintf "%.1f") |> String.concat ";");

  (* Check if list is sorted using windows of 2 *)
  let is_sorted lst =
    windows 2 lst |> List.for_all (function
      | [a; b] -> a <= b
      | _      -> true)
  in
  Printf.printf "is_sorted [1;2;3;4] = %b\n" (is_sorted [1;2;3;4]);
  Printf.printf "is_sorted [1;3;2;4] = %b\n" (is_sorted [1;3;2;4]);

  (* Display each window *)
  windows 3 [1;2;3;4;5]
  |> List.iter (fun w ->
    Printf.printf "  window [%s]\n"
      (w |> List.map string_of_int |> String.concat ";"))
