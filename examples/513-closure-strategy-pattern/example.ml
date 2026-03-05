(* Strategy pattern via higher-order functions in OCaml *)

(* Sort with a custom comparison strategy *)
let sort_with strategy lst =
  List.sort strategy lst

(* Discount calculation strategies *)
let no_discount price = price
let ten_percent price = price *. 0.9
let bulk_discount qty price = if qty >= 10 then price *. 0.85 else price

(* Validation strategies *)
let validate_all validators value =
  List.for_all (fun v -> v value) validators

let () =
  (* Sort strategies *)
  let nums = [3; 1; 4; 1; 5; 9; 2; 6] in
  let asc  = sort_with compare nums in
  let desc = sort_with (fun a b -> compare b a) nums in
  Printf.printf "asc:  [%s]\n" (String.concat ";" (List.map string_of_int asc));
  Printf.printf "desc: [%s]\n" (String.concat ";" (List.map string_of_int desc));

  (* Discount strategies *)
  let strategies = [
    ("none",    no_discount);
    ("10%%",    ten_percent);
    ("bulk(12)",bulk_discount 12);
  ] in
  List.iter (fun (name, strat) ->
    Printf.printf "%s: %.2f\n" name (strat 100.0)
  ) strategies;

  (* Composite validation *)
  let validators = [
    (fun x -> x > 0);
    (fun x -> x < 1000);
    (fun x -> x mod 2 = 0);
  ] in
  Printf.printf "validate 42: %b\n" (validate_all validators 42);
  Printf.printf "validate -1: %b\n" (validate_all validators (-1))
