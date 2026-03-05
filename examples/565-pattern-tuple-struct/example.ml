(* Single-constructor types in OCaml *)
type meters  = Meters  of float
type seconds = Seconds of float
type rgb     = RGB     of int * int * int

let add_m (Meters a) (Meters b) = Meters (a +. b)
let speed (Meters d) (Seconds t) = d /. t
let to_gray (RGB(r,g,b)) = let avg=(r+g+b)/3 in RGB(avg,avg,avg)
let show_rgb (RGB(r,g,b)) = Printf.sprintf "rgb(%d,%d,%d)" r g b

let () =
  let Meters total = add_m (Meters 100.) (Meters 50.) in
  Printf.printf "%.1f m\n" total;
  Printf.printf "%.1f m/s\n" (speed (Meters 200.) (Seconds 10.));
  Printf.printf "%s\n" (show_rgb (to_gray (RGB(255,0,0))))
