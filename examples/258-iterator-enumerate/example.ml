(* 258: Index-value pairs (enumerate)
   OCaml: List.mapi gives (index, value); List.filteri for filtered enumeration.
   For lazy sequences use Seq.mapi. *)

(* Add zero-based indices to a list — OCaml equivalent of enumerate() *)
let enumerate lst =
  List.mapi (fun i x -> (i, x)) lst

let () =
  (* Extract indices *)
  let v = ["a"; "b"; "c"] in
  let indices = List.mapi (fun i _ -> i) v in
  Printf.printf "indices = [%s]\n"
    (indices |> List.map string_of_int |> String.concat ";");

  (* Add index to each value *)
  let nums = [10; 20; 30] in
  let result = List.mapi (fun i x -> x + i) nums in
  Printf.printf "val+idx = [%s]\n"
    (result |> List.map string_of_int |> String.concat ";");

  (* Keep only even-index elements *)
  let v2 = ["a"; "b"; "c"; "d"] in
  let even_idx = List.filteri (fun i _ -> i mod 2 = 0) v2 in
  Printf.printf "even-index elements = [%s]\n" (String.concat ";" even_idx);

  (* Full enumerate for printing *)
  let pairs = enumerate ["x"; "y"; "z"] in
  List.iter (fun (i, s) -> Printf.printf "  %d: %s\n" i s) pairs;

  (* Lazy Seq.mapi — does not allocate the whole list *)
  let lazy_pairs = List.to_seq [10; 20; 30]
                   |> Seq.mapi (fun i x -> (i, x))
                   |> List.of_seq in
  Printf.printf "seq enumerate = [%s]\n"
    (lazy_pairs |> List.map (fun (i,x) -> Printf.sprintf "(%d,%d)" i x)
                |> String.concat ";")
