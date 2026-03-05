(* OCaml: Lwt-style promise with resolver *)

(* Lwt version:
let (promise, resolver) = Lwt.wait () in
Lwt.wakeup resolver 42;
Lwt.bind promise (fun v -> Printf.printf "Got: %d\n" v; Lwt.return ())
*)

(* Sync simulation *)
type 'a shared_state = {
  mutable value: 'a option;
  mutable waiters: (unit -> unit) list;
}

let make_promise () =
  let state = { value = None; waiters = [] } in
  let get () = state.value in
  let fulfill v =
    state.value <- Some v;
    List.iter (fun f -> f ()) state.waiters
  in
  let add_waiter f = state.waiters <- f :: state.waiters in
  (get, fulfill, add_waiter)

let () =
  let (get, fulfill, _) = make_promise () in
  fulfill 42;
  match get () with
  | Some v -> Printf.printf "Got: %d\n" v
  | None -> Printf.printf "Not ready\n"
