(* 264: take_while — yield elements until predicate first returns false.
   OCaml: no built-in, but trivial recursive function; Seq has it lazily. *)

let rec take_while f = function
  | []      -> []
  | x :: xs -> if f x then x :: take_while f xs else []

let () =
  (* Basic take_while *)
  let r1 = take_while (fun x -> x < 4) [1;2;3;4;5] in
  Printf.printf "take_while (<4)  [1..5]  = [%s]\n"
    (r1 |> List.map string_of_int |> String.concat ";");

  (* None match *)
  let r2 = take_while (fun x -> x > 0) [-1; 2; 3] in
  Printf.printf "take_while (>0)  [-1;2;3] = [%s]\n"
    (r2 |> List.map string_of_int |> String.concat ";");

  (* Stops at first failure — does NOT skip and continue *)
  let r3 = take_while (fun x -> x < 3) [1; 2; 5; 1; 2] in
  Printf.printf "stops early [1;2;5;1;2]   = [%s]\n"
    (r3 |> List.map string_of_int |> String.concat ";");

  (* From an infinite-like sequence using Seq *)
  let naturals = Seq.ints 0 in
  let r4 = Seq.take_while (fun x -> x < 5) naturals |> List.of_seq in
  Printf.printf "seq take_while (<5) [0..] = [%s]\n"
    (r4 |> List.map string_of_int |> String.concat ";");

  (* take_while on strings *)
  let words = ["apple"; "apricot"; "banana"; "cherry"] in
  let a_words = take_while (fun w -> String.get w 0 = 'a') words in
  Printf.printf "words starting with 'a': [%s]\n" (String.concat ";" a_words)
