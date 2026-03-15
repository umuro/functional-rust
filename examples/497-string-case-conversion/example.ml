(* 497: String case conversion — upper, lower, snake_case, camelCase, Title Case *)

(* Upper / lower: OCaml stdlib covers ASCII *)
let to_upper = String.uppercase_ascii
let to_lower = String.lowercase_ascii

(* CamelCase → snake_case: insert '_' before each uppercase letter *)
let to_snake_case s =
  let buf = Buffer.create (String.length s * 2) in
  String.iteri (fun i c ->
    if c >= 'A' && c <= 'Z' then begin
      if i > 0 then Buffer.add_char buf '_';
      Buffer.add_char buf (Char.chr (Char.code c + 32))
    end else
      Buffer.add_char buf c
  ) s;
  Buffer.contents buf

(* snake_case → camelCase: capitalize first letter of each word after '_' *)
let to_camel_case s =
  let buf = Buffer.create (String.length s) in
  let words = String.split_on_char '_' s in
  List.iteri (fun i word ->
    match String.length word with
    | 0 -> ()
    | _ ->
      if i = 0 then Buffer.add_string buf word
      else begin
        Buffer.add_char buf (Char.uppercase_ascii word.[0]);
        Buffer.add_string buf (String.sub word 1 (String.length word - 1))
      end
  ) words;
  Buffer.contents buf

(* "hello world" → "Hello World": capitalize first letter of each space-separated word *)
let to_title_case s =
  let words = String.split_on_char ' ' s in
  let capitalize w =
    if String.length w = 0 then w
    else
      String.make 1 (Char.uppercase_ascii w.[0]) ^
      String.sub w 1 (String.length w - 1)
  in
  String.concat " " (List.map capitalize words)

let () =
  assert (to_upper "hello" = "HELLO");
  assert (to_lower "HELLO" = "hello");
  print_endline "upper/lower: ok";

  assert (to_snake_case "MyFunc" = "my_func");
  assert (to_snake_case "MyFuncName" = "my_func_name");
  print_endline "to_snake_case: ok";

  assert (to_camel_case "my_func_name" = "myFuncName");
  assert (to_camel_case "hello" = "hello");
  print_endline "to_camel_case: ok";

  assert (to_title_case "hello world" = "Hello World");
  assert (to_title_case "the quick brown fox" = "The Quick Brown Fox");
  print_endline "to_title_case: ok";

  print_endline "All assertions passed."
