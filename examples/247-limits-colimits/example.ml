(* Limits and colimits are universal constructions in category theory.
   Products = binary limits; Coproducts = binary colimits.
   In types: products = tuples, coproducts = sum types *)

(* Product = limit of discrete diagram {A, B} *)
(* Characterised by projections fst, snd *)
let product_intro a b = (a, b)
let fst (a, _) = a
let snd (_, b) = b

(* Universality: any cone factors uniquely through the product *)
let pair_of f g x = (f x, g x)

(* Coproduct = colimit of discrete diagram {A, B} *)
type ('a, 'b) coprod = Inl of 'a | Inr of 'b

let coprod_inl a = Inl a
let coprod_inr b = Inr b

(* Universality: any cocone factors through the coproduct *)
let coprod_elim f g = function Inl a -> f a | Inr b -> g b

(* Equaliser = limit of parallel arrows *)
(* { x | f(x) = g(x) } *)
let equaliser f g lst = List.filter (fun x -> f x = g x) lst

(* Coequaliser = colimit *)
(* Quotient: identify elements where f(x) = f(y) -> same class *)
let coequaliser f lst =
  let module M = Map.Make(Int) in
  List.fold_left (fun m x -> M.add (f x) x m) M.empty lst
  |> M.bindings |> List.map snd

let () =
  Printf.printf "product (3,4): fst=%d snd=%d\n" (fst (3,4)) (snd (3,4));

  let both = pair_of (fun x -> x * 2) (fun x -> x + 10) in
  Printf.printf "pair_of: %s\n" (let (a,b) = both 5 in Printf.sprintf "(%d,%d)" a b);

  let sum_or_str = coprod_elim string_of_int (fun s -> s ^ "!") in
  Printf.printf "coprod Inl 42: %s\n" (sum_or_str (Inl 42));
  Printf.printf "coprod Inr hi: %s\n" (sum_or_str (Inr "hi"));

  let eq = equaliser (fun x -> x mod 2) (fun x -> x mod 3) [0;1;2;3;4;5;6;12] in
  Printf.printf "equaliser mod2=mod3: [%s]\n" (eq |> List.map string_of_int |> String.concat ";")
