## 2. Lexical conventions

### 2.1 Overview........................................................................................................................................

This clause describes the lexical tokens used in Verilog-AMS HDL source text and their conventions.

### 2.2 Lexical tokens

Verilog-AMS HDL source text files shall be a stream of lexical tokens. A _lexical token_ shall consist of one
or more characters. The layout of tokens in a source file is free format — that is, spaces and newlines shall
not be syntactically significant other than being token separators, except escaped identifiers (see 2.8.1).

The types of lexical tokens in the language are as follows:

```
— white space
— comment
— operator
— number
— string
— identifier
— keyword
```
### 2.3 White space

White space shall contain the characters for spaces, tabs, newlines, and formfeeds. These characters shall be
ignored except when they serve to separate other lexical tokens. However, spaces and tabs shall be consid-
ered significant characters in strings (see 2.7).

### 2.4 Comments

The Verilog-AMS HDL has two forms to introduce comments. A _one-line comment_ shall start with the two
characters // and ends with a newline. _Block comments_ shall start with /* and end with */. Block com-
ments shall not be nested. The one-line comment token // shall not have any special meaning in a block
comment.

comment ::= _// from A.9.2_
one_line_comment
| block_comment

one_line_comment ::= **//** comment_text \n

block_comment ::= **/*** comment_text ***/**

comment_text ::= { Any_ASCII_character }

```
Syntax 2-1—Syntax for comments
```
### 2.5 Operators........................................................................................................................................

Operators are single, double, or triple character sequences and are used in expressions. Clause 4 discusses
the use of operators in expressions.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
12
```
_Unary operators_ shall appear to the left of their operand. _Binary operators_ shall appear between their oper-
ands. A _conditional operator_ shall have two operator characters which separate three operands.

### 2.6 Numbers

_Constant numbers_ can be specified as integer constants (defined in 2.6.1) or real constants.

number ::= _// from A.8.7_
decimal_number
| octal_number
| binary_number
| hex_number
| real_number

real_number^1 ::=
unsigned_number**.** unsigned_number
| unsigned_number [**.** unsigned_number ] exp [ sign ] unsigned_number
| unsigned_number [**.** unsigned_number ] scale_factor

exp ::= **e** | **E**

scale_factor ::= **T** | **G** | **M** | **K** | **k** | **m** | **u** | **n** | **p** | **f** | **a**

decimal_number ::=
unsigned_number
| [ size ] decimal_base unsigned_number
| [ size ] decimal_base x_digit { **_** }
| [ size ] decimal_base z_digit { **_** }

binary_number ::= [ size ] binary_base binary_value

octal_number ::= [ size ] octal_base octal_value

hex_number ::= [ size ] hex_base hex_value

sign ::= **+** | **-**

size ::= non_zero_unsigned_number

non_zero_unsigned_number^1 ::= non_zero_decimal_digit { **_** | decimal_digit}

unsigned_number^1 ::= decimal_digit { **_** | decimal_digit }

binary_value^1 ::= binary_digit { **_** | binary_digit }

octal_value^1 ::= octal_digit { **_** | octal_digit }

hex_value^1 ::= hex_digit { **_** | hex_digit }

decimal_base^1 ::= **'** [ **s** | **S** ] **d** | **'** [ **s** | **S** ] **D**

binary_base^1 ::= **'** [ **s** | **S** ] **b** | **'** [ **s** | **S** ] **B**

octal_base^1 ::= **'** [ **s** | **S** ] **o** | **'** [ **s** | **S** ] **O**

hex_base^1 := **'** [ **s** | **S** ] **h** | **'** [ **s** | **S** ] **H**

non_zero_decimal_digit ::= **1** | **2** | **3** | **4** | **5** | **6** | **7** | **8** | **9**

decimal_digit ::= **0** | **1** | **2** | **3** | **4** | **5** | **6** | **7** | **8** | **9**

binary_digit ::= x_digit | z_digit | **0** | **1**

octal_digit ::= x_digit | z_digit | **0** | **1** | **2** | **3** | **4** | **5** | **6** | **7**

hex_digit ::=
x_digit | z_digit | **0** | **1** | **2** | **3** | **4** | **5** | **6** | **7** | **8** | **9**


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
13
```
```
| a | b | c | d | e | f | A | B | C | D | E | F
```
x_digit ::= **x** | **X**

z_digit ::= **z** | **Z** |**?**

1) Embedded spaces are illegal

```
Syntax 2-2—Syntax for integer and real constants
```
#### 2.6.1 Integer constants

