(* 266: step_by — yield every nth element.
   OCaml: filter by index with List.filteri, or use Seq for lazy striding. *)

(* Step through a list, keeping every nth element starting at index 0 *)
let step_by n lst =
  if n <= 0 then invalid_arg "step_by: n must be positive";
  List.filteri (fun i _ -> i mod n = 0) lst

(* Lazy step using Seq — mirrors Rust's step_by exactly *)
let seq_step_by n seq =
  if n <= 0 then invalid_arg "step_by: n must be positive";
  seq
  |> Seq.mapi (fun i x -> (i, x))
  |> Seq.filter (fun (i, _) -> i mod n = 0)
  |> Seq.map snd

let () =
  (* Step by 3 over 0..9 *)
  let r1 = List.init 10 Fun.id |> step_by 3 in
  Printf.printf "step_by 3 [0..9]  = [%s]\n"
    (r1 |> List.map string_of_int |> String.concat ";");

  (* Step by 2 over a list *)
  let r2 = step_by 2 [1;2;3;4;5] in
  Printf.printf "step_by 2 [1..5]  = [%s]\n"
    (r2 |> List.map string_of_int |> String.concat ";");

  (* Step by 1 is identity *)
  let r3 = step_by 1 [1;2;3;4] in
  Printf.printf "step_by 1 [1..4]  = [%s]\n"
    (r3 |> List.map string_of_int |> String.concat ";");

  (* Multiples of 5 up to 20 *)
  let r4 = List.init 21 Fun.id |> step_by 5 in
  Printf.printf "step_by 5 [0..20] = [%s]\n"
    (r4 |> List.map string_of_int |> String.concat ";");

  (* Lazy: step through an infinite sequence *)
  let lazy_r = seq_step_by 3 (Seq.ints 0)
               |> Seq.take 4
               |> List.of_seq in
  Printf.printf "lazy step_by 3 from 0 (first 4) = [%s]\n"
    (lazy_r |> List.map string_of_int |> String.concat ";")
