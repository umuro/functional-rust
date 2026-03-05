(* Profunctor in OCaml *)

module type PROFUNCTOR = sig
  type ('a, 'b) t
  val dimap : ('c -> 'a) -> ('b -> 'd) -> ('a, 'b) t -> ('c, 'd) t
end

(* Function as profunctor *)
module FunProfunctor : PROFUNCTOR with type ('a, 'b) t = 'a -> 'b = struct
  type ('a, 'b) t = 'a -> 'b
  let dimap pre post f = fun c -> post (f (pre c))
end

let lmap pre f = FunProfunctor.dimap pre Fun.id f
let rmap post f = FunProfunctor.dimap Fun.id post f

let () =
  let open FunProfunctor in
  let f = string_of_int in
  let g = dimap String.length (fun s -> String.length s) f in
  Printf.printf "Result: %d\n" (g "hello")
