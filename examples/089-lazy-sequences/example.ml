(* 089: Lazy Sequences
   OCaml's Seq module provides demand-driven (lazy) sequences *)

(* --- Approach 1: Natural numbers from a seed --- *)

(* Seq.ints is built-in from 4.14; show it manually too *)
let naturals_from n = Seq.iterate (fun x -> x + 1) n

(* --- Approach 2: Fibonacci as an infinite lazy Seq --- *)

let fibs =
  Seq.unfold (fun (a, b) -> Some (a, (b, a + b))) (0, 1)

(* --- Approach 3: Powers of 2 (finite lazy sequence via take) --- *)

let powers_of_2 =
  Seq.unfold (fun exp ->
    if exp >= 4 then None
    else Some (1 lsl exp, exp + 1)
  ) 0

(* --- Approach 4: Seq.map / filter are also lazy --- *)

let lazy_pipeline () =
  (* No computation until List.of_seq forces it *)
  naturals_from 0
  |> Seq.filter (fun x -> x mod 3 = 0)
  |> Seq.map    (fun x -> x * x)
  |> Seq.take   5
  |> List.of_seq

let () =
  Printf.printf "naturals from 0, take 5 = [%s]\n"
    (String.concat "; " (List.map string_of_int
      (List.of_seq (Seq.take 5 (naturals_from 0)))));

  Printf.printf "fibs take 8 = [%s]\n"
    (String.concat "; " (List.map string_of_int
      (List.of_seq (Seq.take 8 fibs))));

  Printf.printf "powers of 2 (0..3) = [%s]\n"
    (String.concat "; " (List.map string_of_int (List.of_seq powers_of_2)));

  Printf.printf "multiples-of-3 squared, take 5 = [%s]\n"
    (String.concat "; " (List.map string_of_int (lazy_pipeline ())))
