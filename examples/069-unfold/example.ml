(* 069: Unfold — generate a sequence from a seed value
   OCaml 4.11+ has Seq.unfold; we also show manual versions *)

(* --- Approach 1: Manual unfold as eager list builder --- *)

let unfold seed f =
  let rec aux acc state =
    match f state with
    | None           -> List.rev acc
    | Some (v, next) -> aux (v :: acc) next
  in
  aux [] seed

let range a b =
  unfold a (fun i -> if i >= b then None else Some (i, i + 1))

let fibs_up_to limit =
  unfold (0, 1) (fun (a, b) ->
    if a > limit then None else Some (a, (b, a + b)))

(* --- Approach 2: Using Seq.unfold (lazy, OCaml 4.11+) --- *)

let collatz_seq n =
  Seq.unfold (fun x ->
    if x = 0 then None
    else if x = 1 then Some (1, 0)     (* emit 1 then stop *)
    else if x mod 2 = 0 then Some (x, x / 2)
    else Some (x, 3 * x + 1)
  ) n

let collatz n = List.of_seq (collatz_seq n)

(* --- Approach 3: Seq.iterate to build an infinite sequence, then take --- *)

let naturals_from start =
  Seq.iterate (fun n -> n + 1) start

let take n seq =
  List.of_seq (Seq.take n seq)

let () =
  Printf.printf "range 1 6 = [%s]\n"
    (String.concat "; " (List.map string_of_int (range 1 6)));
  Printf.printf "fibs up to 20 = [%s]\n"
    (String.concat "; " (List.map string_of_int (fibs_up_to 20)));
  Printf.printf "collatz 6 = [%s]\n"
    (String.concat "; " (List.map string_of_int (collatz 6)));
  Printf.printf "naturals from 5, take 5 = [%s]\n"
    (String.concat "; " (List.map string_of_int (take 5 (naturals_from 5))))
