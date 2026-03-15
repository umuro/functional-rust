(* 1063: Sudoku Solver — Backtracking + Constraints *)

(* Approach 1: Simple backtracking *)
let solve_sudoku board =
  let is_valid board row col num =
    (* Check row *)
    let valid = ref true in
    for c = 0 to 8 do
      if board.(row).(c) = num then valid := false
    done;
    (* Check column *)
    for r = 0 to 8 do
      if board.(r).(col) = num then valid := false
    done;
    (* Check 3x3 box *)
    let br = (row / 3) * 3 and bc = (col / 3) * 3 in
    for r = br to br + 2 do
      for c = bc to bc + 2 do
        if board.(r).(c) = num then valid := false
      done
    done;
    !valid
  in
  let rec solve () =
    (* Find first empty cell *)
    let found = ref None in
    for r = 0 to 8 do
      for c = 0 to 8 do
        if board.(r).(c) = 0 && !found = None then
          found := Some (r, c)
      done
    done;
    match !found with
    | None -> true  (* All filled — solved! *)
    | Some (row, col) ->
      let solved = ref false in
      for num = 1 to 9 do
        if not !solved && is_valid board row col num then begin
          board.(row).(col) <- num;
          if solve () then solved := true
          else board.(row).(col) <- 0
        end
      done;
      !solved
  in
  solve ()

(* Approach 2: With constraint sets for faster validation *)
let solve_sudoku_fast board =
  let rows = Array.init 9 (fun _ -> Array.make 10 false) in
  let cols = Array.init 9 (fun _ -> Array.make 10 false) in
  let boxes = Array.init 9 (fun _ -> Array.make 10 false) in
  (* Initialize constraints *)
  for r = 0 to 8 do
    for c = 0 to 8 do
      let v = board.(r).(c) in
      if v <> 0 then begin
        rows.(r).(v) <- true;
        cols.(c).(v) <- true;
        boxes.((r / 3) * 3 + c / 3).(v) <- true
      end
    done
  done;
  let rec solve () =
    let found = ref None in
    for r = 0 to 8 do
      for c = 0 to 8 do
        if board.(r).(c) = 0 && !found = None then found := Some (r, c)
      done
    done;
    match !found with
    | None -> true
    | Some (r, c) ->
      let b = (r / 3) * 3 + c / 3 in
      let solved = ref false in
      for num = 1 to 9 do
        if not !solved && not rows.(r).(num) && not cols.(c).(num) && not boxes.(b).(num) then begin
          board.(r).(c) <- num;
          rows.(r).(num) <- true;
          cols.(c).(num) <- true;
          boxes.(b).(num) <- true;
          if solve () then solved := true
          else begin
            board.(r).(c) <- 0;
            rows.(r).(num) <- false;
            cols.(c).(num) <- false;
            boxes.(b).(num) <- false
          end
        end
      done;
      !solved
  in
  solve ()

let () =
  let board = [|
    [|5;3;0;0;7;0;0;0;0|];
    [|6;0;0;1;9;5;0;0;0|];
    [|0;9;8;0;0;0;0;6;0|];
    [|8;0;0;0;6;0;0;0;3|];
    [|4;0;0;8;0;3;0;0;1|];
    [|7;0;0;0;2;0;0;0;6|];
    [|0;6;0;0;0;0;2;8;0|];
    [|0;0;0;4;1;9;0;0;5|];
    [|0;0;0;0;8;0;0;7;9|]
  |] in
  assert (solve_sudoku board);
  assert (board.(0).(2) = 4);
  assert (board.(4).(4) = 5);

  let board2 = [|
    [|5;3;0;0;7;0;0;0;0|];
    [|6;0;0;1;9;5;0;0;0|];
    [|0;9;8;0;0;0;0;6;0|];
    [|8;0;0;0;6;0;0;0;3|];
    [|4;0;0;8;0;3;0;0;1|];
    [|7;0;0;0;2;0;0;0;6|];
    [|0;6;0;0;0;0;2;8;0|];
    [|0;0;0;4;1;9;0;0;5|];
    [|0;0;0;0;8;0;0;7;9|]
  |] in
  assert (solve_sudoku_fast board2);
  assert (board2.(0).(2) = 4);

  Printf.printf "✓ All tests passed\n"
