type htree = Leaf of char * int | Node of htree * htree * int

let freq t = match t with Leaf (_,f) -> f | Node (_,_,f) -> f

(* Idiomatic OCaml: sort + merge with List operations *)
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

(* Recursive code extraction — mirrors codes_acc in Rust *)
let rec codes prefix = function
  | Leaf (c, _) -> [(c, prefix)]
  | Node (l, r, _) -> codes (prefix ^ "0") l @ codes (prefix ^ "1") r

let () =
  let freqs = [('a',5);('b',9);('c',12);('d',13);('e',16);('f',45)] in
  let tree = build_tree freqs in
  let result = codes "" tree |> List.sort (fun (a,_) (b,_) -> compare a b) in
  (* Basic assertions *)
  assert (List.length result = 6);
  let total_freq = List.fold_left (fun acc (_,f) -> acc + f) 0 freqs in
  assert (freq tree = total_freq);
  (* 'f' should have the shortest code *)
  let f_len = List.assoc 'f' result |> String.length in
  List.iter (fun (c, code) ->
    if c <> 'f' then assert (String.length code >= f_len)
  ) result;
  List.iter (fun (c, code) ->
    Printf.printf "%c: %s\n" c code) result;
  print_endline "ok"
