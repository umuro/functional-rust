(* Perfect Numbers — Classification *)

type classification = Perfect | Abundant | Deficient | Invalid

(* Version 1: Simple divisor sum *)
let sum_of_divisors n =
  List.init (n - 1) (fun i -> i + 1)
  |> List.filter (fun d -> n mod d = 0)
  |> List.fold_left (+) 0

let classify n =
  if n <= 0 then Invalid
  else
    let s = sum_of_divisors n in
    if s = n then Perfect
    else if s > n then Abundant
    else Deficient

(* Version 2: Optimized with sqrt bound *)
let sum_of_divisors_fast n =
  if n <= 1 then (if n = 1 then 1 else 0)
  else
    let sum = ref 1 in
    let i = ref 2 in
    while !i * !i <= n do
      if n mod !i = 0 then begin
        sum := !sum + !i;
        if !i <> n / !i then sum := !sum + n / !i
      end;
      incr i
    done;
    !sum

let () =
  assert (classify 6 = Perfect);
  assert (classify 28 = Perfect);
  assert (classify 12 = Abundant);
  assert (classify 7 = Deficient);
  assert (classify (-1) = Invalid)
