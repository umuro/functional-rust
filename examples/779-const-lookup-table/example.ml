(* Compile-time lookup tables in OCaml — computed at module load *)

(* CRC-32 table *)
let crc32_table =
  Array.init 256 (fun i ->
    let crc = ref (Int32.of_int i) in
    for _ = 0 to 7 do
      if Int32.logand !crc 1l = 1l then
        crc := Int32.logxor (Int32.shift_right_logical !crc 1) 0xEDB88320l
      else
        crc := Int32.shift_right_logical !crc 1
    done;
    !crc)

let crc32 data =
  let crc = ref 0xFFFFFFFFl in
  String.iter (fun c ->
    let idx = Int32.to_int (Int32.logand
      (Int32.logxor !crc (Int32.of_int (Char.code c))) 0xFFl) in
    crc := Int32.logxor
      (Int32.shift_right_logical !crc 8)
      crc32_table.(idx)
  ) data;
  Int32.logxor !crc 0xFFFFFFFFl

(* ASCII upper-case table *)
let ascii_upper =
  Array.init 256 (fun i ->
    if i >= Char.code 'a' && i <= Char.code 'z'
    then Char.chr (i - 32)
    else Char.chr i)

let uppercase s =
  String.init (String.length s)
    (fun i -> ascii_upper.(Char.code s.[i]))

let () =
  Printf.printf "CRC32(\"hello\") = %08lX\n" (crc32 "hello");
  Printf.printf "uppercase(%S)  = %S\n" "Hello, World!" (uppercase "Hello, World!")
