(* Extension pattern: given a minimal interface, derive many extra functions.
   Like Rust's extension traits, but via OCaml functors. *)

module type ORD = sig
  type t
  val compare : t -> t -> int
end

(* Extend any ORD with derived operations *)
module OrdExt (O : ORD) = struct
  include O
  let ( <  ) a b = compare a b < 0
  let ( >  ) a b = compare a b > 0
  let ( <= ) a b = compare a b <= 0
  let ( >= ) a b = compare a b >= 0
  let ( =  ) a b = compare a b = 0
  let min a b = if a < b then a else b
  let max a b = if a > b then a else b
  let clamp ~lo ~hi x = max lo (min hi x)
  let between ~lo ~hi x = lo <= x && x <= hi
  let sort lst = List.sort compare lst
end

module IntOrd = OrdExt (struct
  type t = int
  let compare = Int.compare
end)

module StringOrd = OrdExt (struct
  type t = string
  let compare = String.compare
end)

let () =
  let open IntOrd in
  Printf.printf "min 3 5 = %d\n" (min 3 5);
  Printf.printf "max 3 5 = %d\n" (max 3 5);
  Printf.printf "clamp 0 10 15 = %d\n" (clamp ~lo:0 ~hi:10 15);
  Printf.printf "between 0 10 7 = %b\n" (between ~lo:0 ~hi:10 7);
  Printf.printf "sort [3;1;4;1;5] = %s\n"
    (sort [3;1;4;1;5] |> List.map string_of_int |> String.concat ",");

  let open StringOrd in
  Printf.printf "sort strings: %s\n"
    (sort ["banana";"apple";"cherry"] |> String.concat ",")
