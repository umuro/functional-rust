(* Run-length Encoding *)
(* OCaml 99 Problems #10 *)

(* First, pack consecutive elements *)
let pack lst =
  let rec aux current acc = function
    | [] -> []
    | [x] -> (x :: current) :: acc
    | h1 :: (h2 :: _ as t) ->
        if h1 = h2 then aux (h1 :: current) acc t
        else aux [] ((h1 :: current) :: acc) t
  in
  List.rev (aux [] [] lst)

(* Then encode by counting *)
let encode lst =
  let rec count n = function
    | [] -> []
    | [x] -> [(n + 1, x)]
    | h1 :: (h2 :: _ as t) -> (n + 1, h1) :: count 0 t
  in
  count 0 (pack lst)

(* Tests *)
let () =
  assert (encode ["a";"a";"b";"c";"c";"c"] = [(2,"a");(1,"b");(3,"c")]);
  assert (encode [] = []);
  assert (encode [1] = [(1,1)]);
  print_endline "✓ OCaml tests passed"
