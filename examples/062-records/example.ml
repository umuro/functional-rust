type point = { x : float; y : float }
type rect = { origin : point; width : float; height : float }

let area { width; height; _ } = width *. height
let perimeter { width; height; _ } = 2.0 *. (width +. height)

let translate dx dy r =
  { r with origin = { x = r.origin.x +. dx; y = r.origin.y +. dy } }

let contains_point r { x; y } =
  x >= r.origin.x && x <= r.origin.x +. r.width &&
  y >= r.origin.y && y <= r.origin.y +. r.height

let () =
  let r = { origin = { x = 0.0; y = 0.0 }; width = 10.0; height = 5.0 } in
  assert (area r = 50.0);
  assert (perimeter r = 30.0);
  let r2 = translate 3.0 4.0 r in
  assert (r2.origin.x = 3.0);
  assert (contains_point r { x = 1.0; y = 1.0 });
  assert (not (contains_point r { x = 11.0; y = 1.0 }));
  print_endline "All assertions passed."
