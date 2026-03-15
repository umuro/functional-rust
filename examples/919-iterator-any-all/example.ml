(* 280. Existential checks: any() and all() - OCaml *)

let () =
  let nums = [2; 4; 6; 8; 10] in
  Printf.printf "All even: %b\n" (List.for_all (fun x -> x mod 2 = 0) nums);
  Printf.printf "Any > 5: %b\n"  (List.exists (fun x -> x > 5) nums);
  Printf.printf "Any odd: %b\n"  (List.exists (fun x -> x mod 2 <> 0) nums);
  Printf.printf "All > 0: %b\n"  (List.for_all (fun x -> x > 0) nums);

  (* Vacuous truth / false *)
  Printf.printf "all [] > 0: %b\n" (List.for_all (fun x -> x > 0) []);
  Printf.printf "any [] > 0: %b\n" (List.exists (fun x -> x > 0) []);

  (* Practical: validate list of strings *)
  let words = ["hello"; "world"; "rust"] in
  let all_lowercase = List.for_all (fun w ->
    String.lowercase_ascii w = w) words in
  Printf.printf "All lowercase: %b\n" all_lowercase
