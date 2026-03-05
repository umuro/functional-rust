(* Module Type Sharing Constraints *)
(* Share types between modules using with constraints *)

module type KEY = sig
  type t
  val compare : t -> t -> int
  val to_string : t -> string
end

module type STORE = sig
  type key
  type 'a t
  val empty : 'a t
  val set : key -> 'a -> 'a t -> 'a t
  val get : key -> 'a t -> 'a option
end

module MakeStore (K : KEY) : STORE with type key = K.t = struct
  type key = K.t
  module M = Map.Make(K)
  type 'a t = 'a M.t
  let empty = M.empty
  let set k v m = M.add k v m
  let get k m = M.find_opt k m
end

module StringStore = MakeStore(struct
  type t = string let compare = String.compare
  let to_string s = s
end)

let s = StringStore.empty |> StringStore.set "key" 42
let () = match StringStore.get "key" s with
  | Some v -> Printf.printf "Found: %d\n" v
  | None -> ()
