(* OCaml's "associated type bounds" are expressed via module type constraints.
   A module type can constrain its type members with :=, with type, or sharing. *)

module type COLLECTION = sig
  type 'a t
  type key
  val empty  : 'a t
  val insert : key -> 'a -> 'a t -> 'a t
  val find   : key -> 'a t -> 'a option
  val size   : 'a t -> int
end

(* Integer-keyed map *)
module IntMap : COLLECTION with type key = int = struct
  type key = int
  type 'a t = (int * 'a) list
  let empty = []
  let insert k v m = (k, v) :: List.filter (fun (k', _) -> k' <> k) m
  let find k m = List.assoc_opt k m
  let size m = List.length m
end

(* String-keyed map *)
module StringMap : COLLECTION with type key = string = struct
  type key = string
  type 'a t = (string * 'a) list
  let empty = []
  let insert k v m = (k, v) :: List.filter (fun (k', _) -> k' <> k) m
  let find k m = List.assoc_opt k m
  let size m = List.length m
end

let () =
  let m = IntMap.(insert 1 "one" (insert 2 "two" empty)) in
  Printf.printf "IntMap size: %d\n" (IntMap.size m);
  (match IntMap.find 1 m with
   | Some v -> Printf.printf "Found: %s\n" v
   | None   -> assert false);

  let sm = StringMap.(insert "a" 1 (insert "b" 2 empty)) in
  Printf.printf "StringMap size: %d\n" (StringMap.size sm)
