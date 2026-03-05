(* Greedy Algorithms: Activity Selection + Huffman Coding in OCaml *)

(* Activity Selection Problem *)
type activity = { id: int; start: int; finish: int }

(* Greedy: sort by finish time, pick non-overlapping activities *)
let activity_selection (activities : activity list) : activity list =
  let sorted = List.sort (fun a b -> compare a.finish b.finish) activities in
  let rec select last_finish = function
    | [] -> []
    | a :: rest ->
      if a.start >= last_finish then
        a :: select a.finish rest
      else
        select last_finish rest
  in
  select 0 sorted

(* Huffman Coding *)
type huffman =
  | Leaf of { symbol: char; freq: int }
  | Node of { freq: int; left: huffman; right: huffman }

let freq_of = function Leaf l -> l.freq | Node n -> n.freq

(* Simple priority queue using sorted list *)
let insert_sorted tree lst =
  let rec ins = function
    | [] -> [tree]
    | h :: t ->
      if freq_of tree <= freq_of h then tree :: h :: t
      else h :: ins t
  in
  ins lst

let huffman_tree (symbols : (char * int) list) : huffman option =
  match symbols with
  | [] -> None
  | _ ->
    let initial = List.sort (fun (_, a) (_, b) -> compare a b)
      (List.map (fun (c, f) -> Leaf { symbol=c; freq=f }) symbols)
    in
    let rec build = function
      | [tree] -> tree
      | a :: b :: rest ->
        let merged = Node { freq = freq_of a + freq_of b; left = a; right = b } in
        build (insert_sorted merged rest)
      | [] -> failwith "empty"
    in
    Some (build initial)

(* Extract code table: symbol -> binary string *)
let huffman_codes (tree : huffman) : (char * string) list =
  let codes = ref [] in
  let rec traverse prefix = function
    | Leaf l -> codes := (l.symbol, if prefix = "" then "0" else prefix) :: !codes
    | Node n ->
      traverse (prefix ^ "0") n.left;
      traverse (prefix ^ "1") n.right
  in
  traverse "" tree;
  List.sort compare !codes

let () =
  (* Activity selection *)
  let acts = [
    {id=1; start=1; finish=4};
    {id=2; start=3; finish=5};
    {id=3; start=0; finish=6};
    {id=4; start=5; finish=7};
    {id=5; start=3; finish=9};
    {id=6; start=5; finish=9};
    {id=7; start=6; finish=10};
    {id=8; start=8; finish=11};
    {id=9; start=8; finish=12};
    {id=10; start=2; finish=14};
  ] in
  let selected = activity_selection acts in
  Printf.printf "Activity selection: %d activities selected (ids: %s)\n"
    (List.length selected)
    (String.concat "," (List.map (fun a -> string_of_int a.id) selected));

  (* Huffman coding *)
  let freqs = [('a', 5); ('b', 2); ('c', 1); ('d', 3); ('e', 4); ('f', 7)] in
  match huffman_tree freqs with
  | None -> ()
  | Some tree ->
    let codes = huffman_codes tree in
    Printf.printf "\nHuffman codes:\n";
    List.iter (fun (c, code) ->
      Printf.printf "  '%c' (freq=%d): %s\n" c
        (List.assoc c freqs) code
    ) codes
