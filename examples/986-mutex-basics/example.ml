(* 986: Mutex-Protected State *)
(* OCaml: Mutex.t wraps a mutable reference — shared counter *)

(* --- Approach 1: Simple shared counter --- *)

let counter = ref 0
let m = Mutex.create ()

let increment () =
  Mutex.lock m;
  incr counter;
  Mutex.unlock m

let () =
  let threads = List.init 10 (fun _ ->
    Thread.create (fun () ->
      for _ = 1 to 100 do increment () done
    ) ()
  ) in
  List.iter Thread.join threads;
  assert (!counter = 1000);
  Printf.printf "Approach 1 (counter): %d\n" !counter

(* --- Approach 2: Mutex-protected record (structured state) --- *)

type bank_account = {
  mutable balance: int;
  mutable transactions: int;
}

let account = { balance = 0; transactions = 0 }
let account_m = Mutex.create ()

let deposit amount =
  Mutex.lock account_m;
  account.balance <- account.balance + amount;
  account.transactions <- account.transactions + 1;
  Mutex.unlock account_m

let withdraw amount =
  Mutex.lock account_m;
  if account.balance >= amount then begin
    account.balance <- account.balance - amount;
    account.transactions <- account.transactions + 1;
    Mutex.unlock account_m;
    true
  end else begin
    Mutex.unlock account_m;
    false
  end

let () =
  let threads = List.init 5 (fun _ ->
    Thread.create (fun () -> deposit 100) ()
  ) in
  List.iter Thread.join threads;
  assert (account.balance = 500);
  assert (account.transactions = 5);
  let ok = withdraw 200 in
  assert ok;
  assert (account.balance = 300);
  Printf.printf "Approach 2 (account): balance=%d txns=%d\n"
    account.balance account.transactions

(* --- Approach 3: with_mutex helper (bracket pattern) --- *)

let with_lock m f =
  Mutex.lock m;
  let result = (try f () with e -> Mutex.unlock m; raise e) in
  Mutex.unlock m;
  result

let shared_list = ref []
let list_m = Mutex.create ()

let () =
  let threads = List.init 5 (fun i ->
    Thread.create (fun () ->
      with_lock list_m (fun () ->
        shared_list := i :: !shared_list
      )
    ) ()
  ) in
  List.iter Thread.join threads;
  let sorted = List.sort compare !shared_list in
  assert (List.length sorted = 5);
  Printf.printf "Approach 3 (with_lock): [%s]\n"
    (String.concat "; " (List.map string_of_int sorted))

let () = Printf.printf "✓ All tests passed\n"
