(* Point-in-Polygon: Ray Casting in OCaml *)

type point = { x: float; y: float }

(* Ray casting: count how many times the +x ray from P crosses polygon edges *)
(* Returns `Inside`, `Outside`, or `OnBoundary` *)
type location = Inside | Outside | OnBoundary

let point_in_polygon (p : point) (polygon : point array) : location =
  let n = Array.length polygon in
  if n = 0 then Outside
  else begin
    let crossings = ref 0 in
    let on_boundary = ref false in
    for i = 0 to n - 1 do
      let a = polygon.(i) in
      let b = polygon.((i + 1) mod n) in
      (* Check if P is on the segment AB *)
      let cross = (b.x -. a.x) *. (p.y -. a.y) -. (b.y -. a.y) *. (p.x -. a.x) in
      let min_x = min a.x b.x and max_x = max a.x b.x in
      let min_y = min a.y b.y and max_y = max a.y b.y in
      if abs_float cross < 1e-12
         && p.x >= min_x && p.x <= max_x
         && p.y >= min_y && p.y <= max_y then
        on_boundary := true;
      (* Ray crossing test *)
      if (a.y <= p.y && b.y > p.y) || (b.y <= p.y && a.y > p.y) then begin
        let x_cross = a.x +. (p.y -. a.y) /. (b.y -. a.y) *. (b.x -. a.x) in
        if p.x < x_cross then incr crossings
      end
    done;
    if !on_boundary then OnBoundary
    else if !crossings mod 2 = 1 then Inside
    else Outside
  end

let string_of_location = function
  | Inside -> "Inside" | Outside -> "Outside" | OnBoundary -> "OnBoundary"

let () =
  (* Square [0,2]×[0,2] *)
  let square = [| {x=0.0;y=0.0}; {x=2.0;y=0.0};
                  {x=2.0;y=2.0}; {x=0.0;y=2.0} |] in
  let tests = [
    ({x=1.0;y=1.0}, "centre");
    ({x=3.0;y=1.0}, "right outside");
    ({x=0.0;y=0.0}, "corner");
    ({x=1.0;y=0.0}, "edge");
    ({x=1.0;y=2.5}, "above");
  ] in
  Printf.printf "Square [0,2]×[0,2]:\n";
  List.iter (fun (p, label) ->
    Printf.printf "  (%g,%g) [%s]: %s\n" p.x p.y label
      (string_of_location (point_in_polygon p square))
  ) tests;

  (* L-shaped polygon *)
  let l_shape = [| {x=0.0;y=0.0}; {x=2.0;y=0.0}; {x=2.0;y=1.0};
                   {x=1.0;y=1.0}; {x=1.0;y=2.0}; {x=0.0;y=2.0} |] in
  Printf.printf "\nL-shape polygon:\n";
  List.iter (fun (p, label) ->
    Printf.printf "  (%g,%g) [%s]: %s\n" p.x p.y label
      (string_of_location (point_in_polygon p l_shape))
  ) [({x=0.5;y=0.5}, "bottom-left leg"); ({x=1.5;y=1.5}, "top-right — outside")]
