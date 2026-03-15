(* Example 097: Flatten Iterator *)
(* flat_map vs flatten *)

(* Approach 1: List.flatten *)
let flatten_lists = List.flatten

let nested = [[1;2;3]; [4;5]; [6;7;8;9]]
(* flatten_lists nested = [1;2;3;4;5;6;7;8;9] *)

(* Approach 2: flat_map = map + flatten *)
let flat_map f lst = List.flatten (List.map f lst)

let words_in_sentences sentences =
  flat_map (String.split_on_char ' ') sentences

let expand_ranges ranges =
  flat_map (fun (lo, hi) -> List.init (hi - lo + 1) (fun i -> lo + i)) ranges

(* Approach 3: Flatten options — filter_map *)
let flatten_options lst =
  List.filter_map Fun.id lst

let parse_ints strs =
  List.filter_map (fun s ->
    try Some (int_of_string s) with _ -> None
  ) strs

(* Nested flatten *)
let deep_flatten lll =
  List.flatten (List.flatten lll)

let flatten_tree t =
  let rec aux = function
    | `Leaf x -> [x]
    | `Node children -> flat_map aux children
  in
  aux t

(* Tests *)
let () =
  assert (flatten_lists [[1;2]; [3;4]; [5]] = [1;2;3;4;5]);

  assert (words_in_sentences ["hello world"; "foo bar baz"]
          = ["hello"; "world"; "foo"; "bar"; "baz"]);

  assert (expand_ranges [(1,3); (7,9)] = [1;2;3;7;8;9]);

  assert (flatten_options [Some 1; None; Some 2; None; Some 3] = [1;2;3]);

  assert (parse_ints ["1"; "abc"; "3"; ""; "5"] = [1;3;5]);

  assert (deep_flatten [[[1;2]; [3]]; [[4]; [5;6]]] = [1;2;3;4;5;6]);

  let tree = `Node [`Leaf 1; `Node [`Leaf 2; `Leaf 3]; `Leaf 4] in
  assert (flatten_tree tree = [1;2;3;4]);

  Printf.printf "✓ All tests passed\n"
