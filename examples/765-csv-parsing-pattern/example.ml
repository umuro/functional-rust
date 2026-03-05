(* CSV parsing without external crates in OCaml *)

(* RFC 4180-compliant CSV field parser *)
let parse_fields line =
  let len = String.length line in
  let fields = ref [] in
  let i = ref 0 in
  while !i <= len do
    if !i = len then begin
      fields := "" :: !fields;
      i := len + 1
    end else if line.[!i] = '"' then begin
      (* Quoted field *)
      incr i;
      let buf = Buffer.create 16 in
      let stop = ref false in
      while not !stop && !i < len do
        if line.[!i] = '"' then begin
          if !i + 1 < len && line.[!i + 1] = '"' then begin
            Buffer.add_char buf '"';
            i := !i + 2
          end else begin
            incr i;
            stop := true
          end
        end else begin
          Buffer.add_char buf line.[!i];
          incr i
        end
      done;
      fields := Buffer.contents buf :: !fields;
      if !i < len && line.[!i] = ',' then incr i
      else if !i >= len then i := len + 1
    end else begin
      (* Unquoted field *)
      let start = !i in
      while !i < len && line.[!i] <> ',' do incr i done;
      fields := String.sub line start (!i - start) :: !fields;
      if !i < len then incr i
      else i := len + 1
    end
  done;
  List.rev !fields

type person = { name: string; age: int; city: string }

let parse_person fields =
  match fields with
  | [name; age_s; city] ->
    (try Some { name; age = int_of_string (String.trim age_s); city }
     with Failure _ -> None)
  | _ -> None

let csv = {|Name,Age,City
Alice,30,Amsterdam
"Bob, Jr.",25,"New York"
Carol,35,Berlin|}

let () =
  let lines = String.split_on_char '\n' csv in
  match lines with
  | [] | [_] -> ()
  | _header :: rows ->
    List.iter (fun line ->
      let fields = parse_fields line in
      match parse_person fields with
      | Some p -> Printf.printf "Person: %s, %d, %s\n" p.name p.age p.city
      | None   -> Printf.printf "Could not parse: %s\n" line
    ) rows
