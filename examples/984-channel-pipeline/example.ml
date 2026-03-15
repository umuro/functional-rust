(* 984: Channel Pipeline *)
(* Chain of processing stages connected by channels *)
(* Each stage reads from one queue, transforms, writes to next *)

(* --- Simple queue+mutex channel abstraction --- *)

type 'a chan = {
  q: 'a Queue.t;
  m: Mutex.t;
  cond: Condition.t;
  mutable closed: bool;
}

let make_chan () = { q = Queue.create (); m = Mutex.create ();
                     cond = Condition.create (); closed = false }

let send c v =
  Mutex.lock c.m;
  Queue.push v c.q;
  Condition.signal c.cond;
  Mutex.unlock c.m

let close_chan c =
  Mutex.lock c.m;
  c.closed <- true;
  Condition.broadcast c.cond;
  Mutex.unlock c.m

let recv c =
  Mutex.lock c.m;
  while Queue.is_empty c.q && not c.closed do
    Condition.wait c.cond c.m
  done;
  let v = if Queue.is_empty c.q then None else Some (Queue.pop c.q) in
  Mutex.unlock c.m;
  v

(* --- Pipeline: producer -> stage1 -> stage2 -> collector --- *)

(* Stage: reads from in_c, applies f, writes to out_c, then closes out_c *)
let make_stage f in_c out_c =
  Thread.create (fun () ->
    let rec loop () =
      match recv in_c with
      | None -> close_chan out_c
      | Some v -> send out_c (f v); loop ()
    in
    loop ()
  ) ()

let () =
  let c0 = make_chan () in (* source *)
  let c1 = make_chan () in (* after stage1: double *)
  let c2 = make_chan () in (* after stage2: add 1 *)
  let c3 = make_chan () in (* after stage3: to string *)

  let _s1 = make_stage (fun x -> x * 2)        c0 c1 in
  let _s2 = make_stage (fun x -> x + 1)         c1 c2 in
  let _s3 = make_stage (fun x -> string_of_int x) c2 c3 in

  (* Producer: feed 1..5 *)
  let producer = Thread.create (fun () ->
    List.iter (fun i -> send c0 i) [1;2;3;4;5];
    close_chan c0
  ) () in

  (* Collector *)
  let results = ref [] in
  let rec collect () =
    match recv c3 with
    | None -> ()
    | Some v -> results := v :: !results; collect ()
  in
  collect ();
  Thread.join producer;

  let results = List.rev !results in
  (* 1->2->3, 2->4->5, 3->6->7, 4->8->9, 5->10->11 *)
  assert (results = ["3";"5";"7";"9";"11"]);
  Printf.printf "Approach 1 (pipeline): [%s]\n" (String.concat "; " results)

let () = Printf.printf "✓ All tests passed\n"
