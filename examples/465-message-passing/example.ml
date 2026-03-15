(* 465. Message passing vs shared memory – OCaml *)
let count_words text =
  List.length (List.filter ((<>) "") (String.split_on_char ' ' text))

(* Message passing: collect results *)
let (send,_,recv) =
  let q=Queue.create () let m=Mutex.create () let c=Condition.create () in
  (fun v -> Mutex.lock m; Queue.push v q; Condition.signal c; Mutex.unlock m),
  (),
  (fun () -> Mutex.lock m; while Queue.is_empty q do Condition.wait c m done;
    let v=Queue.pop q in Mutex.unlock m; v)

(* Shared memory: update global counter *)
let total=ref 0 let mutex=Mutex.create ()

let () =
  let texts=["hello world";"foo bar baz";"one two three four"] in
  let ts=List.map (fun t -> Thread.create (fun () -> send (count_words t)) ()) texts in
  List.iter Thread.join ts;
  let mp_total = List.fold_left (fun a _ -> a + recv ()) 0 texts in
  Printf.printf "message passing: %d\n" mp_total;

  let ts=List.map (fun t -> Thread.create (fun () ->
    let n=count_words t in Mutex.lock mutex; total:= !total+n; Mutex.unlock mutex) ()
  ) texts in
  List.iter Thread.join ts;
  Printf.printf "shared memory: %d\n" !total
