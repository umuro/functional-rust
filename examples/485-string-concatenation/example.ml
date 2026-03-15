(* 485. String concatenation – OCaml *)
let () =
  let a = "Hello" and b = ", " and c = "World!" in
  (* ^ allocates a new string *)
  let s = a ^ b ^ c in
  Printf.printf "%s\n" s;

  (* Efficient: Buffer *)
  let parts = ["the";"quick";"brown";"fox"] in
  let buf = Buffer.create 32 in
  List.iter (fun p -> Buffer.add_string buf p; Buffer.add_char buf ' ') parts;
  Printf.printf "%s\n" (String.trim (Buffer.contents buf));

  (* concat with separator *)
  Printf.printf "%s\n" (String.concat ", " ["a";"b";"c"])