_Integer constants_ can be specified in decimal, hexadecimal, octal, or binary format. There are two forms to
express integer constants. The first form is a simple decimal number, which shall be specified as a sequence
of digits 0 through 9, optionally starting with a plus or minus unary operator. The second form specifies a
based constant, which shall be composed of up to three tokens—an optional size constant, an apostrophe
character (', ASCII 0x27) followed by a base format character, and the digits representing the value of the
number. It shall be legal to macro substitute these three tokens.

The first token, a size constant, shall specify the size of the constant in terms of its exact number of bits. It
shall be specified as a non-zero unsigned decimal number. For example, the size specification for two hexa-
decimal digits is 8, because one hexadecimal digit requires 4 bits.

The second token, a base_format, shall consist of a case insensitive letter specifying the base for the number,
optionally preceded by the single character s (or S) to indicate a signed quantity, preceded by the apostrophe
character. Legal base specifications are d, D, h, H, o, O, b, or B, for the bases decimal, hexadecimal, octal,
and binary respectively. The apostrophe character and the base format character shall not be separated by
any white space.

The third token, an unsigned number, shall consist of digits that are legal for the specified base format. The
unsigned number token shall immediately follow the base format, optionally preceded by white space. The
hexadecimal digits a to f shall be case insensitive.

Simple decimal numbers without the size and the base format shall be treated as signed integers, whereas the
numbers specified with the base format shall be treated as signed integers if the s designator is included or as
unsigned integers if the base format only is used. The s designator does not affect the bit pattern specified,
only its interpretation. A plus or minus operator preceding the size constant is a unary plus or minus opera-
tor. A plus or minus operator between the base format and the number is an illegal syntax. Negative numbers
shall be represented in 2’s complement form.

An x represents the unknown value in hexadecimal, octal, and binary constants. A z represents the high-
impedance value. See 4.1 of IEEE Std 1364 Verilog for a discussion of the Verilog HDL value set. An x
shall set 4 bits to unknown in the hexadecimal base, 3 bits in the octal base, and 1 bit in the binary base. Sim-
ilarly, a z shall set 4 bits, 3 bits, and 1 bit, respectively, to the high-impedance value. If the size of the
unsigned number is smaller than the size specified for the constant, the unsigned number shall be padded to
the left with zeros. If the left-most bit in the unsigned number is an x or a z, then an x or a z shall be used to
pad to the left respectively. If the size of the unsigned number is larger than the size specified for the con-
stant, the unsigned number shall be truncated from the left.

The number of bits that make up an unsized number (which is a simple decimal number or a number without
the size specification) shall be at least 32. Unsized unsigned constants where the high order bit is unknown
(X or x) or three-state (Z or z) shall be extended to the size of the expression containing the constant.

NOTE—In IEEE Std 1364-1995 Verilog HDL, in unsized constants where the high order bit is unknown or three-state,
the x or z was only extended to 32 bits.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
14
```
The use of x and z in defining the value of a number is case insensitive.

When used in a number, the question-mark (?) character is a Verilog-AMS HDL alternative for the z char-
acter. It sets 4 bits to the high-impedance value in hexadecimal numbers, 3 bits in octal, and 1 bit in binary.
The question mark can be used to enhance readability in cases where the high-impedance value is a don’t-
care condition. See the discussion of **casez** and **casex** in 9.5.1 of IEEE Std 1364 Verilog. The question-
mark character is also used in user-defined primitive state tables. See Table 8-1 in 8.1.6 of IEEE Std 1364
Verilog.

In a decimal constant, the unsigned number token shall not contain any x, z, or? digits, unless there is
exactly one digit in the token, indicating that every bit in the decimal constant is x or z.

The underscore character ( _ ) shall be legal anywhere in a number except as the first character. The under-
score character is ignored. This feature can be used to break up long numbers for readability purposes.

Example 1 — Unsized literal constant numbers

```
659 // is a decimal number
'h 837FF // is a hexadecimal number
'o7460 // is an octal number
4af // is illegal (hexadecimal format requires 'h)
```
Example 2 — Sized literal constant numbers

```
4'b1001 // is a 4-bit binary number
5 'D 3 // is a 5-bit decimal number
3'b01x // is a 3-bit number with the least
// significant bit unknown
12'hx // is a 12-bit unknown number
16'hz // is a 16-bit high-impedance number
```
Example 3 — Using sign with literal constant numbers

```
8 'd -6 // this is illegal syntax
-8 'd 6 // this defines the two's complement of 6,
// held in 8 bits—equivalent to -(8'd 6)
4 'shf // this denotes the 4-bit number '1111', to
// be interpreted as a 2's complement number,
// or '-1'. This is equivalent to -4'h 1
-4 'sd15 // this is equivalent to -(-4'd 1), or '0001'
16'sd? // the same as 16'sbz
```
Example 4 — Automatic left padding

```
reg [11:0] a, b, c, d;
initial begin
a = 'h x; // yields xxx
b = 'h 3x; // yields 03x
c = 'h z3; // yields zz3
d = 'h 0z3; // yields 0z3
end
reg [84:0] e, f, g;
e = 'h5; // yields {82{1'b0},3'b101}
f = 'hx; // yields {85{1'hx}}
g = 'hz; // yields {85{1'hz}}
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
15
```
Example 5 — Using underscore character in literal constant numbers

```
27_195_000
16'b0011_0101_0001_1111
32 'h 12ab_f001
```
Sized negative constant numbers and sized signed constant numbers are sign-extended when assigned to a
**reg** data type, regardless of whether or not the **reg** itself is signed.

The default length of x and z is the same as the default length of an integer.

#### 2.6.2 Real constants

The _real constant numbers_ are represented as described by IEEE Std 754, an IEEE standard for double pre-
cision floating point numbers.

Real numbers shall be specified in either decimal notation (e.g., 14.72), in scientific notation (e.g., 39e8,
which indicates 39 multiplied by 10 to the 8th power) or in scaled notation (e.g., 24.7K, which indicates
24.7 multiplied by 10 to the third power). Real numbers expressed with a decimal point shall have at least
one digit on each side of the decimal point. The underscore character is legal anywhere in a real constant
except as the first character of the constant or the first character after the decimal point. The underscore
character is ignored.

Examples:

```
1.2
0.1
2394.26331
1.2E12 // the exponent symbol can be e or E
1.30e-2
0.1e-0
23E10
29E-2
236.123_763_e-12 // underscores are ignored
1.3u
7k
```
The following are invalid forms of real numbers because they do not have at least one digit on each side of
the decimal point:

```
.12
9.
4.E3
.2e-7
.1p
34.M
```
Table 2- 1 describes each symbol and their value used in scaled notation or a real number.

```
Table 2-1—Scaled Symbols and notation
```
```
Symbol Value
```
```
T 1e12
G 1e9
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
16
```
No space is permitted between the number and the symbol. Scale factors are not allowed to be used in defin-
ing digital delays (e.g., #5u).

See 4.2.1.1 for a discussion of real to integer conversion and 4.2.1.2 for a discussion of integer to real con-
version.

### 2.7 String literals

A _string literal_ is a sequence of characters enclosed by double quotes (") and contained on a single line. A
_string literal_ used as an operand in expressions and assignments shall be treated as unsigned integer con-
stants represented by a sequence of 8-bit ASCII values, with one 8-bit ASCII value representing one charac-
ter. The **string** variable data type can be used to store a _string literal_ (see 3.3). Parameters of type
**string** are treated differently and are described in 3.4.6.

Certain characters can only be used in a _string literal_ when preceded by an introductory character called an
_escape character_. Table 2- 2 lists these characters in the right-hand column, with the escape sequence that
represents the character in the left-hand column.

```
M 1e6
K, k 1e3
m 1e-3
u 1e-6
n 1e-9
p 1e-12
f 1e-15
a 1e-18
```
```
Table 2-2—Specifying special characters in string
```
```
Escape
string
```
```
Character produced by
escape string
```
```
\n New line character
\t Tab character
\\ \ character
\" " character
\ddd A character specified in 1–3 octal digits
(0  d  7)
```
```
If less than three characters are used, the fol-
lowing character shall not be an octal digit.
Implementations may issue an error if the char-
acter represented is greater than \377.
```
```
Table 2-1—Scaled Symbols and notation (continued)
```
```
Symbol Value
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
17
```
### 2.8 Identifiers, keywords, and system names ......................................................................................

An _identifier_ shall be used to give an object a unique name so it can be referenced. An _identifier_ shall either
be a _simple identifier_ or an _escaped identifier_ (see 2.8.1). A _simple identifier_ shall be any sequence of letters,
digits, dollar signs ( **$** ), and the underscore characters (_).

The first character of an _identifier_ shall not be a digit or $; it can be a letter or an underscore. _Identifiers_ shall
be case sensitive.

Examples:

```
shiftreg_a
busa_index
error_condition
merge_ab
_bus3
n$657
```
Implementations may set a limit on the maximum length of identifiers, but the limit shall be at least 1024
characters. If an identifier exceeds the implementation-specified length limit, an error shall be reported.

#### 2.8.1 Escaped identifiers

_Escaped identifiers_ shall start with the backslash character (\) and end with white space (space, tab, newline,
or formfeed). They provide a means of including any of the printable ASCII characters in an identifier (the
decimal values 33 through 126 or 21 through 7E in hexadecimal).

Neither the leading backslash character nor the terminating white space is considered to be part of the iden-
tifier. Therefore, an escaped identifier \cpu3 is treated the same as a non-escaped identifier cpu3.

Examples:

```
\busa+index
\-clock
\***error-condition***
\net1/\net2
\{a,b}
\a*(b+c)
```
#### 2.8.2 Keywords

_Keywords_ are predefined simple identifiers which are used to define the language constructs. A Verilog-
AMS HDL keyword preceded by an escape character is not interpreted as a keyword.

All keywords are defined in lowercase only. Annex B lists all defined Verilog-AMS HDL keywords.

#### 2.8.3 System tasks and functions

The $ character introduces a language construct which enables development of user-defined system tasks
and functions. System constructs are not design semantics, but refer to simulator functionality. A name fol-
lowing the $ is interpreted as a _system task_ or a _system function_.

The syntax for a system task or function is given in Syntax 2- 3.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
18
```
analog_system_task_enable ::= _// from A.6.9_
analog_system_task_identifier [ **(** [ analog_expression ] { **,** [ analog_expression ] } **)** ] **;**

