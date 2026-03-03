(* OCaml avoids the Haskell coherence/orphan problem because type class
   instances are just module values, not implicit.
   You explicitly choose which instance to use. *)

(* Semigroup: two implementations of "append" for the same type *)
module type SEMIGROUP = sig
  type t
  val append : t -> t -> t
end

(* Two different semigroup instances for integers *)
module IntSum : SEMIGROUP with type t = int = struct
  type t = int
  let append = ( + )
end

module IntProduct : SEMIGROUP with type t = int = struct
  type t = int
  let append = ( * )
end

(* Explicitly pass the instance — no ambiguity *)
module Fold (S : SEMIGROUP) = struct
  let fold_map f lst =
    match List.map f lst with
    | []     -> failwith "empty list"
    | x :: xs -> List.fold_left S.append x xs
end

module SumFold     = Fold (IntSum)
module ProductFold = Fold (IntProduct)

let () =
  let nums = [1; 2; 3; 4; 5] in
  Printf.printf "sum     = %d\n" (SumFold.fold_map     (fun x -> x) nums);
  Printf.printf "product = %d\n" (ProductFold.fold_map (fun x -> x) nums);

  (* No orphan problem: instances live at module level, chosen explicitly *)
  let concat_strings = Fold (struct
    type t = string
    let append = ( ^ )
  end) in
  Printf.printf "concat  = %s\n" (concat_strings.fold_map (fun x -> x) ["a";"b";"c"])
