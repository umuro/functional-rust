(* Coyoneda: the free functor. Wraps a value and defers the mapping.
   Coyoneda f a = exists b. f b * (b -> a)
   Any type constructor becomes a functor for free via Coyoneda. *)

(* Existential Coyoneda *)
type 'f coyoneda_inj = {
  inject: 'b 'a. ('b -> 'a) -> 'b -> 'f
}

(* Using GADT to pack existential *)
type _ coyoneda =
  | Coyoneda : ('b -> 'a) * 'b list -> 'a coyoneda

(* Constructor *)
let lift lst = Coyoneda ((fun x -> x), lst)

(* fmap: compose the function, don't traverse yet *)
let fmap f (Coyoneda (g, lst)) = Coyoneda ((fun x -> f (g x)), lst)

(* Lower: apply the accumulated function *)
let lower (Coyoneda (f, lst)) = List.map f lst

let () =
  let colist = lift [1; 2; 3; 4; 5] in

  (* Fuse three maps — they compose, only one traversal at lower *)
  let result =
    colist
    |> fmap (fun x -> x * 2)
    |> fmap (fun x -> x + 1)
    |> fmap string_of_int
    |> lower
  in
  Printf.printf "Coyoneda fused: [%s]\n" (String.concat ";" result);

  (* No functor constraint needed — works for any type constructor *)
  Printf.printf "Coyoneda: deferred functor mapping\n"
