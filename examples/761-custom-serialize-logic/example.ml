(* Custom serialization for complex types in OCaml
   Handling sum types (variants), options, and lists *)

type shape =
  | Circle of float
  | Rectangle of float * float
  | Point

(* Serialize a shape to a string representation *)
let serialize_shape = function
  | Circle r -> Printf.sprintf "circle|r=%.6g" r
  | Rectangle (w, h) -> Printf.sprintf "rect|w=%.6g|h=%.6g" w h
  | Point -> "point"

let deserialize_shape s =
  match String.split_on_char '|' s with
  | ["circle"; rv] ->
    (match String.split_on_char '=' rv with
     | ["r"; v] -> (try Some (Circle (float_of_string v)) with _ -> None)
     | _ -> None)
  | ["rect"; wv; hv] ->
    (match String.split_on_char '=' wv, String.split_on_char '=' hv with
     | ["w"; w], ["h"; h] ->
       (try Some (Rectangle (float_of_string w, float_of_string h)) with _ -> None)
     | _ -> None)
  | ["point"] -> Some Point
  | _ -> None

(* Serialize a list of shapes *)
let serialize_shapes shapes =
  let parts = List.map serialize_shape shapes in
  (* length-prefix the list *)
  Printf.sprintf "%d\n%s" (List.length parts) (String.concat "\n" parts)

let () =
  let shapes = [Circle 3.14; Rectangle (2.0, 5.0); Point; Circle 1.0] in
  let s = serialize_shapes shapes in
  Printf.printf "Serialized:\n%s\n\n" s;
  (* deserialize each line after the count *)
  let lines = String.split_on_char '\n' s in
  match lines with
  | [] -> ()
  | _ :: rest ->
    List.iteri (fun i line ->
      if line <> "" then
        match deserialize_shape line with
        | Some sh -> Printf.printf "Shape %d: %s\n" i (serialize_shape sh)
        | None -> Printf.printf "Shape %d: FAILED\n" i
    ) rest
