(* First-Class Modules *)
(* Pack and unpack modules as values *)

module type SHOWABLE = sig
  type t
  val to_string : t -> string
end

let show (type a) (module S : SHOWABLE with type t = a) (x : a) =
  S.to_string x

let int_show = (module struct
  type t = int
  let to_string = string_of_int
end : SHOWABLE with type t = int)

let float_show = (module struct
  type t = float
  let to_string = Printf.sprintf "%.2f"
end : SHOWABLE with type t = float)

let () =
  Printf.printf "%s\n" (show int_show 42);
  Printf.printf "%s\n" (show float_show 3.14)
