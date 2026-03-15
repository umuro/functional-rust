(* 271: find_map — transform each element and return the first Some result.
   OCaml: List.find_map (4.10+) does exactly this.
   Equivalent to: find the first element where f returns Some(x). *)

let () =
  (* Parse strings: find first valid integer *)
  let strings = ["foo"; "bar"; "42"; "baz"] in
  let first_int = List.find_map int_of_string_opt strings in
  Printf.printf "first parseable int = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int first_int);

  (* None when nothing matches *)
  let no_int = List.find_map int_of_string_opt ["foo"; "bar"] in
  Printf.printf "no parseable int = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int no_int);

  (* Find first element satisfying a transform predicate *)
  let nums = [1;2;3;4;5] in
  let first_big = List.find_map
    (fun x -> if x > 3 then Some (x * 10) else None) nums in
  Printf.printf "first > 3, times 10 = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int first_big);

  (* find_map on associations *)
  let db = [("alice", 30); ("bob", 25); ("carol", 35)] in
  let age_of name =
    List.find_map (fun (n, a) -> if n = name then Some a else None) db in
  Printf.printf "age of alice = %s\n"
    (Option.fold ~none:"not found" ~some:string_of_int (age_of "alice"));
  Printf.printf "age of dave  = %s\n"
    (Option.fold ~none:"not found" ~some:string_of_int (age_of "dave"));

  (* Lazy with Seq.find_map *)
  let lazy_result =
    List.to_seq [1;2;3;4;5]
    |> Seq.find_map (fun x -> if x > 3 then Some (x * 100) else None) in
  Printf.printf "seq find_map first >3 *100 = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int lazy_result)
