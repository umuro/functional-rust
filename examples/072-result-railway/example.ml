(* 072: Railway-Oriented Programming *)
(* Chain Results — stay on happy path or switch to error track *)

type order = { item: string; quantity: int; price: float }

(* Individual validation steps *)
let validate_quantity order =
  if order.quantity <= 0 then Error "Quantity must be positive"
  else Ok order

let validate_price order =
  if order.price <= 0.0 then Error "Price must be positive"
  else Ok order

let validate_item order =
  if String.length order.item = 0 then Error "Item name required"
  else Ok order

(* Approach 1: Explicit bind chain *)
let validate_order_bind order =
  validate_item order
  |> Result.bind validate_quantity
  |> Result.bind validate_price

(* Approach 2: Using let* binding operator *)
let ( let* ) = Result.bind

let validate_order_let order =
  let* o = validate_item order in
  let* o = validate_quantity o in
  validate_price o

(* Approach 3: Apply discount if valid *)
let apply_discount pct order =
  if pct < 0.0 || pct > 100.0 then Error "Invalid discount"
  else Ok { order with price = order.price *. (1.0 -. pct /. 100.0) }

let calculate_total order =
  Ok (float_of_int order.quantity *. order.price)

let process_order order discount =
  let* o = validate_order_bind order in
  let* o = apply_discount discount o in
  calculate_total o

(* Tests *)
let () =
  let good = { item = "Widget"; quantity = 5; price = 10.0 } in
  let bad_qty = { item = "Widget"; quantity = -1; price = 10.0 } in
  let bad_item = { item = ""; quantity = 5; price = 10.0 } in
  assert (validate_order_bind good = Ok good);
  assert (validate_order_bind bad_qty = Error "Quantity must be positive");
  assert (validate_order_bind bad_item = Error "Item name required");
  assert (process_order good 10.0 = Ok 45.0);
  assert (process_order good 200.0 = Error "Invalid discount");
  assert (process_order bad_qty 10.0 = Error "Quantity must be positive");
  Printf.printf "✓ All tests passed\n"
