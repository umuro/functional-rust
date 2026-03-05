(* Graph m-Colouring — backtracking O(m^V) *)

let graph_color adj n m =
  let color = Array.make n (-1) in

  let is_safe v c =
    List.for_all (fun u -> color.(u) <> c) adj.(v)
  in

  let rec solve v =
    if v = n then true
    else
      let rec try_color c =
        if c >= m then false
        else if is_safe v c then begin
          color.(v) <- c;
          if solve (v + 1) then true
          else begin color.(v) <- -1; try_color (c + 1) end
        end
        else try_color (c + 1)
      in
      try_color 0
  in
  if solve 0 then Some (Array.copy color) else None

let chromatic_number adj n =
  let rec try_m m =
    if m > n then n
    else match graph_color adj n m with
      | Some _ -> m
      | None   -> try_m (m + 1)
  in
  try_m 1

let () =
  (* Petersen graph — chromatic number = 3 *)
  let n   = 10 in
  let adj = Array.make n [] in
  let add u v = adj.(u) <- v :: adj.(u); adj.(v) <- u :: adj.(v) in
  (* Outer 5-cycle *)
  add 0 1; add 1 2; add 2 3; add 3 4; add 4 0;
  (* Inner pentagram *)
  add 5 7; add 7 9; add 9 6; add 6 8; add 8 5;
  (* Spokes *)
  add 0 5; add 1 6; add 2 7; add 3 8; add 4 9;

  let chi = chromatic_number adj n in
  Printf.printf "Petersen graph chromatic number: %d  (expected 3)\n" chi;
  (match graph_color adj n 3 with
   | None   -> Printf.printf "No 3-colouring found\n"
   | Some c -> Printf.printf "3-colouring: [%s]\n"
       (String.concat "," (Array.to_list (Array.map string_of_int c))))
