let insert k v lst = (k, v) :: lst

let rec lookup k = function
  | []            -> None
  | (k', v) :: t  -> if k = k' then Some v else lookup k t

let rec remove k = function
  | []                        -> []
  | (k', _) :: t when k = k'  -> t
  | h :: t                    -> h :: remove k t

let keys lst = List.map fst lst

let () =
  let d = [] in
  let d = insert "a" 1 d in
  let d = insert "b" 2 d in
  let d = insert "a" 99 d in
  assert (lookup "a" d = Some 99);
  assert (lookup "c" d = None);
  let d' = remove "a" d in
  assert (lookup "a" d' = Some 1);
  assert (keys d = ["a"; "b"; "a"]);
  print_endline "All assertions passed."
