(* Manual Serialize / Deserialize in OCaml
   We define a module type that mirrors Rust's Serialize trait,
   then implement it for a concrete record type. *)

(* ---------- Serializer "trait" as a module type ---------- *)
module type SERIALIZER = sig
  type t
  val serialize : t -> string
  val deserialize : string -> t option
end

(* ---------- A simple key=value wire format ---------- *)
(* Helper: escape '|' and '\' in strings *)
let escape s =
  let buf = Buffer.create (String.length s + 4) in
  String.iter (fun c ->
    if c = '|' || c = '\\' then Buffer.add_char buf '\\';
    Buffer.add_char buf c
  ) s;
  Buffer.contents buf

let unescape s =
  let buf = Buffer.create (String.length s) in
  let len = String.length s in
  let i = ref 0 in
  while !i < len do
    if s.[!i] = '\\' && !i + 1 < len then begin
      Buffer.add_char buf s.[!i + 1];
      i := !i + 2
    end else begin
      Buffer.add_char buf s.[!i];
      i := !i + 1
    end
  done;
  Buffer.contents buf

(* ---------- Domain type ---------- *)
type person = { name: string; age: int; active: bool }

(* ---------- Implementation ---------- *)
module PersonSerializer : SERIALIZER with type t = person = struct
  type t = person

  let serialize p =
    Printf.sprintf "name=%s|age=%d|active=%b"
      (escape p.name) p.age p.active

  let deserialize s =
    let fields = String.split_on_char '|' s in
    let tbl = Hashtbl.create 4 in
    List.iter (fun field ->
      match String.split_on_char '=' field with
      | k :: rest -> Hashtbl.replace tbl k (unescape (String.concat "=" rest))
      | [] -> ()
    ) fields;
    try
      let name   = Hashtbl.find tbl "name" in
      let age    = int_of_string (Hashtbl.find tbl "age") in
      let active = bool_of_string (Hashtbl.find tbl "active") in
      Some { name; age; active }
    with Not_found | Failure _ -> None
end

(* ---------- Demo ---------- *)
let () =
  let alice = { name = "Alice|Wonder"; age = 30; active = true } in
  let encoded = PersonSerializer.serialize alice in
  Printf.printf "Encoded : %s\n" encoded;
  match PersonSerializer.deserialize encoded with
  | Some p ->
    Printf.printf "Decoded : name=%s age=%d active=%b\n" p.name p.age p.active
  | None ->
    Printf.printf "Decode failed!\n"
