(* const fn concept in OCaml
   OCaml evaluates module-level `let` at load time, not truly at compile time.
   We compare this to Rust's guarantee of compile-time evaluation. *)

(* These are computed at module initialization (load time, not compile time in OCaml) *)
let factorial n =
  let rec go acc n = if n <= 1 then acc else go (acc * n) (n - 1) in
  go 1 n

let fac10 = factorial 10        (* computed once at load time *)
let fac12 = factorial 12

(* Powers of two table *)
let pow2 n =
  let rec go acc n = if n = 0 then acc else go (acc * 2) (n - 1)
  in go 1 n

let pow2_table = Array.init 16 pow2   (* [1; 2; 4; 8; ...] *)

(* String hashing — FNV-1a *)
let fnv1a_32 s =
  let offset_basis = 0x811c9dc5l in
  let prime = 0x01000193l in
  String.fold_left (fun hash c ->
    let hash = Int32.logxor hash (Int32.of_int (Char.code c)) in
    Int32.mul hash prime
  ) offset_basis s

let hello_hash = fnv1a_32 "hello"  (* computed once *)

let () =
  Printf.printf "10! = %d\n" fac10;
  Printf.printf "12! = %d\n" fac12;
  Printf.printf "2^8 = %d\n" pow2_table.(8);
  Printf.printf "FNV32(\"hello\") = %ld\n" hello_hash
