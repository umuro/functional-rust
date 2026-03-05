(* Imperative — Ring Buffer *)
(* Circular buffer with mutable arrays *)

type 'a ring_buffer = {
  mutable data : 'a array;
  mutable head : int;
  mutable size : int;
  capacity : int;
}

let create capacity default = {
  data = Array.make capacity default;
  head = 0; size = 0; capacity
}

let push rb x =
  let idx = (rb.head + rb.size) mod rb.capacity in
  rb.data.(idx) <- x;
  if rb.size < rb.capacity then rb.size <- rb.size + 1
  else rb.head <- (rb.head + 1) mod rb.capacity

let to_list rb =
  List.init rb.size (fun i -> rb.data.((rb.head + i) mod rb.capacity))

let rb = create 5 0
let () = List.iter (push rb) [1;2;3;4;5;6;7]
let () = List.iter (fun x -> Printf.printf "%d " x) (to_list rb)
(* Output: 3 4 5 6 7 *)
