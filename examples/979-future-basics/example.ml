(* 979: Future/Promise Basics *)
(* OCaml: Lwt monad concept shown with pure Option/Result monads *)
(* We model the Future/Promise monad pattern without external deps *)

(* --- Approach 1: Model Future as a lazy thunk (pure simulation) --- *)

type 'a future = unit -> 'a

let return_ x : 'a future = fun () -> x

let bind (f : 'a future) (k : 'a -> 'b future) : 'b future =
  fun () -> k (f ()) ()

let map (f : 'a -> 'b) (fut : 'a future) : 'b future =
  fun () -> f (fut ())

let run fut = fut ()

let () =
  let fut = return_ 42 in
  let fut2 = bind fut (fun x -> return_ (x + 1)) in
  let fut3 = map (fun x -> x * 2) fut2 in
  assert (run fut3 = 86);
  Printf.printf "Approach 1 (lazy thunk future): %d\n" (run fut3)

(* --- Approach 2: Future as Result monad (error-aware) --- *)

type ('a, 'e) result_future = unit -> ('a, 'e) result

let ok x : ('a, 'e) result_future = fun () -> Ok x
let err e : ('a, 'e) result_future = fun () -> Error e

let bind_r (f : ('a, 'e) result_future) (k : 'a -> ('b, 'e) result_future) : ('b, 'e) result_future =
  fun () -> match f () with
    | Ok v -> k v ()
    | Error e -> Error e

let () =
  let computation =
    bind_r (ok 10) (fun x ->
    bind_r (ok 20) (fun y ->
    ok (x + y)))
  in
  (match computation () with
  | Ok v -> assert (v = 30); Printf.printf "Approach 2 (result future): %d\n" v
  | Error _ -> assert false)

(* --- Approach 3: Promise with state (mutable cell) --- *)

type 'a promise_state = Pending | Resolved of 'a

type 'a promise = { mutable state : 'a promise_state }

let make_promise () = { state = Pending }

let resolve p v = p.state <- Resolved v

let await p =
  match p.state with
  | Resolved v -> v
  | Pending -> failwith "promise not yet resolved"

let () =
  let p = make_promise () in
  resolve p 99;
  let v = await p in
  assert (v = 99);
  Printf.printf "Approach 3 (promise state): %d\n" v

let () = Printf.printf "✓ All tests passed\n"
