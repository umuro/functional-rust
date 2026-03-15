(* OCaml: Branchless programming patterns *)

(* --- min/max --- *)
(* OCaml's `min` uses polymorphic comparison which can branch.
   For ints, the compiler may or may not emit CMOV. *)

let min_branch a b = if a < b then a else b
let max_branch a b = if a > b then a else b

(* Branchless integer min using arithmetic *)
let min_branchless (a : int) (b : int) =
  (* b + ((a - b) land ((a - b) asr 62)) — uses arithmetic right shift *)
  let diff = a - b in
  b + (diff land (diff asr 62))

let max_branchless (a : int) (b : int) =
  let diff = a - b in
  a - (diff land (diff asr 62))

(* --- clamp --- *)
let clamp_branch lo hi x =
  if x < lo then lo else if x > hi then hi else x

let clamp_branchless lo hi x =
  min_branchless hi (max_branchless lo x)

(* --- absolute value --- *)
let abs_branchless (x : int) =
  let mask = x asr 62 in  (* all-ones if negative, all-zeros if positive *)
  (x + mask) lxor mask

(* --- select without branch --- *)
(* Select a if cond else b, where cond is 0 or 1 *)
let select cond a b =
  (* Branchless: (a - b) * cond + b *)
  let mask = -cond in   (* 0 -> 0, 1 -> -1 (all-ones) *)
  (a land mask) lor (b land (lnot mask))

(* --- Benchmark --- *)
let time_it label f =
  let t0 = Sys.time () in
  let r = f () in
  Printf.printf "%s: %.6fs result=%d\n" label (Sys.time () -. t0) r;
  r

let () =
  let n = 10_000_000 in
  let data = Array.init n (fun i -> (i * 1234567 + 89) mod 1000) in

  let _r1 = time_it "min_branch" (fun () ->
    Array.fold_left (fun acc x -> min_branch acc x) max_int data) in
  let _r2 = time_it "min_branchless" (fun () ->
    Array.fold_left (fun acc x -> min_branchless acc x) max_int data) in

  (* Basic correctness *)
  assert (min_branchless 3 5 = 3);
  assert (min_branchless 7 2 = 2);
  assert (max_branchless 3 5 = 5);
  assert (abs_branchless (-42) = 42);
  assert (abs_branchless 42 = 42);
  assert (clamp_branchless 0 100 (-5) = 0);
  assert (clamp_branchless 0 100 150 = 100);
  assert (clamp_branchless 0 100 50 = 50);
  assert (select 1 10 20 = 10);
  assert (select 0 10 20 = 20);
  Printf.printf "All assertions passed.\n"
