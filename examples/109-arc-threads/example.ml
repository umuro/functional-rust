(* Example 109: Arc<T> — Thread-Safe Sharing *)

(* OCaml's GC is thread-safe by default. Sharing data between
   threads (via Domain in OCaml 5) just works. *)

(* Approach 1: Shared immutable data across "workers" *)
let process_chunk data start len =
  let sum = ref 0 in
  for i = start to start + len - 1 do
    sum := !sum + data.(i)
  done;
  !sum

let approach1 () =
  let data = Array.init 100 (fun i -> i + 1) in
  let sum1 = process_chunk data 0 50 in
  let sum2 = process_chunk data 50 50 in
  let total = sum1 + sum2 in
  assert (total = 5050);
  Printf.printf "Total: %d\n" total

(* Approach 2: Map-reduce pattern *)
let map_reduce mapper reducer init data =
  let mapped = List.map mapper data in
  List.fold_left reducer init mapped

let approach2 () =
  let data = [1; 2; 3; 4; 5] in
  let result = map_reduce (fun x -> x * x) ( + ) 0 data in
  assert (result = 55);
  Printf.printf "Sum of squares: %d\n" result

(* Approach 3: Parallel word count simulation *)
let count_words text =
  let words = String.split_on_char ' ' text in
  List.length (List.filter (fun w -> String.length w > 0) words)

let approach3 () =
  let texts = ["hello world"; "foo bar baz"; "one"] in
  let counts = List.map count_words texts in
  let total = List.fold_left ( + ) 0 counts in
  assert (total = 6);
  Printf.printf "Total words: %d\n" total

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
