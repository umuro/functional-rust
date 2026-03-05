(* 449. join concept – OCaml *)
let join f g =
  let rg = ref None in
  let t = Thread.create (fun () -> rg := Some (g ())) () in
  let rf = f () in Thread.join t;
  (rf, Option.get !rg)

let rec psum arr lo hi =
  if hi-lo <= 500 then
    Array.fold_left (+) 0 (Array.sub arr lo (hi-lo))
  else let mid = (lo+hi)/2 in
    let (l,r) = join (fun () -> psum arr lo mid) (fun () -> psum arr mid hi)
    in l+r

let () =
  let a = Array.init 10000 (fun i -> i+1) in
  Printf.printf "sum = %d\n" (psum a 0 10000)
