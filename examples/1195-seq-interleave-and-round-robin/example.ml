(* Seq — Interleave and Round-Robin *)
(* Merge multiple sequences by alternating *)

let rec interleave s1 s2 () = match s1 () with
  | Seq.Nil -> s2 ()
  | Seq.Cons (x, rest) -> Seq.Cons (x, interleave s2 rest)

let s1 = List.to_seq [1; 3; 5; 7]
let s2 = List.to_seq [2; 4; 6; 8]
let merged = interleave s1 s2 |> List.of_seq
let () = List.iter (fun x -> Printf.printf "%d " x) merged
(* Output: 1 2 3 4 5 6 7 8 *)
