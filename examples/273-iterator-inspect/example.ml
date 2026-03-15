(* 273: inspect — tap into a pipeline with a side-effect, passing values through.
   OCaml idiom: a simple helper that applies a side-effect and returns the value. *)

(* tap: apply f for its side-effect, return x unchanged — the OCaml inspect *)
let tap f x = f x; x

(* Seq version: tap into a sequence pipeline *)
let seq_tap f seq = Seq.map (tap f) seq

let () =
  (* Basic: inspect does not change the result *)
  let result = [1;2;3] |> List.map (tap (fun _ -> ())) in
  Printf.printf "tap no-op: [%s]\n"
    (result |> List.map string_of_int |> String.concat ";");

  (* Collect side-effects while processing *)
  let seen = ref [] in
  let _result = [1;2;3]
    |> List.map (tap (fun x -> seen := x :: !seen))
    |> List.map (fun x -> x * 2) in
  Printf.printf "seen (reversed): [%s]\n"
    (!seen |> List.map string_of_int |> String.concat ";");

  (* Inspect between pipeline stages — see values after filter, before map *)
  let after_filter = ref [] in
  let result2 = List.init 6 (fun i -> i + 1)
    |> List.filter (fun x -> x mod 2 = 0)
    |> List.map (tap (fun x -> after_filter := x :: !after_filter))
    |> List.map (fun x -> x * 10) in
  Printf.printf "after filter (pre-map): [%s]\n"
    (List.rev !after_filter |> List.map string_of_int |> String.concat ";");
  Printf.printf "final result:           [%s]\n"
    (result2 |> List.map string_of_int |> String.concat ";");

  (* Lazy: seq_tap in a Seq pipeline *)
  let log = Buffer.create 32 in
  let lazy_result =
    List.to_seq [1;2;3]
    |> seq_tap (fun x -> Buffer.add_string log (string_of_int x ^ " "))
    |> List.of_seq in
  Printf.printf "seq tap log: %s\n" (Buffer.contents log);
  Printf.printf "seq tap result: [%s]\n"
    (lazy_result |> List.map string_of_int |> String.concat ";")
