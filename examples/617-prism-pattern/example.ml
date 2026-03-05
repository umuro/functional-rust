(* Prism in OCaml *)
type ('s,'a) prism = {
  preview: 's -> 'a option;
  review:  'a -> 's;
}

(* Prisms for a JSON-like type *)
type json = Null | JBool of bool | JNum of float | JStr of string | JArr of json list

let json_bool = {
  preview = (function JBool b -> Some b | _ -> None);
  review  = (fun b -> JBool b);
}

let json_num = {
  preview = (function JNum n -> Some n | _ -> None);
  review  = (fun n -> JNum n);
}

let json_str = {
  preview = (function JStr s -> Some s | _ -> None);
  review  = (fun s -> JStr s);
}

(* Prism law checks *)
let preview_review p a = p.preview (p.review a) = Some a
let review_preview_consistency p s = match p.preview s with
  | None   -> true
  | Some a -> p.review a = s

let () =
  Printf.printf "law review_preview JBool: %b\n" (preview_review json_bool true);
  Printf.printf "law review_preview JNum:  %b\n" (preview_review json_num 42.0);
  Printf.printf "preview JBool on JNum: %s\n"
    (match json_bool.preview (JNum 1.0) with None->"None" | Some b->string_of_bool b)
