(* Example 070: Continuation Monad *)
(* CPS: continuation-passing style *)

(* Approach 1: Basic CPS transforms *)
let add_cps x y k = k (x + y)
let mul_cps x y k = k (x * y)
let square_cps x k = k (x * x)

(* Chain: (3 + 4) * 2 *)
let example1 k = add_cps 3 4 (fun sum -> mul_cps sum 2 k)

(* Approach 2: Cont monad *)
type ('a, 'r) cont = Cont of (('a -> 'r) -> 'r)

let run_cont (Cont f) k = f k
let return_ x = Cont (fun k -> k x)
let bind (Cont m) f = Cont (fun k ->
  m (fun a -> run_cont (f a) k))
let ( >>= ) = bind

(* callcc: capture the current continuation *)
let callcc f = Cont (fun k ->
  run_cont (f (fun a -> Cont (fun _ -> k a))) k)

(* Approach 3: Early exit with callcc *)
let find_first_negative xs =
  callcc (fun exit ->
    List.fold_left (fun acc x ->
      acc >>= fun _ ->
      if x < 0 then exit (Some x)
      else return_ None
    ) (return_ None) xs
  )

let safe_div_cont x y =
  callcc (fun exit ->
    if y = 0 then exit (Error "Division by zero")
    else return_ (Ok (x / y))
  )

let () =
  (* Basic CPS *)
  example1 (fun result -> assert (result = 14));
  square_cps 5 (fun result -> assert (result = 25));

  (* Cont monad *)
  let result = run_cont (
    return_ 5 >>= fun x ->
    return_ (x * 2) >>= fun y ->
    return_ (y + 1)
  ) Fun.id in
  assert (result = 11);

  (* callcc: early exit *)
  let result = run_cont (find_first_negative [1; 2; -3; 4]) Fun.id in
  assert (result = Some (-3));

  let result = run_cont (find_first_negative [1; 2; 3]) Fun.id in
  assert (result = None);

  let result = run_cont (safe_div_cont 10 2) Fun.id in
  assert (result = Ok 5);
  let result = run_cont (safe_div_cont 10 0) Fun.id in
  assert (result = Error "Division by zero");

  Printf.printf "✓ All tests passed\n"
