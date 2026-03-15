(* 498. Safe Unicode truncation – OCaml *)
let truncate_bytes s max_bytes =
  let n = String.length s in
  if n <= max_bytes then s
  else
    (* Walk back to find valid UTF-8 boundary *)
    let i = ref (min max_bytes n) in
    (* Skip continuation bytes (0x80..0xBF) *)
    while !i > 0 && Char.code s.[!i-1] land 0xC0 = 0x80 do decr i done;
    String.sub s 0 !i

let truncate_chars s max_chars =
  let chars = List.init (String.length s) (String.get s) in
  (* crude: count UTF-8 code points *)
  let rec take_chars bytes n =
    if n=0 || bytes=[] then []
    else let b = Char.code (List.hd bytes) in
         let len = if b land 0x80=0 then 1 else if b land 0xE0=0xC0 then 2
                   else if b land 0xF0=0xE0 then 3 else 4 in
         List.filteri (fun i _ -> i<len) bytes @ take_chars (List.filteri (fun i _ -> i>=len) bytes) (n-1)
  in
  let _ = take_chars in
  (* Simple approach: count chars *)
  let count = ref 0 and i = ref 0 and n = String.length s in
  while !i < n && !count < max_chars do
    let b = Char.code s.[!i] in
    let len = if b land 0x80=0 then 1 else if b land 0xE0=0xC0 then 2
              else if b land 0xF0=0xE0 then 3 else 4 in
    i := !i + len; incr count
  done;
  String.sub s 0 !i

let () =
  let s = "Hello, caf\xc3\xa9 world!" in
  Printf.printf "bytes: %d\n" (String.length s);
  Printf.printf "trunc 8 bytes: %s\n" (truncate_bytes s 8);
  Printf.printf "trunc 5 chars: %s\n" (truncate_chars s 5)
