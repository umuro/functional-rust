(* OCaml: no unsafe traits. Thread safety is enforced at runtime via
   Domain and Mutex; the type system does not encode it statically. *)

(* Model the concept: a "Shareable" module type as a contract *)
module type Shareable = sig
  type t
  val share : t -> string
end

module SafeInt : Shareable = struct
  type t = int
  let share n = Printf.sprintf "Sharing int: %d" n
end

module SafeString : Shareable = struct
  type t = string
  let share s = Printf.sprintf "Sharing string: '%s'" s
end

let () =
  print_endline (SafeInt.share 42);
  print_endline (SafeString.share "hello")
