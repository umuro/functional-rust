(* 259: flat_map — the iterator monad's bind
   OCaml: List.concat_map (= map then flatten), also List.filter_map.
   Seq.flat_map is the lazy equivalent. *)

let () =
  (* Expand each number into a range 0..n *)
  let result = List.concat_map (fun n -> List.init n Fun.id) [1; 2; 3] in
  Printf.printf "flat_map expand = [%s]\n"
    (result |> List.map string_of_int |> String.concat ";");

  (* Parse strings, silently dropping non-integers (filter_map pattern) *)
  let strings = ["1"; "x"; "2"; "y"; "3"] in
  let nums = List.filter_map (fun s ->
    match int_of_string_opt s with
    | Some n -> Some n
    | None   -> None
  ) strings in
  Printf.printf "filter_map parse = [%s]\n"
    (nums |> List.map string_of_int |> String.concat ";");

  (* Split sentences into words (flat_map over words) *)
  let sentences = ["hello world"; "foo bar"] in
  let words = List.concat_map (String.split_on_char ' ') sentences in
  Printf.printf "word count = %d\n" (List.length words);
  Printf.printf "words = [%s]\n" (String.concat ";" words);

  (* Lazy Seq.flat_map *)
  let lazy_result =
    List.to_seq [1; 2; 3]
    |> Seq.flat_map (fun n -> List.to_seq (List.init n Fun.id))
    |> List.of_seq in
  Printf.printf "seq flat_map = [%s]\n"
    (lazy_result |> List.map string_of_int |> String.concat ";");

  (* flat_map is monadic bind for lists: it distributes over the list monad *)
  let pairs = List.concat_map (fun x ->
                List.concat_map (fun y -> [(x, y)])
                  [10; 20]
              ) [1; 2] in
  Printf.printf "cartesian pairs = [%s]\n"
    (pairs |> List.map (fun (a,b) -> Printf.sprintf "(%d,%d)" a b)
           |> String.concat ";")
