(* Fibonacci at 'compile time' (load time) in OCaml *)

(* Elegant recursive definition *)
let rec fib_rec n =
  if n <= 1 then n
  else fib_rec (n-1) + fib_rec (n-2)

(* Memoized version (Hashtbl) *)
let memo = Hashtbl.create 100

let rec fib_memo n =
  if n <= 1 then n
  else match Hashtbl.find_opt memo n with
  | Some v -> v
  | None ->
    let v = fib_memo (n-1) + fib_memo (n-2) in
    Hashtbl.replace memo n v;
    v

(* Table (computed once at module load — analogous to const in Rust) *)
let fib_table =
  let t = Array.make 93 0 in
  t.(0) <- 0; t.(1) <- 1;
  for i = 2 to 92 do
    t.(i) <- t.(i-1) + t.(i-2)
  done;
  t

let fib n = fib_table.(n)

let () =
  Printf.printf "Recursive  fib(10) = %d\n" (fib_rec 10);
  Printf.printf "Memoized   fib(40) = %d\n" (fib_memo 40);
  Printf.printf "Table      fib(50) = %d\n" (fib 50);
  Printf.printf "fib(1..10) = ";
  for i = 0 to 10 do
    Printf.printf "%d " (fib i)
  done;
  print_newline ()
