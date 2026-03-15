(* Example 062: Writer Monad *)
(* Accumulate a log alongside computation results *)

type ('w, 'a) writer = Writer of ('a * 'w)

let run_writer (Writer (a, w)) = (a, w)
let return_ x = Writer (x, [])
let bind (Writer (a, w1)) f =
  let Writer (b, w2) = f a in
  Writer (b, w1 @ w2)
let ( >>= ) = bind
let tell w = Writer ((), [w])

(* Approach 1: Logging computation steps *)
let add_with_log x y =
  tell (Printf.sprintf "Adding %d + %d" x y) >>= fun () ->
  let sum = x + y in
  tell (Printf.sprintf "Result: %d" sum) >>= fun () ->
  return_ sum

let multiply_with_log x y =
  tell (Printf.sprintf "Multiplying %d * %d" x y) >>= fun () ->
  return_ (x * y)

let computation =
  add_with_log 3 4 >>= fun sum ->
  multiply_with_log sum 2 >>= fun product ->
  tell "Done!" >>= fun () ->
  return_ product

(* Approach 2: Writer with monoid (string concatenation) *)
type 'a str_writer = StrWriter of ('a * string)

let str_tell s = StrWriter ((), s)
let str_return x = StrWriter (x, "")
let str_bind (StrWriter (a, w1)) f =
  let StrWriter (b, w2) = f a in
  StrWriter (b, w1 ^ w2)

(* Approach 3: Writer for collecting values *)
let collect x = Writer ((), [x])

let gather_evens xs =
  List.fold_left (fun acc x ->
    acc >>= fun () ->
    if x mod 2 = 0 then collect x
    else return_ ()
  ) (return_ ()) xs

let () =
  let (result, log) = run_writer computation in
  assert (result = 14);
  assert (List.length log = 3);
  assert (List.hd log = "Adding 3 + 4");

  let ((), evens) = run_writer (gather_evens [1;2;3;4;5;6]) in
  assert (evens = [2; 4; 6]);

  Printf.printf "✓ All tests passed\n"
