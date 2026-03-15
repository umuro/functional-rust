(* 729: Avoid Allocations — OCaml perspective
   OCaml's GC handles heap allocation, but we can still minimize allocation
   pressure by reusing buffers and using sequences (lazy lists). *)

(* Reusable buffer pattern — avoids allocating a new string each call *)
let buf = Buffer.create 64

let format_record_into_buf name score =
  Buffer.clear buf;
  Buffer.add_string buf name;
  Buffer.add_char buf ':';
  Buffer.add_string buf (string_of_int score);
  Buffer.contents buf   (* still allocates the result string, but buf is reused *)

(* Sequence (lazy) — no intermediate list allocation *)
let sum_squares_seq n =
  Seq.init n (fun i -> i * i)
  |> Seq.fold_left (+) 0

(* Array (stack-ish in OCaml sense — one alloc, reused) *)
let process_with_fixed_buf data =
  let scratch = Array.make 256 0 in
  List.iteri (fun i x ->
    if i < 256 then scratch.(i) <- x * 2
  ) data;
  Array.sub scratch 0 (min (List.length data) 256)

(* String splitting without extra allocation — find positions *)
let count_commas s =
  String.fold_left (fun acc c -> if c = ',' then acc + 1 else acc) 0 s

let () =
  Printf.printf "Record: %s\n" (format_record_into_buf "Alice" 42);
  Printf.printf "Sum of squares 0..9: %d\n" (sum_squares_seq 10);
  Printf.printf "Commas in 'a,b,c,d': %d\n" (count_commas "a,b,c,d");
  let result = process_with_fixed_buf [1;2;3;4;5] in
  Printf.printf "Processed: [%s]\n"
    (Array.to_list result |> List.map string_of_int |> String.concat ";")
