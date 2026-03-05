(* Modules — Include for Extension *)
(* Extend standard library modules with include *)

module MyString = struct
  include String

  let starts_with ~prefix s =
    let plen = length prefix in
    plen <= length s && sub s 0 plen = prefix

  let ends_with ~suffix s =
    let slen = length suffix and len = length s in
    slen <= len && sub s (len - slen) slen = suffix

  let repeat n s =
    let buf = Buffer.create (n * length s) in
    for _ = 1 to n do Buffer.add_string buf s done;
    Buffer.contents buf

  let count_char c s =
    fold_left (fun acc ch -> if ch = c then acc + 1 else acc) 0 s
end

let () =
  Printf.printf "starts: %b\n" (MyString.starts_with ~prefix:"hel" "hello");
  Printf.printf "repeat: %s\n" (MyString.repeat 3 "ab");
  Printf.printf "count 'l': %d\n" (MyString.count_char 'l' "hello world")
