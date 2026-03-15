module StringMap = Map.Make(String)

let tokenize s =
  let s = String.lowercase_ascii s in
  let words = ref [] and buf = Buffer.create 16 in
  let flush () =
    if Buffer.length buf > 0 then begin
      words := Buffer.contents buf :: !words;
      Buffer.clear buf
    end
  in
  String.iter (fun c ->
    if (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') then
      Buffer.add_char buf c
    else flush ()
  ) s;
  flush ();
  List.rev !words

let word_count sentence =
  tokenize sentence
  |> List.fold_left (fun m w ->
       let n = Option.value ~default:0 (StringMap.find_opt w m) in
       StringMap.add w (n + 1) m)
     StringMap.empty

let () =
  let m = word_count "the cat sat on the mat, the cat sat" in
  assert (StringMap.find "the" m = 3);
  assert (StringMap.find "cat" m = 2);
  assert (StringMap.find "sat" m = 2);
  print_endline "All assertions passed."
