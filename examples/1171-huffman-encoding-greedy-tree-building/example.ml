type htree = Leaf of char * int | Node of htree * htree * int

let freq t = match t with Leaf (_,f) -> f | Node (_,_,f) -> f

let build_tree freqs =
  let trees = List.map (fun (c,f) -> Leaf (c,f)) freqs
    |> List.sort (fun a b -> compare (freq a) (freq b)) in
  let rec go = function
    | [t] -> t
    | a :: b :: rest ->
      let merged = Node (a, b, freq a + freq b) in
      let trees = List.sort (fun a b -> compare (freq a) (freq b)) (merged :: rest) in
      go trees
    | [] -> failwith "empty"
  in go trees

let rec codes prefix = function
  | Leaf (c, _) -> [(c, prefix)]
  | Node (l, r, _) -> codes (prefix ^ "0") l @ codes (prefix ^ "1") r

let () =
  let freqs = [('a',5);('b',9);('c',12);('d',13);('e',16);('f',45)] in
  let tree = build_tree freqs in
  let cs = codes "" tree in
  (* 'f' has the highest frequency — gets the shortest code *)
  assert (List.assoc 'f' cs = "0");
  (* 'a' and 'b' are least frequent — get 4-bit codes *)
  assert (String.length (List.assoc 'a' cs) = 4);
  assert (String.length (List.assoc 'b' cs) = 4);
  (* all 6 characters are encoded *)
  assert (List.length cs = 6);
  print_endline "ok"
