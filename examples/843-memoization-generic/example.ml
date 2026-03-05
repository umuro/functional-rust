(* Generic Memoisation in OCaml *)

(* Generic memoize: wrap any function f with a hash-table cache *)
let memoize (f : ('a, 'b) Hashtbl.t -> 'a -> 'b) : ('a -> 'b) =
  let cache = Hashtbl.create 64 in
  let rec go arg =
    match Hashtbl.find_opt cache arg with
    | Some v -> v
    | None ->
      let v = f cache arg in
      Hashtbl.add cache arg v;
      v
  in
  ignore go;  (* go is the memoised version, but we need to close over it *)
  (* Better pattern: pass cache explicitly *)
  let cache2 = Hashtbl.create 64 in
  fun arg ->
    match Hashtbl.find_opt cache2 arg with
    | Some v -> v
    | None ->
      let v = f cache2 arg in
      Hashtbl.add cache2 arg v;
      v

(* Fibonacci with memoisation — the classic example *)
let fib_memo : int -> int =
  let cache = Hashtbl.create 64 in
  let rec fib n =
    if n <= 1 then n
    else
      match Hashtbl.find_opt cache n with
      | Some v -> v
      | None ->
        let v = fib (n - 1) + fib (n - 2) in
        Hashtbl.add cache n v;
        v
  in
  fib

(* Coin change: minimum coins to make amount *)
let coin_change (coins : int list) (amount : int) : int option =
  let cache = Hashtbl.create 128 in
  let inf = max_int / 2 in
  let rec go amt =
    if amt = 0 then 0
    else if amt < 0 then inf
    else
      match Hashtbl.find_opt cache amt with
      | Some v -> v
      | None ->
        let best = List.fold_left (fun best c ->
          min best (1 + go (amt - c))
        ) inf coins in
        Hashtbl.add cache amt best;
        best
  in
  let result = go amount in
  if result >= inf then None else Some result

(* Edit distance with memoisation *)
let edit_distance (s : string) (t : string) : int =
  let m = String.length s and n = String.length t in
  let cache = Hashtbl.create 256 in
  let rec go i j =
    if i = m then n - j
    else if j = n then m - i
    else
      match Hashtbl.find_opt cache (i, j) with
      | Some v -> v
      | None ->
        let v =
          if s.[i] = t.[j] then go (i+1) (j+1)
          else 1 + min (go (i+1) j) (min (go i (j+1)) (go (i+1) (j+1)))
        in
        Hashtbl.add cache (i, j) v;
        v
  in
  go 0 0

let () =
  for n = 0 to 15 do
    Printf.printf "fib(%d) = %d\n" n (fib_memo n)
  done;
  Printf.printf "\ncoin_change([1,5,10,25], 41) = %s\n"
    (match coin_change [1;5;10;25] 41 with Some v -> string_of_int v | None -> "None");
  Printf.printf "coin_change([2], 3) = %s\n"
    (match coin_change [2] 3 with Some v -> string_of_int v | None -> "None");
  Printf.printf "\nedit_distance('kitten', 'sitting') = %d  (expected 3)\n"
    (edit_distance "kitten" "sitting");
  Printf.printf "edit_distance('', 'abc') = %d\n" (edit_distance "" "abc")
