(* 484. Cow<str> concept – OCaml *)
(* OCaml has no Cow; simulate with a type *)
type cow_str = Borrowed of string | Owned of Buffer.t

let make_borrowed s = Borrowed s
let make_owned s = Owned (let b=Buffer.create (String.length s) in Buffer.add_string b s; b)

let to_string = function
  | Borrowed s -> s
  | Owned b -> Buffer.contents b

let ensure_uppercase = function
  | Borrowed s ->
    if String.for_all (fun c -> not(c>='a'&&c<='z')) s then Borrowed s
    else Owned (let b=Buffer.create (String.length s) in
                String.iter (fun c -> Buffer.add_char b (Char.uppercase_ascii c)) s; b)
  | Owned b as o -> o

let () =
  let a = make_borrowed "HELLO" in
  let b = make_borrowed "hello" in
  let ra = ensure_uppercase a in
  let rb = ensure_uppercase b in
  Printf.printf "%s %s\n" (to_string ra) (to_string rb);
  (* a was not re-allocated *)
  Printf.printf "a is borrowed: %b\n" (match ra with Borrowed _ -> true | _ -> false)
