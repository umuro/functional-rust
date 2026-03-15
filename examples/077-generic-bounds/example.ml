(* 077: Generic Bounds
   OCaml uses parametric polymorphism + type-class-like patterns via modules *)

(* --- Approach 1: Polymorphic functions (implicitly "bounded" by usage) --- *)

(* OCaml's structural typing means 'a list works for any 'a *)
let print_list show xs =
  "[" ^ String.concat "; " (List.map show xs) ^ "]"

(* find_max using polymorphic compare *)
let find_max = function
  | [] -> None
  | xs -> Some (List.fold_left max (List.hd xs) (List.tl xs))

let contains xs x = List.mem x xs

let larger a b = if a >= b then a else b

(* --- Approach 2: Explicit comparator functions (like bounds on PartialOrd) --- *)

let find_max_by cmp = function
  | [] -> None
  | xs -> Some (List.fold_left (fun a b -> if cmp a b >= 0 then a else b) (List.hd xs) (List.tl xs))

(* --- Approach 3: Module-based bounds (like trait constraints) --- *)

module type PRINTABLE = sig
  type t
  val to_string : t -> string
end

module type COMPARABLE = sig
  type t
  val compare : t -> t -> int
end

(* A function that requires both printable and comparable *)
module PrintAndCompare (P : PRINTABLE) (C : COMPARABLE with type t = P.t) = struct
  let show_max a b =
    if C.compare a b >= 0
    then Printf.sprintf "max = %s" (P.to_string a)
    else Printf.sprintf "max = %s" (P.to_string b)
end

module IntPC = PrintAndCompare
  (struct type t = int let to_string = string_of_int end)
  (struct type t = int let compare = Int.compare end)

let () =
  Printf.printf "print_list int [1;2;3] = %s\n"
    (print_list string_of_int [1;2;3]);
  Printf.printf "find_max [3;1;4;1;5] = %s\n"
    (match find_max [3;1;4;1;5] with Some v -> string_of_int v | None -> "None");
  Printf.printf "contains [1;2;3] 2 = %b\n" (contains [1;2;3] 2);
  Printf.printf "larger 10 20 = %d\n" (larger 10 20);
  Printf.printf "larger \"z\" \"a\" = %s\n" (larger "z" "a");
  Printf.printf "%s\n" (IntPC.show_max 10 20)
