(* In a functor category, objects are functors and morphisms are
   natural transformations. In OCaml, functors + polymorphic functions. *)

(* Functor = type constructor with map *)
module type FUNCTOR = sig
  type 'a t
  val map : ('a -> 'b) -> 'a t -> 'b t
end

(* Natural transformation: polymorphic function between two functors *)
(* nat_trans F G = forall a. F.t a -> G.t a *)

module ListF : FUNCTOR = struct
  type 'a t = 'a list
  let map = List.map
end

module OptionF : FUNCTOR = struct
  type 'a t = 'a option
  let map = Option.map
end

(* Natural transformation: list -> option (head) *)
let list_to_option : 'a list -> 'a option = function
  | []     -> None
  | x :: _ -> Some x

(* Naturality condition: map f . nat = nat . map f *)
let check_naturality () =
  let f x = x * 2 in
  let lst = [1; 2; 3] in
  (* list_to_option (map f lst) = map f (list_to_option lst) *)
  let lhs = list_to_option (List.map f lst) in
  let rhs = Option.map f (list_to_option lst) in
  assert (lhs = rhs);
  Printf.printf "Naturality: list_to_option is a natural transformation\n"

(* Another nat trans: option -> list *)
let option_to_list : 'a option -> 'a list = function
  | None   -> []
  | Some x -> [x]

let () =
  check_naturality ();
  Printf.printf "list_to_option [1;2;3] = %s\n"
    (match list_to_option [1;2;3] with Some n -> string_of_int n | None -> "None");
  Printf.printf "option_to_list (Some 42) = [%s]\n"
    (option_to_list (Some 42) |> List.map string_of_int |> String.concat ";")
