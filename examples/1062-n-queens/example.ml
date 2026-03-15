(* 1062: N-Queens — Backtracking
   Place N queens on an N×N board with no conflicts.
   Three approaches: imperative backtrack, functional, and bit-mask count. *)

(* Approach 1: Backtracking with boolean arrays *)
let solve_n_queens n =
  let solutions = ref [] in
  let cols  = Array.make n false in
  let diag1 = Array.make (2*n - 1) false in  (* row - col + n - 1 *)
  let diag2 = Array.make (2*n - 1) false in  (* row + col *)
  let board = Array.make n 0 in
  let rec place row =
    if row = n then solutions := Array.to_list board :: !solutions
    else
      for col = 0 to n - 1 do
        let d1 = row + n - 1 - col and d2 = row + col in
        if not cols.(col) && not diag1.(d1) && not diag2.(d2) then begin
          board.(row) <- col;
          cols.(col) <- true; diag1.(d1) <- true; diag2.(d2) <- true;
          place (row + 1);
          cols.(col) <- false; diag1.(d1) <- false; diag2.(d2) <- false
        end
      done
  in
  place 0;
  !solutions

(* Approach 2: Purely functional — queens list stores column of each row *)
let solve_n_queens_func n =
  let is_safe queens col =
    let row = List.length queens in
    List.for_all (fun (i, c) ->
      c <> col && abs (row - i) <> abs (col - c)
    ) (List.mapi (fun i c -> (i, c)) queens)
  in
  let rec solve queens row acc =
    if row = n then queens :: acc
    else
      List.fold_left (fun acc col ->
        if is_safe queens col
        then solve (queens @ [col]) (row + 1) acc
        else acc
      ) acc (List.init n (fun i -> i))
  in
  solve [] 0 []

(* Approach 3: Bitmask — count only *)
let solve_n_queens_bits n =
  let full = (1 lsl n) - 1 in
  let rec count row cols diag1 diag2 =
    if row = n then 1
    else
      let available = full land (lnot (cols lor diag1 lor diag2)) in
      let total = ref 0 in
      let bits = ref available in
      while !bits <> 0 do
        let bit = !bits land (- !bits) in  (* lowest set bit *)
        total := !total + count (row+1) (cols lor bit) ((diag1 lor bit) lsl 1) ((diag2 lor bit) lsr 1);
        bits := !bits land (!bits - 1)
      done;
      !total
  in
  count 0 0 0 0

let () =
  assert (List.length (solve_n_queens      4) = 2);
  assert (List.length (solve_n_queens      8) = 92);
  assert (List.length (solve_n_queens_func 4) = 2);
  assert (List.length (solve_n_queens_func 8) = 92);
  assert (solve_n_queens_bits 4 = 2);
  assert (solve_n_queens_bits 8 = 92);
  Printf.printf "All N-queens tests passed.\n"
