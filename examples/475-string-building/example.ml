(* 475: String Building Patterns
   OCaml idioms for building strings efficiently. *)

let () =
  (* push_str / push equivalent: Buffer.add_string / Buffer.add_char *)
  let buf = Buffer.create 8 in
  Buffer.add_string buf "hi";
  Buffer.add_char   buf '!';
  assert (Buffer.contents buf = "hi!");
  Printf.printf "buffer push: \"%s\"\n%!" (Buffer.contents buf);

  (* join: String.concat *)
  let joined = String.concat "-" ["a";"b";"c"] in
  assert (joined = "a-b-c");
  Printf.printf "join: \"%s\"\n%!" joined;

  (* collect (chars reversed): List.map + String.concat *)
  let reversed =
    "abc" |> String.to_seq |> List.of_seq |> List.rev
    |> List.map (String.make 1) |> String.concat ""
  in
  assert (reversed = "cba");
  Printf.printf "reversed: \"%s\"\n%!" reversed;

  (* repeat: String.init + concat *)
  let repeated = String.concat "" (List.init 3 (fun _ -> "ha")) in
  assert (repeated = "hahaha");
  Printf.printf "repeat: \"%s\"\n%!" repeated;

  (* with_capacity equivalent: Buffer.create hints initial size *)
  let large_buf = Buffer.create 100 in
  assert (Buffer.length large_buf = 0);
  Printf.printf "pre-allocated buffer capacity: ok\n%!";

  (* Efficient join of many integers *)
  let nums = List.init 5 (fun i -> string_of_int (i + 1)) in
  let csv  = String.concat "," nums in
  assert (csv = "1,2,3,4,5");
  Printf.printf "int join: \"%s\"\n%!" csv
