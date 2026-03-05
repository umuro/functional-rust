(* OCaml: timeout with Lwt *)

(* With Lwt:
let with_timeout timeout f =
  Lwt.pick [
    f ();
    Lwt_unix.sleep timeout >>= fun () -> Lwt.fail Timeout
  ]
*)

(* Sync simulation with threads *)
let with_timeout_sync timeout_sec f =
  let result = ref None in
  let done_flag = ref false in
  let t = Thread.create (fun () ->
    result := Some (f ());
    done_flag := true
  ) () in
  let start = Unix.gettimeofday () in
  while not !done_flag && Unix.gettimeofday () -. start < timeout_sec do
    Thread.delay 0.001
  done;
  !result

let () =
  match with_timeout_sync 0.1 (fun () -> Thread.delay 0.02; 42) with
  | Some v -> Printf.printf "Got: %d\n" v
  | None -> Printf.printf "Timeout\n"
