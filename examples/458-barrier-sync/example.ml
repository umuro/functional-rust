(* 458. Barrier – OCaml manual *)
let make_barrier n =
  let cnt=ref 0 let m=Mutex.create () let c=Condition.create () let gen=ref 0 in
  (fun () ->
    Mutex.lock m; incr cnt;
    let g = !gen in
    if !cnt=n then (cnt:=0; incr gen; Condition.broadcast c)
    else (while !gen=g do Condition.wait c m done);
    Mutex.unlock m)

let () =
  let n=4 in let b=make_barrier n in
  let ts = Array.init n (fun id ->
    Thread.create (fun () ->
      for ph=1 to 3 do
        Thread.delay (0.005 *. float_of_int (id+1));
        b ();
        if id=0 then Printf.printf "phase %d done\n%!" ph
      done) ()
  ) in Array.iter Thread.join ts
