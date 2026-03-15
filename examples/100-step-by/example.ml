(* 100: Step By
   OCaml has no built-in range step, but we can build one easily.
   Using List.init or a recursive helper to generate stepped sequences. *)

(* Generate [start, start+step, start+2*step, ...] while < stop *)
let range_step start stop step =
  let rec aux i acc =
    if i >= stop then List.rev acc
    else aux (i + step) (i :: acc)
  in
  aux start []

(* Alternatively using Seq for a lazy version *)
let seq_step start stop step =
  Seq.unfold (fun i ->
    if i >= stop then None
    else Some (i, i + step)
  ) start

let () =
  assert (range_step 0 10 2 = [0; 2; 4; 6; 8]);
  assert (range_step 0 100 25 = [0; 25; 50; 75]);
  assert (range_step 0 20 5 = [0; 5; 10; 15]);
  assert (range_step 0 3 1 = [0; 1; 2]);

  (* Lazy Seq version *)
  let lazy_result = seq_step 0 10 2 |> List.of_seq in
  assert (lazy_result = [0; 2; 4; 6; 8]);

  let v = range_step 0 10 2 in
  Printf.printf "step_by 2 from 0 to 10: [%s]\n"
    (String.concat "; " (List.map string_of_int v))
