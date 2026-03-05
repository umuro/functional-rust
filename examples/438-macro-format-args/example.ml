(* format_args! concept in OCaml *)

(* OCaml's Format module has similar deferred formatting *)

let format_to_buffer buf fmt_fn =
  let b = Buffer.create 64 in
  let f = Format.formatter_of_buffer b in
  fmt_fn f;
  Format.pp_print_flush f ();
  Buffer.add_buffer buf b

(* Format without allocation to a custom formatter *)
let to_stderr fmt_fn =
  let f = Format.formatter_of_out_channel stderr in
  fmt_fn f;
  Format.pp_print_flush f ()

let with_prefix prefix fmt_fn =
  let b = Buffer.create 64 in
  let f = Format.formatter_of_buffer b in
  fmt_fn f;
  Format.pp_print_flush f ();
  prefix ^ Buffer.contents b

let () =
  let buf = Buffer.create 64 in
  format_to_buffer buf (fun f ->
    Format.fprintf f "Hello, %s! Value = %d" "World" 42
  );
  Printf.printf "Buffer: %s\n" (Buffer.contents buf);
  let msg = with_prefix "[INFO] " (fun f ->
    Format.fprintf f "Server started on port %d" 8080
  ) in
  Printf.printf "%s\n" msg
