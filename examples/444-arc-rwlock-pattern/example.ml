(* 444. RwLock pattern – OCaml *)
(* OCaml stdlib has no RwLock; simulate with Mutex *)
let config = ref [("host","localhost");("port","8080")]
let mutex = Mutex.create ()

let read_config k =
  Mutex.lock mutex;
  let v = List.assoc_opt k !config in
  Mutex.unlock mutex; v

let write_config k v =
  Mutex.lock mutex;
  config := (k,v) :: List.filter (fun (a,_) -> a<>k) !config;
  Mutex.unlock mutex

let () =
  let readers = List.init 3 (fun _ ->
    Thread.create (fun () ->
      for _ = 1 to 2 do
        let h = Option.value ~default:"?" (read_config "host") in
        Printf.printf "host=%s\n%!" h
      done) ()
  ) in
  let writer = Thread.create (fun () ->
    Thread.delay 0.01; write_config "host" "example.com"
  ) () in
  List.iter Thread.join readers; Thread.join writer;
  Printf.printf "final: %s\n" (Option.value ~default:"?" (read_config "host"))
