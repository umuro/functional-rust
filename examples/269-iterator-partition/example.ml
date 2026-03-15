(* 269: partition — split a list into two by a predicate.
   OCaml: List.partition does exactly this in a single pass. *)

let () =
  (* Even/odd partition *)
  let (evens, odds) = List.partition (fun x -> x mod 2 = 0) [1;2;3;4;5;6] in
  Printf.printf "evens = [%s]\n"
    (evens |> List.map string_of_int |> String.concat ";");
  Printf.printf "odds  = [%s]\n"
    (odds  |> List.map string_of_int |> String.concat ";");

  (* Partition Results into Ok and Error values *)
  let results = [Ok 1; Error "e1"; Ok 3; Error "e2"] in
  let (oks, errs) = List.partition Result.is_ok results in
  Printf.printf "ok count = %d  err count = %d\n"
    (List.length oks) (List.length errs);

  (* All in one partition — empty second list *)
  let (pos, neg) = List.partition (fun x -> x > 0) [2;4;6] in
  Printf.printf "|positive| = %d  |non-positive| = %d\n"
    (List.length pos) (List.length neg);

  (* Partition strings by length *)
  let words = ["hi"; "hello"; "yo"; "world"; "a"] in
  let (short, long) = List.partition (fun w -> String.length w <= 2) words in
  Printf.printf "short words: [%s]\n" (String.concat ";" short);
  Printf.printf "long  words: [%s]\n" (String.concat ";"; long);

  (* partition_map (OCaml 4.12+): split and transform simultaneously *)
  let mixed = ["1"; "x"; "2"; "y"; "3"] in
  let (nums, strs) = List.partition_map (fun s ->
    match int_of_string_opt s with
    | Some n -> Left n
    | None   -> Right s
  ) mixed in
  Printf.printf "parsed ints: [%s]\n"
    (nums |> List.map string_of_int |> String.concat ";");
  Printf.printf "non-ints:    [%s]\n" (String.concat ";" strs)
