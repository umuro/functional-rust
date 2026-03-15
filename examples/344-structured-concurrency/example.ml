(* 344: Structured Concurrency
   OCaml 5 Domains provide structured parallelism:
   all spawned domains are joined before the enclosing scope returns. *)

(* Run a function in a new domain and wait for its result *)
let scoped_work f =
  let d = Domain.spawn f in
  Domain.join d

(* Parallel sum via recursive domain splitting *)
let rec parallel_sum arr lo hi =
  let len = hi - lo in
  if len < 100 then
    (* Small slice: compute directly *)
    Array.sub arr lo len |> Array.fold_left (+) 0
  else begin
    let mid = lo + len / 2 in
    (* Spawn left half in a new domain *)
    let left_d = Domain.spawn (fun () -> parallel_sum arr lo mid) in
    (* Compute right half in current domain *)
    let right  = parallel_sum arr mid hi in
    let left   = Domain.join left_d in
    left + right
  end

let () =
  (* scoped_work test *)
  let r = scoped_work (fun () -> 42) in
  assert (r = 42);
  Printf.printf "scoped_work returned: %d\n%!" r;

  (* parallel_sum test *)
  let nums = Array.init 1000 (fun i -> i + 1) in   (* 1..1000 *)
  let total = parallel_sum nums 0 (Array.length nums) in
  assert (total = 500500);
  Printf.printf "parallel_sum(1..1000) = %d\n%!" total
