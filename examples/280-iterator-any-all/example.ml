(* 280: any (∃) and all (∀) — short-circuit existential/universal checks.
   OCaml: List.exists and List.for_all; Seq.exists and Seq.for_all for lazy. *)

let () =
  (* any: true if at least one element satisfies the predicate *)
  Printf.printf "any (=2) [1;2;3]    = %b\n" (List.exists (( = ) 2) [1;2;3]);
  Printf.printf "any (=9) [1;2;3]    = %b\n" (List.exists (( = ) 9) [1;2;3]);

  (* all: true if every element satisfies the predicate *)
  Printf.printf "all even [2;4;6]    = %b\n"
    (List.for_all (fun x -> x mod 2 = 0) [2;4;6]);
  Printf.printf "all even [1;2;3]    = %b\n"
    (List.for_all (fun x -> x mod 2 = 0) [1;2;3]);

  (* Vacuous truth: all holds on empty list; any is false on empty *)
  Printf.printf "all false []        = %b (vacuously true)\n"
    (List.for_all (fun _ -> false) []);
  Printf.printf "any true  []        = %b\n"
    (List.exists (fun _ -> true) []);

  (* Lazy short-circuit using Seq — stops as soon as answer is known *)
  let log = ref 0 in
  let counting_pred x = incr log; x > 3 in
  let _ = List.to_seq [1;2;4;5] |> Seq.exists counting_pred in
  Printf.printf "seq any: evaluated %d elements (short-circuits at first true)\n" !log;

  log := 0;
  let _ = List.to_seq [1;2;3;4] |> Seq.for_all (fun x -> incr log; x > 0) in
  Printf.printf "seq all: evaluated %d elements (stops at first false)\n" !log;

  (* Practical: check all results are Ok *)
  let results = [Ok 1; Ok 2; Ok 3] in
  Printf.printf "all Ok: %b\n" (List.for_all Result.is_ok results);

  let with_err = [Ok 1; Error "e"; Ok 3] in
  Printf.printf "any Error: %b\n" (List.exists Result.is_error with_err)
