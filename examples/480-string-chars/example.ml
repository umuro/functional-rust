(* 480. chars() – OCaml *)
let () =
  let s = "Hello, World! 🌍" in
  Printf.printf "byte_len=%d\n" (String.length s);
  String.iter (fun c -> Printf.printf "%c " c) (String.sub s 0 7); print_newline ();
  let upper = String.map Char.uppercase_ascii s in
  Printf.printf "%s\n" upper;
  let alpha = String.concat "" (
    String.to_seq s |> Seq.filter (fun c -> Char.code c < 128 && (c>='a'&&c<='z'||c>='A'&&c<='Z'))
    |> Seq.map (String.make 1) |> List.of_seq) in
  Printf.printf "alpha: %s\n" alpha
