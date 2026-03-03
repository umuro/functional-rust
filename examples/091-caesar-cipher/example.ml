let shift_char n c =
  if c >= 'a' && c <= 'z' then
    Char.chr ((Char.code c - Char.code 'a' + n) mod 26 + Char.code 'a')
  else if c >= 'A' && c <= 'Z' then
    Char.chr ((Char.code c - Char.code 'A' + n) mod 26 + Char.code 'A')
  else c

let caesar n s = String.map (shift_char n) s
let decrypt n = caesar (26 - n)

let () =
  let msg = "Hello World" in
  let enc = caesar 13 msg in
  Printf.printf "Encrypted: %s\nDecrypted: %s\n" enc (decrypt 13 enc)
