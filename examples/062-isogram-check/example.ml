(* Isogram Check — Detecting duplicate characters using a set *)

let is_isogram s =
  let chars = s |> String.lowercase_ascii |> String.to_seq
    |> Seq.filter (fun c -> c >= 'a' && c <= 'z')
    |> List.of_seq in
  let unique = List.sort_uniq Char.compare chars in
  List.length chars = List.length unique

(* Recursive approach with accumulator set *)
let is_isogram_recursive s =
  let lower = String.lowercase_ascii s in
  let module CS = Set.Make(Char) in
  let rec check i seen =
    if i >= String.length lower then true
    else
      let c = lower.[i] in
      if c >= 'a' && c <= 'z' then
        if CS.mem c seen then false
        else check (i + 1) (CS.add c seen)
      else check (i + 1) seen
  in
  check 0 CS.empty

let () =
  assert (is_isogram "lumberjacks");
  assert (not (is_isogram "eleven"));
  assert (is_isogram "");
  assert (is_isogram_recursive "subdermatoglyphic");
  assert (not (is_isogram_recursive "eleven"));
  Printf.printf "All isogram tests passed!\n"
