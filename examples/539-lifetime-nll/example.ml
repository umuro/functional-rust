(* OCaml: GC means borrows are never a concern — always safe *)
let () =
  (* Pattern that NLL enables in Rust *)
  let v = ref [1; 2; 3; 4; 5] in

  (* Use then mutate — always fine in OCaml *)
  let first = List.hd !v in
  v := !v @ [6];  (* mutation after reading first *)
  Printf.printf "first: %d, now: [%s]\n" first
    (String.concat ";" (List.map string_of_int !v));

  (* Borrow in conditional, then mutate *)
  let data = ref [|10; 20; 30|] in
  let max_val = Array.fold_left max 0 !data in
  if max_val > 15 then
    !data.(0) <- !data.(0) * 2;
  Printf.printf "data: [%s]\n" (String.concat ";" (Array.to_list (Array.map string_of_int !data)))
