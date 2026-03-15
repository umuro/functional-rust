(* Example 115: Vec Operations Functionally — OCaml List Functions → Rust *)

(* Approach 1: map, filter, fold *)
let approach1 () =
  let data = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10] in
  let evens = List.filter (fun x -> x mod 2 = 0) data in
  let doubled = List.map (fun x -> x * 2) evens in
  let sum = List.fold_left ( + ) 0 doubled in
  assert (sum = 60);
  Printf.printf "Sum of doubled evens: %d\n" sum

(* Approach 2: zip, unzip, partition *)
let zip a b = List.map2 (fun x y -> (x, y)) a b
let unzip lst = (List.map fst lst, List.map snd lst)

let approach2 () =
  let names = ["Alice"; "Bob"; "Charlie"] in
  let ages = [30; 25; 35] in
  let pairs = zip names ages in
  let (young, old) = List.partition (fun (_, age) -> age < 30) pairs in
  assert (List.length young = 1);
  assert (List.length old = 2);
  Printf.printf "Young: %d, Old: %d\n" (List.length young) (List.length old)

(* Approach 3: flat_map, scan, windows *)
let flat_map f lst = List.concat (List.map f lst)
let scan f init lst =
  let rec go acc state = function
    | [] -> List.rev acc
    | x :: rest ->
      let next = f state x in
      go (next :: acc) next rest
  in go [init] init lst

let approach3 () =
  let nested = [[1;2]; [3]; [4;5;6]] in
  let flat = flat_map (fun x -> x) nested in
  assert (flat = [1;2;3;4;5;6]);
  let running = scan ( + ) 0 [1;2;3;4;5] in
  assert (running = [0;1;3;6;10;15]);
  Printf.printf "Flat: %s\n" (String.concat "," (List.map string_of_int flat))

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
