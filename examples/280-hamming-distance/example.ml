let hamming s1 s2 =
  if String.length s1 <> String.length s2 then
    Error "strands must be of equal length"
  else
    let dist = ref 0 in
    String.iteri (fun i c ->
      if c <> s2.[i] then incr dist
    ) s1;
    Ok !dist

(* Pure functional version *)
let hamming_fp s1 s2 =
  if String.length s1 <> String.length s2 then Error "unequal"
  else
    Ok (Seq.zip (String.to_seq s1) (String.to_seq s2)
    |> Seq.fold_left (fun acc (a, b) -> if a <> b then acc + 1 else acc) 0)

let () =
  (match hamming "GAGCCTACTAACGGGAT" "CATCGTAATGACGGCCT" with
  | Ok d -> Printf.printf "Hamming distance: %d\n" d
  | Error e -> Printf.printf "Error: %s\n" e);
  assert (hamming "" "" = Ok 0);
  assert (hamming "GGACTGA" "GGACTGA" = Ok 0);
  assert (hamming "GGACTGA" "GGACTGT" = Ok 1);
  assert (hamming "GAGCCTACTAACGGGAT" "CATCGTAATGACGGCCT" = Ok 7);
  assert (Result.is_error (hamming "AB" "ABC"));
  print_endline "ok"
