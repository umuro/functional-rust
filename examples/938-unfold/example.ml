(* 938: Unfold — Generating Sequences from Seeds

   Unfold is the dual of fold: fold consumes a structure into a value;
   unfold produces a structure from a seed value.

   OCaml's Seq module has `Seq.unfold` (4.11+) which is lazy/infinite-safe.
   We show both the list-based and the lazy Seq version. *)

(* ── List-based unfold (eager, finite) ───────────────────────────────────── *)

(* unfold: apply f to seed; if Some (value, next_seed), emit value and continue *)
let unfold seed f =
  let rec go acc s =
    match f s with
    | None -> List.rev acc
    | Some (v, next) -> go (v :: acc) next
  in
  go [] seed

(* Range [a, b] inclusive *)
let range a b = unfold a (fun i -> if i > b then None else Some (i, i + 1))

(* Countdown from n to 0 *)
let countdown n = unfold n (fun i -> if i < 0 then None else Some (i, i - 1))

(* Collatz sequence starting from n *)
let collatz n =
  unfold n (fun x ->
    if x = 0 then None
    else if x = 1 then Some (1, 0)
    else if x mod 2 = 0 then Some (x, x / 2)
    else Some (x, 3 * x + 1))

(* ── Lazy/infinite sequences using Seq.unfold ───────────────────────────── *)

(* Infinite Fibonacci sequence as a lazy Seq *)
let fibs_seq () =
  Seq.unfold (fun (a, b) -> Some (a, (b, a + b))) (0, 1)

(* Natural numbers 0, 1, 2, ... *)
let nats () = Seq.unfold (fun n -> Some (n, n + 1)) 0

(* Repeat a value indefinitely *)
let repeat x = Seq.unfold (fun () -> Some (x, ())) ()

(* ── Seq utilities ────────────────────────────────────────────────────────── *)

let seq_take n s = List.of_seq (Seq.take n s)

let () =
  (* range *)
  assert (range 1 5 = [1; 2; 3; 4; 5]);
  assert (range 5 3 = []);   (* empty when a > b *)

  (* countdown *)
  assert (countdown 5 = [5; 4; 3; 2; 1; 0]);
  assert (countdown 0 = [0]);

  (* collatz *)
  assert (collatz 6 = [6; 3; 10; 5; 16; 8; 4; 2; 1]);
  assert (collatz 1 = [1]);

  (* fibs_seq: lazy, take first 8 *)
  let first8 = seq_take 8 (fibs_seq ()) in
  assert (first8 = [0; 1; 1; 2; 3; 5; 8; 13]);

  (* nats: take first 5 *)
  assert (seq_take 5 (nats ()) = [0; 1; 2; 3; 4]);

  (* repeat: take 4 *)
  assert (seq_take 4 (repeat 7) = [7; 7; 7; 7]);

  (* Seq.unfold available since OCaml 4.11 *)
  let powers_of_2 = Seq.unfold (fun n -> Some (n, n * 2)) 1 in
  assert (seq_take 6 powers_of_2 = [1; 2; 4; 8; 16; 32]);

  print_endline "938-unfold: all tests passed"
