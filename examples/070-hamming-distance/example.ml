exception Invalid_argument of string

let chars_of_string s =
  List.init (String.length s) (String.get s)

let hamming_distance s1 s2 =
  if String.length s1 <> String.length s2 then
    raise (Invalid_argument "strands must be of equal length");
  List.combine (chars_of_string s1) (chars_of_string s2)
  |> List.filter (fun (a, b) -> a <> b)
  |> List.length

let hamming_fold s1 s2 =
  if String.length s1 <> String.length s2 then
    raise (Invalid_argument "strands must be of equal length");
  List.fold_left2
    (fun acc c1 c2 -> if c1 <> c2 then acc + 1 else acc)
    0
    (chars_of_string s1)
    (chars_of_string s2)

let () =
  assert (hamming_distance "GAGCCTACTAACGGGAT" "CATCGTAATGACGGCCT" = 7);
  assert (hamming_distance "AAAA" "AAAA" = 0);
  assert (hamming_fold "AAAA" "TTTT" = 4);
  print_endline "All assertions passed."
