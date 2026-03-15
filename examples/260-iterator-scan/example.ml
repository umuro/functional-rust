(* 260: Stateful scan — like fold but emits each intermediate state.
   OCaml has no built-in scan, but it's easy to build with List.fold_left
   or using Seq for lazy evaluation. *)

(* scan_left: like fold_left but returns all intermediate accumulators *)
let scan_left f init lst =
  let (_, result) = List.fold_left
    (fun (acc, out) x ->
      let acc' = f acc x in
      (acc', out @ [acc']))
    (init, [])
    lst
  in result

(* Lazy version using Seq *)
let seq_scan f init seq =
  (* Unfold approach: track current accumulator and remaining elements *)
  let open Seq in
  let rec go acc s () =
    match s () with
    | Nil        -> Nil
    | Cons (x, rest) ->
      let acc' = f acc x in
      Cons (acc', go acc' rest)
  in
  go init seq

let () =
  (* Running sum: emit prefix sums *)
  let running_sum = scan_left ( + ) 0 [1; 2; 3; 4; 5] in
  Printf.printf "running sum  = [%s]\n"
    (running_sum |> List.map string_of_int |> String.concat ";");

  (* Running product *)
  let running_prod = scan_left ( * ) 1 [1; 2; 3; 4] in
  Printf.printf "running prod = [%s]\n"
    (running_prod |> List.map string_of_int |> String.concat ";");

  (* Early stop — take_while on the scan result *)
  let cum_sum = scan_left ( + ) 0 [1; 2; 3; 4; 5] in
  let until_exceeded =
    let rec take = function
      | [] -> []
      | x :: rest -> if x > 6 then [] else x :: take rest
    in take cum_sum
  in
  Printf.printf "scan until > 6 = [%s]\n"
    (until_exceeded |> List.map string_of_int |> String.concat ";");

  (* Lazy seq_scan — no allocation until consumed *)
  let lazy_scan =
    seq_scan ( + ) 0 (List.to_seq [1;2;3;4;5])
    |> List.of_seq in
  Printf.printf "lazy running sum = [%s]\n"
    (lazy_scan |> List.map string_of_int |> String.concat ";")
