(* 1020: try_fold — Fold that short-circuits on error *)

(* Approach 1: Manual try_fold *)
let try_fold f init lst =
  let rec aux acc = function
    | [] -> Ok acc
    | x :: rest ->
      match f acc x with
      | Error e -> Error e
      | Ok acc' -> aux acc' rest
  in
  aux init lst

(* Approach 2: Using Seq for lazy evaluation *)
let try_fold_seq f init seq =
  let rec aux acc seq =
    match seq () with
    | Seq.Nil -> Ok acc
    | Seq.Cons (x, rest) ->
      match f acc x with
      | Error e -> Error e
      | Ok acc' -> aux acc' rest
  in
  aux init seq

(* Example: sum numbers, but reject negatives *)
let sum_positive acc n =
  if n < 0 then Error (Printf.sprintf "negative number: %d" n)
  else Ok (acc + n)

(* Example: build string, but limit length *)
let concat_limited acc s =
  let result = acc ^ s in
  if String.length result > 20 then Error "result too long"
  else Ok result

let test_try_fold () =
  assert (try_fold sum_positive 0 [1; 2; 3] = Ok 6);
  (match try_fold sum_positive 0 [1; -2; 3] with
   | Error e -> assert (e = "negative number: -2")
   | Ok _ -> assert false);
  (* Short-circuits: [3] never processed *)
  assert (try_fold sum_positive 0 [] = Ok 0);
  Printf.printf "  Approach 1 (list try_fold): passed\n"

let test_try_fold_seq () =
  let seq = List.to_seq [1; 2; 3] in
  assert (try_fold_seq sum_positive 0 seq = Ok 6);
  let seq = List.to_seq [1; -2; 3] in
  (match try_fold_seq sum_positive 0 seq with
   | Error _ -> ()
   | Ok _ -> assert false);
  Printf.printf "  Approach 2 (seq try_fold): passed\n"

let test_concat () =
  assert (try_fold concat_limited "" ["hello"; " "; "world"] = Ok "hello world");
  (match try_fold concat_limited "" ["hello"; " "; "world!!!!!!!!!!!!!!!!"] with
   | Error e -> assert (e = "result too long")
   | Ok _ -> assert false);
  Printf.printf "  Concat example: passed\n"

let () =
  Printf.printf "Testing try_fold:\n";
  test_try_fold ();
  test_try_fold_seq ();
  test_concat ();
  Printf.printf "✓ All tests passed\n"
