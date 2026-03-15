(* 098: Partition Iterator
   List.partition splits a list into two lists based on a predicate.
   OCaml returns (matching, non_matching) — mirroring Rust's partition. *)

(* Split a list into (evens, odds) *)
let partition_evens_odds lst =
  List.partition (fun x -> x mod 2 = 0) lst

(* Generic partition using List.partition *)
let partition pred lst = List.partition pred lst

let () =
  let (evens, odds) = partition_evens_odds [1; 2; 3; 4; 5; 6] in
  assert (evens = [2; 4; 6]);
  assert (odds  = [1; 3; 5]);

  let (a, b) = partition (fun _ -> true) [1; 2; 3] in
  assert (a = [1; 2; 3]);
  assert (b = []);

  let (a2, b2) = partition (fun _ -> false) [1; 2; 3] in
  assert (a2 = []);
  assert (b2 = [1; 2; 3]);

  let (a3, b3) = partition (fun _ -> true) ([] : int list) in
  assert (a3 = []);
  assert (b3 = []);

  Printf.printf "evens: [%s]  odds: [%s]\n"
    (String.concat "; " (List.map string_of_int evens))
    (String.concat "; " (List.map string_of_int odds))
