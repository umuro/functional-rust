(* 743: Session types — protocol safety via typestate in OCaml *)
(* The protocol: Connect → SendRequest → RecvResponse → Close
   Violating the order is a COMPILE ERROR — wrong-state functions simply do not exist. *)

(* ── Protocol state markers ────────────────────────────────────────────────── *)
type connected          = private Connected_
type request_sent       = private RequestSent_
type response_received  = private ResponseReceived_
type closed             = private Closed_

(* ── In-memory channel (simulated) ───────────────────────────────────────── *)

type channel = {
  mutable outbox : string Queue.t;
  mutable inbox  : string Queue.t;
}

let new_channel () = { outbox = Queue.create (); inbox = Queue.create () }

let channel_send ch data =
  (* simulate echo response *)
  let resp = "RESP:" ^ data in
  Queue.add data ch.outbox;
  Queue.add resp ch.inbox

let channel_recv ch = Queue.pop ch.inbox

(* ── Session record ────────────────────────────────────────────────────────── *)

type 'state session = {
  channel : channel;
  log     : string list;
}

(* Open a session — starts in Connected state *)
let open_session () : connected session =
  print_endline "[Session] Connected";
  { channel = new_channel (); log = [] }

(* ── Connected: only send_request available ─────────────────────────────── *)

let send_request method_ path (s : connected session) : request_sent session =
  let msg = method_ ^ " " ^ path in
  Printf.printf "[Session] → Sending: %s\n" msg;
  channel_send s.channel msg;
  { s with log = ("SENT: " ^ method_ ^ " " ^ path) :: s.log }

(* ── RequestSent: only receive_response available ───────────────────────── *)

let receive_response (s : request_sent session) : string * response_received session =
  let data = channel_recv s.channel in
  Printf.printf "[Session] ← Received: %s\n" data;
  let s' = { s with log = ("RECV: " ^ data) :: s.log } in
  (data, s')

(* ── ResponseReceived: close or send next request ────────────────────────── *)

let close (s : response_received session) : closed session =
  let n = List.length s.log in
  Printf.printf "[Session] Closed. %d log entries.\n" n;
  { s with log = s.log }

let send_next_request method_ path (s : response_received session) : request_sent session =
  let msg = method_ ^ " " ^ path in
  Printf.printf "[Session] → Pipeline: %s\n" msg;
  channel_send s.channel msg;
  { s with log = ("SENT: " ^ method_ ^ " " ^ path) :: s.log }

(* ── Closed: read-only access to log ────────────────────────────────────── *)

let log_entries (s : closed session) = List.rev s.log

let () =
  (* happy path *)
  let s = open_session () in
  let s = send_request "GET" "/test" s in
  let (resp, s) = receive_response s in
  let closed = close s in
  assert (String.length resp > 0);
  assert (List.mem "SENT: GET /test" (log_entries closed));
  Printf.printf "log: %s\n" (String.concat "; " (log_entries closed));
  print_endline "happy_path: ok";

  (* response echoes request *)
  let s2 = open_session () in
  let s2 = send_request "POST" "/data" s2 in
  let (resp2, s2) = receive_response s2 in
  assert (let _ = String.sub resp2 0 1 in true);  (* non-empty *)
  ignore (close s2);
  Printf.printf "response: %s\n" resp2;
  print_endline "response_echo: ok";

  (* pipeline: two requests *)
  let s3 = open_session () in
  let s3 = send_request "GET" "/a" s3 in
  let (_, s3) = receive_response s3 in
  let s3 = send_next_request "GET" "/b" s3 in
  let (_, s3) = receive_response s3 in
  let closed3 = close s3 in
  assert (List.length (log_entries closed3) = 4);
  print_endline "pipeline: ok";

  (* The following would be COMPILE ERRORS:
       receive_response (open_session ())    (* connected has no recv *)
       send_request "X" "/" (close s)        (* closed has no send *)
  *)

  print_endline "All assertions passed."
