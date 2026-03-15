(* 350: Oneshot Channel
   Send exactly one value from one domain to another.
   Built on a Mutex + Condition, matching Rust's oneshot pattern. *)

type 'a oneshot = {
  mutex : Mutex.t;
  cond  : Condition.t;
  value : 'a option ref;
}

type 'a sender   = { state : 'a oneshot }
type 'a receiver = { state : 'a oneshot }

let make_oneshot () =
  let state = { mutex = Mutex.create (); cond = Condition.create (); value = ref None } in
  ({ state }, { state })

let send tx v =
  Mutex.lock tx.state.mutex;
  tx.state.value := Some v;
  Condition.signal tx.state.cond;
  Mutex.unlock tx.state.mutex

(* Block until a value arrives *)
let recv rx =
  Mutex.lock rx.state.mutex;
  while !(rx.state.value) = None do
    Condition.wait rx.state.cond rx.state.mutex
  done;
  let v = Option.get !(rx.state.value) in
  Mutex.unlock rx.state.mutex;
  v

(* Non-blocking: returns None if not yet sent *)
let try_recv rx =
  Mutex.lock rx.state.mutex;
  let v = !(rx.state.value) in
  Mutex.unlock rx.state.mutex;
  v

let () =
  (* Same-domain send/recv *)
  let (tx, rx) = make_oneshot () in
  send tx 42;
  assert (recv rx = 42);
  Printf.printf "oneshot: sent and received 42\n%!";

  (* Cross-domain *)
  let (tx2, rx2) = make_oneshot () in
  let d = Domain.spawn (fun () -> send tx2 "hello") in
  let v = recv rx2 in
  Domain.join d;
  assert (v = "hello");
  Printf.printf "cross-domain oneshot: %s\n%!" v;

  (* try_recv before send returns None *)
  let (_, rx3) = make_oneshot () in
  assert (try_recv rx3 = None);
  Printf.printf "try_recv before send: None\n%!"
