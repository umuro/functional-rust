(* Command pattern as data in OCaml *)
type cmd =
  | MoveTo  of float * float
  | LineTo  of float * float
  | ArcTo   of float * float * float
  | SetColor of string
  | Undo

type point = { mutable x: float; mutable y: float }
type state = { pos: point; mutable color: string; mutable history: cmd list }

let mk_state () = { pos={x=0.;y=0.}; color="black"; history=[] }

let execute s cmd =
  (match cmd with
  | MoveTo(x,y)   -> s.pos.x<-x; s.pos.y<-y
  | LineTo(x,y)   -> Printf.printf "line (%.1f,%.1f)->(%.1f,%.1f)\n" s.pos.x s.pos.y x y;
                     s.pos.x<-x; s.pos.y<-y
  | ArcTo(x,y,r)  -> Printf.printf "arc to (%.1f,%.1f) r=%.1f\n" x y r;
                     s.pos.x<-x; s.pos.y<-y
  | SetColor c    -> s.color<-c
  | Undo          -> ());
  s.history <- cmd :: s.history

let () =
  let s = mk_state () in
  List.iter (execute s)
    [MoveTo(0.,0.); SetColor "red"; LineTo(10.,0.); LineTo(10.,10.); LineTo(0.,10.)]
