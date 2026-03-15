(* 1025: Network Error Classification (Simulated)
   Rich error variants with methods (is_retryable, is_client_error).
   OCaml uses module functions instead of methods, and record/variant types
   for structured errors. Retry logic is a clean recursive function. *)

type net_error =
  | Timeout of { seconds: float }
  | ConnectionRefused of string
  | DnsResolutionFailed of string
  | TlsError of string
  | HttpError of { status: int; body: string }

let string_of_net_error = function
  | Timeout { seconds }         -> Printf.sprintf "timeout after %.1fs" seconds
  | ConnectionRefused host      -> Printf.sprintf "connection refused: %s" host
  | DnsResolutionFailed host    -> Printf.sprintf "DNS failed: %s" host
  | TlsError msg                -> Printf.sprintf "TLS error: %s" msg
  | HttpError { status; body }  -> Printf.sprintf "HTTP %d: %s" status body

let is_retryable = function
  | Timeout _            -> true
  | ConnectionRefused _  -> true
  | DnsResolutionFailed _ -> false
  | TlsError _           -> false
  | HttpError { status; _ } -> status >= 500

let is_client_error = function
  | HttpError { status; _ } -> status >= 400 && status < 500
  | _ -> false

(* Simulated network call *)
let fetch url =
  match url with
  | "" -> Error (DnsResolutionFailed "empty url")
  | "http://timeout"  -> Error (Timeout { seconds = 30.0 })
  | "http://refused"  -> Error (ConnectionRefused "refused:80")
  | "http://500" -> Error (HttpError { status = 500; body = "Internal Server Error" })
  | "http://404" -> Error (HttpError { status = 404; body = "Not Found" })
  | u -> Ok (Printf.sprintf "response from %s" u)

(* Retry logic — recursive with decreasing retries *)
let rec fetch_with_retry url attempts =
  match fetch url with
  | Ok resp -> Ok resp
  | Error e when is_retryable e && attempts > 0 ->
    fetch_with_retry url (attempts - 1)
  | Error e -> Error e

let () =
  assert (Result.is_ok (fetch "http://example.com"));

  (match fetch "http://timeout" with
   | Error (Timeout _) -> assert (is_retryable (Timeout { seconds = 1.0 }))
   | _ -> assert false);

  (match fetch "http://refused" with
   | Error (ConnectionRefused _) -> assert (is_retryable (ConnectionRefused ""))
   | _ -> assert false);

  (match fetch "" with
   | Error (DnsResolutionFailed _) -> assert (not (is_retryable (DnsResolutionFailed "")))
   | _ -> assert false);

  (match fetch "http://500" with
   | Error (HttpError { status = 500; _ } as e) ->
     assert (is_retryable e);
     assert (not (is_client_error e))
   | _ -> assert false);

  (match fetch "http://404" with
   | Error (HttpError { status = 404; _ } as e) ->
     assert (not (is_retryable e));
     assert (is_client_error e)
   | _ -> assert false);

  assert (Result.is_ok (fetch_with_retry "http://example.com" 3));
  assert (Result.is_error (fetch_with_retry "http://timeout" 2));
  assert (Result.is_error (fetch_with_retry "http://404" 3));

  let err = Timeout { seconds = 5.0 } in
  assert (string_of_net_error err = "timeout after 5.0s");

  Printf.printf "Network error tests passed\n"
