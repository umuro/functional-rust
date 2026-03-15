(* 263: Fixed-size chunks iteration.
   Split a list into non-overlapping sub-lists of at most n elements.
   The last chunk may be smaller. *)

let chunks n lst =
  if n <= 0 then invalid_arg "chunks: n must be positive";
  let arr = Array.of_list lst in
  let len = Array.length arr in
  let num_chunks = (len + n - 1) / n in
  List.init num_chunks (fun i ->
    let start = i * n in
    let stop  = min (start + n) len in
    Array.to_list (Array.sub arr start (stop - start)))

(* chunks_exact: only full chunks, returns remainder separately *)
let chunks_exact n lst =
  let arr = Array.of_list lst in
  let len = Array.length arr in
  let full = len / n in
  let exact = List.init full (fun i ->
    Array.to_list (Array.sub arr (i * n) n)) in
  let remainder = Array.to_list (Array.sub arr (full * n) (len - full * n)) in
  (exact, remainder)

let () =
  let data = [1; 2; 3; 4; 5] in

  (* Basic chunks *)
  let cs = chunks 2 data in
  Printf.printf "chunks(2) count = %d\n" (List.length cs);
  Printf.printf "chunks(2)[0] = [%s]\n"
    (List.nth cs 0 |> List.map string_of_int |> String.concat ";");
  Printf.printf "chunks(2)[2] = [%s]\n"
    (List.nth cs 2 |> List.map string_of_int |> String.concat ";");

  (* chunks_exact with remainder *)
  let (exact, remainder) = chunks_exact 2 data in
  Printf.printf "chunks_exact(2) count = %d  remainder=[%s]\n"
    (List.length exact)
    (remainder |> List.map string_of_int |> String.concat ";");

  (* Divisible — all chunks have same length *)
  let data2 = [1;2;3;4] in
  let cs2 = chunks 2 data2 in
  let all_same_len = List.for_all (fun c -> List.length c = 2) cs2 in
  Printf.printf "divisible chunks all len=2: %b\n" all_same_len;

  (* Display all chunks *)
  chunks 2 [1;2;3;4;5]
  |> List.iteri (fun i c ->
    Printf.printf "  chunk[%d] = [%s]\n" i
      (c |> List.map string_of_int |> String.concat ";"))