system_task_enable ::= system_task_identifier [ **(** [ expression ] { **,** [ expression ] } **)** ] **;**

analog_system_function_call ::= _// from A.8.2_
analog_system_function_identifier [ **(** [ analog_expression ] { **,** [ analog_expression ] } **)** ]

system_function_call ::= system_function_identifier
[ **(** expression { **,** expression } **)** ]

system_function_identifier^1 ::= **$** [ **a** - **zA** - **Z0** - **9_$** ] { [ **a** - **zA** - **Z0** - **9_$** ] } _// from A.9.3_

system_task_identifier^1 ::= **$** [ **a** - **zA** - **Z0** - **9_$** ] { [ **a** - **zA** - **Z0** - **9_$** ] }

```
Syntax 2-3—Syntax for system tasks and functions
```
The $ _identifier_ system task or function can be defined in five places,

```
— A standard set of $ identifier system tasks and functions, as defined in Clause 17 and Clause 18 of
IEEE Std 1364 Verilog.
— Additional $ identifier system tasks and functions defined using the PLI, as described in Clause 20 of
IEEE Std 1364 Verilog.
— Additional $ identifier system tasks and functions defined in Clause 4 and Clause 9 of this standard.
— Additional $ identifier system tasks and functions defined using the VPI as described in Clause 11
and Clause 12 of this standard.
— Additional $ identifier system tasks and functions defined by software implementations.
```
Any valid identifier, including keywords already in use in contexts other than this construct, can be used as a
system task or function name.

