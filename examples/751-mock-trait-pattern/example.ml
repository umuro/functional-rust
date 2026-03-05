(* 751: Mock Trait Pattern — OCaml module-based mocking *)

(* Interface as module type *)
module type EMAIL_SENDER = sig
  val send : to_:string -> subject:string -> body:string -> (unit, string) result
end

(* Real implementation *)
module SmtpSender : EMAIL_SENDER = struct
  let send ~to_ ~subject ~body =
    Printf.printf "[SMTP] To: %s | Subject: %s | Body: %s\n" to_ subject body;
    Ok ()
end

(* Mock: records all calls *)
module MockSender : sig
  include EMAIL_SENDER
  val calls : unit -> (string * string * string) list
  val reset : unit -> unit
end = struct
  let recorded = ref []

  let send ~to_ ~subject ~body =
    recorded := (to_, subject, body) :: !recorded;
    Ok ()

  let calls () = List.rev !recorded
  let reset () = recorded := []
end

(* Business logic — parametrized over EMAIL_SENDER *)
module UserService (Sender : EMAIL_SENDER) = struct
  let welcome_user email name =
    Sender.send
      ~to_:email
      ~subject:"Welcome!"
      ~body:(Printf.sprintf "Hi %s, welcome aboard!" name)
end

(* Tests *)
let () =
  MockSender.reset ();

  let module SUT = UserService(MockSender) in
  let _ = SUT.welcome_user "alice@example.com" "Alice" in
  let _ = SUT.welcome_user "bob@example.com" "Bob" in

  let calls = MockSender.calls () in
  assert (List.length calls = 2);
  let (to_, subj, body) = List.hd calls in
  assert (to_ = "alice@example.com");
  assert (subj = "Welcome!");
  assert (String.length body > 0);

  Printf.printf "Mock tests passed! %d call(s) recorded.\n" (List.length calls)
