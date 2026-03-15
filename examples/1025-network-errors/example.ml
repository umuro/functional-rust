(* 1025: Network Error Classification (Simulated) *)
(* Classifying and handling network-like errors *)

type net_error =
  | Timeout of float          (* seconds waited *)
  | ConnectionRefused of string  (* host *)
  | DnsResolutionFailed of string
  | TlsError of string
  | HttpError of int * string    (* status code, body *)

let string_of_net_error = function
  | Timeout secs -> Printf.sprintf "timeout after %.1fs" secs
  | ConnectionRefused host -> Printf.sprintf "connection refused: %s" host
  | DnsResolutionFailed host -> Printf.sprintf "DNS failed: %s" host
  | TlsError msg -> Printf.sprintf "TLS error: %s" msg
  | HttpError (code, body) -> Printf.sprintf "HTTP %d: %s" code body

let is_retryable = function
  | Timeout _ -> true
  | ConnectionRefused _ -> true
  | DnsResolutionFailed _ -> false  (* unlikely to change *)
  | TlsError _ -> false
  | HttpError (code, _) -> code >= 500  (* server errors are retryable *)

(* Simulated network call *)
let fetch url =
  if String.length url = 0 then Error (DnsResolutionFailed "empty url")
  else if url = "http://timeout" then Error (Timeout 30.0)
  else if url = "http://refused" then Error (ConnectionRefused "refused:80")
  else if url = "http://500" then Error (HttpError (500, "Internal Server Error"))
  else if url = "http://404" then Error (HttpError (404, "Not Found"))
  else Ok (Printf.sprintf "response from %s" url)

(* Retry logic *)
let rec fetch_with_retry url retries =
  match fetch url with
  | Ok _ as result -> result
  | Error e when is_retryable e && retries > 0 ->
    fetch_with_retry url (retries - 1)
  | Error _ as result -> result

let test_errors () =
  assert (fetch "http://example.com" = Ok "response from http://example.com");
  (match fetch "" with
   | Error (DnsResolutionFailed _) -> ()
   | _ -> assert false);
  (match fetch "http://timeout" with
   | Error (Timeout _) -> ()
   | _ -> assert false);
  Printf.printf "  Error classification: passed\n"

let test_retryable () =
  assert (is_retryable (Timeout 30.0));
  assert (is_retryable (ConnectionRefused "host"));
  assert (not (is_retryable (DnsResolutionFailed "host")));
  assert (is_retryable (HttpError (503, "Unavailable")));
  assert (not (is_retryable (HttpError (404, "Not Found"))));
  Printf.printf "  Retryable classification: passed\n"

let test_retry () =
  (* This will retry but still fail (simulated always-timeout) *)
  (match fetch_with_retry "http://timeout" 3 with
   | Error (Timeout _) -> ()
   | _ -> assert false);
  (* Success doesn't need retry *)
  assert (fetch_with_retry "http://example.com" 3
          = Ok "response from http://example.com");
  Printf.printf "  Retry logic: passed\n"

let () =
  Printf.printf "Testing network errors:\n";
  test_errors ();
  test_retryable ();
  test_retry ();
  Printf.printf "✓ All tests passed\n"
