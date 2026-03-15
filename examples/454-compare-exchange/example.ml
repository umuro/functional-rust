(* 454: Compare-and-Exchange (CAS)
   OCaml 5's Atomic.compare_and_set implements the CAS primitive.
   We demonstrate CAS-based increment and CAS-based atomic-max. *)

(* CAS increment: atomically increment an int Atomic *)
let cas_increment a =
  let rec loop () =
    let cur = Atomic.get a in
    (* compare_and_set returns true on success, false if the value changed *)
    if not (Atomic.compare_and_set a cur (cur + 1))
    then loop ()  (* retry on conflict *)
  in
  loop ()

(* Atomic max: set a to max(a, v) using a CAS loop *)
let atomic_max a v =
  let rec loop () =
    let cur = Atomic.get a in
    if v > cur then begin
      if not (Atomic.compare_and_set a cur v)
      then loop ()  (* another thread changed it; retry *)
    end
  in
  loop ()

let () =
  (* CAS increment: 100 sequential increments *)
  let a = Atomic.make 0 in
  for _ = 1 to 100 do cas_increment a done;
  assert (Atomic.get a = 100);
  Printf.printf "cas_increment x100 = %d\n%!" (Atomic.get a);

  (* CAS failure: compare_and_set with wrong expected value *)
  let b = Atomic.make 5 in
  let success = Atomic.compare_and_set b 99 0 in  (* expected 99, actual 5 → fail *)
  assert (not success);
  assert (Atomic.get b = 5);
  Printf.printf "cas failure (expected 99, got 5): %b\n%!" (not success);

  (* Atomic max across concurrent domains *)
  let m = Atomic.make min_int in
  let vals = [5;3;8;1;9;2] in
  let domains = List.map (fun v ->
    Domain.spawn (fun () -> atomic_max m v)) vals
  in
  List.iter Domain.join domains;
  assert (Atomic.get m = 9);
  Printf.printf "atomic_max of [5;3;8;1;9;2] = %d\n%!" (Atomic.get m)
