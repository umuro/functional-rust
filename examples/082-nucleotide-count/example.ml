(* Nucleotide Count — Bioinformatics *)

module CMap = Map.Make(Char)

(* Version 1: Using Map for counting *)
let nucleotide_count dna =
  let init = List.fold_left (fun m c -> CMap.add c 0 m)
    CMap.empty ['A';'C';'G';'T'] in
  String.fold_left (fun m c ->
    match CMap.find_opt c m with
    | Some n -> CMap.add c (n + 1) m
    | None -> failwith ("invalid nucleotide: " ^ String.make 1 c)
  ) init dna

(* Version 2: Using a simple assoc list *)
let nucleotide_count_simple dna =
  let counts = [('A', ref 0); ('C', ref 0); ('G', ref 0); ('T', ref 0)] in
  String.iter (fun c ->
    (List.assoc c counts) := !(List.assoc c counts) + 1
  ) dna;
  List.map (fun (c, r) -> (c, !r)) counts

let () =
  let counts = nucleotide_count "GATTACA" in
  assert (CMap.find 'A' counts = 3);
  assert (CMap.find 'T' counts = 2)
