(* Grand tour of FP in OCaml *)
(* A mini order-processing system *)

type currency = USD | EUR | GBP
type price = { amount: float; currency: currency }
type product = { id: string; name: string; price: price; stock: int }
type order_item = { product: product; qty: int }
type order = { id: string; items: order_item list; discount: float }

(* Pure functions *)
let item_subtotal item = item.product.price.amount *. float_of_int item.qty
let order_total order =
  let subtotal = List.fold_left (fun acc i -> acc +. item_subtotal i) 0.0 order.items in
  subtotal *. (1.0 -. order.discount)

let validate_order order =
  let check_stock item =
    if item.qty <= item.product.stock then Ok item
    else Error (Printf.sprintf "%s: only %d in stock" item.product.name item.product.stock)
  in
  let rec validate = function
    | []      -> Ok []
    | x :: xs ->
      match (check_stock x, validate xs) with
      | (Ok i, Ok rest) -> Ok (i :: rest)
      | (Error e, _) | (_, Error e) -> Error e
  in
  match validate order.items with
  | Error e -> Error e
  | Ok _    -> if order.discount < 0.0 || order.discount > 1.0
               then Error "invalid discount" else Ok order

let () =
  let p1 = { id="A"; name="Widget"; price={amount=9.99;currency=USD}; stock=10 } in
  let p2 = { id="B"; name="Gadget"; price={amount=24.99;currency=USD}; stock=2 } in
  let order = { id="ORD-001"; discount=0.1;
    items=[{product=p1;qty=2};{product=p2;qty=1}] } in
  (match validate_order order with
  | Ok o   -> Printf.printf "Order total: $%.2f\n" (order_total o)
  | Error e -> Printf.printf "Error: %s\n" e)
