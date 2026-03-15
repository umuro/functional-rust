(* 481. Byte-level strings – OCaml *)
let () =
  let s = "Hello, World!" in
  let b = Bytes.of_string s in
  Printf.printf "bytes: ";
  Bytes.iter (fun c -> Printf.printf "%02x " (Char.code c)) b;
  print_newline ();
  let csum = Bytes.fold_left (fun a c -> a + Char.code c) 0 b in
  Printf.printf "checksum=%d\n" csum;
  let low = Bytes.map (fun c ->
    if c >= 'A' && c <= 'Z' then Char.chr (Char.code c + 32) else c) b in
  Printf.printf "lower: %s\n" (Bytes.to_string low);
  let raw = [|72;101;108;108;111|] in
  let s2 = Bytes.init (Array.length raw) (fun i -> Char.chr raw.(i)) in
  Printf.printf "from bytes: %s\n" (Bytes.to_string s2)
