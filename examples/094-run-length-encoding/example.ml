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

let () =
  Printf.printf "%s\n" (encode "AABCCCDEEEE")
