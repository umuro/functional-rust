(* 487. String interning – OCaml *)
module StringSet = Set.Make(String)

(* Simple interner: returns the canonical string *)
let table : (string, string) Hashtbl.t = Hashtbl.create 64

let intern s =
  match Hashtbl.find_opt table s with
  | Some v -> v
  | None -> Hashtbl.add table s s; s

let () =
  let a = intern "hello" in
  let b = intern "world" in
  let c = intern "hello" in
  Printf.printf "a=%s b=%s c=%s\n" a b c;
  (* In OCaml, physical equality (==) checks pointer *)
  Printf.printf "a==c: %b\n" (a == c);  (* true: same string *)
  Printf.printf "a==b: %b\n" (a == b);  (* false *)
  Printf.printf "intern table size: %d\n" (Hashtbl.length table)
