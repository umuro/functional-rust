(* 1053: Coin Change — Minimum Coins for Amount *)

(* Approach 1: Bottom-up DP *)
let coin_change_dp coins amount =
  let max_val = amount + 1 in
  let dp = Array.make (amount + 1) max_val in
  dp.(0) <- 0;
  for i = 1 to amount do
    List.iter (fun coin ->
      if coin <= i && dp.(i - coin) + 1 < dp.(i) then
        dp.(i) <- dp.(i - coin) + 1
    ) coins
  done;
  if dp.(amount) > amount then -1 else dp.(amount)

(* Approach 2: Recursive with memoization *)
let coin_change_memo coins amount =
  let cache = Hashtbl.create 128 in
  let rec solve amt =
    if amt = 0 then 0
    else if amt < 0 then max_int
    else
      match Hashtbl.find_opt cache amt with
      | Some v -> v
      | None ->
        let result = List.fold_left (fun best coin ->
          let sub = solve (amt - coin) in
          if sub < max_int then min best (sub + 1) else best
        ) max_int coins in
        Hashtbl.add cache amt result;
        result
  in
  let r = solve amount in
  if r = max_int then -1 else r

let () =
  (* Standard test cases *)
  assert (coin_change_dp [1; 5; 10; 25] 30 = 2);  (* 25 + 5 *)
  assert (coin_change_dp [1; 5; 10; 25] 11 = 2);  (* 10 + 1 *)
  assert (coin_change_dp [2] 3 = -1);               (* impossible *)
  assert (coin_change_dp [1] 0 = 0);                (* zero amount *)
  assert (coin_change_dp [1; 2; 5] 11 = 3);         (* 5 + 5 + 1 *)

  assert (coin_change_memo [1; 5; 10; 25] 30 = 2);
  assert (coin_change_memo [1; 5; 10; 25] 11 = 2);
  assert (coin_change_memo [2] 3 = -1);
  assert (coin_change_memo [1] 0 = 0);
  assert (coin_change_memo [1; 2; 5] 11 = 3);

  Printf.printf "✓ All tests passed\n"
