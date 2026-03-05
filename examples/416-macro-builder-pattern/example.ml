(* Builder pattern in OCaml *)

type 'a or_error = Ok of 'a | Error of string

type request_config = {
  url: string;
  method_: string;
  timeout_ms: int;
  max_retries: int;
  headers: (string * string) list;
}

type request_builder = {
  mutable url: string option;
  mutable method_: string;
  mutable timeout_ms: int;
  mutable max_retries: int;
  mutable headers: (string * string) list;
}

let new_builder () = {
  url = None; method_ = "GET"; timeout_ms = 5000;
  max_retries = 3; headers = [];
}

let set_url b url = b.url <- Some url; b
let set_method b m = b.method_ <- m; b
let set_timeout b t = b.timeout_ms <- t; b
let add_header b k v = b.headers <- (k,v) :: b.headers; b

let build b = match b.url with
  | None -> Error "url is required"
  | Some url -> Ok {
      url; method_ = b.method_;
      timeout_ms = b.timeout_ms;
      max_retries = b.max_retries;
      headers = b.headers;
    }

let () =
  let result =
    new_builder ()
    |> (fun b -> set_url b "https://api.example.com")
    |> (fun b -> set_method b "POST")
    |> (fun b -> set_timeout b 10000)
    |> build
  in
  match result with
  | Ok cfg -> Printf.printf "Request to %s (%s) timeout=%dms\n"
      cfg.url cfg.method_ cfg.timeout_ms
  | Error e -> Printf.printf "Error: %s\n" e
