(* Backtracking Framework in OCaml — N-Queens and Permutations *)

(* N-Queens: board.(row) = column of queen in that row *)
(* Check if placing a queen at (row, col) is safe given placements so far *)
let is_safe (board : int array) (row : int) (col : int) : bool =
  let ok = ref true in
  for r = 0 to row - 1 do
    let c = board.(r) in
    if c = col || abs (c - col) = abs (r - row) then
      ok := false
  done;
  !ok

(* Solve N-Queens: returns all solutions *)
let n_queens (n : int) : int array list =
  let board = Array.make n 0 in
  let solutions = ref [] in
  let rec solve row =
    if row = n then
      solutions := Array.copy board :: !solutions
    else
      for col = 0 to n - 1 do
        if is_safe board row col then begin
          board.(row) <- col;
          solve (row + 1);
          (* backtrack: no explicit undo needed as we overwrite *)
        end
      done
  in
  solve 0;
  !solutions

(* Print a board *)
let print_board (board : int array) =
  Array.iter (fun col ->
    let row = String.init (Array.length board) (fun c -> if c = col then 'Q' else '.') in
    print_string (row ^ "\n")
  ) board;
  print_newline ()

(* Generic permutation via backtracking *)
let permutations (xs : 'a list) : 'a list list =
  let result = ref [] in
  let n = List.length xs in
  let arr = Array.of_list xs in
  let used = Array.make n false in
  let current = Array.make n (List.hd xs) in
  let rec gen depth =
    if depth = n then
      result := Array.to_list current :: !result
    else
      for i = 0 to n - 1 do
        if not used.(i) then begin
          used.(i) <- true;
          current.(depth) <- arr.(i);
          gen (depth + 1);
          used.(i) <- false  (* backtrack *)
        end
      done
  in
  gen 0;
  !result

let () =
  let sols = n_queens 4 in
  Printf.printf "4-Queens: %d solutions\n" (List.length sols);
  print_board (List.hd sols);
  Printf.printf "8-Queens: %d solutions\n" (List.length (n_queens 8));
  let perms = permutations [1; 2; 3] in
  Printf.printf "permutations([1,2,3]): %d\n" (List.length perms);
  List.iter (fun p ->
    Printf.printf "  [%s]\n" (String.concat "," (List.map string_of_int p))
  ) (List.sort compare perms)
