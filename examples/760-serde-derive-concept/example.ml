(* Derive-based serialization concept in OCaml
   We simulate what [@@deriving] does by writing the expansion manually,
   then compare with the PPX-generated version (commented). *)

(* ---------- The "trait" (module type) ---------- *)
module type SERIALIZABLE = sig
  type t
  val fields : string list           (* field names in order *)
  val to_assoc : t -> (string * string) list
  val of_assoc : (string * string) list -> t option
end

(* ---------- Generic serializer using the module ---------- *)
module JsonLike (S : SERIALIZABLE) = struct
  let serialize v =
    let pairs = S.to_assoc v in
    let inner =
      List.map (fun (k, v) -> Printf.sprintf "%S: %S" k v) pairs
      |> String.concat ", "
    in
    Printf.sprintf "{ %s }" inner

  let deserialize s =
    (* Toy parser: strip braces, split on ", ", split each on ": " *)
    let s = String.trim s in
    let s = String.sub s 1 (String.length s - 2) |> String.trim in
    let pairs =
      String.split_on_char ',' s
      |> List.filter_map (fun pair ->
        match String.split_on_char ':' (String.trim pair) with
        | [k; v] ->
          let unquote x =
            let x = String.trim x in
            if String.length x >= 2 && x.[0] = '"'
            then String.sub x 1 (String.length x - 2)
            else x
          in
          Some (unquote k, unquote v)
        | _ -> None)
    in
    S.of_assoc pairs
end

(* ---------- Domain type ---------- *)
(* What [@@deriving serialize] would generate: *)
type color = { r: int; g: int; b: int }

(* Manual "derive" expansion *)
module ColorSerializable : SERIALIZABLE with type t = color = struct
  type t = color
  let fields = ["r"; "g"; "b"]
  let to_assoc c = [
    ("r", string_of_int c.r);
    ("g", string_of_int c.g);
    ("b", string_of_int c.b);
  ]
  let of_assoc pairs =
    let find k = List.assoc_opt k pairs in
    match find "r", find "g", find "b" with
    | Some r, Some g, Some b ->
      (try Some { r = int_of_string r; g = int_of_string g; b = int_of_string b }
       with Failure _ -> None)
    | _ -> None
end

module ColorJson = JsonLike(ColorSerializable)

let () =
  let red = { r = 255; g = 0; b = 0 } in
  let encoded = ColorJson.serialize red in
  Printf.printf "Serialized: %s\n" encoded;
  match ColorJson.deserialize encoded with
  | Some c -> Printf.printf "Deserialized: r=%d g=%d b=%d\n" c.r c.g c.b
  | None   -> Printf.printf "Failed to deserialize\n"
