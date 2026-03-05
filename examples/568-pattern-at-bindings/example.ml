(* @ (as) bindings in OCaml *)
type event = Click of int*int | Key of char | Resize of int*int

let handle = function
  | Click(x,y) as e when x>0 && y>0 ->
    Printf.printf "valid click at (%d,%d): %s\n" x y (match e with Click _->"click"|_->"?")
  | Key c when c >= 'a' && c <= 'z' -> Printf.printf "lower: %c\n" c
  | Key c when c >= 'A' && c <= 'Z' -> Printf.printf "upper: %c\n" c
  | Key c  -> Printf.printf "other key: %c\n" c
  | Resize(w,h) -> Printf.printf "resize %dx%d\n" w h
  | _ -> ()

let () =
  List.iter handle [Click(10,20); Click(-1,5); Key 'a'; Key 'Z'; Resize(800,600)]
