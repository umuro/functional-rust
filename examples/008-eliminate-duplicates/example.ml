(* Eliminate Duplicates *)
(* OCaml 99 Problems #8 *)

(* Eliminate consecutive duplicates *)
let rec compress = function
  | [] -> []
  | [x] -> [x]
  | h1 :: (h2 :: _ as t) ->
      if h1 = h2 then compress t
      else h1 :: compress t

(* Tests *)
let () =
  assert (compress ["a";"a";"a";"b";"c";"c";"d";"e";"e";"e"] = ["a";"b";"c";"d";"e"]);
  assert (compress [] = []);
  assert (compress [1] = [1]);
  print_endline "✓ OCaml tests passed"
