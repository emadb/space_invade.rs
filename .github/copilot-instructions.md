Here is the Markdown source code for the instruction file. You can copy the content of the code block below, save it as `8080_context.md`, and use it with your coding assistant.

```markdown
# Intel 8080 Architecture and Instruction Reference

This document outlines the technical details, programming model, and emulation requirements for the Intel 8080 microprocessor.

## 1. Programming Model

The 8080 is an 8-bit NMOS microprocessor with a 16-bit address bus, capable of addressing up to 64 KB of memory.

### Registers

- **Accumulator (A):** 8-bit primary arithmetic register.
- **General Purpose:** 8-bit registers **B, C, D, E, H, L**.
- **Pairs:** Registers can be accessed as 16-bit pairs:
  - **BC** (B = High, C = Low)
  - **DE** (D = High, E = Low)
  - **HL** (H = High, L = Low) - Often used as the primary memory pointer (address `M`).
- **Stack Pointer (SP):** 16-bit register pointing to the current top of the stack.
- **Program Counter (PC):** 16-bit register holding the address of the next instruction.

### Processor Status Word (PSW) & Flags

The **PSW** consists of the Accumulator and the Flag Register treated as a 16-bit unit (Push/Pop PSW).

**Flag Register Layout:** `S Z 0 AC 0 P 1 C`

| Bit   | Flag               | Description                                                |
| :---- | :----------------- | :--------------------------------------------------------- |
| **7** | **S** (Sign)       | Set if the MSB (bit 7) of the result is 1.                 |
| **6** | **Z** (Zero)       | Set if the result is 0.                                    |
| **5** | -                  | **Unused**. Always **0**.                                  |
| **4** | **AC** (Aux Carry) | Set if carry occurs from bit 3 to bit 4 (nibble overflow). |
| **3** | -                  | **Unused**. Always **0**.                                  |
| **2** | **P** (Parity)     | Set if the result has **Even Parity** (even number of 1s). |
| **1** | -                  | **Unused**. Always **1**.                                  |
| **0** | **C** (Carry)      | Set if operation generates a carry (add) or borrow (sub).  |

## 2. Instruction Specifics & Emulation Pitfalls

### Arithmetic & Logic

- **Subtraction (`SUB`, `SBB`, `CMP`):**
  - The 8080 uses the Carry flag as a **Borrow** flag for subtraction.
  - $C = 1$ if the operation required a borrow (Unsigned operand > Accumulator).
  - $C = 0$ if no borrow occurred.
  - _Implementation:_ Internally often calculated as inverted carry from two's complement addition.
- **Increment/Decrement (`INR`, `DCR`):**
  - These instructions affect **Z, S, P, AC**.
  - **Important:** They do **NOT** affect the **Carry (C)** flag.
- **Logical AND (`ANA`, `ANI`):**
  - Clears the Carry (C) flag.
  - _Note:_ The AC flag is often documented to be cleared, but on the 8080 silicon, it may reflect the logical OR of bit 3 of the operands (distinct from Z80/8085 behavior).
- **Logical OR/XOR (`ORA`, `XRA`, `ORI`, `XRI`):**
  - Clears both Carry (C) and Auxiliary Carry (AC) flags.
- **Decimal Adjust (`DAA`):**
  - Adjusts Accumulator to BCD format after addition.
  - Logic depends on both the value of A and the status of AC and C flags.

### Stack & Branching

- **Stack Growth:** Downward (High address to Low address).
- **`PUSH`:** Decrements SP by 2. Stores High byte at `SP-1`, Low byte at `SP-2`.
- **`POP`:** Increments SP by 2. Loads Low byte from `SP`, High byte from `SP+1`.
- **`CALL`:** Pushes the PC (address of _next_ instruction) onto the stack and jumps.
- **`PCHL`:** Loads the contents of HL directly into the PC (Jump Indirect).
- **`SPHL`:** Loads the contents of HL directly into the SP.
- **`XTHL`:** Exchanges HL with the top of the stack.

### Input / Output

- **`IN` / `OUT`:** The 8080 uses 8-bit port addresses.
- **Bus Mirroring:** The port address is duplicated on both the lower (A0-A7) and upper (A8-A15) address lines during I/O cycles.

## 3. Interrupts

- **State:** Controlled by `EI` (Enable) and `DI` (Disable).
- **Execution:** When an interrupt line is asserted and enabled, the CPU fetches an instruction from the data bus (usually `RST n`).
- **`RST n`:** Equivalent to a 1-byte `CALL` to address `n * 8` (e.g., `RST 1` -> `0x0008`).

## 4. Verification

Reliable emulation should be verified against historical test suites:

1.  **CPUDIAG (Microcosm Associates):** Basic instruction test.
2.  **8080EXER:** Comprehensive flag and cycle-accuracy test (generating CRCs of execution logs).
3.  **TST8080:** Tests logic operations and branching.
```

