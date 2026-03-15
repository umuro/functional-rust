(* 1051: Fibonacci with HashMap Memoization *)

(* Approach 1: Recursive with Hashtbl memoization *)
let fib_memo () =
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

(* Approach 2: Functional memoization with ref to Map *)
module IntMap = Map.Make(Int)

let fib_map n =
  let cache = ref IntMap.empty in
  let rec fib n =
    if n <= 1 then n
    else
      match IntMap.find_opt n !cache with
      | Some v -> v
      | None ->
        let v = fib (n - 1) + fib (n - 2) in
        cache := IntMap.add n v !cache;
        v
  in
  fib n

(* Approach 3: CPS with memoization *)
let fib_cps n =
  let cache = Hashtbl.create 64 in
  let rec fib n k =
    if n <= 1 then k n
    else
      match Hashtbl.find_opt cache n with
      | Some v -> k v
      | None ->
        fib (n - 1) (fun a ->
          fib (n - 2) (fun b ->
            let v = a + b in
            Hashtbl.add cache n v;
            k v))
  in
  fib n Fun.id

let () =
  let fib = fib_memo () in
  assert (fib 0 = 0);
  assert (fib 1 = 1);
  assert (fib 10 = 55);
  assert (fib 20 = 6765);
  assert (fib 30 = 832040);

  assert (fib_map 10 = 55);
  assert (fib_map 20 = 6765);
  assert (fib_map 30 = 832040);

  assert (fib_cps 10 = 55);
  assert (fib_cps 20 = 6765);
  assert (fib_cps 30 = 832040);

  Printf.printf "✓ All tests passed\n"
