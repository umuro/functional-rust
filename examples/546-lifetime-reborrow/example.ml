(* Reborrowing concept in OCaml — refs can be accessed multiple times *)
let () =
  let x = ref 42 in
  (* Can "reborrow" (read) multiple times from same ref *)
  let a = !x in
  let b = !x in
  Printf.printf "a=%d, b=%d\n" a b;

  (* Modify, then read again *)
  x := !x + 10;
  let c = !x in
  Printf.printf "after mutation: c=%d\n" c
