(* 277: count — consume an iterator and return the total number of elements.
   OCaml: List.length for lists; Seq.fold_left for lazy sequences. *)

(* count_if: count elements satisfying a predicate *)
let count_if pred lst = List.fold_left (fun n x -> if pred x then n + 1 else n) 0 lst

let () =
  (* Basic count *)
  Printf.printf "length 1..10 = %d\n" (List.length (List.init 10 (fun i -> i + 1)));

  (* Count filtered elements *)
  let evens = count_if (fun x -> x mod 2 = 0) (List.init 10 (fun i -> i + 1)) in
  Printf.printf "even count in 1..10 = %d\n" evens;

  (* Empty *)
  Printf.printf "length [] = %d\n" (List.length []);

  (* Count vowels in a string *)
  let vowels = String.to_seq "hello"
    |> Seq.fold_left (fun n c ->
        if String.contains "aeiou" c then n + 1 else n) 0 in
  Printf.printf "vowels in \"hello\" = %d\n" vowels;

  (* Lazy count via Seq *)
  let lazy_count = Seq.ints 1
    |> Seq.take_while (fun x -> x <= 100)
    |> Seq.fold_left (fun n _ -> n + 1) 0 in
  Printf.printf "lazy count 1..100 = %d\n" lazy_count;

  (* Count words in sentences *)
  let sentences = ["hello world"; "foo bar baz"; "one"] in
  let word_count = List.fold_left (fun n s ->
    n + List.length (String.split_on_char ' ' s)) 0 sentences in
  Printf.printf "total words = %d\n" word_count;

  (* Count distinct elements using a set *)
  let items = [1;2;2;3;3;3;4] in
  let module S = Set.Make(Int) in
  let distinct = List.fold_left (fun s x -> S.add x s) S.empty items |> S.cardinal in
  Printf.printf "distinct elements = %d\n" distinct
