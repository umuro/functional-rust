(* 279: nth — access the element at index n (0-based), returning None if out of bounds.
   OCaml: List.nth_opt (4.05+); also Seq-based for lazy access. *)

let () =
  let v = [10; 20; 30; 40] in

  (* nth: return None when out of bounds *)
  Printf.printf "nth 2 [10;20;30;40] = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int (List.nth_opt v 2));

  (* Out of bounds *)
  Printf.printf "nth 5 [1;2]         = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int (List.nth_opt [1;2] 5));

  (* nth 0 *)
  Printf.printf "nth 0 [99]          = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int (List.nth_opt [99] 0));

  (* Consuming property: on a mutable sequence, nth(n) advances position.
     For Seq, we can implement this using Seq.drop + Seq.uncons. *)
  let seq_nth n seq =
    seq |> Seq.drop n |> Seq.uncons |> Option.map fst
  in
  let seq = List.to_seq [1;2;3;4;5] in
  Printf.printf "seq nth 1 = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int (seq_nth 1 seq));

  (* Stateful: consume and advance — using ref to track remaining Seq *)
  let iter = ref (List.to_seq [1;2;3;4;5]) in
  let advance_nth n =
    (* drop n elements from the ref'd sequence *)
    iter := Seq.drop n !iter;
    match (!iter) () with
    | Seq.Nil        -> None
    | Seq.Cons (x, rest) -> iter := rest; Some x
  in
  Printf.printf "advance nth(1) = %s (consumes 0,1)\n"
    (Option.fold ~none:"None" ~some:string_of_int (advance_nth 1));
  Printf.printf "advance nth(0) = %s (now at 2)\n"
    (Option.fold ~none:"None" ~some:string_of_int (advance_nth 0))
