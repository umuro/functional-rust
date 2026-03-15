(* Example 060: State Monad *)
(* Thread state through computations without explicit passing *)

(* State monad: 'a state = State of (state -> 'a * state) *)
type ('s, 'a) state = State of ('s -> 'a * 's)

let run_state (State f) s = f s
let return_ x = State (fun s -> (x, s))
let bind m f = State (fun s ->
  let (a, s') = run_state m s in
  run_state (f a) s')
let ( >>= ) = bind

let get = State (fun s -> (s, s))
let put s = State (fun _ -> ((), s))
let modify f = State (fun s -> ((), f s))

(* Approach 1: Counter *)
let tick = get >>= fun n -> put (n + 1) >>= fun () -> return_ n

let count3 =
  tick >>= fun a ->
  tick >>= fun b ->
  tick >>= fun c ->
  return_ (a, b, c)

(* Approach 2: Stack operations *)
let push x = modify (fun stack -> x :: stack)
let pop = get >>= fun stack ->
  match stack with
  | [] -> return_ None
  | x :: rest -> put rest >>= fun () -> return_ (Some x)

let stack_ops =
  push 1 >>= fun () ->
  push 2 >>= fun () ->
  push 3 >>= fun () ->
  pop >>= fun a ->
  pop >>= fun b ->
  return_ (a, b)

(* Approach 3: Label generator *)
let fresh_label prefix =
  get >>= fun n ->
  put (n + 1) >>= fun () ->
  return_ (Printf.sprintf "%s_%d" prefix n)

let gen_labels =
  fresh_label "var" >>= fun a ->
  fresh_label "tmp" >>= fun b ->
  fresh_label "var" >>= fun c ->
  return_ [a; b; c]

let () =
  let (result, final_state) = run_state count3 0 in
  assert (result = (0, 1, 2));
  assert (final_state = 3);

  let ((a, b), final_stack) = run_state stack_ops [] in
  assert (a = Some 3);
  assert (b = Some 2);
  assert (final_stack = [1]);

  let (labels, _) = run_state gen_labels 0 in
  assert (labels = ["var_0"; "tmp_1"; "var_2"]);

  Printf.printf "✓ All tests passed\n"
