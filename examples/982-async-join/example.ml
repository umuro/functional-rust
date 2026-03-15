(* 982: Join Parallel Async *)
(* OCaml: Lwt.both p1 p2 runs them "concurrently" and waits for both *)

(* --- Approach 1: Simulate Lwt.both with threads --- *)

let parallel_both f1 f2 =
  let t1 = Thread.create f1 () in
  let t2 = Thread.create f2 () in
  (* In Lwt: Lwt.both returns (v1, v2) when both resolve *)
  (* With threads, we join both *)
  Thread.join t1;
  Thread.join t2

let result1 = ref 0
let result2 = ref 0

let () =
  parallel_both
    (fun () -> result1 := 6 * 7)
    (fun () -> result2 := 10 + 20);
  assert (!result1 = 42);
  assert (!result2 = 30);
  Printf.printf "Approach 1 (parallel threads): %d, %d\n" !result1 !result2

(* --- Approach 2: Lwt.both concept — collect results via mutex --- *)

let compute_parallel tasks =
  let m = Mutex.create () in
  let results = Array.make (List.length tasks) 0 in
  let threads = List.mapi (fun i f ->
    Thread.create (fun () ->
      let v = f () in
      Mutex.lock m;
      results.(i) <- v;
      Mutex.unlock m
    ) ()
  ) tasks in
  List.iter Thread.join threads;
  Array.to_list results

let () =
  let tasks = [
    (fun () -> 2 + 2);
    (fun () -> 3 * 3);
    (fun () -> 10 - 1);
  ] in
  let results = compute_parallel tasks in
  assert (results = [4; 9; 9]);
  Printf.printf "Approach 2 (parallel collect): [%s]\n"
    (String.concat "; " (List.map string_of_int results))

(* --- Approach 3: Join all, sum results --- *)

let parallel_sum ns =
  let total = ref 0 in
  let m = Mutex.create () in
  let threads = List.map (fun n ->
    Thread.create (fun () ->
      (* simulate work: n * n *)
      let v = n * n in
      Mutex.lock m;
      total := !total + v;
      Mutex.unlock m
    ) ()
  ) ns in
  List.iter Thread.join threads;
  !total

let () =
  (* 1^2 + 2^2 + 3^2 + 4^2 = 1+4+9+16 = 30 *)
  let s = parallel_sum [1;2;3;4] in
  assert (s = 30);
  Printf.printf "Approach 3 (parallel sum): %d\n" s

let () = Printf.printf "✓ All tests passed\n"
