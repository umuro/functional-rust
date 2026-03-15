(* 702: Unsafe traits — thread-safety contracts in OCaml *)
(* Rust uses unsafe trait for Send/Sync: the implementor promises properties
   the compiler cannot verify.

   In OCaml:
   - There is no unsafe trait concept; the type system and GC handle thread safety.
   - OCaml 5 introduces parallel Domains. Any value can be sent across domains
     unless it contains mutation (then protect with Mutex or use Atomic).
   - The OCaml analog of "marking a type as thread-safe" is using Atomic / Mutex
     and documenting the invariant in comments.
   - Module signatures act as the "contract" (like a trait); implementation
     details are hidden behind the signature. *)

(* Thread-safe counter — analogous to AtomicCounter implementing ThreadSafe *)
module AtomicCounter : sig
  type t
  val create : int -> t
  val get : t -> int
  val increment : t -> unit
  val describe : t -> string
end = struct
  type t = { value : int Atomic.t }

  let create n = { value = Atomic.make n }
  let get c = Atomic.get c.value
  let increment c = Atomic.incr c.value
  let describe c = Printf.sprintf "AtomicCounter(%d)" (get c)
end

(* demonstrate safe concurrent use across domains *)
let test_concurrent_increment () =
  let c = AtomicCounter.create 0 in
  let domains =
    Array.init 10 (fun _ ->
      Domain.spawn (fun () -> AtomicCounter.increment c)
    )
  in
  Array.iter Domain.join domains;
  let final = AtomicCounter.get c in
  assert (final = 10);
  Printf.printf "concurrent increment result: %d\n" final

(* Module signature as "trait bound": any module satisfying this signature
   is "ThreadSafe" in the OCaml sense *)
module type ThreadSafe = sig
  type t
  val describe : t -> string
end

(* A function that accepts any ThreadSafe module — analogous to
   run_in_thread<T: ThreadSafe + 'static>(val: Arc<T>) *)
let run_in_domain (type a) (module M : ThreadSafe with type t = a) (v : a) =
  let result = ref "" in
  let d = Domain.spawn (fun () ->
    result := M.describe v
  ) in
  Domain.join d;
  !result

let () =
  (* describe *)
  let c = AtomicCounter.create 42 in
  assert (AtomicCounter.describe c = "AtomicCounter(42)");
  print_endline "describe: ok";

  (* concurrent increment *)
  test_concurrent_increment ();
  print_endline "concurrent: ok";

  (* run via module — analogous to run_in_thread *)
  let c2 = AtomicCounter.create 7 in
  let desc = run_in_domain (module AtomicCounter) c2 in
  assert (desc = "AtomicCounter(7)");
  Printf.printf "run_in_domain: %s\n" desc;

  print_endline "All assertions passed."
