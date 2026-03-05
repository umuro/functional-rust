(* 289. Extending collections with extend() - OCaml *)
(* OCaml: List.append or @ to extend lists *)

let () =
  let base = [1; 2; 3] in
  let extension = [4; 5; 6] in
  let combined = base @ extension in
  Printf.printf "Extended: %s\n"
    (String.concat ", " (List.map string_of_int combined));

  (* Extend with multiple sources *)
  let all = base @ [4; 5] @ [6; 7; 8] in
  Printf.printf "Multi-extend: %s\n"
    (String.concat ", " (List.map string_of_int all));

  (* Buffer extension pattern *)
  let buf = Buffer.create 16 in
  Buffer.add_string buf "hello";
  Buffer.add_string buf " ";
  Buffer.add_string buf "world";
  Printf.printf "Buffer: %s\n" (Buffer.contents buf)
