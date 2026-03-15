(* Example 091: Zip and Unzip *)
(* OCaml List.combine/split → Rust zip/unzip *)

(* Approach 1: Basic zip/unzip *)
let zip = List.combine
let unzip = List.split

(* Approach 2: zip_with / map2 *)
let zip_with f xs ys = List.map2 f xs ys

let dot_product xs ys =
  zip_with ( * ) xs ys |> List.fold_left ( + ) 0

let pairwise_max xs ys =
  zip_with max xs ys

(* Approach 3: Zip with index *)
let zip_with_index lst =
  List.mapi (fun i x -> (i, x)) lst

let enumerate = zip_with_index

(* Zip longest — pad shorter list *)
let rec zip_longest ~default_a ~default_b xs ys =
  match xs, ys with
  | [], [] -> []
  | x :: xs', y :: ys' -> (x, y) :: zip_longest ~default_a ~default_b xs' ys'
  | x :: xs', [] -> (x, default_b) :: zip_longest ~default_a ~default_b xs' []
  | [], y :: ys' -> (default_a, y) :: zip_longest ~default_a ~default_b [] ys'

(* Transpose matrix using zip *)
let transpose = function
  | [] | [] :: _ -> []
  | rows ->
    let n = List.length (List.hd rows) in
    List.init n (fun i -> List.map (fun row -> List.nth row i) rows)

(* Tests *)
let () =
  assert (zip [1;2;3] ["a";"b";"c"] = [(1,"a"); (2,"b"); (3,"c")]);
  assert (unzip [(1,"a"); (2,"b"); (3,"c")] = ([1;2;3], ["a";"b";"c"]));

  assert (dot_product [1;2;3] [4;5;6] = 32);
  assert (pairwise_max [1;3;2] [2;1;4] = [2;3;4]);

  assert (enumerate ["a";"b";"c"] = [(0,"a"); (1,"b"); (2,"c")]);

  assert (zip_longest ~default_a:0 ~default_b:0 [1;2;3] [4;5]
          = [(1,4); (2,5); (3,0)]);

  assert (transpose [[1;2;3]; [4;5;6]] = [[1;4]; [2;5]; [3;6]]);

  Printf.printf "✓ All tests passed\n"
