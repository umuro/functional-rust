(* Example 088: Iterator Consumers *)
(* fold, collect, sum, max, find, position *)

(* Approach 1: fold — the universal consumer *)
let sum lst = List.fold_left (+) 0 lst
let product lst = List.fold_left ( * ) 1 lst
let concat_strs lst = List.fold_left (fun acc s -> acc ^ s) "" lst

(* Approach 2: Specific consumers *)
let find_first pred lst =
  try Some (List.find pred lst) with Not_found -> None

let find_position pred lst =
  let rec aux i = function
    | [] -> None
    | x :: _ when pred x -> Some i
    | _ :: rest -> aux (i + 1) rest
  in
  aux 0 lst

let max_of = function
  | [] -> None
  | x :: rest -> Some (List.fold_left max x rest)

let min_of = function
  | [] -> None
  | x :: rest -> Some (List.fold_left min x rest)

let count pred lst =
  List.fold_left (fun acc x -> if pred x then acc + 1 else acc) 0 lst

let any pred lst = List.exists pred lst
let all pred lst = List.for_all pred lst

(* Approach 3: Complex consumers *)
let group_by key lst =
  let tbl = Hashtbl.create 16 in
  List.iter (fun x ->
    let k = key x in
    let existing = try Hashtbl.find tbl k with Not_found -> [] in
    Hashtbl.replace tbl k (x :: existing)
  ) lst;
  Hashtbl.fold (fun k v acc -> (k, List.rev v) :: acc) tbl []

let frequencies lst =
  let tbl = Hashtbl.create 16 in
  List.iter (fun x ->
    let c = try Hashtbl.find tbl x with Not_found -> 0 in
    Hashtbl.replace tbl x (c + 1)
  ) lst;
  Hashtbl.fold (fun k v acc -> (k, v) :: acc) tbl []

(* Tests *)
let () =
  assert (sum [1; 2; 3; 4; 5] = 15);
  assert (product [1; 2; 3; 4; 5] = 120);
  assert (concat_strs ["a"; "b"; "c"] = "abc");

  assert (find_first (fun x -> x > 3) [1;2;3;4;5] = Some 4);
  assert (find_first (fun x -> x > 10) [1;2;3] = None);

  assert (find_position (fun x -> x > 3) [1;2;3;4;5] = Some 3);
  assert (find_position (fun x -> x > 10) [1;2;3] = None);

  assert (max_of [3; 1; 4; 1; 5; 9] = Some 9);
  assert (min_of [3; 1; 4; 1; 5; 9] = Some 1);
  assert (max_of [] = None);

  assert (count (fun x -> x mod 2 = 0) [1;2;3;4;5;6] = 3);
  assert (any (fun x -> x > 5) [1;2;3;4;5;6] = true);
  assert (all (fun x -> x > 0) [1;2;3] = true);
  assert (all (fun x -> x > 0) [1;-2;3] = false);

  let freqs = frequencies [1;2;1;3;2;1] in
  let freq_1 = List.assoc 1 freqs in
  assert (freq_1 = 3);

  Printf.printf "✓ All tests passed\n"
