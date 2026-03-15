(* 994: MapReduce *)
(* Parallel map with threads, collect results, reduce *)

(* --- Approach 1: Parallel map → collect → fold (reduce) --- *)

let parallel_map f xs =
  let n = List.length xs in
  let results = Array.make n None in
  let threads = List.mapi (fun i x ->
    Thread.create (fun () ->
      results.(i) <- Some (f x)
    ) ()
  ) xs in
  List.iter Thread.join threads;
  Array.to_list results |> List.filter_map Fun.id

let map_reduce ~map_fn ~reduce_fn ~init xs =
  let mapped = parallel_map map_fn xs in
  List.fold_left reduce_fn init mapped

let () =
  (* Word count simulation: count chars in each word, sum total *)
  let words = ["hello"; "world"; "ocaml"; "functional"; "programming"] in
  let total_chars = map_reduce
    ~map_fn:(fun w -> String.length w)
    ~reduce_fn:(+)
    ~init:0
    words
  in
  assert (total_chars = 5+5+5+10+11);
  Printf.printf "Approach 1 (char count): %d\n" total_chars

(* --- Approach 2: Map with chunking (divide-and-conquer) --- *)

let chunk_parallel_map f xs num_workers =
  let arr = Array.of_list xs in
  let n = Array.length arr in
  let chunk_size = max 1 ((n + num_workers - 1) / num_workers) in
  let results = Array.make n (f arr.(0)) in  (* placeholder *)
  let threads = List.init num_workers (fun w ->
    Thread.create (fun () ->
      let start = w * chunk_size in
      let stop = min n ((w + 1) * chunk_size) in
      for i = start to stop - 1 do
        results.(i) <- f arr.(i)
      done
    ) ()
  ) in
  List.iter Thread.join threads;
  Array.to_list results

let () =
  let nums = List.init 20 (fun i -> i + 1) in  (* 1..20 *)
  let squares = chunk_parallel_map (fun x -> x * x) nums 4 in
  let sum = List.fold_left (+) 0 squares in
  assert (sum = 2870);
  Printf.printf "Approach 2 (chunked map-reduce): sum_squares=%d\n" sum

(* --- Approach 3: MapReduce with string processing --- *)

let () =
  let sentences = ["the quick brown fox"; "jumps over the lazy"; "dog today"] in
  let word_counts = map_reduce
    ~map_fn:(fun s ->
      String.split_on_char ' ' s |> List.length)
    ~reduce_fn:(+)
    ~init:0
    sentences
  in
  (* 4+4+2 = 10 *)
  assert (word_counts = 10);
  Printf.printf "Approach 3 (word count): %d words\n" word_counts

let () = Printf.printf "✓ All tests passed\n"
