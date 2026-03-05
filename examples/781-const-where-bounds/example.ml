(* Where bounds on const parameters — OCaml via functor constraints *)

(* We encode "non-zero" at the type level using a module constraint *)
module type POSITIVE = sig
  val n : int
  val proof : unit (* force explicit instantiation *)
end

(* Helper: create a POSITIVE module, failing if n <= 0 *)
let make_positive n =
  if n <= 0 then failwith (Printf.sprintf "N must be positive, got %d" n);
  (module struct let n = n let proof = () end : POSITIVE)

(* Power-of-two constraint *)
module type POW2 = sig
  val n : int
  val log2 : int
end

let is_power_of_two n = n > 0 && (n land (n - 1)) = 0

let log2 n =
  let rec go acc n = if n <= 1 then acc else go (acc + 1) (n lsr 1) in
  go 0 n

let make_pow2 n =
  if not (is_power_of_two n) then
    failwith (Printf.sprintf "%d is not a power of two" n);
  (module struct let n = n let log2 = log2 n end : POW2)

module PowerOfTwoBuf (S : POW2) = struct
  let mask = S.n - 1  (* fast modulo for power-of-two sizes *)
  let modulo i = i land mask
  let capacity = S.n
end

let () =
  let (module B8) = make_pow2 8 in
  let module Buf = PowerOfTwoBuf((val make_pow2 8 : POW2)) in
  Printf.printf "Capacity: %d, log2=%d\n" B8.n B8.log2;
  Printf.printf "modulo(10, 8) = %d\n" (Buf.modulo 10)
