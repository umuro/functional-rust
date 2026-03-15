(* 479. String replacing – OCaml *)
let replace_all s pat rep =
  let lp=String.length pat and ls=String.length s in
  let buf=Buffer.create ls in
  let i=ref 0 in
  while !i <= ls-lp do
    if String.sub s !i lp = pat
    then (Buffer.add_string buf rep; i:= !i+lp)
    else (Buffer.add_char buf s.[!i]; incr i)
  done;
  while !i < ls do Buffer.add_char buf s.[!i]; incr i done;
  Buffer.contents buf

let () =
  let s = "Hello, World! Hello, OCaml!" in
  Printf.printf "%s\n" (replace_all s "Hello" "Hi");
  Printf.printf "%s\n" (String.map (fun c -> if c=' ' then '_' else c) s)
