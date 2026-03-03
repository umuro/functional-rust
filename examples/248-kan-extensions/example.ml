(* Kan extensions: the most general way to extend functors.
   Right Kan extension: Ran K F a = forall b. (a -> K b) -> F b
   Left Kan extension:  Lan K F a = exists b. K b * (F b -> a)
   Every concept in category theory is a Kan extension! *)

(* Right Kan extension (codensity) of the identity functor *)
(* Ran Id F a = forall b. (a -> b) -> F b
   This is the Yoneda lemma! *)

(* Codensity monad: Ran F F
   type 'a codensity = { run : 'b. ('a -> F 'b) -> F 'b } *)
type 'a codensity = { run : 'b. ('a -> 'b list) -> 'b list }

let return_c x = { run = (fun k -> k x) }

let bind_c m f = { run = (fun k -> m.run (fun a -> (f a).run k)) }

(* Lift a list computation *)
let lift_c lst = { run = (fun k -> List.concat_map k lst) }

(* Lower back to list *)
let lower_c c = c.run (fun x -> [x])

let () =
  (* Codensity improves asymptotic complexity of left-nested binds *)
  let program =
    bind_c (lift_c [1; 2; 3]) (fun x ->
    bind_c (lift_c [10; 20])  (fun y ->
    return_c (x + y)))
  in
  let result = lower_c program in
  Printf.printf "codensity: [%s]\n"
    (result |> List.map string_of_int |> String.concat ";");

  (* Verify same as direct computation *)
  let direct = List.concat_map (fun x ->
    List.map (fun y -> x + y) [10; 20]) [1; 2; 3] in
  assert (result = direct);
  Printf.printf "Kan extension (codensity) = direct computation\n"
