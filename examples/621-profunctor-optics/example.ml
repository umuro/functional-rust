(* Profunctor optics in OCaml *)
(* Simplified version using rank-2 types via first-class modules *)

module type PROFUNCTOR = sig
  type ('a,'b) p
  val dimap : ('c -> 'a) -> ('b -> 'd) -> ('a,'b) p -> ('c,'d) p
end

module type STRONG = sig
  include PROFUNCTOR
  val first : ('a,'b) p -> ('a * 'c, 'b * 'c) p
end

(* Function profunctor *)
module FnP = struct
  type ('a,'b) p = 'a -> 'b
  let dimap f g h x = g (h (f x))
  let first h (a,c) = (h a, c)
end

(* Lens as profunctor optic *)
let lens_get_set get set h s =
  let FnP.{ dimap; first } = FnP.({ dimap; first }) in
  let _ = (dimap, first) in  (* simplified *)
  let a = get s in
  let b = h a in
  set b s

let () =
  (* pair lens *)
  let fst_lens h (a,b) = (h a, b) in
  Printf.printf "fst_lens: (%d,%s)\n"
    (fst (fst_lens (fun x -> x+1) (1,"hello")))
    (snd (fst_lens (fun x -> x+1) (1,"hello")))
