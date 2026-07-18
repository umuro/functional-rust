(* Sort By Length *)
(* OCaml 99 Problems #28 *)

let sort_by_length lists =
  List.stable_sort (fun a b -> compare (List.length a) (List.length b)) lists

let sort_by_length_freq lists =
  let freq = Hashtbl.create 16 in
  List.iter
    (fun l ->
       let len = List.length l in
       let count = try Hashtbl.find freq len with Not_found -> 0 in
       Hashtbl.replace freq len (count + 1))
    lists;
  List.stable_sort
    (fun a b -> compare (Hashtbl.find freq (List.length a)) (Hashtbl.find freq (List.length b)))
    lists

(* Tests *)
let () =
  assert (sort_by_length [[1; 2; 3]; [1]; [1; 2]] = [[1]; [1; 2]; [1; 2; 3]]);
  assert (sort_by_length [[1; 1]; [2; 2]; [3]] = [[3]; [1; 1]; [2; 2]]);

  let lists =
    [ [1; 2; 3]; [1; 2]; [1; 2; 3; 4]; [1; 2]; [1; 2; 3; 4; 5]; [1; 2]; [1] ]
  in
  assert (
    sort_by_length_freq lists
    = [ [1; 2; 3]; [1; 2; 3; 4]; [1; 2; 3; 4; 5]; [1]; [1; 2]; [1; 2]; [1; 2] ]);

  print_endline "✓ OCaml tests passed"
