let rec quicksort = function
  | [] -> []
  | pivot :: rest ->
    let left, right = List.partition (fun x -> x < pivot) rest in
    quicksort left @ [pivot] @ quicksort right

let () =
  let sorted = quicksort [3; 6; 8; 10; 1; 2; 1] in
  List.iter (Printf.printf "%d ") sorted;
  print_newline ();
  assert (quicksort [] = []);
  assert (quicksort [1] = [1]);
  assert (quicksort [3; 6; 8; 10; 1; 2; 1] = [1; 1; 2; 3; 6; 8; 10]);
  assert (quicksort [5; 4; 3; 2; 1] = [1; 2; 3; 4; 5]);
  print_endline "ok"
