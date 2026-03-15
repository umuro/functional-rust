(* 1052: Fibonacci Bottom-Up DP
   Three approaches: array DP, O(1) space with two vars, and fold. *)

(* Approach 1: Array-based bottom-up DP *)
let fib_array n =
  if n <= 1 then n
  else begin
    let dp = Array.make (n + 1) 0 in
    dp.(1) <- 1;
    for i = 2 to n do
      dp.(i) <- dp.(i-1) + dp.(i-2)
    done;
    dp.(n)
  end

(* Approach 2: O(1) space — two variables *)
let fib_const n =
  if n <= 1 then n
  else
    let a = ref 0 and b = ref 1 in
    for _ = 2 to n do
      let t = !a + !b in
      a := !b;
      b := t
    done;
    !b

(* Approach 3: Purely functional with List.fold_left — no mutation *)
let fib_fold n =
  if n <= 1 then n
  else
    let (_, b) =
      List.fold_left
        (fun (a, b) _ -> (b, a + b))
        (0, 1)
        (List.init (n - 1) (fun _ -> ()))
    in
    b

(* Approach 4: Generate first N Fibonacci numbers via unfold *)
let fib_list n =
  (* Iteratively build the list of the first n Fibonacci numbers *)
  let rec go i a b acc =
    if i = 0 then List.rev acc
    else go (i - 1) b (a + b) (a :: acc)
  in
  go n 0 1 []

let () =
  let cases = [(0,0);(1,1);(2,1);(5,5);(10,55);(20,6765);(30,832040)] in

  List.iter (fun (n, expected) ->
    assert (fib_array n = expected);
    assert (fib_const n = expected);
    assert (fib_fold  n = expected)
  ) cases;

  (* First 10 Fibonacci numbers *)
  let first10 = fib_list 10 in
  assert (first10 = [0;1;1;2;3;5;8;13;21;34]);

  Printf.printf "All fibonacci-dp tests passed.\n"