Examples:

```
$display ("display a message");
$finish ;
```
#### 2.8.4 Compiler directives

The ` character (the ASCII value 0x60, called grave accent) introduces a language construct used to imple-
ment compiler directives. The compiler behavior dictated by a compiler directive shall take effect as soon as
the compiler reads the directive. The directive shall remain in effect for the rest of the compilation unless a
different compiler directive specifies otherwise. A compiler directive in one description file can therefore
control compilation behavior in multiple description files.

The _`identifier_ compiler directive construct can be defined in three places

```
— A standard set of `identifier compiler directives defined in Clause 19 of IEEE Std 1364 Verilog.
— Additional ` identifier compiler directives defined in Clause 10 of this standard.
— Additional ` identifier compiler directives defined by software implementations.
```
Any valid identifier, including keywords already in use in contexts other than this construct, can be used as a
compiler directive name.

(^1) The $ character in a system_function_identifier, system_task_identifier, or system_parameter_identifier shall not be followed by
white_space. A system_function_identifier or system_task_identifier shall not be escaped.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
19
```
Example:

```
`define WORDSIZE 8
```
### 2.9 Attributes........................................................................................................................................

With the proliferation of tools other than simulators that use Verilog-AMS HDL as their source, a mecha-
nism is included for specifying properties about objects, statements and groups of statements in the HDL
source that can be used by various tools, including simulators, to control the operation or behavior of the
tool. These properties shall be referred to as _attributes_. This section specifies the syntactic mechanism that
shall be used for specifying attributes.

The syntax is found in Syntax 2- 4.

attribute_instance ::= **(*** attr_spec { **,** attr_spec } ***)** _// from A.9.1_

attr_spec ::= attr_name [ **=** constant_expression ]

attr_name ::= identifier

```
Syntax 2-4—Syntax for attributes
```
An _attribute_instance_ can appear in the Verilog-AMS description as a prefix attached to a declaration, a
module item, a statement, or a port connection. It can appear as a suffix to an operator or a Verilog-AMS
function name in an expression.

If a value is not specifically assigned to the attribute, then its value shall be 1. If the same attribute name is
defined more than once for the same language element, the last attribute value shall be used and a tool can
give a warning that a duplicate attribute specification has occurred.

Nesting of attribute instances is disallowed. It shall be illegal to specify the value of an attribute with a con-
stant expression that contains an attribute instance.

Example 1 — The following example shows how to attach attributes to a case statement:

```
(* full_case, parallel_case *)
case (foo)
<rest_of_case_statement>
```
or

```
(* full_case=1 *)
(* parallel_case=1 *) // Multiple attribute instances also OK
case (foo)
<rest_of_case_statement>
```
or

```
(* full_case, // no value assigned
parallel_case=1 *)
case (foo)
<rest_of_case_statement>
```
Example 2 — To attach the full_case attribute, but NOT the parallel_case attribute:


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
20
```
```
(* full_case *) // parallel_case not specified
case (foo)
<rest_of_case_statement>
```
or

