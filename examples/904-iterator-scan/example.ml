(* 260. Stateful accumulation with scan() - OCaml *)

let scan init f lst =
  let (_, result) = List.fold_left (fun (acc, acc_list) x ->
    let new_acc = f acc x in
    (new_acc, acc_list @ [new_acc])
  ) (init, []) lst in
  result

let () =
  let nums = [1; 2; 3; 4; 5] in
  let running_sum = scan 0 (+) nums in
  Printf.printf "Running sum: %s\n"
    (String.concat ", " (List.map string_of_int running_sum));

  let running_product = scan 1 ( * ) nums in
  Printf.printf "Running product: %s\n"
    (String.concat ", " (List.map string_of_int running_product));

  let transactions = [100; -30; 50; -80; 200] in
  let balances = scan 0 (+) transactions in
  Printf.printf "Balances: %s\n"
    (String.concat ", " (List.map string_of_int balances))
