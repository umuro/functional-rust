(* Direct recursion *)
let rec fib_naive = function
  | 0 -> 0 | 1 -> 1
  | n -> fib_naive (n-1) + fib_naive (n-2)

(* Tail-recursive with accumulator *)
let fib_tail n =
  let rec go a b = function
    | 0 -> a
    | n -> go b (a + b) (n - 1)
  in go 0 1 n

(* Using fold *)
let fib_fold n =
  let a, _ = List.init n Fun.id
    |> List.fold_left (fun (a, b) _ -> (b, a + b)) (0, 1)
  in a

let () =
  for i = 0 to 10 do
    assert (fib_naive i = fib_tail i);
    assert (fib_naive i = fib_fold i);
  done;
  assert (fib_tail 10 = 55);
  print_endline "ok"
