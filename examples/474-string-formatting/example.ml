(* 474: String Formatting
   OCaml uses Printf.sprintf / Format.asprintf for formatted strings.
   Buffer is the write!/writeln! equivalent. *)

let () =
  (* Right / left alignment — pad with spaces *)
  let right_aligned = Printf.sprintf "%5s" "hi" in
  assert (right_aligned = "   hi");
  Printf.printf "right-align: \"%s\"\n%!" right_aligned;

  let left_aligned = Printf.sprintf "%-5s" "hi" in
  assert (left_aligned = "hi   ");
  Printf.printf "left-align:  \"%s\"\n%!" left_aligned;

  (* Numeric formatting *)
  let hex = Printf.sprintf "%x" 255 in
  assert (hex = "ff");
  Printf.printf "hex 255: \"%s\"\n%!" hex;

  let floatf = Printf.sprintf "%.2f" 3.14159 in
  assert (floatf = "3.14");
  Printf.printf "float .2f: \"%s\"\n%!" floatf;

  (* Write to a Buffer — equivalent to write!() *)
  let buf = Buffer.create 16 in
  Buffer.add_string buf (string_of_int 42);
  assert (Buffer.contents buf = "42");
  Printf.printf "buffer write: \"%s\"\n%!" (Buffer.contents buf);

  (* Debug-style printing of a list *)
  let lst = [1;2;3] in
  let debug = "[" ^ (lst |> List.map string_of_int |> String.concat "; ") ^ "]" in
  assert (debug = "[1; 2; 3]");
  Printf.printf "debug list: %s\n%!" debug;

  (* Format.asprintf for richer formatting *)
  let msg = Format.asprintf "items: %a"
    (Format.pp_print_list ~pp_sep:(fun ppf () -> Format.pp_print_string ppf ", ")
       Format.pp_print_int)
    [1;2;3]
  in
  Printf.printf "format list: %s\n%!" msg;

  (* Zero-padded integer *)
  let zp = Printf.sprintf "%05d" 42 in
  assert (zp = "00042");
  Printf.printf "zero-padded: \"%s\"\n%!" zp
