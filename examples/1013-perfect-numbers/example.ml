(* Perfect Numbers *)
(* Number classification by aliquot sum — Result type *)

let classify n =
  let aliquot = function
    | 1 -> 0
    | n when n > 1 ->
      let rec sum_factors acc factor =
        if factor > n / 2 then acc
        else if n mod factor = 0 then sum_factors (acc + factor) (factor + 1)
        else sum_factors acc (factor + 1)
      in sum_factors 0 1
    | _ -> 0
  in
  if n < 1 then Error "Classification is only possible for positive integers."
  else
    let s = aliquot n in
    if s = n then Ok "perfect"
    else if s > n then Ok "abundant"
    else Ok "deficient"
