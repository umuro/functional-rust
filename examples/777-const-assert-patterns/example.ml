(* Compile-time assertion concept in OCaml
   OCaml can check some things at module load (top-level), giving load-time errors.
   Not truly compile-time, but serves a similar purpose. *)

(* ── Module-level invariant checks ──────────────────────────────────────────── *)

(* Check that a magic constant is valid *)
let () =
  let magic = 0xCAFEBABE in
  if magic land 0xFFFF0000 <> 0xCAFE0000 then
    failwith "FATAL: invalid magic constant — this is a build-time error"

(* Check platform word size *)
let () =
  if Sys.int_size < 63 then
    failwith "Requires 64-bit platform"

(* Phantom type trick to enforce invariants at the type level *)
type validated
type unvalidated
type 'state port = Port of int

let validate_port (Port n as p : unvalidated port) : validated port option =
  if n > 0 && n <= 65535 then Some (Port n : validated port) else None

let use_port (Port n : validated port) =
  Printf.printf "Using port %d (guaranteed valid by type)\n" n

let () =
  (* This is the OCaml analog: we catch invalid configs early *)
  let port = Port 8080 in
  match validate_port port with
  | Some vp -> use_port vp
  | None    -> failwith "invalid port"
