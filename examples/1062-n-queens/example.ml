(* 1062: N-Queens — Backtracking *)

(* Approach 1: Backtracking with column/diagonal tracking *)
let solve_n_queens n =
  let solutions = ref [] in
  let cols = Array.make n false in
  let diag1 = Array.make (2 * n - 1) false in  (* row - col + n - 1 *)
  let diag2 = Array.make (2 * n - 1) false in  (* row + col *)
  let board = Array.make n 0 in  (* board.(row) = col of queen *)
  let rec place row =
    if row = n then
      solutions := Array.to_list board :: !solutions
    else
      for col = 0 to n - 1 do
        let d1 = row - col + n - 1 in
        let d2 = row + col in
        if not cols.(col) && not diag1.(d1) && not diag2.(d2) then begin
          board.(row) <- col;
          cols.(col) <- true;
          diag1.(d1) <- true;
          diag2.(d2) <- true;
          place (row + 1);
          cols.(col) <- false;
          diag1.(d1) <- false;
          diag2.(d2) <- false
        end
      done
  in
  place 0;
  List.rev !solutions

(* Approach 2: Functional with list accumulation *)
let solve_n_queens_func n =
  let is_safe queens col =
    let row = List.length queens in
    List.mapi (fun i c ->
      c <> col && abs (row - i) <> abs (col - c)
    ) queens
    |> List.for_all Fun.id
  in
  let rec solve queens row =
    if row = n then [List.rev queens]
    else
      List.init n Fun.id
      |> List.filter (is_safe queens)
      |> List.concat_map (fun col -> solve (col :: queens) (row + 1))
  in
  solve [] 0

let () =
  let solutions = solve_n_queens 4 in
  assert (List.length solutions = 2);

  let solutions8 = solve_n_queens 8 in
  assert (List.length solutions8 = 92);

  let func_solutions = solve_n_queens_func 4 in
  assert (List.length func_solutions = 2);

  let func8 = solve_n_queens_func 8 in
  assert (List.length func8 = 92);

  Printf.printf "✓ All tests passed\n"
