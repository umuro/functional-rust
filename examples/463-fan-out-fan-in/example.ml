(* 463. Fan-out / fan-in – OCaml *)
let fan_map n f items =
  let wq=Queue.create () and rq=Queue.create () in
  let wm=Mutex.create () and rm=Mutex.create () in
  let rc=Condition.create () in
  List.iter (fun x->Queue.push x wq) items;
  let ws=Array.init n (fun _ -> Thread.create (fun () ->
    let go=ref true in while !go do
      Mutex.lock wm;
      if Queue.is_empty wq then (Mutex.unlock wm; go:=false)
      else let x=Queue.pop wq in (Mutex.unlock wm;
        let r=f x in Mutex.lock rm; Queue.push r rq; Condition.signal rc; Mutex.unlock rm)
    done) ()
  ) in
  Array.iter Thread.join ws;
  let results=ref [] in
  while not (Queue.is_empty rq) do results:=Queue.pop rq :: !results done;
  !results

let () =
  let r = fan_map 4 (fun x->x*x) (List.init 12 (fun i->i+1)) in
  Printf.printf "%s\n" (String.concat " " (List.map string_of_int (List.sort compare r)))
