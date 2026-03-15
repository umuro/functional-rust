(* 743: Session Types — OCaml simulation
   We simulate a simple send/receive protocol using phantom types *)

(* Protocol states *)
type ready        = Ready
type sent_request = SentRequest
type done_        = Done

(* Session parameterized by protocol state *)
type 'state session = {
  channel: Buffer.t;
  state_name: string;
}

let new_session () : ready session =
  { channel = Buffer.create 64; state_name = "Ready" }

(* send_request: Ready → SentRequest *)
let send_request (s: ready session) msg : sent_request session =
  Buffer.add_string s.channel (Printf.sprintf "REQUEST:%s" msg);
  Printf.printf "[Protocol] Sent request: %s\n" msg;
  { s with state_name = "SentRequest" }

(* receive_response: SentRequest → Done *)
let receive_response (s: sent_request session) : string * done_ session =
  let req = Buffer.contents s.channel in
  let resp = Printf.sprintf "RESPONSE to %s" req in
  Printf.printf "[Protocol] Received: %s\n" resp;
  (resp, { s with state_name = "Done" })

let () =
  let s = new_session () in
  let s = send_request s "GET /data" in
  let (resp, _done) = receive_response s in
  Printf.printf "Final response: %s\n" resp
  (* send_request _done "second" ← type error: done session ≠ ready session *)
