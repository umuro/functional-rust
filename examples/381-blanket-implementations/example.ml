(* Blanket implementations simulated via functors in OCaml *)

module type Printable = sig
  type t
  val to_string : t -> string
end

module type Describable = sig
  type t
  val describe : t -> string
end

(* "Blanket impl": any Printable is also Describable *)
module MakeDescribable (P : Printable) : Describable with type t = P.t = struct
  type t = P.t
  let describe x = "Value: " ^ P.to_string x
end

module IntPrintable = struct
  type t = int
  let to_string = string_of_int
end

module FloatPrintable = struct
  type t = float
  let to_string = string_of_float
end

module IntDescribable = MakeDescribable(IntPrintable)
module FloatDescribable = MakeDescribable(FloatPrintable)

let () =
  Printf.printf "%s\n" (IntDescribable.describe 42);
  Printf.printf "%s\n" (FloatDescribable.describe 3.14)
