(* 096: Exact-Size Iterator
   OCaml: Array.length and List.length are O(1)/O(n); Seq.length_up_to etc. *)

(* --- Approach 1: Known-size collections --- *)

let array_demo () =
  let v = [|1;2;3;4;5|] in
  Printf.printf "array length = %d\n" (Array.length v);  (* O(1) *)
  (* Ranges: can compute size before materialising *)
  let range_len start stop = max 0 (stop - start) in
  Printf.printf "range 0..10 len = %d\n" (range_len 0 10)

(* --- Approach 2: enumerate with index --- *)

let enumerate xs =
  List.mapi (fun i x -> (i, x)) xs

let () =
  array_demo ();

  let v = ["a";"b";"c"] in
  let indexed = enumerate v in
  List.iter (fun (i, s) -> Printf.printf "(%d, %s) " i s) indexed;
  print_newline ();

  (* chunks_exact analogue: drop the remainder *)
  let chunks_exact n xs =
    let arr = Array.of_list xs in
    let len = Array.length arr in
    let full = len / n in     (* number of complete chunks *)
    let remainder = Array.to_list (Array.sub arr (full * n) (len mod n)) in
    let chunks = List.init full (fun i ->
      Array.to_list (Array.sub arr (i * n) n)) in
    (chunks, remainder)
  in

  let (c, r) = chunks_exact 2 [1;2;3;4;5] in
  Printf.printf "chunks_exact 2 [1..5]:\n";
  List.iter (fun chunk ->
    Printf.printf "  [%s]\n" (String.concat ";" (List.map string_of_int chunk))
  ) c;
  Printf.printf "remainder = [%s]\n"
    (String.concat "; " (List.map string_of_int r))
