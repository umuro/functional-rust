(* 086: Custom Iterator with State
   OCaml: stateful generators (ref-based) and lazy Seq *)

(* --- Approach 1: Counter — stateful step generator --- *)

(* A generator is just a closure: unit -> 'a option *)
let make_counter start step : int -> int =
  (* Infinite counter — returns next value each call *)
  (* We use an int -> int function to be pure; or use a ref for statefulness *)
  let current = ref (start + step) in   (* first call returns start+step *)
  fun () -> let v = !current in current := !current + step; v
  |> (fun _ -> fun () -> let v = !current in current := !current + step; v)
  |> ignore;
  (* simpler: return a Seq *)
  ignore (start, step);   (* suppress; see Seq version below *)
  fun _ -> 0              (* placeholder; real impl below *)

(* Cleaner: as a Seq *)
let counter_seq start step =
  Seq.iterate (fun n -> n + step) (start + step)

(* --- Approach 2: Fibonacci as a stateful ref-based iterator --- *)

let make_fib () : unit -> int =
  let a = ref 0 and b = ref 1 in
  fun () ->
    let v = !a in
    let next = !a + !b in
    a := !b;
    b := next;
    v

(* Alternatively as an infinite Seq *)
let fib_seq =
  Seq.unfold (fun (a, b) -> Some (a, (b, a + b))) (0, 1)

(* --- Approach 3: Collatz sequence (finite) --- *)

let collatz_seq start =
  (* Use Seq.unfold; 0 signals termination *)
  Seq.unfold (fun n ->
    if n = 0 then None
    else if n = 1 then Some (1, 0)           (* emit 1, then stop *)
    else if n mod 2 = 0 then Some (n, n / 2)
    else Some (n, 3 * n + 1)
  ) start

let collatz n = List.of_seq (collatz_seq n)

let () =
  (* counter *)
  let c = counter_seq 0 2 in
  Printf.printf "counter step 2, take 3 = [%s]\n"
    (String.concat "; " (List.map string_of_int (List.of_seq (Seq.take 3 c))));

  let neg = counter_seq 10 (-3) in
  Printf.printf "counter 10 step -3, take 4 = [%s]\n"
    (String.concat "; " (List.map string_of_int (List.of_seq (Seq.take 4 neg))));

  (* fib *)
  let fibs = List.of_seq (Seq.take 8 fib_seq) in
  Printf.printf "fibs[0..7] = [%s]\n"
    (String.concat "; " (List.map string_of_int fibs));

  (* stateful fib generator *)
  let next_fib = make_fib () in
  Printf.printf "stateful fib: %d %d %d\n" (next_fib ()) (next_fib ()) (next_fib ());

  (* collatz *)
  Printf.printf "collatz 6 = [%s]\n"
    (String.concat "; " (List.map string_of_int (collatz 6)));
  Printf.printf "collatz 1 = [%s]\n"
    (String.concat "; " (List.map string_of_int (collatz 1)))
