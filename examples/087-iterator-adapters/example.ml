(* 087: Iterator Adapters — build custom adapters
   In OCaml, adapters are just higher-order functions over Seq or lists *)

(* --- Approach 1: Custom map over Seq --- *)

(* Seq.map already exists; here we re-implement for understanding *)
let rec my_seq_map f seq () =
  match seq () with
  | Seq.Nil        -> Seq.Nil
  | Seq.Cons (x, rest) -> Seq.Cons (f x, my_seq_map f rest)

(* --- Approach 2: Custom filter over Seq --- *)

let rec my_seq_filter pred seq () =
  match seq () with
  | Seq.Nil -> Seq.Nil
  | Seq.Cons (x, rest) ->
    if pred x then Seq.Cons (x, my_seq_filter pred rest)
    else my_seq_filter pred rest ()

(* --- Approach 3: Custom take over Seq --- *)

let my_seq_take n seq =
  Seq.take n seq   (* stdlib has this; show composition below *)

(* --- Approach 4: Compose adapters in a pipeline --- *)

let range start stop =
  Seq.unfold (fun i -> if i >= stop then None else Some (i, i + 1)) start

let compose_pipeline () =
  (* even squares from an infinite sequence *)
  Seq.ints 0                    (* 0, 1, 2, 3, ... *)
  |> my_seq_filter (fun x -> x mod 2 = 0)
  |> my_seq_map    (fun x -> x * x)
  |> Seq.take 5
  |> List.of_seq

let () =
  (* my_map *)
  let doubled = Seq.ints 0 |> my_seq_map (fun x -> x * 2) |> Seq.take 5 |> List.of_seq in
  Printf.printf "my_map (*2) [0..4] = [%s]\n"
    (String.concat "; " (List.map string_of_int doubled));

  (* my_filter *)
  let filtered = range 0 5 |> my_seq_filter (fun x -> x > 2) |> List.of_seq in
  Printf.printf "my_filter (>2) [0..4] = [%s]\n"
    (String.concat "; " (List.map string_of_int filtered));

  (* my_take *)
  let taken = Seq.ints 0 |> my_seq_take 3 |> List.of_seq in
  Printf.printf "my_take 3 = [%s]\n"
    (String.concat "; " (List.map string_of_int taken));

  (* composed pipeline *)
  Printf.printf "even squares [0..] take 5 = [%s]\n"
    (String.concat "; " (List.map string_of_int (compose_pipeline ())))
