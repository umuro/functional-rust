(* 448. Parallel map – OCaml manual *)
let parallel_map f arr =
  let n = Array.length arr in
  let res = Array.make n (f arr.(0)) in
  let nt = 4 in
  let chunk = (n + nt - 1) / nt in
  let ts = Array.init nt (fun t ->
    let lo = t*chunk and hi = min n ((t+1)*chunk) in
    Thread.create (fun () ->
      for i = lo to hi-1 do res.(i) <- f arr.(i) done) ()
  ) in
  Array.iter Thread.join ts; res

let () =
  let data = Array.init 1000 (fun i -> float_of_int (i+1)) in
  let sq = parallel_map (fun x -> x*.x) data in
  Printf.printf "sum of squares = %.0f\n" (Array.fold_left (+.) 0. sq)
