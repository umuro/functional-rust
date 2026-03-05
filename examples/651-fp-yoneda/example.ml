(* Yoneda Lemma in OCaml *)

(* Yoneda f a ≅ forall b. (a -> b) -> f b *)
module type YONEDA = sig
  type 'a f
  type 'a t
  val lift : 'a f -> 'a t
  val lower : 'a t -> 'a f
  val map : ('a -> 'b) -> 'a t -> 'b t
end

(* Yoneda for list - enables map fusion *)
module YonedaList = struct
  type 'a t = { run : 'r. ('a -> 'r) -> 'r list }
  
  let lift xs = { run = fun f -> List.map f xs }
  let lower y = y.run Fun.id
  let map f y = { run = fun g -> y.run (fun x -> g (f x)) }
end

(* Coyoneda - dual construction *)
type ('a, 'b) coyoneda = Coyoneda : 'x * ('x -> 'b) -> ('a, 'b) coyoneda

let lift_coyo x = Coyoneda (x, Fun.id)
let map_coyo f (Coyoneda (x, g)) = Coyoneda (x, fun y -> f (g y))
let lower_coyo (Coyoneda (x, f)) = f x

let () =
  let open YonedaList in
  let result = [1; 2; 3; 4; 5]
    |> lift
    |> map (fun x -> x + 1)
    |> map (fun x -> x * 2)
    |> lower in
  List.iter (Printf.printf "%d ") result
