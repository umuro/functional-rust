let rec sum_naive = function
  | []     -> 0
  | h :: t -> h + sum_naive t

let sum_tr lst =
  let rec go acc = function
    | []     -> acc
    | h :: t -> go (acc + h) t
  in go 0 lst

let rev_tr lst =
  let rec go acc = function
    | []     -> acc
    | h :: t -> go (h :: acc) t
  in go [] lst

let fib_tr n =
  let rec go a b = function
    | 0 -> a
    | n -> go b (a + b) (n - 1)
  in go 0 1 n

let () =
  let big = List.init 100_000 (fun i -> i + 1) in
  assert (sum_tr big = 5000050000);
  assert (List.hd (rev_tr big) = 100000);
  assert (fib_tr 10 = 55);
  assert (fib_tr 40 = 102334155);
  print_endline "All assertions passed."
