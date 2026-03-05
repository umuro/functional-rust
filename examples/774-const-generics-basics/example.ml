(* Const generics concept in OCaml
   OCaml doesn't have const generics directly — we simulate with modules and phantom types *)

(* ── Module-level "const" size ───────────────────────────────────────────────── *)
module type SIZE = sig val n : int end

module Three : SIZE = struct let n = 3 end
module Four  : SIZE = struct let n = 4 end

(* ── Fixed-size vector functor ────────────────────────────────────────────────── *)
module FixedVec (S : SIZE) = struct
  type t = float array

  let create init = Array.make S.n init

  let sum v =
    assert (Array.length v = S.n);
    Array.fold_left ( +. ) 0.0 v

  let dot a b =
    assert (Array.length a = S.n && Array.length b = S.n);
    Array.init S.n (fun i -> a.(i) *. b.(i))
    |> Array.fold_left ( +. ) 0.0

  let size = S.n
end

module Vec3 = FixedVec(Three)
module Vec4 = FixedVec(Four)

let () =
  let a = [|1.0; 2.0; 3.0|] in
  let b = [|4.0; 5.0; 6.0|] in
  Printf.printf "Vec3 size: %d\n" Vec3.size;
  Printf.printf "sum a:     %.1f\n" (Vec3.sum a);
  Printf.printf "dot a.b:   %.1f\n" (Vec3.dot a b);  (* 1*4+2*5+3*6 = 32 *)

  let v4 = Vec4.create 1.0 in
  Printf.printf "Vec4 size: %d\n" Vec4.size;
  Printf.printf "sum v4:    %.1f\n" (Vec4.sum v4)
