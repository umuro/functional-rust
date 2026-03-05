(* Prime Factors *)
(* Integer factorization by trial division *)

let factors_of n =
  let rec go n factor acc =
    if n <= 1L then List.rev acc
    else if Int64.rem n factor = 0L then
      go (Int64.div n factor) factor (factor :: acc)
    else
      go n (Int64.add factor 1L) acc
  in
  go n 2L []
