(* 442. Scoped threads – OCaml *)
(* Classic OCaml: must join manually before data goes out of scope *)
let parallel_sum arr =
  let n = Array.length arr in
  let mid = n / 2 in
  let left  = ref 0 in
  let right = ref 0 in
  let t1 = Thread.create (fun () ->
    left := Array.fold_left (+) 0 (Array.sub arr 0 mid)) () in
  let t2 = Thread.create (fun () ->
    right := Array.fold_left (+) 0 (Array.sub arr mid (n-mid))) () in
  Thread.join t1; Thread.join t2;
  !left + !right

let () =
  let data = Array.init 100 (fun i -> i+1) in
  Printf.printf "Sum = %d (expected 5050)\n" (parallel_sum data)
