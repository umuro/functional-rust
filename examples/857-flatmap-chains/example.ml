(* Example 058: FlatMap/Bind Chains *)
(* Long monadic chains for sequential computation with early exit *)

let ( >>= ) m f = match m with None -> None | Some x -> f x
let return_ x = Some x

(* Approach 1: Multi-step data processing pipeline *)
let parse_json s =
  if String.length s > 0 && s.[0] = '{' then Some s else None

let extract_field key json =
  (* simplified: look for "key":"value" *)
  let pattern = Printf.sprintf "\"%s\":\"" key in
  match String.split_on_char '"' json with
  | _ when not (String.contains json ':') -> None
  | parts ->
    let rec find = function
      | k :: _ :: v :: _ when k = key -> Some v
      | _ :: rest -> find rest
      | [] -> None
    in find parts

let validate_length min max s =
  let len = String.length s in
  if len >= min && len <= max then Some s else None

let to_uppercase s = Some (String.uppercase_ascii s)

let process_name json =
  parse_json json >>= fun j ->
  extract_field "name" j >>= fun name ->
  validate_length 1 50 name >>= fun valid ->
  to_uppercase valid

(* Approach 2: Database-like lookup chain *)
type user = { id: int; dept_id: int; name: string }
type dept = { d_id: int; mgr_id: int; d_name: string }

let users = [
  { id = 1; dept_id = 10; name = "Alice" };
  { id = 2; dept_id = 20; name = "Bob" }
]

let depts = [
  { d_id = 10; mgr_id = 2; d_name = "Engineering" };
  { d_id = 20; mgr_id = 1; d_name = "Marketing" }
]

let find_user id = List.find_opt (fun u -> u.id = id) users
let find_dept id = List.find_opt (fun d -> d.d_id = id) depts

let find_manager_dept_name user_id =
  find_user user_id >>= fun user ->
  find_dept user.dept_id >>= fun dept ->
  find_user dept.mgr_id >>= fun manager ->
  Some (Printf.sprintf "%s's manager is %s in %s" user.name manager.name dept.d_name)

(* Approach 3: Computation with accumulator *)
let step_add n acc = if acc + n > 100 then None else Some (acc + n)
let step_mul n acc = if acc * n > 100 then None else Some (acc * n)

let compute () =
  return_ 0 >>=
  step_add 10 >>=
  step_mul 3 >>=
  step_add 20 >>=
  step_add 40

let () =
  assert (process_name "{\"name\":\"alice\"}" = Some "ALICE");
  assert (process_name "not json" = None);
  assert (find_manager_dept_name 1 = Some "Alice's manager is Bob in Engineering");
  assert (find_manager_dept_name 99 = None);
  assert (compute () = Some 90);
  Printf.printf "✓ All tests passed\n"
