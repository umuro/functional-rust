module CS = Set.Make(Char)

let alphabet = 
  List.init 26 (fun i -> Char.chr (i + Char.code 'a'))
  |> CS.of_list

let is_pangram s =
  let chars = s |> String.lowercase_ascii |> String.to_seq
    |> Seq.filter (fun c -> c >= 'a' && c <= 'z')
    |> CS.of_seq in
  CS.subset alphabet chars

let () =
  Printf.printf "%b\n" (is_pangram "The quick brown fox jumps over the lazy dog");
  Printf.printf "%b\n" (is_pangram "Hello world")
