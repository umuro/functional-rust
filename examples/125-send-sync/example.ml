(* Example 125: Send and Sync — Thread Safety Guarantees *)

(* OCaml 5 has domains for parallelism. Before that, the GIL
   prevented true parallelism. Send/Sync concepts don't exist. *)

(* Approach 1: Thread-safe immutable data *)
let approach1 () =
  let data = [1; 2; 3; 4; 5] in
  (* In OCaml 5 with domains, immutable data is inherently safe *)
  let sum = List.fold_left ( + ) 0 data in
  assert (sum = 15);
  Printf.printf "Sum: %d (thread-safe: immutable)\n" sum

(* Approach 2: Mutex for shared mutable state *)
let approach2 () =
  let counter = Mutex.create () in
  let count = ref 0 in
  (* Simulating thread-safe mutation *)
  for _ = 1 to 10 do
    Mutex.lock counter;
    incr count;
    Mutex.unlock counter
  done;
  assert (!count = 10);
  Printf.printf "Counter: %d (mutex-protected)\n" !count

(* Approach 3: Message passing via channels *)
let approach3 () =
  (* OCaml doesn't have std channels like Rust, but we can simulate *)
  let queue = Queue.create () in
  for i = 1 to 5 do
    Queue.add i queue
  done;
  let sum = ref 0 in
  while not (Queue.is_empty queue) do
    sum := !sum + Queue.pop queue
  done;
  assert (!sum = 15);
  Printf.printf "Channel sum: %d\n" !sum

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
