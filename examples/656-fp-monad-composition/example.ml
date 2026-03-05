(* Monad Composition in OCaml *)

(* OptionT transformer over Result *)
module OptionT (M : sig
  type 'a t
  val return : 'a -> 'a t
  val bind : 'a t -> ('a -> 'b t) -> 'b t
end) = struct
  type 'a t = 'a option M.t
  
  let return x = M.return (Some x)
  
  let bind m f = M.bind m (function
    | None -> M.return None
    | Some a -> f a)
end

(* Direct composition: Result (Option a) e *)
type ('a, 'e) option_result = ('a option, 'e) result

let bind_option_result m f = match m with
  | Error e -> Error e
  | Ok None -> Ok None
  | Ok (Some a) -> f a

let () =
  let parse s = match int_of_string_opt s with
    | None -> Error "parse error"
    | Some n -> if n > 0 then Ok (Some n) else Ok None
  in
  let result = bind_option_result (parse "42") (fun n -> Ok (Some (n * 2))) in
  match result with
  | Error e -> Printf.printf "Error: %s\n" e
  | Ok None -> print_endline "None"
  | Ok (Some n) -> Printf.printf "Some %d\n" n
