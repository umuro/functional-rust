(* Example 086: Custom Iterator — Fibonacci and Range with Step *)

(* Approach 1: Fibonacci via Seq *)
let fibonacci () =
  let rec aux a b () =
    Seq.Cons (a, aux b (a + b))
  in
  aux 0 1

let seq_take n seq =
  let rec aux n seq acc =
    if n = 0 then List.rev acc
    else match seq () with
    | Seq.Nil -> List.rev acc
    | Seq.Cons (x, rest) -> aux (n - 1) rest (x :: acc)
  in
  aux n seq []

(* Approach 2: Range with step *)
type 'a step_range = {
  mutable current : 'a;
  stop : 'a;
  step : 'a;
  add : 'a -> 'a -> 'a;
  compare : 'a -> 'a -> int;
}

let step_range_next sr =
  if sr.compare sr.current sr.stop >= 0 then None
  else begin
    let v = sr.current in
    sr.current <- sr.add sr.current sr.step;
    Some v
  end

let int_step_range start stop step =
  { current = start; stop; step; add = (+); compare }

let float_step_range start stop step =
  { current = start; stop; step; add = (+.); compare = Float.compare }

let step_range_to_list sr =
  let rec aux acc =
    match step_range_next sr with
    | None -> List.rev acc
    | Some v -> aux (v :: acc)
  in
  aux []

(* Approach 3: Collatz sequence *)
let collatz n =
  let current = ref n in
  let done_ = ref false in
  fun () ->
    if !done_ then Seq.Nil
    else begin
      let v = !current in
      if v = 1 then (done_ := true; Seq.Cons (v, fun () -> Seq.Nil))
      else begin
        current := if v mod 2 = 0 then v / 2 else 3 * v + 1;
        Seq.Cons (v, fun () -> Seq.Nil) (* simplified *)
      end
    end

let collatz_list n =
  let rec aux v acc =
    if v = 1 then List.rev (1 :: acc)
    else aux (if v mod 2 = 0 then v / 2 else 3 * v + 1) (v :: acc)
  in
  aux n []

(* Tests *)
let () =
  let fibs = seq_take 10 (fibonacci ()) in
  assert (fibs = [0; 1; 1; 2; 3; 5; 8; 13; 21; 34]);

  let r = int_step_range 0 10 2 in
  assert (step_range_to_list r = [0; 2; 4; 6; 8]);

  let r2 = int_step_range 0 10 3 in
  assert (step_range_to_list r2 = [0; 3; 6; 9]);

  let r3 = float_step_range 0.0 1.0 0.25 in
  let result = step_range_to_list r3 in
  assert (List.length result = 4);

  let c = collatz_list 6 in
  assert (c = [6; 3; 10; 5; 16; 8; 4; 2; 1]);

  Printf.printf "✓ All tests passed\n"
