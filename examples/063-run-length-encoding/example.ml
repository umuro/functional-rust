(* Run-Length Encoding — String Compression *)

let encode s =
  let n = String.length s in
  if n = 0 then "" else
  let buf = Buffer.create n in
  let rec go i c count =
    if i = n then begin
      if count > 1 then Buffer.add_string buf (string_of_int count);
      Buffer.add_char buf c
    end else if s.[i] = c then go (i+1) c (count+1)
    else begin
      if count > 1 then Buffer.add_string buf (string_of_int count);
      Buffer.add_char buf c;
      go (i+1) s.[i] 1
    end
  in
  go 1 s.[0] 1;
  Buffer.contents buf

let decode s =
  let buf = Buffer.create (String.length s) in
  let rec go i count =
    if i >= String.length s then ()
    else
      let c = s.[i] in
      if c >= '0' && c <= '9' then
        go (i+1) (count * 10 + Char.code c - Char.code '0')
      else begin
        let n = if count = 0 then 1 else count in
        for _ = 1 to n do Buffer.add_char buf c done;
        go (i+1) 0
      end
  in
  go 0 0;
  Buffer.contents buf

let () =
  assert (encode "AABCCCDEEEE" = "2AB3CD4E");
  assert (decode "2AB3CD4E" = "AABCCCDEEEE");
  assert (encode "" = "");
  Printf.printf "All RLE tests passed!\n"
