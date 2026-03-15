(* 495. Template strings – OCaml *)
let render template vars =
  List.fold_left (fun s (k,v) ->
    let placeholder = "{{" ^ k ^ "}}" in
    let buf = Buffer.create (String.length s) in
    let ls = String.length s and lp = String.length placeholder in
    let i = ref 0 in
    while !i <= ls - lp do
      if String.sub s !i lp = placeholder
      then (Buffer.add_string buf v; i := !i + lp)
      else (Buffer.add_char buf s.[!i]; incr i)
    done;
    while !i < ls do Buffer.add_char buf s.[!i]; incr i done;
    Buffer.contents buf
  ) template vars

let () =
  let tmpl = "Hello, {{name}}! You have {{count}} messages." in
  let vars = [("name","Alice");("count","5")] in
  Printf.printf "%s\n" (render tmpl vars)
