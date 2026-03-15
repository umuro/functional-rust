(* 1053: Coin Change — Minimum Coins for Amount
   Bottom-up DP, top-down memoization, and BFS approaches. *)

(* Approach 1: Bottom-up DP *)
let coin_change_dp coins amount =
  if amount = 0 then 0
  else begin
    let inf = amount + 1 in
    let dp = Array.make (amount + 1) inf in
    dp.(0) <- 0;
    for i = 1 to amount do
      List.iter (fun coin ->
        if coin <= i && dp.(i - coin) + 1 < dp.(i) then
          dp.(i) <- dp.(i - coin) + 1
      ) coins
    done;
    if dp.(amount) > amount then -1 else dp.(amount)
  end

(* Approach 2: Top-down memoization *)
let coin_change_memo coins amount =
  let cache = Hashtbl.create 64 in
  let rec solve amt =
    if amt = 0 then 0
    else if amt < 0 then max_int
    else match Hashtbl.find_opt cache amt with
    | Some v -> v
    | None ->
      let result =
        List.fold_left (fun best coin ->
          let sub = solve (amt - coin) in
          if sub < max_int then min best (sub + 1) else best
        ) max_int coins
      in
      Hashtbl.add cache amt result; result
  in
  let r = solve amount in
  if r = max_int then -1 else r

(* Approach 3: BFS — shortest path to amount *)
let coin_change_bfs coins amount =
  if amount = 0 then 0
  else begin
    let visited = Array.make (amount + 1) false in
    let q = Queue.create () in
    Queue.add (0, 0) q;
    visited.(0) <- true;
    let result = ref (-1) in
    while not (Queue.is_empty q) && !result = -1 do
      let (current, steps) = Queue.pop q in
      List.iter (fun coin ->
        let next = current + coin in
        if next = amount then result := steps + 1
        else if next < amount && not visited.(next) then begin
          visited.(next) <- true;
          Queue.add (next, steps + 1) q
        end
      ) coins
    done;
    !result
  end

let () =
  let cases = [
    ([1;5;10;25], 30, 2);
    ([1;5;10;25], 11, 2);
    ([2],          3, -1);
    ([1],          0, 0);
    ([1;2;5],     11, 3);
  ] in
  List.iter (fun (coins, amount, expected) ->
    assert (coin_change_dp   coins amount = expected);
    assert (coin_change_memo coins amount = expected);
    assert (coin_change_bfs  coins amount = expected)
  ) cases;
  Printf.printf "All coin-change tests passed.\n"
