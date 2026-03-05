(* Binary Search *)
(* Divide-and-conquer search on sorted arrays *)

let find xs value =
  let rec go lo hi =
    if lo > hi then Error "value not in array"
    else
      let mid = lo + (hi - lo) / 2 in
      if xs.(mid) < value then go (mid + 1) hi
      else if xs.(mid) > value then go lo (mid - 1)
      else Ok mid
  in
  if Array.length xs = 0 then Error "value not in array"
  else go 0 (Array.length xs - 1)
