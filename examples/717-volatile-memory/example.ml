(* OCaml: Volatile-like access using Bigarray (closest standard library analog)
   Bigarray operations are not reordered by the OCaml compiler in the same way
   that GC'd values can be — but OCaml has no true "volatile" keyword. *)

open Bigarray

(* Create a Bigarray backed by a raw byte region — analogous to mmap'd MMIO *)
type reg32 = (int32, int32_elt, c_layout) Array1.t

let make_mmio_region size_words : reg32 =
  Array1.create int32 c_layout size_words

(* Register offsets (word indices) *)
let status_reg = 0
let data_reg   = 1
let ctrl_reg   = 2

(* Status register bits *)
let tx_ready = Int32.of_int 0x01
let rx_ready = Int32.of_int 0x02

(* Volatile-style read: Bigarray prevents the compiler from caching the value *)
let mmio_read (regs : reg32) offset =
  regs.{offset}  (* Bigarray read — not cached by the compiler *)

let mmio_write (regs : reg32) offset value =
  regs.{offset} <- value

(* Wait until TX ready bit is set *)
let wait_tx_ready regs =
  let rec loop () =
    let status = mmio_read regs status_reg in
    if Int32.logand status tx_ready = tx_ready then ()
    else loop ()
  in
  loop ()

(* Simulate writing a byte to a UART-like device *)
let uart_send regs byte =
  wait_tx_ready regs;
  mmio_write regs data_reg (Int32.of_int byte)

(* Simulate hardware: set TX_READY flag after 3 reads *)
let () =
  let regs = make_mmio_region 8 in
  (* Hardware simulation: pre-set status *)
  mmio_write regs status_reg tx_ready;
  mmio_write regs data_reg Int32.zero;

  uart_send regs (Char.code 'H');
  uart_send regs (Char.code 'i');

  Printf.printf "Status: 0x%08lx\n" (mmio_read regs status_reg);
  Printf.printf "Last data written: %d\n" (Int32.to_int (mmio_read regs data_reg));

  (* Show why volatile matters: without it, a compiler could optimise
     the loop body to `if false then …` after seeing the first read. *)
  Printf.printf "MMIO demo complete\n"
