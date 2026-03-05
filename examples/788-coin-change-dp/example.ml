(* Coin change: minimum coins in OCaml *)

(* ── Recursive with memoisation (top-down) ──────────────────────────────────── *)
let coin_change_memo coins amount =
  let memo = Hashtbl.create 128 in
  let rec dp n =
    if n = 0 then Some 0
    else if n < 0 then None
    else match Hashtbl.find_opt memo n with
    | Some v -> v
    | None ->
      let best =
        List.fold_left (fun best coin ->
          match dp (n - coin) with
          | None -> best
          | Some sub ->
            match best with
            | None -> Some (1 + sub)
            | Some b -> Some (min b (1 + sub))
        ) None coins
      in
      Hashtbl.replace memo n best;
      best
  in
  dp amount

(* ── Tabulation (bottom-up) ────────────────────────────────────────────────────── *)
let coin_change_tab coins amount =
  let dp = Array.make (amount + 1) (amount + 1) in (* sentinel = amount+1 *)
  dp.(0) <- 0;
  for i = 1 to amount do
    List.iter (fun coin ->
      if coin <= i && dp.(i - coin) + 1 < dp.(i) then
        dp.(i) <- dp.(i - coin) + 1
    ) coins
  done;
  if dp.(amount) > amount then None else Some dp.(amount)

(* ── Count ways ─────────────────────────────────────────────────────────────── *)
let count_ways coins amount =
  let dp = Array.make (amount + 1) 0 in
  dp.(0) <- 1;
  List.iter (fun coin ->
    for i = coin to amount do
      dp.(i) <- dp.(i) + dp.(i - coin)
    done
  ) coins;
  dp.(amount)

let () =
  let coins = [1; 5; 10; 25] in
  let amounts = [0; 11; 30; 41; 100] in
  List.iter (fun amt ->
    let m = coin_change_memo coins amt in
    let t = coin_change_tab  coins amt in
    let w = count_ways coins amt in
    Printf.printf "amount=%3d: min_coins=%s  ways=%d\n"
      amt
      (match m with Some n -> string_of_int n | None -> "none")
      w;
    assert (m = t)  (* both methods agree *)
  ) amounts
