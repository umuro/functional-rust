(* 475. String building – OCaml *)
let () =
  let buf = Buffer.create 64 in
  Buffer.add_string buf "Hello";
  Buffer.add_char buf ',';
  Buffer.add_string buf " World!";
  Printf.printf "%s\n" (Buffer.contents buf);

  let words = ["the";"quick";"brown";"fox"] in
  Printf.printf "%s\n" (String.concat " " words);

  let chars = ['H';'e';'l';'l';'o'] in
  let s = String.init (List.length chars) (List.nth chars) in
  Printf.printf "%s\n" s;

  let repeated = String.concat "" (List.init 4 (fun _ -> "ab")) in
  Printf.printf "%s\n" repeated
