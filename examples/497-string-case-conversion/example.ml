(* 497. Case conversion – OCaml *)
let to_snake_case s =
  let buf = Buffer.create (String.length s) in
  String.iteri (fun i c ->
    if c >= 'A' && c <= 'Z' then begin
      if i > 0 then Buffer.add_char buf '_';
      Buffer.add_char buf (Char.lowercase_ascii c)
    end else Buffer.add_char buf c
  ) s;
  Buffer.contents buf

let to_title_case s =
  let words = String.split_on_char ' ' s in
  let cap w = if w = "" then w
              else String.make 1 (Char.uppercase_ascii w.[0]) ^
                   String.sub w 1 (String.length w - 1) in
  String.concat " " (List.map cap words)

let () =
  let s = "hello, World! café" in
  Printf.printf "upper: %s\n" (String.map Char.uppercase_ascii s);
  Printf.printf "lower: %s\n" (String.map Char.lowercase_ascii s);
  Printf.printf "snake: %s\n" (to_snake_case "MyFunctionName");
  Printf.printf "title: %s\n" (to_title_case "the quick brown fox")
