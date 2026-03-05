(* Clone vs Copy semantics in OCaml *)

(* All OCaml values are implicitly copyable via GC *)

(* Simulate Copy-like semantics with simple values *)
let demonstrate_copy () =
  let x = 42 in
  let y = x in  (* Both valid — OCaml integers are like Rust Copy *)
  Printf.printf "x=%d y=%d\n" x y

(* Simulate Clone-like with explicit copy functions *)
type dna = { sequence: string; length: int }

let clone_dna dna =
  { sequence = String.copy dna.sequence; length = dna.length }

let () =
  demonstrate_copy ();
  let dna1 = { sequence = "ATCG"; length = 4 } in
  let dna2 = clone_dna dna1 in
  Printf.printf "DNA1: %s  DNA2: %s\n" dna1.sequence dna2.sequence;
  Printf.printf "Same reference: %b\n" (dna1.sequence == dna2.sequence)
