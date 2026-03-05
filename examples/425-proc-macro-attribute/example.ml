(* Attribute macro concepts in OCaml via decorators/wrappers *)

(* OCaml doesn't have attribute macros, but we can simulate via *)
(* higher-order functions / wrapper patterns *)

(* Simulate #[log_calls] *)
let with_logging name f =
  fun args ->
    Printf.printf "[CALL] %s(%s)\n" name (String.concat ", " (List.map string_of_int args));
    let result = f args in
    Printf.printf "[RETURN] %s -> %d\n" name result;
    result

(* Simulate #[memoize] *)
let memoize f =
  let cache = Hashtbl.create 16 in
  fun x ->
    match Hashtbl.find_opt cache x with
    | Some v -> v
    | None ->
      let v = f x in
      Hashtbl.add cache x v;
      v

let rec fib n =
  if n <= 1 then n
  else fib (n-1) + fib (n-2)

let fib_memo = memoize (fun n ->
  let rec go n = if n <= 1 then n else go (n-1) + go (n-2) in go n)

let sum_list = with_logging "sum_list" (fun xs ->
  List.fold_left (+) 0 xs)

let () =
  Printf.printf "sum: %d\n" (sum_list [1;2;3;4;5]);
  Printf.printf "fib(10): %d\n" (fib_memo 10)
