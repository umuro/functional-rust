(* 256. Chaining iterators with chain() - OCaml *)
(* OCaml uses @ or List.append to concatenate lists eagerly *)

let () =
  let first = [1; 2; 3] in
  let second = [4; 5; 6] in
  (* @ operator is syntactic sugar for List.append -- allocates a new list *)
  let chained = first @ second in
  List.iter (fun x -> Printf.printf "%d " x) chained;
  print_newline ();

  let words_a = ["hello"; "world"] in
  let words_b = ["foo"; "bar"] in
  let all_words = List.append words_a words_b in
  List.iter (fun w -> Printf.printf "%s " w) all_words;
  print_newline ();

  (* Simulating lazy chaining with Seq module *)
  let seq1 = List.to_seq [10; 20; 30] in
  let seq2 = List.to_seq [40; 50; 60] in
  let chained_seq = Seq.append seq1 seq2 in
  Seq.iter (fun x -> Printf.printf "%d " x) chained_seq;
  print_newline ()
