(* Typeclass-style optics: HasX pattern in OCaml *)

(* A module type for things that have a 'name' field *)
module type HAS_NAME = sig
  type t
  val get_name : t -> string
  val set_name : string -> t -> t
end

module type HAS_AGE = sig
  type t
  val get_age : t -> int
  val set_age : int -> t -> t
end

(* Generic operation: capitalize name for anything with a name *)
module UpdateName (H : HAS_NAME) = struct
  let capitalize x = H.set_name (String.capitalize_ascii (H.get_name x)) x
end

(* Concrete types *)
type person  = { name: string; age: int }
type account = { name: string; balance: float }

module PersonName : HAS_NAME with type t = person = struct
  type t = person
  let get_name p = p.name
  let set_name n p = { p with name = n }
end

module AccountName : HAS_NAME with type t = account = struct
  type t = account
  let get_name a = a.name
  let set_name n a = { a with name = n }
end

module UP = UpdateName(PersonName)
module UA = UpdateName(AccountName)

let () =
  let p = { name="alice"; age=30 } in
  let a = { name="bob_account"; balance=1000.0 } in
  Printf.printf "capitalized person: %s\n" (UP.capitalize p).name;
  Printf.printf "capitalized account: %s\n" (UA.capitalize a).name
