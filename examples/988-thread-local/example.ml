(* 988: Thread-Local Storage
   Per-thread data: each thread has its own independent copy.
   OCaml: Domain.DLS (Domain-Local Storage) in OCaml 5 is the idiomatic way.
   We also show the Thread + Hashtbl approach for OCaml 4 compatibility. *)

(* --- OCaml 5: Domain.DLS — true thread-local keys --- *)
let demo_dls () =
  Printf.printf "=== Domain.DLS (OCaml 5 domain-local storage) ===\n";

  (* Each domain gets its own independent copy of these values *)
  let counter_key : int Domain.DLS.key = Domain.DLS.new_key (fun () -> 0) in
  let name_key    : string Domain.DLS.key = Domain.DLS.new_key (fun () -> "unnamed") in

  let n_domains = 4 in
  let results = Array.make n_domains (0, "") in
  let mutex = Mutex.create () in

  let domains = Array.init n_domains (fun i ->
    Domain.spawn (fun () ->
      (* Each domain sets its own local values *)
      Domain.DLS.set counter_key (i * 10);
      Domain.DLS.set name_key (Printf.sprintf "domain-%d" i);
      (* Do some work — no interference between domains *)
      for _ = 1 to 5 do
        let v = Domain.DLS.get counter_key in
        Domain.DLS.set counter_key (v + 1)
      done;
      let final_count = Domain.DLS.get counter_key in
      let name = Domain.DLS.get name_key in
      Mutex.lock mutex;
      results.(i) <- (final_count, name);
      Mutex.unlock mutex
    )
  ) in
  Array.iter Domain.join domains;

  Array.iteri (fun i (cnt, name) ->
    Printf.printf "  domain %d: counter=%d name=%s (expected %d)\n"
      i cnt name (i * 10 + 5)
  ) results

(* --- Thread-local via Thread id → Hashtbl lookup (OCaml 4 compatible) --- *)
module ThreadLocal = struct
  type 'a t = {
    table : (int, 'a) Hashtbl.t;
    mutex : Mutex.t;
    init  : unit -> 'a;
  }

  let create init = { table = Hashtbl.create 8; mutex = Mutex.create (); init }

  let get tl =
    let tid = Thread.id (Thread.self ()) in
    Mutex.lock tl.mutex;
    let v = match Hashtbl.find_opt tl.table tid with
      | Some v -> v
      | None ->
        let v = tl.init () in
        Hashtbl.add tl.table tid v;
        v
    in
    Mutex.unlock tl.mutex;
    v

  let set tl v =
    let tid = Thread.id (Thread.self ()) in
    Mutex.lock tl.mutex;
    Hashtbl.replace tl.table tid v;
    Mutex.unlock tl.mutex

  let modify tl f =
    let v = get tl in
    set tl (f v)
end

let demo_thread_local () =
  Printf.printf "\n=== ThreadLocal (Hashtbl-based, OCaml 4 compatible) ===\n";

  let local_count = ThreadLocal.create (fun () -> 0) in
  let local_buf   = ThreadLocal.create (fun () -> Buffer.create 16) in

  let results = ref [] in
  let mutex = Mutex.create () in

  let threads = List.init 4 (fun i ->
    Thread.create (fun () ->
      (* Each thread independently counts and buffers *)
      for j = 1 to 5 do
        ThreadLocal.modify local_count (fun c -> c + 1);
        let buf = ThreadLocal.get local_buf in
        Buffer.add_string buf (Printf.sprintf "t%d.%d " i j)
      done;
      let count = ThreadLocal.get local_count in
      let buf_str = Buffer.contents (ThreadLocal.get local_buf) in
      Mutex.lock mutex;
      results := (i, count, buf_str) :: !results;
      Mutex.unlock mutex
    ) ()
  ) in
  List.iter Thread.join threads;

  List.sort (fun (a,_,_) (b,_,_) -> compare a b) !results
  |> List.iter (fun (i, cnt, buf) ->
    Printf.printf "  thread %d: count=%d buf=%s\n" i cnt buf)

let () =
  demo_dls ();
  demo_thread_local ();

  Printf.printf "\n=== Thread-local random state (practical use case) ===\n";
  (* Each thread gets its own seeded Random.State so results are reproducible *)
  let tl_rand = ThreadLocal.create (fun () ->
    Random.State.make [| Thread.id (Thread.self ()) |]
  ) in

  let samples = ref [] in
  let mutex = Mutex.create () in
  let threads = List.init 3 (fun tid ->
    Thread.create (fun () ->
      let state = ThreadLocal.get tl_rand in
      let nums = List.init 3 (fun _ -> Random.State.int state 100) in
      Mutex.lock mutex;
      samples := (tid, nums) :: !samples;
      Mutex.unlock mutex
    ) ()
  ) in
  List.iter Thread.join threads;
  List.sort (fun (a,_) (b,_) -> compare a b) !samples
  |> List.iter (fun (tid, nums) ->
    Printf.printf "  thread %d: %s\n" tid
      (String.concat " " (List.map string_of_int nums)))
