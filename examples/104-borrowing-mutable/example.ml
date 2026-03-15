(* 104: Mutable Borrowing
   OCaml has no borrow checker: mutation is explicit via ref/mutable fields.
   This shows OCaml's approach to controlled mutation and demonstrates
   the same patterns Rust enforces at compile time. *)

(* Rust's &mut T = OCaml's ref passing (explicit, always single writer) *)
let increment (x : int ref) =
  x := !x + 1

(* Mutable list append via Buffer-style accumulator *)
let push_doubled (buf : int list ref) value =
  buf := !buf @ [value * 2]

(* Swap first and last element in an array *)
let swap_first_last arr =
  let n = Array.length arr in
  if n >= 2 then begin
    let tmp = arr.(0) in
    arr.(0) <- arr.(n - 1);
    arr.(n - 1) <- tmp
  end

(* In OCaml, there is no rule preventing two mutable references to the same
   value simultaneously — the programmer is responsible for coordination.
   Rust enforces unique writer at compile time; OCaml trusts the programmer. *)

let () =
  let x = ref 0 in
  increment x;
  increment x;
  assert (!x = 2);

  let v = ref [1; 2] in
  push_doubled v 3;
  assert (!v = [1; 2; 6]);

  let arr = [| 1; 2; 3; 4; 5 |] in
  swap_first_last arr;
  assert (arr = [| 5; 2; 3; 4; 1 |]);

  Printf.printf "All mutable-borrowing demos passed.\n"
