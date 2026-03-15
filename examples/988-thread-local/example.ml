(* 988: Thread-Local Storage *)
(* OCaml 5: Domain.DLS (domain-local storage). OCaml < 5: Thread.self() map *)

(* --- Approach 1: Simulate thread-local via Thread.self() hash table --- *)

let tls : (int, int ref) Hashtbl.t = Hashtbl.create 16
let tls_m = Mutex.create ()

let get_tls () =
  let id = Thread.id (Thread.self ()) in
  Mutex.lock tls_m;
  let v = match Hashtbl.find_opt tls id with
    | Some r -> r
    | None -> let r = ref 0 in Hashtbl.add tls id r; r
  in
  Mutex.unlock tls_m;
  v

let set_tls v =
  let cell = get_tls () in
  cell := v

let read_tls () = !(get_tls ())

let () =
  let results = ref [] in
  let m = Mutex.create () in
  let threads = List.init 5 (fun i ->
    Thread.create (fun () ->
      set_tls (i * 10);
      (* Other thread's changes don't affect ours *)
      Thread.yield ();
      let v = read_tls () in
      Mutex.lock m;
      results := v :: !results;
      Mutex.unlock m
    ) ()
  ) in
  List.iter Thread.join threads;
  let sorted = List.sort compare !results in
  assert (sorted = [0; 10; 20; 30; 40]);
  Printf.printf "Approach 1 (thread-local): [%s]\n"
    (String.concat "; " (List.map string_of_int sorted))

(* --- Approach 2: Per-thread accumulator (independent state) --- *)

let () =
  let all_sums = ref [] in
  let m = Mutex.create () in
  let threads = List.init 4 (fun id ->
    Thread.create (fun () ->
      (* Each thread accumulates independently *)
      let local_sum = ref 0 in
      for i = 1 to 10 do
        local_sum := !local_sum + i * id
      done;
      Mutex.lock m;
      all_sums := !local_sum :: !all_sums;
      Mutex.unlock m
    ) ()
  ) in
  List.iter Thread.join threads;
  (* sum of: 0, 55, 110, 165 = 330 *)
  let total = List.fold_left (+) 0 !all_sums in
  assert (total = 330);
  Printf.printf "Approach 2 (per-thread sum): total=%d\n" total

let () = Printf.printf "✓ All tests passed\n"
