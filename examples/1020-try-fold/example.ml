(* 1020: try_fold — Fold that short-circuits on error
   Rust's try_fold stops at the first Err and returns it.
   In OCaml we write a fold that propagates errors using Result.bind,
   or use a recursive helper that stops early. *)

(* Approach 1: fold with short-circuit via Result.bind *)
let try_fold f init lst =
  List.fold_left (fun acc x ->
    Result.bind acc (fun a -> f a x)
  ) (Ok init) lst

(* Sum positive numbers only — fail on negatives *)
let sum_positive numbers =
  try_fold (fun acc n ->
    if n < 0 then Error (Printf.sprintf "negative number: %d" n)
    else Ok (acc + n)
  ) 0 numbers

(* Concat strings up to a max length *)
let concat_limited strings max_len =
  try_fold (fun acc s ->
    let next = acc ^ s in
    if String.length next > max_len then
      Error (Printf.sprintf "result too long: %d > %d" (String.length next) max_len)
    else Ok next
  ) "" strings

(* Product without overflow (using option for overflow check) *)
let product_no_overflow numbers =
  try_fold (fun acc n ->
    (* Simple overflow check: compare before/after *)
    if acc <> 0 && abs n > max_int / abs acc then
      Error (Printf.sprintf "overflow at %d * %d" acc n)
    else Ok (acc * n)
  ) 1 numbers

let () =
  assert (sum_positive [1; 2; 3] = Ok 6);
  assert (sum_positive [1; -2; 3] = Error "negative number: -2");
  assert (sum_positive [] = Ok 0);

  assert (concat_limited ["hello"; " "; "world"] 20 = Ok "hello world");
  (match concat_limited ["hello"; " "; "world!!!!!!!!!!!!"] 10 with
   | Error msg -> assert (String.length msg > 0)
   | _ -> assert false);

  assert (product_no_overflow [2; 3; 4] = Ok 24);

  (* Short-circuit proof: stop at first error *)
  let count = ref 0 in
  let result =
    try_fold (fun acc n ->
      incr count;
      if n < 0 then Error "negative" else Ok (acc + n)
    ) 0 [1; -2; 3; 4; 5]
  in
  assert (Result.is_error result);
  assert (!count = 2);  (* only processed 1 and -2 *)

  (* Regular fold vs try_fold *)
  let regular_sum = List.fold_left (+) 0 [1; 2; 3] in
  assert (regular_sum = 6);

  Printf.printf "sum_positive [1;2;3] = %s\n"
    (match sum_positive [1; 2; 3] with Ok n -> string_of_int n | Error e -> e)
