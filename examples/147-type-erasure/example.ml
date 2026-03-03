(* Type erasure via existential types (first-class modules in OCaml).
   Pack a value with its operations; the concrete type is hidden. *)

(* A "showable" value: we only know it can be converted to string *)
module type SHOWABLE = sig
  type t
  val value  : t
  val show   : t -> string
end

type showable = (module SHOWABLE)

let pack_int n : showable =
  (module struct
    type t = int
    let value = n
    let show = string_of_int
  end)

let pack_float f : showable =
  (module struct
    type t = float
    let value = f
    let show = string_of_float
  end)

let pack_bool b : showable =
  (module struct
    type t = bool
    let value = b
    let show = string_of_bool
  end)

let show_any (s : showable) =
  let module S = (val s) in
  S.show S.value

let () =
  let items = [
    pack_int 42;
    pack_float 3.14;
    pack_bool true;
    pack_int (-7);
  ] in
  Printf.printf "Erased values:\n";
  List.iter (fun s -> Printf.printf "  %s\n" (show_any s)) items
