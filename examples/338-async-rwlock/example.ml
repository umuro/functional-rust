(* OCaml: No stdlib RwLock, simulate with Mutex *)

(* OCaml doesn't have a built-in RwLock.
   For read-heavy workloads, you'd use:
   - A regular Mutex (simple but less concurrent)
   - Lwt_rwlock from the Lwt library
   - A custom implementation
*)

let shared_db () =
  let data = Hashtbl.create 16 in
  let m = Mutex.create () in
  let read k =
    Mutex.lock m;
    let v = Hashtbl.find_opt data k in
    Mutex.unlock m;
    v
  in
  let write k v =
    Mutex.lock m;
    Hashtbl.replace data k v;
    Mutex.unlock m
  in
  (read, write)

let () =
  let (read, write) = shared_db () in
  write "x" 10;
  match read "x" with
  | Some v -> Printf.printf "x = %d\n" v
  | None -> Printf.printf "x not found\n"
