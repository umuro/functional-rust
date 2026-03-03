(* Yoneda lemma: Nat(Hom(A,-), F) ≅ F(A)
   In Haskell/OCaml: (forall b. (a -> b) -> f b) ≅ f a
   The "free theorem" that justifies many optimizations. *)

(* The Yoneda embedding *)
type ('a, 'f) yoneda = { run : 'b. ('a -> 'b) -> 'f }

(* Yoneda for list functor *)
type 'a yoneda_list = { run_list : 'b. ('a -> 'b) -> 'b list }

(* Convert list to yoneda *)
let to_yoneda : 'a list -> 'a yoneda_list =
  fun lst -> { run_list = (fun f -> List.map f lst) }

(* Convert yoneda back to list *)
let from_yoneda : 'a yoneda_list -> 'a list =
  fun y -> y.run_list (fun x -> x)  (* use identity *)

(* fmap for free! — no actual mapping happens until runYoneda *)
let fmap_yoneda f y =
  { run_list = (fun g -> y.run_list (fun a -> g (f a)) ) }

let () =
  let lst = [1; 2; 3; 4; 5] in
  let y = to_yoneda lst in

  (* Fuse multiple maps — only traverses once! *)
  let y' = fmap_yoneda (fun x -> x + 1) (fmap_yoneda (fun x -> x * 2) y) in
  let result = from_yoneda y' in
  Printf.printf "Yoneda fused maps: [%s]\n"
    (result |> List.map string_of_int |> String.concat ";");

  (* Same result as direct maps *)
  let direct = List.map (fun x -> x * 2 + 1) lst in
  assert (result = direct);
  Printf.printf "Yoneda ≅ direct: same result\n"
