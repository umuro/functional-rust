(* Fixed-size containers with module-level size in OCaml *)

module type CAPACITY = sig val cap : int end

(* Ring buffer *)
module RingBuffer (C : CAPACITY) = struct
  type 'a t = {
    mutable data: 'a array;
    mutable head: int;
    mutable size: int;
    default: 'a;
  }

  let create default =
    { data = Array.make C.cap default; head = 0; size = 0; default }

  let push rb v =
    let tail = (rb.head + rb.size) mod C.cap in
    rb.data.(tail) <- v;
    if rb.size < C.cap then rb.size <- rb.size + 1
    else rb.head <- (rb.head + 1) mod C.cap

  let to_list rb =
    List.init rb.size (fun i -> rb.data.((rb.head + i) mod C.cap))

  let capacity = C.cap
end

module Cap5 = struct let cap = 5 end
module Ring5 = RingBuffer(Cap5)

let () =
  let rb = Ring5.create 0 in
  Printf.printf "Capacity: %d\n" Ring5.capacity;
  for i = 1 to 7 do Ring5.push rb i done;
  (* Only last 5 should remain *)
  Printf.printf "Contents: [%s]\n"
    (String.concat "; " (List.map string_of_int (Ring5.to_list rb)))
