(* 256: Chaining iterators
   OCaml equivalent: List.append / (@) for lists,
   or Seq.append for lazy sequences. *)

(* Chain two lists — eager, like Rust's chain().collect() *)
let chain a b = a @ b

(* Lazy chain using Seq — mirrors Rust's lazy chain() *)
let seq_chain a b = Seq.append a b

let () =
  let a = [1; 2; 3] and b = [4; 5; 6] in

  (* Eager list append *)
  let result = chain a b in
  Printf.printf "chain [1;2;3] [4;5;6] = [%s]\n"
    (result |> List.map string_of_int |> String.concat ";");

  (* Empty chain *)
  let r2 = chain [] [1; 2] in
  Printf.printf "chain [] [1;2] = [%s]\n"
    (r2 |> List.map string_of_int |> String.concat ";");

  (* Count elements via lazy Seq *)
  let count = Seq.append (List.to_seq a) (List.to_seq b)
              |> Seq.fold_left (fun n _ -> n + 1) 0 in
  Printf.printf "seq chain count = %d\n" count;

  (* Chain multiple lists *)
  let multi = List.concat [[1;2]; [3;4]; [5;6]] in
  Printf.printf "concat [[1;2];[3;4];[5;6]] = [%s]\n"
    (multi |> List.map string_of_int |> String.concat ";");

  (* Lazy Seq chain is non-allocating until consumed *)
  let lazy_r = seq_chain (List.to_seq [1;2;3]) (List.to_seq [4;5;6])
               |> List.of_seq in
  Printf.printf "lazy chain = [%s]\n"
    (lazy_r |> List.map string_of_int |> String.concat ";")
