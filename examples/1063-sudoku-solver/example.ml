(* 1063: Sudoku Solver — Backtracking with Constraint Arrays *)

(* Approach 1: Simple backtracking with inline validation *)
let solve_sudoku board =
  let b = Array.map Array.copy board in
  let is_valid r c num =
    let ok = ref true in
    for i = 0 to 8 do
      if b.(r).(i) = num || b.(i).(c) = num then ok := false
    done;
    let br = (r / 3) * 3 and bc = (c / 3) * 3 in
    for dr = 0 to 2 do for dc = 0 to 2 do
      if b.(br + dr).(bc + dc) = num then ok := false
    done done;
    !ok
  in
  let solved = ref false in
  let rec solve () =
    if !solved then ()
    else begin
      (* Find first empty cell *)
      let found = ref false in
      let r = ref 0 and c_ = ref 0 in
      while !r < 9 && not !found do
        let c = ref 0 in
        while !c < 9 && not !found do
          if b.(!r).(!c) = 0 then begin found := true; c_ := !c end
          else incr c
        done;
        if not !found then incr r
      done;
      if not !found then solved := true  (* all cells filled *)
      else
        for num = 1 to 9 do
          if not !solved && is_valid !r !c_ num then begin
            b.(!r).(!c_) <- num;
            solve ();
            if not !solved then b.(!r).(!c_) <- 0
          end
        done
    end
  in
  solve ();
  if !solved then Some b else None

(* Approach 2: With O(1) constraint arrays for each row/col/box *)
let solve_sudoku_fast board =
  let b = Array.map Array.copy board in
  let rows  = Array.init 9 (fun _ -> Array.make 10 false) in
  let cols  = Array.init 9 (fun _ -> Array.make 10 false) in
  let boxes = Array.init 9 (fun _ -> Array.make 10 false) in
  (* Initialize constraint arrays from given clues *)
  for r = 0 to 8 do for c = 0 to 8 do
    let v = b.(r).(c) in
    if v <> 0 then begin
      rows.(r).(v) <- true;
      cols.(c).(v) <- true;
      boxes.((r/3)*3 + c/3).(v) <- true
    end
  done done;
  let solved = ref false in
  let rec solve () =
    if !solved then ()
    else begin
      let found = ref false and fr = ref 0 and fc = ref 0 in
      (try
        for r = 0 to 8 do for c = 0 to 8 do
          if b.(r).(c) = 0 then begin fr := r; fc := c; found := true; raise Exit end
        done done
      with Exit -> ());
      if not !found then solved := true
      else
        let bx = (!fr/3)*3 + !fc/3 in
        for num = 1 to 9 do
          if not !solved
             && not rows.(!fr).(num)
             && not cols.(!fc).(num)
             && not boxes.(bx).(num) then begin
            b.(!fr).(!fc) <- num;
            rows.(!fr).(num) <- true;
            cols.(!fc).(num) <- true;
            boxes.(bx).(num) <- true;
            solve ();
            if not !solved then begin
              b.(!fr).(!fc) <- 0;
              rows.(!fr).(num) <- false;
              cols.(!fc).(num) <- false;
              boxes.(bx).(num) <- false
            end
          end
        done
    end
  in
  solve ();
  if !solved then Some b else None

let test_board () =
  [| [|5;3;0;0;7;0;0;0;0|];
     [|6;0;0;1;9;5;0;0;0|];
     [|0;9;8;0;0;0;0;6;0|];
     [|8;0;0;0;6;0;0;0;3|];
     [|4;0;0;8;0;3;0;0;1|];
     [|7;0;0;0;2;0;0;0;6|];
     [|0;6;0;0;0;0;2;8;0|];
     [|0;0;0;4;1;9;0;0;5|];
     [|0;0;0;0;8;0;0;7;9|] |]

let () =
  (match solve_sudoku (test_board ()) with
   | None -> assert false
   | Some b ->
     assert (b.(0).(2) = 4);
     assert (b.(4).(4) = 5));

  (match solve_sudoku_fast (test_board ()) with
   | None -> assert false
   | Some b ->
     assert (b.(0).(2) = 4);
     assert (b.(4).(4) = 5));

  Printf.printf "All sudoku tests passed.\n"
