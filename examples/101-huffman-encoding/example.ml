(* 101: Huffman Encoding — Greedy Tree Building *)

(* Idiomatic OCaml: algebraic type for the Huffman tree *)
type htree = Leaf of char * int | Node of htree * htree * int

let freq = function Leaf (_, f) -> f | Node (_, _, f) -> f

(* Build tree by repeatedly merging two cheapest nodes *)
let build_tree freqs =
  let trees =
    List.map (fun (c, f) -> Leaf (c, f)) freqs
    |> List.sort (fun a b -> compare (freq a) (freq b))
  in
  let rec go = function
    | [ t ] -> t
    | a :: b :: rest ->
      let merged = Node (a, b, freq a + freq b) in
      let trees =
        List.sort (fun a b -> compare (freq a) (freq b)) (merged :: rest)
      in
      go trees
    | [] -> failwith "empty frequency list"
  in
  go trees

(* Traverse tree to generate prefix codes *)
let rec codes prefix = function
  | Leaf (c, _) -> [ (c, prefix) ]
  | Node (l, r, _) -> codes (prefix ^ "0") l @ codes (prefix ^ "1") r

(* Recursive alternative using explicit pattern match on list head *)
let rec build_tree_rec = function
  | [] -> failwith "empty"
  | [ t ] -> t
  | sorted ->
    let a = List.nth sorted 0 in
    let b = List.nth sorted 1 in
    let rest = List.filteri (fun i _ -> i >= 2) sorted in
    let merged = Node (a, b, freq a + freq b) in
    let next = List.sort (fun x y -> compare (freq x) (freq y)) (merged :: rest) in
    build_tree_rec next

let () =
  let freqs = [ ('a', 5); ('b', 9); ('c', 12); ('d', 13); ('e', 16); ('f', 45) ] in
  let tree = build_tree freqs in
  let cs = codes "" tree |> List.sort (fun (a, _) (b, _) -> Char.compare a b) in
  List.iter (fun (c, code) -> Printf.printf "%c: %s\n" c code) cs;

  (* Verify root frequency equals sum of all input frequencies *)
  assert (freq tree = List.fold_left (fun acc (_, f) -> acc + f) 0 freqs);

  (* Verify f (highest freq) has shorter code than a (lowest freq) *)
  let f_code = List.assoc 'f' cs in
  let a_code = List.assoc 'a' cs in
  assert (String.length f_code < String.length a_code);

  (* Both builders produce same root frequency *)
  let tree2 =
    List.map (fun (c, f) -> Leaf (c, f)) freqs
    |> List.sort (fun a b -> compare (freq a) (freq b))
    |> build_tree_rec
  in
  assert (freq tree = freq tree2);

  print_endline "ok"
