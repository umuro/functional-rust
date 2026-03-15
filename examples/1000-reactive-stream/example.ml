(* 1000: Reactive Stream *)
(* Push-based Observable<T> with map/filter/subscribe *)

(* --- Observable: a source that pushes values to subscribers --- *)

type 'a observer = {
  on_next: 'a -> unit;
  on_error: exn -> unit;
  on_complete: unit -> unit;
}

type 'a observable = { subscribe: 'a observer -> unit -> unit }
(* subscribe returns an unsubscribe function *)

let make_observer ?(on_error=fun _ -> ()) ?(on_complete=fun () -> ()) on_next =
  { on_next; on_error; on_complete }

(* --- Source: emit values from a list --- *)

let from_list xs =
  { subscribe = fun observer ->
    List.iter observer.on_next xs;
    observer.on_complete ();
    fun () -> () (* unsubscribe is no-op for completed streams *)
  }

(* --- Operators --- *)

let map f obs =
  { subscribe = fun observer ->
    obs.subscribe {
      on_next = (fun v -> observer.on_next (f v));
      on_error = observer.on_error;
      on_complete = observer.on_complete;
    }
  }

let filter pred obs =
  { subscribe = fun observer ->
    obs.subscribe {
      on_next = (fun v -> if pred v then observer.on_next v);
      on_error = observer.on_error;
      on_complete = observer.on_complete;
    }
  }

let take n obs =
  { subscribe = fun observer ->
    let count = ref 0 in
    obs.subscribe {
      on_next = (fun v ->
        if !count < n then begin
          incr count;
          observer.on_next v;
          if !count = n then observer.on_complete ()
        end);
      on_error = observer.on_error;
      on_complete = observer.on_complete;
    }
  }

(* --- Approach 1: Chain operators --- *)

let () =
  let results = ref [] in
  let source = from_list [1;2;3;4;5;6;7;8;9;10] in
  let stream =
    source
    |> filter (fun x -> x mod 2 = 0)  (* keep even *)
    |> map (fun x -> x * x)            (* square *)
    |> take 3                          (* first 3 *)
  in
  let observer = make_observer (fun v -> results := v :: !results) in
  let _ = stream.subscribe observer in
  let results = List.rev !results in
  assert (results = [4; 16; 36]);
  Printf.printf "Approach 1 (reactive chain): [%s]\n"
    (String.concat "; " (List.map string_of_int results))

(* --- Approach 2: Subject (hot observable — broadcast to multiple) --- *)

type 'a subject = {
  mutable observers: ('a observer) list;
  mutable completed: bool;
}

let make_subject () = { observers = []; completed = false }

let subject_subscribe subj obs =
  if not subj.completed then
    subj.observers <- obs :: subj.observers;
  fun () -> subj.observers <- List.filter (fun o -> o != obs) subj.observers

let subject_next subj v =
  List.iter (fun o -> o.on_next v) subj.observers

let subject_complete subj =
  subj.completed <- true;
  List.iter (fun o -> o.on_complete ()) subj.observers

let () =
  let subj = make_subject () in
  let r1 = ref [] and r2 = ref [] in
  let _ = subject_subscribe subj (make_observer (fun v -> r1 := v :: !r1)) in
  let _ = subject_subscribe subj (make_observer (fun v -> r2 := v :: !r2)) in
  List.iter (subject_next subj) [10; 20; 30];
  subject_complete subj;
  assert (List.rev !r1 = [10;20;30]);
  assert (List.rev !r2 = [10;20;30]);
  Printf.printf "Approach 2 (subject): r1=%d r2=%d items\n"
    (List.length !r1) (List.length !r2)

let () = Printf.printf "✓ All tests passed\n"
