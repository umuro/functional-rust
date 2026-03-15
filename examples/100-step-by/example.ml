(* 100: Step By *)

let step_by start stop step =
  let rec aux acc n =
    if n >= stop then List.rev acc
    else aux (n :: acc) (n + step)
  in
  aux [] start

let range_step start stop step =
  Seq.unfold (fun n -> if n >= stop then None else Some (n, n + step)) start

(* Tests *)
let () =
  assert (step_by 0 10 2 = [0; 2; 4; 6; 8]);
  assert (step_by 0 100 25 = [0; 25; 50; 75]);
  assert (List.of_seq (range_step 0 20 5) = [0; 5; 10; 15]);
  Printf.printf "✓ All tests passed\n"