```
(* full_case=1, parallel_case = 0 *)
case (foo)
<rest_of_case_statement>
```
Example 3 — To attach an attribute to a module definition:

```
(* optimize_power *)
module mod1 (<port_list>);
```
or

```
(* optimize_power=1 *)
module mod1 (<port_list>);
```
Example 4 — To attach an attribute to a module instantiation:

```
(* optimize_power=0 *)
mod1 synth1 (<port_list>);
```
Example 5 — To attach an attribute to a **reg** declaration:

```
(* fsm_state *) reg [7:0] state1;
(* fsm_state=1 *) reg [3:0] state2, state3;
reg [3:0] reg1; // this reg does NOT have fsm_state set
(* fsm_state=0 *) reg [3:0] reg2; // nor does this one
```
Example 6 — To attach an attribute to an operator:

```
a = b + (* mode = "cla" *) c;
```
```
This sets the value for the attribute mode to be the string cla.
```
Example 7 — To attach an attribute to a Verilog function call:

```
a = add (* mode = "cla" *) (b, c);
```
Example 8 — To attach an attribute to a conditional operator:

```
a = b? (* no_glitch *) c : d;
```
#### 2.9.1 Syntax................................................................................................................................

The syntax for legal statements with attributes is shown in Syntax 2- 5 through Syntax 2- 10.

The syntax for module declaration attributes is given in Syntax 2- 5.

module_declaration ::= _// from A.1.2_
{ attribute_instance } module_keyword module_identifier [ module_parameter_port_list ]
list_of_ports **;** { module_item }


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
21
```
```
endmodule
| { attribute_instance } module_keyword module_identifier [ module_parameter_port_list ]
[ list_of_port_declarations ] ; { non_port_module_item }
endmodule
```
```
Syntax 2-5—Syntax for module declaration attributes
```
The syntax for port declaration attributes is given in Syntax 2- 6.

port_declaration ::= _// from A.1.3_
{attribute_instance} inout_declaration
| {attribute_instance} input_declaration
| {attribute_instance} output_declaration

```
Syntax 2-6—Syntax for port declaration attributes
```
The syntax for module item attributes is given in Syntax 2- 7.

module_item ::= _// from A.1.4_
port_declaration **;**
| non_port_module_item

module_or_generate_item ::=
{ attribute_instance } module_or_generate_item_declaration
| { attribute_instance } local_parameter_declaration **;**
| { attribute_instance } parameter_override
| { attribute_instance } continuous_assign
| { attribute_instance } gate_instantiation
| { attribute_instance } udp_instantiation
| { attribute_instance } module_instantiation
| { attribute_instance } initial_construct
| { attribute_instance } always_construct
| { attribute_instance } loop_generate_construct
| { attribute_instance } conditional_generate_construct
| { attribute_instance } analog_construct

module_or_generate_item_declaration ::=
net_declaration
| reg_declaration
| integer_declaration
| real_declaration
| time_declaration
| realtime_declaration
| event_declaration
| genvar_declaration
| task_declaration
| function_declaration
| branch_declaration
| analog_function_declaration

non_port_module_item ::=
module_or_generate_item
| generate_region


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
22
```
```
| specify_block
| { attribute_instance } parameter_declaration ;
| { attribute_instance } specparam_declaration
| aliasparam_declaration
```
```
Syntax 2-7—Syntax for module item attributes
```
The syntax for function port, task, and block attributes is given in Syntax 2- 8.

