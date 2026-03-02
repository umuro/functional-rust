(* Pack Consecutive *)
(* OCaml 99 Problems #9 *)

(* Pack consecutive duplicates *)
let pack lst =
  let rec aux current acc = function
    | [] -> []
    | [x] -> (x :: current) :: acc
    | h1 :: (h2 :: _ as t) ->
        if h1 = h2 then aux (h1 :: current) acc t
        else aux [] ((h1 :: current) :: acc) t
  in
  List.rev (aux [] [] lst)

(* Tests *)
let () =
  assert (pack ["a";"a";"b";"c";"c";"c"] = [["a";"a"];["b"];["c";"c";"c"]]);
  assert (pack [] = []);
  assert (pack [1] = [[1]]);
  print_endline "✓ OCaml tests passed"
