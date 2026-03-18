type htree = Leaf of char * int | Node of htree * htree * int

let freq t = match t with Leaf (_,f) -> f | Node (_,_,f) -> f

(* Idiomatic OCaml: greedy merge using List.sort on each step *)
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

(* Recursive tree traversal to produce codes *)
let rec codes prefix = function
  | Leaf (c, _) -> [(c, prefix)]
  | Node (l, r, _) -> codes (prefix ^ "0") l @ codes (prefix ^ "1") r

let () =
  let freqs = [('a',5);('b',9);('c',12);('d',13);('e',16);('f',45)] in
  let tree = build_tree freqs in
  let table = codes "" tree in
  (* 'f' gets the shortest code — verify *)
  let f_code = List.assoc 'f' table in
  let min_len = List.fold_left (fun m (_,c) -> min m (String.length c)) max_int table in
  assert (String.length f_code = min_len);
  (* All 6 characters get a code *)
  assert (List.length table = 6);
  (* Total frequency in root *)
  assert (freq tree = 100);
  table |> List.sort (fun (a,_) (b,_) -> compare a b)
        |> List.iter (fun (c, code) -> Printf.printf "%c: %s\n" c code);
  print_endline "ok"
