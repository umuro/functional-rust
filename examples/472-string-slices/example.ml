(* 472: String Slices and Byte Boundaries
   Rust &str slices are UTF-8 byte ranges; panics on non-char boundaries.
   OCaml's String.sub works on byte indices; for Unicode use the
   uchar / Uutf libraries. Here we show the stdlib approach. *)

(* Safe sub-string: returns None instead of raising on out-of-range *)
let safe_sub s pos len =
  if pos < 0 || len < 0 || pos + len > String.length s then None
  else Some (String.sub s pos len)

(* Character count vs byte length — they differ for multi-byte UTF-8 *)
let byte_length s = String.length s

(* Count Unicode code points by decoding UTF-8 manually *)
let char_count s =
  let len = String.length s in
  let count = ref 0 in
  let i = ref 0 in
  while !i < len do
    let b = Char.code s.[!i] in
    incr count;
    (* Determine how many bytes this code point uses *)
    let step =
      if b land 0x80 = 0    then 1   (* ASCII *)
      else if b land 0xE0 = 0xC0 then 2
      else if b land 0xF0 = 0xE0 then 3
      else 4
    in
    i := !i + step
  done;
  !count

(* Byte-index of each ASCII character in a string *)
let char_indices s =
  List.init (String.length s) (fun i -> (i, s.[i]))

let () =
  (* ASCII slicing — byte indices match character indices *)
  assert (String.sub "hello" 0 3 = "hel");
  Printf.printf "sub \"hello\" 0 3 = \"%s\"\n%!" (String.sub "hello" 0 3);

  (* Safe sub *)
  assert (safe_sub "hello" 1 3 = Some "ell");
  assert (safe_sub "hello" 0 99 = None);
  Printf.printf "safe_sub: %s / %s\n%!"
    (safe_sub "hello" 1 3 |> Option.value ~default:"none")
    (safe_sub "hello" 0 99 |> Option.value ~default:"none");

  (* "café": 5 bytes (é = 2 bytes in UTF-8), 4 characters *)
  let cafe = "caf\xC3\xA9" in   (* café in UTF-8 *)
  assert (byte_length cafe = 5);
  assert (char_count  cafe = 4);
  Printf.printf "\"café\" byte_length=%d char_count=%d\n%!"
    (byte_length cafe) (char_count cafe);

  (* char_indices on ASCII string *)
  let idx = char_indices "abc" in
  assert (idx = [(0,'a');(1,'b');(2,'c')]);
  Printf.printf "char_indices \"abc\": %s\n%!"
    (idx |> List.map (fun (i,c) -> Printf.sprintf "(%d,'%c')" i c)
     |> String.concat " ")