function_port_list ::= _// from A.2.6_
{ attribute_instance } tf_input_declaration { **,** { attribute_instance } tf_input_declaration }

task_item_declaration ::= _// from A.2.7_
block_item_declaration
| { attribute_instance } tf_input_declaration **;**
| { attribute_instance } tf_output_declaration **;**
| { attribute_instance } tf_inout_declaration **;**

task_port_item ::=
{ attribute_instance } tf_input_declaration
| { attribute_instance } tf_output_declaration
| { attribute_instance } tf_inout_declaration

block_item_declaration ::= _// from A.2.8_
{ attribute_instance } **reg** [ discipline_identifier ] [ **signed** ] [ range ]
list_of_block_variable_identifiers **;**
| { attribute_instance } **integer** list_of_block_variable_identifiers **;**
| { attribute_instance } **time** list_of_block_variable_identifiers **;**
| { attribute_instance } **real** list_of_block_real_identifiers **;**
| { attribute_instance } **realtime** list_of_block_real_identifiers **;**
| { attribute_instance } event_declaration
| { attribute_instance } local_parameter_declaration **;**
| { attribute_instance } parameter_declaration **;**

```
Syntax 2-8—Syntax for function port, task, and block attributes
```
The syntax for port connection attributes is given in Syntax 2- 9.

ordered_port_connection ::= { attribute_instance } [ expression ] _// from A.4.1_

named_port_connection ::= { attribute_instance }**.** port_identifier **(** [ expression ] **)**

```
Syntax 2-9—Syntax for port connection attributes
```
The syntax for udp attributes is given in Syntax 2- 10.

udp_declaration ::= _// from A.5.1_
{ attribute_instance } **primitive** udp_identifier **(** udp_port_list **) ;**
udp_port_declaration { udp_port_declaration }
udp_body
**endprimitive**
| { attribute_instance } **primitive** udp_identifier **(** udp_declaration_port_list **) ;**
udp_body


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
23
```
```
endprimitive
```
udp_output_declaration ::= _// from A.5.2_
{ attribute_instance } **output** port_identifier
| { attribute_instance } **output** [ discipline_identifier ] **reg** port_identifier [ **=** constant_expression ]

udp_input_declaration ::= { attribute_instance } **input** list_of_port_identifiers

udp_reg_declaration ::= { attribute_instance } **reg** [ discipline_identifier ] variable_identifier

```
Syntax 2-10—Syntax for udp attributes
```
#### 2.9.2 Standard attributes.............................................................................................................

The Verilog-AMS HDL standardizes the following attributes:

```
— The desc attribute is used to generate help messages when attached to parameter, variable and net
declarations within a module. The attribute must be assigned a string. See 3.4.3.
— The units attribute is used to describe the units of the parameter or variable which it is attached to
within a module. The attribute must be assigned a string. See 3.4.3.
— The op attribute is used to indicate whether a parameter or variable should be included in a short
report of the most useful operating-point values. The attribute must be assigned a value, which must
be either "yes" or "no". If the attribute is specified with the value "no", then the parameter or vari-
able will be omitted from the short report; otherwise, the parameter or variable will be included.
— The multiplicity attribute is used to describe how the value of a parameter or variable should be
scaled for reporting. The attribute must be assigned one of the following string values: "multiply",
"divide", or "none". If the attribute is specified with the value "multiply", the value for the
associated parameter or variable will be multiplied by the value of $mfactor for the instance in
any report of operating-point values. If the attribute is specified with the value "divide", the value
for the associated parameter or variable will be divided by the value of $mfactor for the instance
in any report of operating-point values. If the multiplicity attribute is not specified, or specified
with the value "none", then no scaling will be performed. Note that this scaling only applies to oper-
ating-point value reports; it does not affect the automatic scaling detailed in 6.3.6.
```
Example - The following example shows how to attach standard attributes to a variable:
(* desc="effective resistance", units="Ohms", op="yes",
multiplicity="divide" *)
**real** reff;


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
24
```
