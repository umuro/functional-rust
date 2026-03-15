(* 347: Blocking in Async
   In OCaml 5, blocking work is offloaded to a new Domain so it does not
   stall the main domain (analogous to tokio::task::spawn_blocking). *)

let blocking_computation n =
  (* Simulate CPU/IO-bound work *)
  Unix.sleepf 0.01;
  let rec product acc k = if k > n then acc else product (acc * k) (k + 1) in
  product 1 1

(* Run a blocking function in a dedicated domain *)
let spawn_blocking f = Domain.spawn f

(* Run a batch of items, each in its own domain, collect results *)
let run_blocking_batch items f =
  let domains = List.map (fun item -> Domain.spawn (fun () -> f item)) items in
  List.map Domain.join domains

let () =
  (* Single blocking computation *)
  assert (blocking_computation 5 = 120);
  Printf.printf "blocking_computation(5) = %d\n%!" (blocking_computation 5);

  (* Offload to another domain *)
  let handle = spawn_blocking (fun () -> 2 + 2) in
  assert (Domain.join handle = 4);
  Printf.printf "spawn_blocking(2+2) = %d\n%!" 4;

  (* Batch: each item runs in its own domain *)
  let results = run_blocking_batch [1;2;3] (fun x -> x * 2) in
  assert (results = [2;4;6]);
  Printf.printf "batch results: %s\n%!"
    (results |> List.map string_of_int |> String.concat ", ")
