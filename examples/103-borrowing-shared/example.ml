(* 103: Shared Borrowing — &T (multiple readers, no writer)
   OCaml's default is immutable values — sharing is always safe.
   There is no borrow checker; multiple aliases to the same value are allowed.
   This is analogous to Rust's unlimited &T borrows. *)

let sum data = List.fold_left (+) 0 data

let count data = List.length data

(* Multiple uses of the same list — no conflict *)
let average data =
  let s = sum data   in   (* "borrow 1" *)
  let c = count data in   (* "borrow 2" — perfectly fine in OCaml *)
  float_of_int s /. float_of_int c

let first_and_last = function
  | [] -> None
  | data ->
    let first = List.hd data in
    let last  = List.nth data (List.length data - 1) in
    Some (first, last)

(* Multiple "references" (bindings) to the same list — all readable simultaneously *)
let demonstrate_multiple_borrows () =
  let data = [1; 2; 3; 4; 5] in
  let r1 = data in
  let r2 = data in
  let r3 = data in
  (* All three are valid at the same time — no restrictions *)
  assert (List.nth r1 0 = 1);
  assert (List.nth r2 1 = 2);
  assert (List.nth r3 2 = 3)

let () =
  let data = [1; 2; 3; 4; 5] in
  let avg = average data in
  assert (Float.abs (avg -. 3.0) < 0.001);

  assert (first_and_last [10; 20; 30] = Some (10, 30));
  assert (first_and_last [] = None);

  demonstrate_multiple_borrows ();

  let v = [1; 2; 3] in
  let r1 = v in
  let r2 = v in
  assert (List.length r1 = List.length r2);

  Printf.printf "average [1..5] = %.1f\n" (average [1; 2; 3; 4; 5])
