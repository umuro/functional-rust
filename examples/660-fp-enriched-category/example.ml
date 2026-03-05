(* Enriched Categories in OCaml *)

(* Bool-enriched: Preorder *)
module Preorder = struct
  type 'a t = {
    elements : 'a list;
    leq : 'a -> 'a -> bool;
  }
  
  let create elements leq = { elements; leq }
  let is_related pre a b = pre.leq a b
end

(* [0,∞]-enriched: Metric space *)
module Metric = struct
  type 'a t = 'a -> 'a -> float
  
  let create f = f
  let distance m a b = m a b
end

(* Cost-enriched category for graphs *)
module CostCategory = struct
  type 'v t = {
    mutable edges : ('v * 'v, float) Hashtbl.t;
  }
  
  let create () = { edges = Hashtbl.create 16 }
  
  let set_cost g a b c = Hashtbl.replace g.edges (a, b) c
  
  let get_cost g a b =
    try Hashtbl.find g.edges (a, b)
    with Not_found -> infinity
end

let () =
  let pre = Preorder.create [1; 2; 3] (fun a b -> a <= b) in
  Printf.printf "1 ≤ 2: %b\n" (Preorder.is_related pre 1 2);
  
  let metric = Metric.create (fun a b -> abs_float (a -. b)) in
  Printf.printf "d(3, 7) = %.1f\n" (Metric.distance metric 3.0 7.0)
