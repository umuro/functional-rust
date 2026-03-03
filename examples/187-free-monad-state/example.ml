(* State monad encoded as a free monad.
   No mutation — the state is threaded through the interpreter. *)

type 's state_f =
  | Get : ('s -> 'a) -> 'a state_f
  | Put : 's * 'a -> 'a state_f

type 'a free =
  | Pure : 'a -> 'a free
  | Free : 'a free state_f -> 'a free

let get ()   = Free (Get  (fun s -> Pure s))
let put s    = Free (Put  (s, Pure ()))

let rec bind m f = match m with
  | Pure x -> f x
  | Free (Get cont)    -> Free (Get  (fun s -> bind (cont s) f))
  | Free (Put (s, n))  -> Free (Put  (s, bind n f))

let run init program =
  let rec go s = function
    | Pure x          -> (x, s)
    | Free (Get cont) -> go s (cont s)
    | Free (Put (s', next)) -> go s' next
  in go init program

let () =
  (* Increment counter 3 times *)
  let program =
    bind (get ()) (fun n ->
    bind (put (n + 1)) (fun () ->
    bind (get ()) (fun n ->
    bind (put (n + 1)) (fun () ->
    bind (get ()) (fun n ->
    bind (put (n + 1)) (fun () ->
    bind (get ()) (fun final ->
    Pure final)))))))
  in
  let (result, final_state) = run 0 program in
  Printf.printf "result=%d state=%d\n" result final_state;
  assert (result = 3 && final_state = 3);
  Printf.printf "State free monad works\n"
