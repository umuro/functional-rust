(* 072: Railway-Oriented Programming
   Chain Results — stay on the happy path or switch to the error track *)

type order = {
  item     : string;
  quantity : int;
  price    : float;
}

(* Monadic bind: if Ok carry on; if Error short-circuit *)
let ( >>= ) = Result.bind

(* --- Individual validation steps --- *)

let validate_item order =
  if String.length order.item = 0
  then Error "Item name required"
  else Ok order

let validate_quantity order =
  if order.quantity <= 0
  then Error "Quantity must be positive"
  else Ok order

let validate_price order =
  if order.price <= 0.0
  then Error "Price must be positive"
  else Ok order

(* --- Approach 1: and_then chain (>>= bind) --- *)

let validate_order_chain order =
  Ok order
  >>= validate_item
  >>= validate_quantity
  >>= validate_price

(* --- Approach 2: Explicit sequential let-style --- *)

let validate_order_let order =
  match validate_item order with
  | Error e -> Error e
  | Ok o ->
    match validate_quantity o with
    | Error e -> Error e
    | Ok o -> validate_price o

(* --- Approach 3: Full pipeline with discount and total --- *)

let apply_discount pct order =
  if pct < 0.0 || pct > 100.0
  then Error "Invalid discount"
  else Ok { order with price = order.price *. (1.0 -. pct /. 100.0) }

let calculate_total order =
  float_of_int order.quantity *. order.price

let process_order order discount =
  validate_order_chain order
  >>= apply_discount discount
  |> Result.map calculate_total

let () =
  let good  = { item = "Widget"; quantity = 5; price = 10.0 } in
  let bad_q = { item = "Widget"; quantity = -1; price = 10.0 } in
  let bad_i = { item = ""; quantity = 5; price = 10.0 } in

  Printf.printf "valid order: %s\n"
    (match validate_order_chain good with Ok _ -> "Ok" | Error e -> e);
  Printf.printf "bad quantity: %s\n"
    (match validate_order_chain bad_q with Ok _ -> "Ok" | Error e -> e);
  Printf.printf "bad item: %s\n"
    (match validate_order_chain bad_i with Ok _ -> "Ok" | Error e -> e);
  Printf.printf "process with 10%% discount: %s\n"
    (match process_order good 10.0 with Ok v -> string_of_float v | Error e -> e);
  Printf.printf "process with bad discount: %s\n"
    (match process_order good 200.0 with Ok v -> string_of_float v | Error e -> e)
