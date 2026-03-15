(* 356: HashMap Patterns
   OCaml's Hashtbl module provides mutable hash maps.
   For functional (persistent) maps use the Map functor.
   Common patterns: word count, group_by, frequency analysis. *)

(* Word count using Hashtbl *)
let word_count text =
  let tbl = Hashtbl.create 16 in
  String.split_on_char ' ' text
  |> List.filter (fun s -> s <> "")
  |> List.iter (fun word ->
       let n = try Hashtbl.find tbl word with Not_found -> 0 in
       Hashtbl.replace tbl word (n + 1));
  tbl

(* Group items by a key function *)
let group_by items key_of =
  let tbl = Hashtbl.create 16 in
  List.iter (fun item ->
    let k = key_of item in
    let xs = try Hashtbl.find tbl k with Not_found -> [] in
    Hashtbl.replace tbl k (item :: xs)
  ) items;
  (* Reverse each group to restore insertion order *)
  Hashtbl.iter (fun k xs -> Hashtbl.replace tbl k (List.rev xs)) tbl;
  tbl

(* Top-N entries by frequency *)
let frequency_top_n tbl n =
  Hashtbl.fold (fun k v acc -> (k, v) :: acc) tbl []
  |> List.sort (fun (_, a) (_, b) -> compare b a)
  |> List.filteri (fun i _ -> i < n)

let () =
  (* Word count *)
  let wc = word_count "the cat sat on the mat" in
  assert (Hashtbl.find wc "the" = 2);
  assert (Hashtbl.find wc "cat" = 1);
  Printf.printf "word_count \"the\"=%d \"cat\"=%d\n%!"
    (Hashtbl.find wc "the") (Hashtbl.find wc "cat");

  (* Group by parity *)
  let grouped = group_by [1;2;3;4;5] (fun x -> if x mod 2 = 0 then "even" else "odd") in
  assert (List.length (Hashtbl.find grouped "even") = 2);
  assert (List.length (Hashtbl.find grouped "odd")  = 3);
  Printf.printf "even=%d odd=%d\n%!"
    (List.length (Hashtbl.find grouped "even"))
    (List.length (Hashtbl.find grouped "odd"));

  (* Top-2 by frequency *)
  let wc2 = word_count "a a a b b c" in
  let top = frequency_top_n wc2 2 in
  assert (fst (List.nth top 0) = "a");
  assert (fst (List.nth top 1) = "b");
  Printf.printf "top-2: %s, %s\n%!"
    (fst (List.nth top 0)) (fst (List.nth top 1))
