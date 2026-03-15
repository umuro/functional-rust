(* 085: Iterator Trait — implement from scratch
   OCaml uses the Seq module for lazy sequences or custom generators *)

(* --- Approach 1: A range "iterator" as a stateful ref-based generator --- *)

(* In OCaml, a simple iterator is a unit -> 'a option function *)
type 'a gen = unit -> 'a option

let make_range start stop : int gen =
  let current = ref start in
  fun () ->
    if !current >= stop then None
    else begin
      let v = !current in
      incr current;
      Some v
    end

let gen_to_list gen =
  let rec aux acc =
    match gen () with
    | None   -> List.rev acc
    | Some v -> aux (v :: acc)
  in
  aux []

(* --- Approach 2: Fibonacci as a Seq (lazy, infinite) --- *)

let fibonacci : int Seq.t =
  Seq.unfold (fun (a, b) -> Some (a, (b, a + b))) (0, 1)

(* --- Approach 3: Range as Seq — compose with stdlib list operations --- *)

let range_seq start stop =
  Seq.unfold (fun i ->
    if i >= stop then None else Some (i, i + 1)
  ) start

(* Once we have a Seq we get map/filter/fold for free *)
let demo_free_methods () =
  range_seq 0 10
  |> Seq.filter (fun x -> x mod 2 = 0)
  |> Seq.map    (fun x -> x * x)
  |> List.of_seq

let () =
  (* range generator *)
  let r = make_range 0 5 in
  Printf.printf "range 0..5 = [%s]\n"
    (String.concat "; " (List.map string_of_int (gen_to_list r)));

  (* empty range *)
  let empty = make_range 5 5 in
  Printf.printf "empty range = [%s]\n"
    (String.concat "; " (List.map string_of_int (gen_to_list empty)));

  (* fibonacci (take first 8) *)
  let fibs = fibonacci |> Seq.take 8 |> List.of_seq in
  Printf.printf "fibs[0..7] = [%s]\n"
    (String.concat "; " (List.map string_of_int fibs));

  (* free methods demo *)
  Printf.printf "even squares 0..9 = [%s]\n"
    (String.concat "; " (List.map string_of_int (demo_free_methods ())));

  (* sum via fold *)
  let s = range_seq 1 6 |> Seq.fold_left ( + ) 0 in
  Printf.printf "sum 1..5 = %d\n" s