8080 instruction encoding:

Conventions in instruction source:
D = Destination register (8 bit)
S = Source register (8 bit)
RP = Register pair (16 bit) # = 8 or 16 bit immediate operand
a = 16 bit Memory address
p = 8 bit port address
ccc = Conditional

Conventions in instruction encoding:
db = Data byte (8 bit)
lb = Low byte of 16 bit value
hb = High byte of 16 bit value
pa = Port address (8 bit)

Dest and Source reg fields:
111=A (Accumulator)
000=B
001=C
010=D
011=E
100=H
101=L
110=M (Memory reference through address in H:L)

Register pair 'RP' fields:
00=BC (B:C as 16 bit register)
01=DE (D:E as 16 bit register)
10=HL (H:L as 16 bit register)
11=SP (Stack pointer, refers to PSW (FLAGS:A) for PUSH/POP)

Condition code 'CCC' fields: (FLAGS: S Z x A x P x C)
000=NZ ('Z'ero flag not set)
001=Z ('Z'ero flag set)
010=NC ('C'arry flag not set)
011=C ('C'arry flag set)
100=PO ('P'arity flag not set - ODD)
101=PE ('P'arity flag set - EVEN)
110=P ('S'ign flag not set - POSITIVE)
111=M ('S'ign flag set - MINUS)

## Inst Encoding Flags Description

MOV D,S 01DDDSSS - Move register to register
MVI D,# 00DDD110 db - Move immediate to register
LXI RP,# 00RP0001 lb hb - Load register pair immediate
LDA a 00111010 lb hb - Load A from memory
STA a 00110010 lb hb - Store A to memory
LHLD a 00101010 lb hb - Load H:L from memory
SHLD a 00100010 lb hb - Store H:L to memory
LDAX RP 00RP1010 *1 - Load indirect through BC or DE
STAX RP 00RP0010 *1 - Store indirect through BC or DE
XCHG 11101011 - Exchange DE and HL content
ADD S 10000SSS ZSPCA Add register to A
ADI # 11000110 db ZSCPA Add immediate to A
ADC S 10001SSS ZSCPA Add register to A with carry
ACI # 11001110 db ZSCPA Add immediate to A with carry
SUB S 10010SSS ZSCPA Subtract register from A
SUI # 11010110 db ZSCPA Subtract immediate from A
SBB S 10011SSS ZSCPA Subtract register from A with borrow
SBI # 11011110 db ZSCPA Subtract immediate from A with borrow
INR D 00DDD100 ZSPA Increment register
DCR D 00DDD101 ZSPA Decrement register
INX RP 00RP0011 - Increment register pair
DCX RP 00RP1011 - Decrement register pair
DAD RP 00RP1001 C Add register pair to HL (16 bit add)
DAA 00100111 ZSPCA Decimal Adjust accumulator
ANA S 10100SSS ZSCPA AND register with A
ANI # 11100110 db ZSPCA AND immediate with A
ORA S 10110SSS ZSPCA OR register with A
ORI # 11110110 ZSPCA OR immediate with A
XRA S 10101SSS ZSPCA ExclusiveOR register with A
XRI # 11101110 db ZSPCA ExclusiveOR immediate with A
CMP S 10111SSS ZSPCA Compare register with A
CPI # 11111110 ZSPCA Compare immediate with A
RLC 00000111 C Rotate A left
RRC 00001111 C Rotate A right
RAL 00010111 C Rotate A left through carry
RAR 00011111 C Rotate A right through carry
CMA 00101111 - Compliment A
CMC 00111111 C Compliment Carry flag
STC 00110111 C Set Carry flag
JMP a 11000011 lb hb - Unconditional jump
Jccc a 11CCC010 lb hb - Conditional jump
CALL a 11001101 lb hb - Unconditional subroutine call
Cccc a 11CCC100 lb hb - Conditional subroutine call
RET 11001001 - Unconditional return from subroutine
Rccc 11CCC000 - Conditional return from subroutine
RST n 11NNN111 - Restart (Call n*8)
PCHL 11101001 - Jump to address in H:L
PUSH RP 11RP0101 *2 - Push register pair on the stack
POP RP 11RP0001 *2 *2 Pop register pair from the stack
XTHL 11100011 - Swap H:L with top word on stack
SPHL 11111001 - Set SP to content of H:L
IN p 11011011 pa - Read input port into A
OUT p 11010011 pa - Write A to output port
EI 11111011 - Enable interrupts
DI 11110011 - Disable interrupts
HLT 01110110 - Halt processor
NOP 00000000 - No operation

\*1 = Only RP=00(BC) and 01(DE) are allowed for LDAX/STAX

\*2 = RP=11 refers to PSW for PUSH/POP (cannot push/pop SP).
When PSW is POP'd, ALL flags are affected.

In the folder ./doc you can find more information about the 8080 and the Space invaders game.
