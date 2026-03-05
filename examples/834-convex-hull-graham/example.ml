(* Convex Hull: Graham Scan in OCaml *)

type point = { x: float; y: float }

let dist2 a b =
  let dx = a.x -. b.x and dy = a.y -. b.y in
  dx *. dx +. dy *. dy

(* Cross product of vectors (b-a) and (c-a) *)
(* > 0: CCW (left turn), < 0: CW (right turn), = 0: collinear *)
let cross a b c =
  (b.x -. a.x) *. (c.y -. a.y) -. (b.y -. a.y) *. (c.x -. a.x)

(* Graham scan: returns convex hull in CCW order *)
let convex_hull (points : point list) : point list =
  let n = List.length points in
  if n <= 1 then points
  else begin
    let pts = Array.of_list points in
    (* Find bottom-most (then left-most) point as pivot *)
    let pivot = Array.fold_left (fun best p ->
      if p.y < best.y || (p.y = best.y && p.x < best.x) then p else best
    ) pts.(0) pts in
    (* Sort by polar angle with respect to pivot *)
    let sorted = Array.copy pts in
    Array.sort (fun a b ->
      if a = pivot then -1
      else if b = pivot then 1
      else
        let c = cross pivot a b in
        if c > 0.0 then -1
        else if c < 0.0 then 1
        else compare (dist2 pivot a) (dist2 pivot b)
    ) sorted;
    (* Stack-based Graham scan *)
    let stack = Array.make n {x=0.0; y=0.0} in
    let top = ref (-1) in
    Array.iter (fun p ->
      while !top >= 1 && cross stack.(!top - 1) stack.(!top) p <= 0.0 do
        decr top
      done;
      incr top;
      stack.(!top) <- p
    ) sorted;
    Array.to_list (Array.sub stack 0 (!top + 1))
  end

let () =
  let points = [
    {x=0.0;y=0.0}; {x=1.0;y=1.0}; {x=2.0;y=2.0};
    {x=0.0;y=2.0}; {x=2.0;y=0.0}; {x=1.0;y=0.0};
  ] in
  let hull = convex_hull points in
  Printf.printf "Hull (%d points):\n" (List.length hull);
  List.iter (fun p -> Printf.printf "  (%.1f, %.1f)\n" p.x p.y) hull;

  let square = [
    {x=0.0;y=0.0}; {x=1.0;y=0.0}; {x=1.0;y=1.0}; {x=0.0;y=1.0};
    {x=0.5;y=0.5}  (* interior point *)
  ] in
  let h2 = convex_hull square in
  Printf.printf "\nSquare hull (%d points, interior excluded):\n" (List.length h2);
  List.iter (fun p -> Printf.printf "  (%.1f, %.1f)\n" p.x p.y) h2
