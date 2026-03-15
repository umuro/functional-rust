(* Example 134: Higher-Kinded Types Simulation *)
(* OCaml has natural HKTs via module system *)

(* Approach 1: Functor typeclass via modules *)
module type FUNCTOR = sig
  type 'a t
  val map : ('a -> 'b) -> 'a t -> 'b t
end

module ListFunctor : FUNCTOR with type 'a t = 'a list = struct
  type 'a t = 'a list
  let map = List.map
end

module OptionFunctor : FUNCTOR with type 'a t = 'a option = struct
  type 'a t = 'a option
  let map f = function None -> None | Some x -> Some (f x)
end

(* Approach 2: Monad via modules *)
module type MONAD = sig
  type 'a t
  val return_ : 'a -> 'a t
  val bind : 'a t -> ('a -> 'b t) -> 'b t
end

module OptionMonad : MONAD with type 'a t = 'a option = struct
  type 'a t = 'a option
  let return_ x = Some x
  let bind m f = match m with None -> None | Some x -> f x
end

module ListMonad : MONAD with type 'a t = 'a list = struct
  type 'a t = 'a list
  let return_ x = [x]
  let bind m f = List.concat_map f m
end

(* Approach 3: Generic algorithms over functors *)
module DoubleAll (F : FUNCTOR) = struct
  let double_all xs = F.map (fun x -> x * 2) xs
end

module DoubleList = DoubleAll(ListFunctor)
module DoubleOption = DoubleAll(OptionFunctor)

(* Tests *)
let () =
  assert (ListFunctor.map (fun x -> x + 1) [1;2;3] = [2;3;4]);
  assert (OptionFunctor.map (fun x -> x * 2) (Some 5) = Some 10);
  assert (OptionFunctor.map (fun x -> x * 2) None = None);
  assert (OptionMonad.bind (Some 5) (fun x -> Some (x * 2)) = Some 10);
  assert (OptionMonad.bind None (fun x -> Some (x * 2)) = None);
  assert (ListMonad.bind [1;2;3] (fun x -> [x; x*10]) = [1;10;2;20;3;30]);
  assert (DoubleList.double_all [1;2;3] = [2;4;6]);
  assert (DoubleOption.double_all (Some 5) = Some 10);
  Printf.printf "✓ All tests passed\n"
