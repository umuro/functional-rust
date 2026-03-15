(* 1052: Fibonacci Bottom-Up DP with O(1) Space *)

(* Approach 1: Array-based bottom-up DP *)
let fib_array n =
  if n <= 1 then n
  else begin
    let dp = Array.make (n + 1) 0 in
    dp.(1) <- 1;
    for i = 2 to n do
      dp.(i) <- dp.(i - 1) + dp.(i - 2)
    done;
    dp.(n)
  end

(* Approach 2: O(1) space — two variables *)
let fib_const n =
  if n <= 1 then n
  else begin
    let a = ref 0 in
    let b = ref 1 in
    for _ = 2 to n do
      let t = !a + !b in
      a := !b;
      b := t
    done;
    !b
  end

(* Approach 3: Functional fold with tuple *)
let fib_fold n =
  if n <= 1 then n
  else
    let (_, b) =
      List.init (n - 1) Fun.id
      |> List.fold_left (fun (a, b) _ -> (b, a + b)) (0, 1)
    in
    b

let () =
  List.iter (fun (n, expected) ->
    assert (fib_array n = expected);
    assert (fib_const n = expected);
    assert (fib_fold n = expected)
  ) [(0, 0); (1, 1); (2, 1); (5, 5); (10, 55); (20, 6765); (30, 832040)];
  Printf.printf "✓ All tests passed\n"
