(* Line Segment Intersection in OCaml *)

type point = { x: float; y: float }

(* Cross product of vectors (b-a) and (c-a) *)
let cross a b c =
  (b.x -. a.x) *. (c.y -. a.y) -. (b.y -. a.y) *. (c.x -. a.x)

let sign x = if x > 0.0 then 1 else if x < 0.0 then -1 else 0

(* Is point p on segment [a, b]? (assuming p is collinear with a, b) *)
let on_segment a b p =
  min a.x b.x <= p.x && p.x <= max a.x b.x &&
  min a.y b.y <= p.y && p.y <= max a.y b.y

(* Do segments AB and CD intersect? *)
let segments_intersect a b c d =
  let d1 = cross c d a in
  let d2 = cross c d b in
  let d3 = cross a b c in
  let d4 = cross a b d in
  if sign d1 * sign d2 < 0 && sign d3 * sign d4 < 0 then
    true  (* Proper intersection *)
  else if d1 = 0.0 && on_segment c d a then true
  else if d2 = 0.0 && on_segment c d b then true
  else if d3 = 0.0 && on_segment a b c then true
  else if d4 = 0.0 && on_segment a b d then true
  else false

(* Find actual intersection point for non-parallel segments *)
let intersection_point a b c d =
  let denom = (b.x -. a.x) *. (d.y -. c.y) -. (b.y -. a.y) *. (d.x -. c.x) in
  if abs_float denom < 1e-12 then None  (* Parallel *)
  else
    let t = ((c.x -. a.x) *. (d.y -. c.y) -. (c.y -. a.y) *. (d.x -. c.x)) /. denom in
    if t < 0.0 || t > 1.0 then None
    else
      let s = ((c.x -. a.x) *. (b.y -. a.y) -. (c.y -. a.y) *. (b.x -. a.x)) /. denom in
      if s < 0.0 || s > 1.0 then None
      else Some { x = a.x +. t *. (b.x -. a.x); y = a.y +. t *. (b.y -. a.y) }

let () =
  let cases = [
    ({x=0.0;y=0.0}, {x=2.0;y=2.0}, {x=0.0;y=2.0}, {x=2.0;y=0.0}, true);
    ({x=0.0;y=0.0}, {x=1.0;y=0.0}, {x=2.0;y=0.0}, {x=3.0;y=0.0}, false);
    ({x=0.0;y=0.0}, {x=1.0;y=1.0}, {x=1.0;y=0.0}, {x=2.0;y=1.0}, false);
    ({x=0.0;y=0.0}, {x=2.0;y=0.0}, {x=1.0;y=-1.0}, {x=1.0;y=1.0}, true);
  ] in
  List.iter (fun (a, b, c, d, expected) ->
    let result = segments_intersect a b c d in
    Printf.printf "(%g,%g)-(%g,%g) vs (%g,%g)-(%g,%g): %b (expected %b)\n"
      a.x a.y b.x b.y c.x c.y d.x d.y result expected;
    match intersection_point a b c d with
    | Some p -> Printf.printf "  Intersection at (%g, %g)\n" p.x p.y
    | None -> ()
  ) cases
