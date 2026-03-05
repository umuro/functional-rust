(* Generic structs parameterised by const in OCaml — using functors *)

module type DIMS = sig val rows : int val cols : int end

module Matrix (D : DIMS) = struct
  type t = { data: float array array }

  let create () =
    { data = Array.init D.rows (fun _ -> Array.make D.cols 0.0) }

  let get m r c = m.data.(r).(c)
  let set m r c v = m.data.(r).(c) <- v

  let rows = D.rows
  let cols = D.cols

  let identity () =
    assert (D.rows = D.cols);
    let m = create () in
    for i = 0 to D.rows - 1 do set m i i 1.0 done;
    m

  let print m =
    for r = 0 to D.rows - 1 do
      for c = 0 to D.cols - 1 do
        Printf.printf " %6.2f" (get m r c)
      done;
      print_newline ()
    done
end

(* Multiply M1(R×N) × M2(N×C) → M3(R×C) *)
module MatMul (R : sig val n : int end)
              (N : sig val n : int end)
              (C : sig val n : int end) = struct
  module MA = Matrix(struct let rows = R.n let cols = N.n end)
  module MB = Matrix(struct let rows = N.n let cols = C.n end)
  module MC = Matrix(struct let rows = R.n let cols = C.n end)

  let multiply a b =
    let c = MC.create () in
    for r = 0 to R.n - 1 do
      for col = 0 to C.n - 1 do
        let s = ref 0.0 in
        for k = 0 to N.n - 1 do
          s := !s +. MA.get a r k *. MB.get b k col
        done;
        MC.set c r col !s
      done
    done;
    c
end

module D2 = struct let n = 2 end
module D3 = struct let n = 3 end

module M2x3 = Matrix(struct let rows = 2 let cols = 3 end)
module M3x2 = Matrix(struct let rows = 3 let cols = 2 end)
module Mul23 = MatMul(D2)(D3)(D2)

let () =
  let a = M2x3.create () in
  M2x3.set a 0 0 1.0; M2x3.set a 0 1 2.0; M2x3.set a 0 2 3.0;
  M2x3.set a 1 0 4.0; M2x3.set a 1 1 5.0; M2x3.set a 1 2 6.0;
  Printf.printf "A (2×3):\n"; M2x3.print a
