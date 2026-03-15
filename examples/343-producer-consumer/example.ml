(* 343: Producer-Consumer Pattern
   Bounded buffer with Mutex + Condition variables, mirroring Rust's Condvar pattern. *)

type 'a bounded_buffer = {
  data     : 'a Queue.t;
  capacity : int;
  mutex    : Mutex.t;
  not_empty: Condition.t;
  not_full : Condition.t;
}

let make_buffer capacity = {
  data      = Queue.create ();
  capacity;
  mutex     = Mutex.create ();
  not_empty = Condition.create ();
  not_full  = Condition.create ();
}

(* Block until there is room, then enqueue *)
let put buf item =
  Mutex.lock buf.mutex;
  while Queue.length buf.data >= buf.capacity do
    Condition.wait buf.not_full buf.mutex
  done;
  Queue.push item buf.data;
  Condition.signal buf.not_empty;
  Mutex.unlock buf.mutex

(* Block until an item is available, then dequeue *)
let take buf =
  Mutex.lock buf.mutex;
  while Queue.is_empty buf.data do
    Condition.wait buf.not_empty buf.mutex
  done;
  let item = Queue.pop buf.data in
  Condition.signal buf.not_full;
  Mutex.unlock buf.mutex;
  item

let () =
  let buf = make_buffer 2 in
  (* Producer domain *)
  let producer = Domain.spawn (fun () ->
    for i = 0 to 4 do put buf i done)
  in
  (* Consumer: collect 5 items in the main domain *)
  let results = Array.init 5 (fun _ -> take buf) in
  Domain.join producer;
  assert (results = [|0;1;2;3;4|]);
  Printf.printf "Producer-consumer: received %s\n%!"
    (Array.to_seq results |> Seq.map string_of_int
     |> List.of_seq |> String.concat ", ")
