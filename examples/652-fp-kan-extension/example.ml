(* Kan Extensions in OCaml *)

(* Right Kan Extension: Ran_K F (d) = ∫_c [D(d, Kc), F c] *)
(* Codensity = Ran_Id Id *)

module Codensity = struct
  type 'a t = { run : 'r. ('a -> 'r) -> 'r }
  
  let pure a = { run = fun k -> k a }
  
  let bind m f = { run = fun k -> m.run (fun a -> (f a).run k) }
  
  let lower m = m.run Fun.id
end

(* Left Kan Extension: Lan_K F (d) = ∫^c D(Kc, d) × F c *)
(* Density = Lan_Id Id *)

module Density = struct
  type 'a t = D : 'x * ('x -> 'a) -> 'a t
  
  let pure a = D (a, Fun.id)
  let extract (D (x, f)) = f x
end

let () =
  let open Codensity in
  let result = pure 1
    |> fun m -> bind m (fun x -> pure (x + 1))
    |> fun m -> bind m (fun x -> pure (x * 2))
    |> lower in
  Printf.printf "Codensity result: %d\n" result
