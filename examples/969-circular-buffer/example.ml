(* 969: Circular Buffer (Ring Buffer)
   Fixed-capacity FIFO with O(1) push/pop using modular arithmetic.
   When full, oldest element is overwritten (streaming/audio use case). *)

type 'a circ_buf = {
  buf      : 'a array;
  capacity : int;
  mutable head : int;  (* index of next read *)
  mutable len  : int;  (* number of valid elements *)
  dummy    : 'a;
}

let create capacity dummy =
  assert (capacity > 0);
  { buf = Array.make capacity dummy; capacity; head = 0; len = 0; dummy }

let is_empty b = b.len = 0
let is_full  b = b.len = b.capacity
let length   b = b.len

(* Push: adds to the tail; overwrites oldest element when full *)
let push b x =
  let tail = (b.head + b.len) mod b.capacity in
  b.buf.(tail) <- x;
  if is_full b then
    (* overwrite: advance head so oldest is lost *)
    b.head <- (b.head + 1) mod b.capacity
  else
    b.len <- b.len + 1

(* Pop: removes from the head; returns None when empty *)
let pop b =
  if is_empty b then None
  else begin
    let x = b.buf.(b.head) in
    b.buf.(b.head) <- b.dummy;
    b.head <- (b.head + 1) mod b.capacity;
    b.len <- b.len - 1;
    Some x
  end

(* Peek at head without removing *)
let peek b =
  if is_empty b then None else Some b.buf.(b.head)

(* Peek at element i positions from head (0 = head) *)
let peek_at b i =
  if i < 0 || i >= b.len then None
  else Some b.buf.((b.head + i) mod b.capacity)

(* Iterate in FIFO order *)
let iter f b =
  for i = 0 to b.len - 1 do
    f b.buf.((b.head + i) mod b.capacity)
  done

let to_list b =
  let acc = ref [] in
  for i = b.len - 1 downto 0 do
    acc := b.buf.((b.head + i) mod b.capacity) :: !acc
  done;
  !acc

let () =
  Printf.printf "=== Basic FIFO usage ===\n";
  let b : int circ_buf = create 5 0 in
  List.iter (push b) [1; 2; 3; 4; 5];
  Printf.printf "full = %b, len = %d\n" (is_full b) (length b);
  Printf.printf "peek = %d\n" (Option.get (peek b));
  Printf.printf "pop: %d %d\n"
    (Option.get (pop b)) (Option.get (pop b));
  push b 6;
  Printf.printf "after pop×2 + push 6: [%s]\n"
    (String.concat "; " (List.map string_of_int (to_list b)));

  Printf.printf "\n=== Overwrite mode (streaming) ===\n";
  let ring : int circ_buf = create 3 0 in
  for i = 1 to 6 do
    push ring i;
    Printf.printf "push %d → [%s] (len=%d)\n" i
      (String.concat "; " (List.map string_of_int (to_list ring)))
      (length ring)
  done;
  Printf.printf "After pushing 1..6 into capacity-3 buffer:\n";
  Printf.printf "  contents: [%s] (only last 3 kept)\n"
    (String.concat "; " (List.map string_of_int (to_list ring)));

  Printf.printf "\n=== Sliding window of last N values ===\n";
  let window : float circ_buf = create 4 0.0 in
  let values = [|1.0; 2.0; 3.0; 4.0; 5.0; 6.0; 7.0|] in
  Array.iter (fun v ->
    push window v;
    let lst = to_list window in
    let avg = List.fold_left ( +. ) 0.0 lst /. float_of_int (List.length lst) in
    Printf.printf "  add %.1f → window=[%s] avg=%.2f\n" v
      (String.concat "," (List.map (Printf.sprintf "%.1f") lst)) avg
  ) values
