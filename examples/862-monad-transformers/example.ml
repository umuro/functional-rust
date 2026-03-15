(* Example 063: Monad Transformers *)
(* Stacking monads: OptionT over Result *)

(* OptionT wraps Result<Option<'a>, 'e> *)
type ('a, 'e) option_t = ('a option, 'e) result

let return_ x : ('a, 'e) option_t = Ok (Some x)
let fail e : ('a, 'e) option_t = Error e
let none : ('a, 'e) option_t = Ok None

let bind (m : ('a, 'e) option_t) (f : 'a -> ('b, 'e) option_t) : ('b, 'e) option_t =
  match m with
  | Error e -> Error e
  | Ok None -> Ok None
  | Ok (Some a) -> f a

let ( >>= ) = bind

(* Approach 1: Database operations that may fail or return nothing *)
let find_user id : (string, string) option_t =
  if id > 0 then Ok (Some (Printf.sprintf "User_%d" id))
  else if id = 0 then Ok None
  else Error "Invalid ID"

let find_email name : (string, string) option_t =
  if name = "User_1" then Ok (Some "user1@example.com")
  else if name = "User_2" then Ok None  (* exists but no email *)
  else Error "DB connection failed"

let get_user_email id =
  find_user id >>= fun name ->
  find_email name

(* Approach 2: Lifting from inner monads *)
let lift_result (r : ('a, 'e) result) : ('a, 'e) option_t =
  Result.map (fun x -> Some x) r

let lift_option (o : 'a option) : ('a, 'e) option_t =
  Ok o

(* Approach 3: Combining with map *)
let map f m = bind m (fun x -> return_ (f x))

let get_upper_email id =
  map String.uppercase_ascii (get_user_email id)

let () =
  assert (get_user_email 1 = Ok (Some "user1@example.com"));
  assert (get_user_email 0 = Ok None);
  assert (get_user_email (-1) = Error "Invalid ID");
  assert (get_user_email 2 = Ok None);

  assert (get_upper_email 1 = Ok (Some "USER1@EXAMPLE.COM"));

  assert (lift_result (Ok 42) = Ok (Some 42));
  assert (lift_result (Error "e") = (Error "e" : (int, string) option_t));
  assert (lift_option (Some 42) = Ok (Some 42));
  assert (lift_option None = (Ok None : (int, string) option_t));

  Printf.printf "✓ All tests passed\n"
