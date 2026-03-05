(* Fibonacci: memoisation vs tabulation in OCaml
   OCaml's strength: elegant recursion with memoization via Hashtbl *)

(* 1. Naive recursion — exponential O(2^n) *)
let rec fib_naive n =
  if n <= 1 then n else fib_naive (n-1) + fib_naive (n-2)

(* 2. Top-down memoisation — O(n) time, O(n) space *)
let memo : (int, int) Hashtbl.t = Hashtbl.create 100

let rec fib_memo n =
  if n <= 1 then n
  else match Hashtbl.find_opt memo n with
  | Some v -> v
  | None ->
    let v = fib_memo (n-1) + fib_memo (n-2) in
    Hashtbl.replace memo n v;
    v

(* 3. Bottom-up tabulation — O(n) time, O(n) space *)
let fib_tab n =
  if n <= 1 then n
  else begin
    let t = Array.make (n+1) 0 in
    t.(1) <- 1;
    for i = 2 to n do
      t.(i) <- t.(i-1) + t.(i-2)
    done;
    t.(n)
  end

(* 4. Space-optimised — O(1) space *)
let fib_opt n =
  if n = 0 then 0
  else begin
    let a = ref 0 and b = ref 1 in
    for _ = 2 to n do
      let c = !a + !b in
      a := !b; b := c
    done;
    !b
  end

let () =
  Printf.printf "%-25s" "naive fib(10..20):";
  for i = 10 to 20 do Printf.printf " %d" (fib_naive i) done; print_newline ();
  Printf.printf "%-25s" "memo  fib(50):";
  Printf.printf " %d\n" (fib_memo 50);
  Printf.printf "%-25s" "tab   fib(50):";
  Printf.printf " %d\n" (fib_tab  50);
  Printf.printf "%-25s" "opt   fib(50):";
  Printf.printf " %d\n" (fib_opt  50)
