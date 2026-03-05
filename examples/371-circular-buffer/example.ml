(* OCaml: circular buffer with array *)

type 'a ring = {
  data : 'a option array;
  capacity : int;
  mutable head : int;
  mutable tail : int;
  mutable size : int;
}

let make cap = { data=Array.make cap None; capacity=cap; head=0; tail=0; size=0 }

let push r x =
  if r.size = r.capacity then failwith "full"
  else begin
    r.data.(r.tail) <- Some x;
    r.tail <- (r.tail + 1) mod r.capacity;
    r.size <- r.size + 1
  end

let pop r =
  if r.size = 0 then None
  else begin
    let v = r.data.(r.head) in
    r.data.(r.head) <- None;
    r.head <- (r.head + 1) mod r.capacity;
    r.size <- r.size - 1;
    v
  end

let () =
  let r = make 4 in
  push r 1; push r 2; push r 3;
  Printf.printf "Pop: %s\n" (Option.fold ~none:"?" ~some:string_of_int (pop r));
  push r 4; push r 5;
  while r.size > 0 do
    Printf.printf "%s " (Option.fold ~none:"?" ~some:string_of_int (pop r))
  done; print_newline ()
