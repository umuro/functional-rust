(* 969: Circular/Ring Buffer *)
(* Fixed-capacity FIFO. Write overwrites oldest when full. *)

type 'a ring = {
  data: 'a array;
  capacity: int;
  mutable head: int;   (* index to read next *)
  mutable tail: int;   (* index to write next *)
  mutable count: int;
}

let create capacity default =
  { data = Array.make capacity default;
    capacity;
    head = 0;
    tail = 0;
    count = 0 }

let is_full r = r.count = r.capacity
let is_empty r = r.count = 0
let size r = r.count

(* Push: if full, overwrite oldest (advance head) *)
let push r x =
  if is_full r then begin
    r.data.(r.tail) <- x;
    r.tail <- (r.tail + 1) mod r.capacity;
    r.head <- (r.head + 1) mod r.capacity  (* overwrite oldest *)
  end else begin
    r.data.(r.tail) <- x;
    r.tail <- (r.tail + 1) mod r.capacity;
    r.count <- r.count + 1
  end

(* Pop: returns oldest element *)
let pop r =
  if is_empty r then None
  else begin
    let x = r.data.(r.head) in
    r.head <- (r.head + 1) mod r.capacity;
    r.count <- r.count - 1;
    Some x
  end

let peek r =
  if is_empty r then None
  else Some r.data.(r.head)

let () =
  let r = create 4 0 in
  assert (is_empty r);

  push r 1;
  push r 2;
  push r 3;
  push r 4;
  assert (is_full r);
  assert (size r = 4);
  assert (peek r = Some 1);

  (* Overwrite: push 5 overwrites 1 *)
  push r 5;
  assert (size r = 4);
  assert (peek r = Some 2);

  assert (pop r = Some 2);
  assert (pop r = Some 3);
  assert (size r = 2);

  push r 6;
  push r 7;
  push r 8;
  assert (is_full r);

  assert (pop r = Some 4);
  assert (pop r = Some 5);
  assert (pop r = Some 6);
  assert (pop r = Some 7);
  assert (pop r = Some 8);
  assert (is_empty r);
  assert (pop r = None);

  Printf.printf "✓ All tests passed\n"
