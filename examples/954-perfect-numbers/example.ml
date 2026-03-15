(* 954: Perfect Numbers — Classification

   Perfect: sum of proper divisors = n
   Abundant: sum > n
   Deficient: sum < n *)

type classification =
  | Perfect
  | Abundant
  | Deficient
  | Invalid

(* ── Approach 1: Simple O(n) sum of divisors ─────────────────────────────── *)

let sum_of_divisors n =
  (* Proper divisors: 1..n-1 that divide n *)
  let rec go i acc =
    if i >= n then acc
    else if n mod i = 0 then go (i + 1) (acc + i)
    else go (i + 1) acc
  in
  if n <= 0 then 0 else go 1 0

let classify n =
  if n = 0 then Invalid
  else
    let s = sum_of_divisors n in
    match compare s n with
    | 0 -> Perfect
    | c when c > 0 -> Abundant
    | _ -> Deficient

(* ── Approach 2: O(√n) optimised ────────────────────────────────────────── *)

let sum_of_divisors_fast n =
  if n <= 1 then 0
  else begin
    (* 1 is always a proper divisor for n > 1 *)
    let sum = ref 1 in
    let i = ref 2 in
    while !i * !i <= n do
      if n mod !i = 0 then begin
        sum := !sum + !i;
        if !i <> n / !i then
          sum := !sum + (n / !i)
      end;
      incr i
    done;
    !sum
  end

(* ── Approach 3: Functional with List.init ────────────────────────────────── *)

let sum_of_divisors_func n =
  if n <= 1 then 0
  else
    List.fold_left ( + ) 0
      (List.filter (fun d -> n mod d = 0)
         (List.init (n - 1) (fun i -> i + 1)))

(* ── Find perfect numbers up to limit ────────────────────────────────────── *)

let perfect_numbers_up_to limit =
  List.filter (fun n -> classify n = Perfect)
    (List.init (limit - 1) (fun i -> i + 2))

let () =
  assert (classify 6   = Perfect);
  assert (classify 28  = Perfect);
  assert (classify 12  = Abundant);
  assert (classify 7   = Deficient);
  assert (classify 0   = Invalid);
  assert (classify 1   = Deficient);  (* sum_of_divisors 1 = 0 < 1 *)

  (* fast version matches naive for 1..100 *)
  List.iter (fun n ->
    assert (sum_of_divisors n = sum_of_divisors_fast n)
  ) (List.init 100 (fun i -> i + 1));

  (* functional version matches for small n *)
  List.iter (fun n ->
    assert (sum_of_divisors n = sum_of_divisors_func n)
  ) (List.init 50 (fun i -> i + 1));

  (* perfect numbers up to 1000 *)
  let perfects = perfect_numbers_up_to 1000 in
  assert (perfects = [6; 28; 496]);

  (* 8128 is also perfect *)
  assert (classify 8128 = Perfect);

  (* abundant: 12 = 1+2+3+4+6 = 16 > 12 *)
  assert (sum_of_divisors 12 = 16);

  print_endline "954-perfect-numbers: all tests passed"
