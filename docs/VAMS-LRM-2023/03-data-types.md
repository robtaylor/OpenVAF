## 3. Data types.................................................................................................................................................

### 3.1 Overview........................................................................................................................................

Verilog-AMS HDL supports **integer** , **genvar** , **real** , and **parameter** data types as found in IEEE Std
1364 Verilog. It includes the **string** data type defined by IEEE Std 1800 SystemVerilog_._ It also modifies
the **parameter** data types. Plus, it extends the net data types to support a new type called **wreal** to model
real value nets.

Verilog-AMS HDL introduces a new data type, called _net_discipline_ , for representing analog nets and
declaring _disciplines_ of all nets and regs. The _disciplines_ define the **domain** and the natures of **poten-
tial** and **flow** and their associated attributes for _continuous_ domains.

### 3.2 Integer and real data types

The syntax for declaring **integer** and **real** is shown in Syntax 3- 1.

integer_declaration ::= **integer** list_of_variable_identifiers **;** _// from A.2.1.3_

real_declaration ::= **real** list_of_real_identifiers **;**

list_of_real_identifiers ::= real_type { **,** real_type } _// from A.2.3_

list_of_variable_identifiers ::= variable_type { **,** variable_type }

real_type ::= _// from A.2.2.1_
real_identifier { dimension } [ **=** constant_assignment_pattern ]
| real_identifier **=** constant_expression

variable_type ::=
variable_identifier { dimension } [ **=** constant_assignment_pattern ]
| variable_identifier **=** constant_expression

dimension ::= **[** dimension_constant_expression **:** dimension_constant_expression **]** _// from A.2.5_

```
Syntax 3-1—Syntax for integer and real declarations
```
An _integer_ declaration declares one or more variables of type integer. These variables can hold values rang-
ing from -2^31 to 2^31 -1. Arrays of integers can be declared using a range which defines the upper and lower
indices of the array. Both indices shall be constant expressions and shall evaluate to a positive integer, a neg-
ative integer, or zero ( 0 ).

Arithmetic operations performed on integer variables produce 2’s complement results.

A _real_ declaration declares one or more variables of type **real**. The real variables are stored as 64-bit quan-
tities, as described by IEEE Std 754, an IEEE standard for double precision floating point numbers.

Arrays of **parameter** can be declared using a range which defines the upper and lower indices of the
array. Both indices shall be constant expressions and shall evaluate to a positive integer, a negative integer,
or zero ( 0 ).

Integers are initialized at the start of a simulation depending on how they are used. Integer variables whose
values are assigned in an analog context default to an initial value of zero ( 0 ). Integer variables whose values
are assigned in a digital context default to an initial value of x. Real variables are initialized to zero ( 0 ) at the
start of a simulation.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
25
```
Examples:

```
integer a[1:64]; // an array of 64 integer values
real float; // a variable to store real value
real gain_factor[1:30]; // array of 30 gain multipliers
// with floating point values
integer flag_array[0:8][0:3]; // a multidimensional integer array
real vtable[0:16][0:7][0:64]; // a multidimensional real array
```
See 4.2.1.1 for a discussion of real to integer conversion and 4.2.1.2 for a discussion of integer to real con-
version.

#### 3.2.1 Output variables

The standard attributes for descriptions and units, described in 2.9.2, have a special meaning for variables
declared at module scope. Module scope variables with a description or units attribute, or both, shall be
known as output variables, and Verilog-AMS simulators shall provide access to their values. SPICE-like
simulators print the names, values, units, and descriptions of output variables for SPICE primitives along
with voltages and currents when displaying operating-point information, and these variables are available
for plotting as a function of time (or the swept variable of a dc sweep).

For example, a module for a MOS transistor with the following declaration at module scope provides the
output variable cgs.

```
(* desc="gate-source capacitance", units="F" *)
real cgs;
```
An operating-point display from the simulator might include the following information:

```
cgs = 4.21e-15 F gate-source capacitance
```
Units and descriptions specified for block-level variables shall be ignored by the simulator, but can be used
for documentation purposes.

### 3.3 String data type

Verilog-AMS includes the **string** data type from IEEE Std 1800 SystemVerilog, which is an ordered col-
lection of characters. The length of a **string** variable is the number of characters in the collection. Vari-
ables of type **string** are dynamic as their length may vary during simulation.

IEEE Std 1364 Verilog supports string literals, but only at the lexical level. In Verilog, string literals behave
like packed arrays of a width that is a multiple of 8 bits. A string literal assigned to a packed array of an inte-
gral variable of a different size is either truncated to the size of the variable or padded with zeroes to the left
as necessary.

In Verilog-AMS, string literals behave exactly the same as in Verilog. However, Verilog-AMS also supports
the **string** data type to which a string literal can be assigned. When using the **string** data type instead of
an integral variable, strings can be of arbitrary length and no truncation occurs. Literal strings are implicitly
converted to the **string** type when assigned to a **string** type or used in an expression involving
**string** type operands.

The **string** variables can take on the special value "", which is the empty string. A **string** shall not con-
tain the special character "\0".


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
26
```
The syntax to declare a **string** is as follows:

```
string variable_name [ = initial_value ] ;
```
where _variable_name_ is a valid identifier and the optional _initial_value_ can be a string literal, the value ""
for an empty string, or a string type constant expression, such as a string parameter (see 3.4.6). For example:

```
parameter string default_name = "John Smith";
string myName = default_name;
```
If an initial value is not specified in the declaration, the variable is initialized to "", the empty string. An
empty string has zero length.

Arrays and multidimensional arrays of string are also supported. For example:

```
string names[1:3] = '{"first", "middle", "last"};
string paths[0:2][0:1] =
'{ '{"dir1", "fileA"}, '{"dir2", "fileA"},'{"dir1","fileB"}};
```
Verilog-AMS provides a set of operators that can be used to manipulate combinations of string variables and
string literals. The basic operators defined on the **string** data type are listed in Table 3- 3.

A string literal can be assigned to a **string** or an integral type. If their size differs, the literal is right justi-
fied and either truncated on the left or zero filled on the left, as necessary. For example:

```
reg [8*4:1] h = "hello"; // assigns to h "ello"
reg [10:0] a = "A"; // assigns to a 'b000_0100_0001
```
A **string** or a string literal can be assigned directly to a **string** variable. A **string** cannot be assigned
to an integral type. A string literal assigned to a **string** variable is converted according to the following
steps:

```
— All "\0" characters in the string literal are ignored (i.e., removed from the string ).
— If the result of the first step is an empty string literal, the string is assigned the empty string.
— Otherwise, the string is assigned the remaining characters in the string literal.
```
For example:

```
string s1 = "hello\0world"; // sets s1 to "helloworld"
```
As a second example:

```
reg [15:0] r;
integer i = 1;
string b = "";
string a = {"Hi", b};
b = "Hi"; // OK
b = {5{"Hi"}}; // OK
a = {i{"Hi"}}; // OK (non constant replication)
r = {i{"Hi"}}; // invalid (non constant replication)
a = {i{b}}; // OK
a = {a,b}; // OK
a = {"Hi",b}; // OK
r = {"H",""}; // yields "H\0". "" is converted to 8’b0
b = {"H",""}; // yields "H". "" is the empty string
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
27
```
### 3.4 Parameters......................................................................................................................................

The syntax for parameter declarations is shown in Syntax 3- 2.

The list of parameter assignments shall be a comma-separated list of assignments, where the right hand side
of the assignment, called the initializer, shall be a constant expression, that is, an expression containing only
constant numbers and previously defined parameters.

For parameters defined as arrays, the initializer shall be a _constant_assignment_pattern_ expression which is
a list of constant expressions containing only constant numbers and previously defined parameters using an
assignment pattern (see 4.2.14) , i.e. within **'{** and **}** delimiters.

Parameters represent constants, hence it is illegal to modify their value at runtime. However, parameters can
be modified at compilation time to have values which are different from those specified in the declaration
assignment. This allows customization of module instances. A parameter can be modified with the defparam
statement or in the _module_instantiation_ statement.

local_parameter_declaration ::= _// from A.2.1.1_
**localparam** [ **signed** ] [ range ] list_of_param_assignments
| **localparam** parameter_type list_of_param_assignments

parameter_declaration ::=
**parameter** [ **signed** ] [ range ] list_of_param_assignments
| **parameter** parameter_type list_of_param_assignments

```
Table 3-3—String operators
```
```
Operator Semantics
```
```
Str1 == Str2 Equality. Checks whether the two strings are equal. Result is 1 if they are equal and
0 if they are not. Both strings can be of type string , or one of them can be a
string literal which is implicitly converted to a string type for the comparison. If
both operands are string literals, the operator is the same Verilog equality operator
as for integer types.
Str1 != Str2 Inequality. Logical negation of ==
Str1 < Str2
Str1 <= Str2
Str1 > Str2
Str1 >= Str2
```
```
Comparison. Relational operators return 1 if the corresponding condition is true
using the lexicographical ordering of the two strings Str1 and Str2. Both oper-
ands can be of type string , or one of them can be a string literal which is implic-
itly converted to a string type for the comparison.
{Str1,Str2,...,Strn} Concatenation. Each operand can be of type string or a string literal (it shall be
implicitly converted to type string ). If at least one operand is of type string ,
then the expression evaluates to the concatenated string and is of type string. If
all the operands are string literals, then the expression behaves like a Verilog con-
catenation of integral types; if the result is then used in an expression involving
string types, it is implicitly converted to the string type.
{multiplier{Str}} Replication. Str can be of type string or a string literal. multiplier must be
of integral type and can be nonconstant. If multiplier is nonconstant or Str is
of type string , the result is a string containing N concatenated copies of Str,
where N is specified by multiplier. If Str is a literal and multiplier is
constant, the expression behaves like numeric replication in Verilog (if the result is
used in another expression involving string types, it is implicitly converted to the
string type).
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
28
```
specparam_declaration ::= **specparam** [ range ] list_of_specparam_assignments **;**

parameter_type ::=
**integer** | **real** | **realtime** | **time** | **string**

aliasparam_declaration ::= **aliasparam** parameter_identifier **=** parameter_identifier **;**

list_of_param_assignments ::= param_assignment { **,** param_assignment } _// from A.2.3_

param_assignment ::= _// from A.2.4_
parameter_identifier **=** constant_mintypmax_expression { value_range }
| parameter_identifier range **=** constant_assignment_pattern { value_range }

range ::= **[** msb_constant_expression **:** lsb_constant_expression **]** _// from A.2.5_

value_range ::=
value_range_type **(** value_range_expression **:** value_range_expression **)**
| value_range_type **(** value_range_expression **:** value_range_expression **]**
| value_range_type **[** value_range_expression **:** value_range_expression **)**
| value_range_type **[** value_range_expression **:** value_range_expression **]**
| value_range_type **'{** string { **,** string } **}**
| **exclude** constant_expression

value_range_type :: = **from** | **exclude**

value_range_expression ::= constant_expression | **-inf** | **inf**

```
Syntax 3-2—Syntax for parameter declaration
```
By nature, analog behavioral specifications are characterized more extensively in terms of parameters than
their digital counterparts. There are three fundamental extensions to the parameter declarations defined in
IEEE Std 1364 Verilog:

```
— A range of permissible values can be defined for each parameter. In IEEE Std 1364 Verilog, this
check had to be done in the user’s model or was left as an implementation specific detail.
— Parameter arrays of basic integer and real data types can be specified.
— String parameters may be declared.
```
#### 3.4.1 Type specification

The parameter declaration can contain an optional _type_ specification. In this sense, the **parameter** key-
word acts more as a type qualifier than a type specifier. A default value for the parameter shall be specified.

The following examples illustrate this concept:

```
parameter real slew_rate = 1e-3;
parameter integer size = 16;
```
If the _type_ of a parameter is not specified, it is derived from the type of the final value assigned to the param-
eter, after any value overrides have been applied, as in IEEE Std 1364 Verilog. Note that the _type_ of a string
parameter (see 3.4.6) and any of the array parameters (see 3.4.4) is mandatory.

If the type of the parameter is specified as **integer** or **real** , and the value assigned to the parameter con-
flicts with the type of the parameter, the value is converted to the type of the parameter (see 4.2.1.1). No
conversion shall be applied for strings; it shall be an error to assign a numeric value to a parameter declared
as **string** or to assign a string value to a **real** parameter, whether that parameter was declared as **real**
or had its type derived from the type of the value of the constant expression.

Example:


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
29
```
```
parameter real size = 10;
```
Here, size is coerced to 10.0.

#### 3.4.2 Value range specification ..................................................................................................

A parameter declaration can contain optional specifications of the permissible range of the values of a
parameter. More than one range can be specified for inclusion or exclusion of values as legal values for the
parameter.

Brackets, **[** and **]** , indicate inclusion of the end points in the value range. Parentheses, **(** and **)** , indicate
exclusion of the end points from the value range. It is possible to include one end point and not the other
using **[ )** and **( ]**. The first expression in the range shall be numerically smaller than the second expres-
sion in the range.

Examples:

```
parameter real neg_rail = -15 from [-50:0);
parameter integer pos_rail = 15 from (0:50);
parameter real gain = 1 from [1:1000];
```
Here, the default value for neg_rail is -15 and it is only allowed to acquire values within the range of
-50 <= neg_rail < 0. Similarly, the default value for parameter pos_rail is 15 and it is only allowed
to acquire values within the range of 0 < pos_rail < 50. And, the default value for gain is 1 and it is
allowed to acquire values within the range of 1 <= gain <= 1000.

The keyword **inf** can be used to indicate infinity. If preceded by a negative sign, it indicates negative infin-
ity.

Example:

```
parameter real val3=0 from [0: inf ) exclude (10:20) exclude (30:40];
```
A single value can be excluded from the possible valid values for a parameter.

Example:

```
parameter real res = 1.0 exclude 0;
```
Here, the value of a parameter is checked against the specified range. Range checking applies to the value of
the parameter for the instance and not against the default values specified in the device. It shall be an error
only if the value of the parameter is out of range during simulation.

Valid values of string parameters are indicated differently. The **from** keyword may be used with a list of
valid string values, or the exclude keyword may be used with a list of invalid string values. In either case, the
list is constructed using an assignment pattern (see 4.2.14), i.e. enclosed in braces preceded by an apostro-
phe, **'{ }** , and the items are separated by commas.

Examples:

```
parameter string transistortype = "NMOS" from ' { "NMOS", "PMOS" };
parameter string filename = "output.dat" exclude ' { "" };
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
30
```
#### 3.4.3 Parameter units and descriptions.......................................................................................

The standard attributes for descriptions and units, described in 2.9.2, can be used for parameters.

Example:

```
(* desc="Resistance", units="Ohms" *)
parameter real res = 1.0 from [0: inf );
```
The units and descriptions are only for documentation of the module; in particular, no dimensional analysis
is performed on the units. However, it is often important for the user to know the units of a parameter, such
as an angle that could be specified in radians or degrees. It should be noted that the **‘timescale** directive
of IEEE Std 1364 Verilog also affects units throughout the module, which can be confusing to the user.

The units and descriptions are of particular value for compact models, where the number of parameters is
large and the description is not always clear from the parameter name. Simulators can use this information
when generating help messages for a module; many SPICE-like simulators can generate help messages with
this information for built-in primitives.

Units and descriptions specified for block-level parameters shall be ignored by the simulator, but can be
used for documentation purposes.

#### 3.4.4 Parameter arrays................................................................................................................

Verilog-AMS HDL includes behavioral extensions which utilize arrays. It requires these arrays be initialized
in their definitions and allows overriding their values, as with other parameter types.Parameter arrays have
the following restrictions. Failure to follow these restrictions shall result in an error.

```
— A type of a parameter array shall be given in the declaration.
— An array assigned to an instance of a module to override the default value of an array parameter
shall be of the exact size of the parameter array, as determined by its declaration.
— Since array range in the parameter array declaration may depend on previously-declared parameters,
the array size may be changed by overriding the appropriate parameters. If the array size is changed,
the parameter array shall be assigned an array of the new size from the same module as the parame-
ter assignment that changed the parameter array size.
```
Example:

```
parameter real poles[0:3] = '{ 1.0, 3.198, 4.554, 2.00 };
```
#### 3.4.5 Local parameters

IEEE Std 1364 Verilog local parameters, identified by the **localparam** keyword, are identical to parame-
ters except that they cannot directly be modified with the **defparam** statement or by the ordered or named
parameter value assignment, as described in 6.3. Local parameters can be assigned constant expressions con-
taining parameters, which can be modified with **defparam** statements or module instance parameter value
assignments.

#### 3.4.6 String parameters

String parameters can be declared. Strings are useful for parameters that act as flags, where the correspon-
dence between numerical values and the flag values may not be obvious. The set of allowed values for the
string can be specified as a comma-separated list of strings inside curly braces. String parameters may be
used with the string operators listed in Table 3- 3.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
31
```
Example:

```
module ebersmoll (c,b,e);
inout c, b, e;
electrical c, b, e;
parameter string transistortype = "NPN" from ' { "NPN", "PNP" };
parameter real alphaf = 0.99 from (0: inf );
parameter real alphar = 0.5 from (0: inf );
parameter real ies = 1.0e-17 from (0: inf );
parameter real ics = 1.0e-17 from (0: inf );
real sign, ifor, irev;
analog begin
sign = (transistortype == "NPN")? 1.0 : -1.0;
ifor = ies * ( limexp (sign*V(b,e)/ $vt )-1);
irev = ics * ( limexp (sign*V(b,c)/ $vt )-1);
I(b,e) <+ sign*(ifor - alphar * irev);
I(b,c) <+ sign*(irev - alphaf * ifor);
end
endmodule
```
Note how the string parameter transistortype associates the string "PNP" with a negative one (-1) value
for the variable sign. It is common in compact modeling of transistors for the equations to be formulated
for NPN or NMOS devices, and behavior of a PNP or PMOS can be described by multiplying all the volt-
ages and currents by -1, even though the “p” denotes positively-charged carriers in the channel of the
PMOS.

#### 3.4.7 Parameter aliases

Aliases can be defined for parameters. This allows an alternate name to be used when overriding module
parameter values as described in 6.3. Parameters with different names may be used for the same purpose in
different simulators; some compact models accept parameter names with the letter “O” in place of the num-
ber “0.”

Parameter aliases are subject to the following rules.

```
— The type of an alias ( real , integer , or string ) shall be determined by the original parameter,
as is its range of allowed values, if specified.
— The alias_identifier shall not occur anywhere else in the module; in particular, it shall not conflict
with a different parameter_identifier , and the equations in the module shall reference the parameter
by its original name, not the alias.
— Multiple aliases can point to the same parameter.
— When overriding parameters, it shall be an error to specify an override for a parameter by its original
name and one or more aliases, or by more than one alias, regardless of how the override is done (by
name or using the defparam statement).
— When the simulator generates a list of parameter values used, such as for an operating point analysis,
only the original name shall appear in the list.
```
For example, suppose a module named nmos2 has the following declarations in the module:

```
parameter real dtemp = 0 from [-‘P_CELSIUS0: inf );
aliasparam trise = dtemp;
```
Then the following two instantiations of the module are valid:

```
nmos2 #(.trise(5)) m1(.d(d), .g(g), .s(s), .b(b));
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
32
```
```
nmos2 #(.dtemp(5)) m2(.d(d), .g(g), .s(s), .b(b));
```
and the value of the parameter dtemp, as used in the module equations for both instances, is 5.

This last instantiation is an error:

```
nmos2 #(.trise(5), .dtemp(5)) m3(.d(d), .g(g), .s(s), .b(b)); //error
```
because an override is specified for the parameter dtemp and its alias, even though the values are equal.

Parameter aliases may also be declared for the hierarchical parameter system functions (see 9.18) as in the
example below:

```
aliasparam m = $mfactor;
```
#### 3.4.8 Multidimensional parameter array examples

The following example demonstrates the usage of a multidimensional real array parameter and various
usages of assignment patterns.
**module** test;
electrical out[0:2];
electrical in[0:2];
/* Instantiate crosstalk module passing a
* multidimensional parameter array literal
* for channel coupling
*/
crosstalk #(.c('{'{0.0,0.1,0.1},'{0.1,0.0,0.1},'{0.1,0.1,0.0}}))
C1(out,in,1'b1);

```
gen G1(in);
sink S1(out);
endmodule
```
```
module crosstalk(out, in, distort_enable);
input in[0:2];
input distort_enable;
output out[0:2];
// A multidimensional real parameter array for channel coupling
parameter real c[0:2][0:2] =
'{'{0.0,0.2,0.2},'{0.2,0.0,0.2},'{0.2,0.2,0.0}};
```
```
electrical in[0:2];
electrical out[0:2];
```
```
/* A multidimensional real variable to hold the distortion calculations
* all elements are initialized to 0.0 using
* an assignment pattern and replication operator
*/
real distort[0:2][0:2] = '{ 3{ '{3{0.0}}}};
```
```
/* multidimensional string to flag excessive distortion
* all elements are initialized to " " using
* an assignment pattern and replication operator
*/
string above_0p5[0:2][0:2] = '{ 3{ '{3{" "}}}};
```
```
real in_val[0:2];
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
33
```
```
integer ii, jj;
analog begin
// assign to variable using an assignment pattern
in_val = '{V(in[0]),V(in[1]),V(in[2])};
```
```
if (distort_enable) begin
for ( ii=0; ii <= 2; ii=ii+1 ) begin
for (jj=0; jj<= 2; jj=jj+1 ) begin
distort[ii][jj] = c[ii][jj]*in_val[jj];
if (distort[ii][jj] > 0.1)
above_0p5[ii][jj] = "*";
end
end
end
```
```
V(out[0]) <+ in_val[0] + distort[0][1] + distort[0][2];
V(out[1]) <+ distort[1][0] + in_val[1] + distort[1][2];
V(out[2]) <+ distort[2][0] + distort[2][1] + in_val[2];
```
```
@( final_step ) begin
$display ("Table of distortions greater than 0.5");
$display ("#012"); // write the table header
for ( ii=0; ii <= 2; ii=ii+1 ) begin
$write ("%0d",ii); // %0d means write int in minimum width
for (jj=0; jj<= 2; jj=jj+1 ) begin
$write (above_0p5[ii][jj]);
end
$display ; // print a newline
end
end
end
endmodule
```
### 3.5 Genvars

_Genvars_ are integer-valued variables which compose static expressions for instantiating structure behavior-
ally such as accessing analog signals within behavioral looping constructs. The syntax for declaring genvar
variables is shown in Syntax 3- 3.

genvar_declaration ::= _// from A.4.2_
**genvar** list_of_genvar_identifiers **;**

list_of_genvar_identifiers ::=
genvar_identifier { **,** genvar_identifier }

```
Syntax 3-3—Syntax for genvar declaration
```
The static nature of genvar variables is derived from the limitations upon the contexts in which their values
can be assigned.

Examples:

```
genvar i;
analog begin
...
for (i = 0; i < 8; i = i + 1) begin
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
34
```
```
V(out[i]) <+ transition (value[i], td, tr);
end
```
```
end
```
The genvar variable i can only be assigned within the for-loop control. Assignments to the genvar variable
i can consist only of expressions of static values, e.g., parameters, literals, and other genvar variables.

### 3.6 Net_discipline

In addition to the data types supported by IEEE Std 1364 Verilog, an additional data type, _net_discipline_ , is
introduced in Verilog-AMS HDL for continuous time and mixed-signal simulation. _net_discipline_ is used to
declare analog nets, as well as declaring the domains of digital nets and regs.

A signal can be digital, analog, or mixed, and is a hierarchical collection of nets which are contiguous
(because of port connections). For analog and mixed signals, a single node is associated with all continuous
net segments of the signal. The fundamental characteristic of analog and mixed signals is the values of the
associated _node_ are determined by the simultaneous solution of equations defined by the instances con-
nected to the node using Kirchhoff’s conservation laws. In general, a _node_ represents a point of physical
connections between nets of continuous-time description and it obeys conservation-law semantics.

A net is characterized by the discipline it follows. For example, all low-voltage nets have certain common
characteristics, all mechanical nets have certain common characteristics, etc. Therefore, a _net_ is always
declared as a type of discipline. In this sense, a discipline is a user-defined type for declaring a net.

A _discipline_ is characterized by the domain and the attributes defined in the _natures_ for **potential** and
**flow**.

#### 3.6.1 Natures

A _nature_ is a collection of attributes. In Verilog-AMS HDL, there are several pre-defined attributes. In addi-
tion, user-defined attributes can be declared and assigned constant values in a nature.

The nature declarations are at the same level as discipline and module declarations in the source text. That is,
natures are declared at the top level and nature declarations do not nest inside other nature declarations, dis-
cipline declarations, or module declarations.

The syntax for defining a nature is shown in Syntax 3- 4.

nature_declaration ::= _// from A.1.6_
**nature** nature_identifier [ **:** parent_nature ] [ **;** ]
{ nature_item }
**endnature**

parent_nature ::=
nature_identifier
| discipline_identifier**.** potential_or_flow

nature_item ::= nature_attribute

nature_attribute ::= nature_attribute_identifier **=** nature_attribute_expression **;**

potential_or_flow ::= **potential** | **flow** _// from A.1.7_

nature_attribute_identifier ::= _// from A.9.3_
**abstol** | **access** | **ddt_nature** | **idt_nature** | **units** | identifier


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
35
```
```
Syntax 3-4—Syntax for nature declaration
```
A nature shall be defined between the keywords **nature** and **endnature**. Each nature definition shall
have a unique identifier as the name of the nature and shall include all the required attributes specified in
3.6.1.2.

Examples:

```
nature current;
units = "A";
access = I;
idt_nature = charge;
abstol = 1u;
endnature
```
```
nature voltage;
units = "V";
access = V;
abstol = 1u;
endnature
```
**3.6.1.1 Derived natures**

A nature can be derived from an already declared nature. This allows the new nature to have the same attri-
butes as the attributes of the existing nature. The new nature is called a _derived nature_ and the existing
nature is called a _parent nature._ If a nature is not derived from any other nature, it is called a _base nature_.

In order to derive a new nature from an existing nature, the new nature name shall be followed by a colon
( **:** ) and the name of the parent nature in the nature definition.

A derived nature can declare additional attributes or override attribute values of the parent nature, with cer-
tain restrictions (as outlined in 3.6.1.2) for the predefined attributes.

The attributes of the derived nature are accessed in the same manner as accessing attributes of any other
nature.

Examples:

```
nature ttl_curr;
units = "A";
access = I;
abstol = 1u;
endnature
```
```
// An alias
nature ttl_net_curr : ttl_curr;
endnature
```
```
nature new_curr : ttl_curr; // derived, but different
abstol = 1m; // modified for this nature
maxval = 12.3; // new attribute for this nature
endnature
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
36
```
**3.6.1.2 Attributes**

Attributes define the value of certain quantities which characterize the nature. There are five predefined
attributes: **abstol** , **access** , **idt_nature** , **ddt_nature** , and **units**. In addition, user-defined attri-
butes can be defined in a nature (see 3.6.1.3). Attribute declaration assigns a constant expression to the attri-
bute name, as shown in the example in 3.6.1.1.

**abstol**

```
The abstol attribute is a real value constant expression that provides a tolerance measure (metric)
for convergence of potential or flow calculations. It specifies the maximum negligible value for sig-
nals associated with the nature.
This attribute is required for all base natures. It is legal for a derived nature to change abstol , but
if left unspecified it shall inherit the abstol from its parent nature.
```
**access**

```
The access attribute identifies the name for the access function. When the nature is used to bind a
potential, the name is used as an access function for the potential; when the nature is used to bind a
flow, the name is used as an access function for the flow. The usage of access functions is described
further in 4.4.
This attribute is required for all base natures. The constant expression assigned to it shall be an iden-
tifier (by name, not as a string).
It is illegal for a derived nature to change the access attribute; the derived nature always inherits the
access attribute of its parent nature.
```
**idt_nature**

```
The idt_nature attribute provides a relationship between a nature and the nature representing its
time integral.
idt_nature can be used to reduce the need to specify tolerances on the idt() operator. If this
operator is applied directly on nets, the tolerance can be taken from the node, which eliminates the
need to give a tolerance with the operator.
If specified, the constant expression assigned to idt_nature shall be the name (not a string) of a
nature which is defined elsewhere. It is possible for a nature to be self-referencing with respect to its
idt_nature attribute. In other words, the value of idt_nature can be the nature that the
attribute itself is associated with.
The idt_nature attribute is optional; the default value is the nature itself. While it is possible
to override the parent’s value of idt_nature using a derived nature, the nature thus specified
shall be related (share the same base nature) to the nature the parent uses for its idt_nature.
```
**ddt_nature**

```
The ddt_nature attribute provides a relationship between a nature and the nature representing its
time derivative.
ddt_nature can be used to reduce the need to specify tolerances on the ddt() operator. If this
operator is applied directly on nets, the tolerance can be taken from the node, eliminating the need to
give a tolerance with the operator.
If specified, the constant expression assigned to ddt_nature shall be the name (not a string) of a
nature which is defined elsewhere. It is possible for a nature to be self-referencing with respect to its
ddt_nature attribute. In other words, the value of ddt_nature can be the nature that the
attribute itself is associated with.
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
37
```
```
The ddt_nature attribute is optional; the default value is the nature itself. While it is possible
to override the parent’s value of ddt_nature using a derived nature, the nature thus specified
shall be related (share the same base nature) to the nature the parent uses for its ddt_nature.
```
**units**

```
The units attribute provides a binding between the value of the access function and the units for
that value. The units field is provided so simulators can annotate the continuous signals with their
units and is also used in the net compatibility rule check.
This attribute is required for all base natures. It is illegal for a derived nature to define or change the
units ; the derived nature always inherits its parent nature units. If specified, the constant
expression assigned to it shall be a string.
```
**3.6.1.3 User-defined attributes**

In addition to the predefined attributes listed above, a nature can specify other attributes which can be useful
for analog modeling. Typical examples include certain maximum and minimum values to define a valid
range.

A user-defined attribute can be declared in the same manner as any predefined attribute. The name of the
attribute shall be unique in the nature being defined and the value being assigned to the attribute shall be
constant.

#### 3.6.2 Disciplines.........................................................................................................................

A _discipline_ description consists of specifying a **domain** type and binding any _natures_ to **potential** or
**flow**.

The syntax for declaring a discipline is shown in Syntax 3- 5.

discipline_declaration ::= _// from A.1.7_
**discipline** discipline_identifier [ **;** ]
{ discipline_item }
**enddiscipline**

discipline_item ::=
nature_binding
| discipline_domain_binding
| nature_attribute_override

nature_binding ::= potential_or_flow nature_identifier **;**

potential_or_flow ::= **potential** | **flow**

discipline_domain_binding ::= **domain** discrete_or_continuous **;**

discrete_or_continuous ::= **discrete | continuous**

nature_attribute_override ::= potential_or_flow**.** nature_attribute

```
Syntax 3-5—Syntax for discipline declaration
```
A _discipline_ shall be defined between the keywords **discipline** and **enddiscipline**. Each discipline
shall have a unique identifier as the name of the discipline.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
38
```
The discipline declarations are at the same level as _nature_ and _module_ declarations in the source text. That
is, disciplines are declared at the top level and discipline declarations do not nest inside other discipline dec-
larations, nature declarations, or module declarations. Analog behavioral nets (nodes) must have a discipline
defined for them but interconnect and digital nets do not. It is possible to set the discipline of interconnect
and digital nets through discipline declaration with hierarchical references to these nets. It shall be an error
to hierarchically override the discipline of a net that was explicitly declared unless it is a compatible disci-
pline.

**3.6.2.1 Nature binding**

Each discipline can bind a _nature_ to its **potential** and **flow**.

Only the name of the nature is specified in the discipline. The nature binding for potential is specified using
the keyword **potential**. The nature binding for flow is specified using the keyword **flow**.

The access function defined in the nature bound to potential is used in the model to describe the signal-flow
which obeys Kirchhoff’s Potential Law (KPL). This access function is called the _potential access function_.

The access function defined in the nature bound to flow is used in the model to describe a quantity which
obeys Kirchhoff’s Flow Law (KFL). This access function is called the _flow access function_.

Disciplines with two natures are called _conservative disciplines_ and the nets associated with conservative
disciplines are called _conservative nets_. Conservative disciplines shall not have the same _nature_ specified
for both the **potential** and the **flow**. Disciplines with a single nature are called _signal-flow disciplines_
and the nets with signal-flow disciplines are called _signal-flow nets_. A signal-flow discipline may specify
either the potential or the flow nature, as shown in the following examples.

Examples:

Conservative discipline

```
discipline electrical;
potential Voltage;
flow Current;
enddiscipline
```
Signal-flow disciplines

```
discipline voltage;
potential Voltage;
enddiscipline
```
```
discipline current;
flow Current;
enddiscipline
```
Multi-disciplinary example

Disciplines in Verilog-AMS HDL allow designs of multiple disciplines to be easily defined and simulated.
Disciplines can be used to allow unique tolerances based on the size of the signals and outputs displayed in
the actual units of the discipline. This example shows how an application spanning multiple disciplines can
be modeled in Verilog-AMS HDL. It models a DC-motor driven by a voltage source.

```
module motorckt;
parameter real freq=100;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
39
```
```
electrical gnd ; ground gnd;
```
```
electrical drive;
rotational shaft;
```
```
motor m1 (drive, gnd, shaft);
vsine #(.freq(freq), .ampl(1.0)) v1 (drive, gnd);
```
```
endmodule
```
```
// vp: positive terminal [V,A] vn: negative terminal [V,A]
// shaft:motor shaft [rad,Nm]
// INSTANCE parameters
// Km = motor constant [Vs/rad] Kf = flux constant [Nm/A]
// j = inertia factor [Nms^2/rad] D= drag (friction) [Nms/rad]
// Rm = motor resistance [Ohms] Lm = motor inductance [H]
// A model of a DC motor driving a shaft
module motor(vp, vn, shaft);
inout vp, vn, shaft;
electrical vp, vn;
rotational shaft;
```
```
parameter real Km = 4.5, Kf = 6.2;
parameter real j = 0.004, D = 0.1;
parameter real Rm = 5.0, Lm = 0.02;
```
```
analog begin
V(vp, vn) <+ Km*Theta(shaft) + Rm*I(vp, vn) + ddt (Lm*I(vp, vn));
Tau(shaft) <+ Kf*I(vp, vn) - D*Theta(shaft) - ddt (j*Theta(shaft));
end
endmodule
```
**3.6.2.2 Domain binding**

Analog signal values are represented in continuous time, whereas digital signal values are represented in dis-
crete time. The **domain** attribute of the discipline stores this property of the signal. It takes two possible
values, **discrete** or **continuous.** Signals with continuous-time domains are real valued. Signals with
discrete-time domains can either be binary ( 0 , 1 , X, or Z), integer or real values.

Examples:

```
discipline electrical;
domain continuous ;
potential Voltage;
flow Current;
enddiscipline
```
```
discipline ddiscrete;
domain discrete ;
enddiscipline
```
The **domain** attribute is optional. The default value for **domain** is **continuous** for disciplines which
specify **nature** bindings. It is an error for a discipline to have a **domain** binding of **discrete** if it has
**nature** bindings.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
40
```
**3.6.2.3 Natureless disciplines and domainless disciplines**

It is possible to define a discipline with no nature bindings. These are known as natureless disciplines (his-
torically referred to as empty disciplines).

Such disciplines may have a **domain** binding or they may be domainless, thus allowing the domain to be
determined by the connectivity of the net (see 7.4 and Annex F).

Disciplines without a **domain** binding and without a **nature** binding are known as domainless disciplines.
The **domain** binding of a discipline with **nature** bindings defaults to **continuous** if not specified. A
**discipline** with **nature** bindings cannot be a domainless discipline.

Example:

```
discipline natureless;
domain continuous ;
enddiscipline
```
```
discipline domainless
enddiscipline
```
Usage of domainless disciplines and continuous natureless disciplines is discouraged. Domainless and con-
tinuous natureless disciplines are provided for backward compatibility with previous versions of the Ver-
ilog-AMS and Verilog-A standards. Furthermore, domainless disciplines are deprecated and the definition
of a domainless discipline may be made an error in future versions of Verilog-AMS HDL.

**3.6.2.4 Discipline of nets and undeclared nets**

It is possible for a module to have nets where there are no discipline declarations. If such a net appears
bound only to ports in module instantiations, it may have no declaration at all or may be declared to have a
net type such as **wire** , **tri** , **wand** , **wor** , etc. If it is referenced in behavioral code, then it must have a net
type.

In these cases, the net shall be treated as having no discipline. If the net is referenced in behavioral code,
then it shall be treated as having no discipline with a domain binding of **discrete** , otherwise it shall be
treated as having no discipline and no domain binding. If a net has a wire type but is not connected to behav-
ioral code (interconnect) and it resolved to domain **discrete** then its wire type shall be used in any net
type resolution steps per IEEE Std 1364 Verilog.

The discipline and domain of all nets of a mixed or continuous signal is determined by discipline resolution
if these nets do not already have a declared discipline and domain binding (see 7.4 and Annex F).

**3.6.2.5 Overriding nature attributes from discipline**

A discipline can override the value of the bound nature for the pre-defined attributes (except as restricted by
3.6.1.2), as shown for the flow ttl_curr in the example below. To do so from a discipline declaration, the
bound nature and attribute needs to be defined, as shown for the **abstol** value within the discipline ttl in
the example below. The general form is shown as the _nature_attribute_override_ nonterminal in Syntax 3- 5 :
the keyword **flow** or **potential** , then the hierarchical separator**.** and the attribute name, and, finally,
set all of this equal to ( **=** ) the new value (e.g., flow.abstol = 10u).

Examples:

```
nature ttl_curr;
units = "A";
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
41
```
```
access = I;
abstol = 1u;
endnature
```
```
nature ttl_volt;
units = "V";
access = V;
abstol = 100u;
endnature
```
```
discipline ttl;
potential ttl_volt;
flow ttl_curr;
flow. abstol = 10u;
enddiscipline
```
**3.6.2.6 Deriving natures from disciplines**

A nature can be derived from the _nature_ bound to the **potential** or **flow** in a discipline. This allows the
new nature to have the same attributes as the attributes for the nature bound to the **potential** or the **flow**
of the discipline.

If the nature bound to the potential or the flow of a discipline changes, the new nature shall automatically
inherit the attributes for the changed nature.

In order to derive a new nature from flow or potential of a discipline, the nature declaration shall also
include the discipline name followed by the hierarchical separator (**.** ) and the keyword **flow** or **poten-
tial** , as shown for ttl_net_curr in the example below.

A nature derived from the flow or potential of a discipline can declare additional attributes or override val-
ues of the attributes already declared.

Examples:

```
nature ttl_net_curr : ttl.flow; // from the example in 3.6.2.5
endnature // abstol = 10u as modified in ttl
```
```
nature ttl_net_volt : ttl.potential; // from the example in 3.6.2.5
abstol = 1m; // modified for this nature
maxval = 12.3; // new attribute for this nature
endnature
```
**3.6.2.7 User-defined attributes**

Like natures, a **discipline** can specify user-defined attributes. Discipline user-defined attributes are use-
ful for the same reasons as nature user-defined attributes (see 3.6.1.3).

#### 3.6.3 Net_discipline declaration.................................................................................................

Each _net_discipline_ declaration associates nets with an already declared discipline. Syntax 3- 6 shows how to
declare disciplines of nets and regs.

net_declaration ::= _// from A.2.1.3_
...
| discipline_identifier [ range ] list_of_net_identifiers **;**


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
42
```
```
| discipline_identifier [ range ] list_of_net_decl_assignments ;
...
```
range ::= **[** msb_constant_expression **:** lsb_constant_expression **]** _// from A.2.5_

list_of_net_decl_assignments ::= net_decl_assignment { **,** net_decl_assignment } _// from A.2.3_

list_of_net_identifiers ::= ams_net_identifier { **,** ams_net_identifier }

net_decl_assignment ::= ams_net_identifier **=** expression _// from A.2.4_

```
Syntax 3-6—Syntax for net discipline declaration
```
If a range is specified for a net, the net is called a _vector net_ ; otherwise it is called a _scalar net_. A vector net
is also called a _bus_.

Examples:

```
electrical [MSB:LSB] n1; // MSB and LSB are parameters
voltage [5:0] n2, n3;
magnetic inductor;
ddiscrete [10:1] connector1;
```
Nets represent the abstraction of information about signals. As with ports, nets represent component inter-
connections. Nets declared in the module interface define the ports to the module (see 6.5).

A net used for modeling a conservative system shall have a discipline with both access functions ( **poten-
tial** and **flow** ) defined. When modeling a signal-flow system, the discipline of a net can have only
**potential** access functions. When modeling a discrete system, the discipline of a net can only have a
**domain** of **discrete** defined.

Nets declared with a natureless discipline or declared without a discipline do not have declared natures, so
such nets can not be used in analog behavioral descriptions (because the access functions are not known).
However, such nets can be used in structural descriptions, where they inherit the natures from the ports of
the instances of modules that connect to them.

**3.6.3.1 Net descriptions**

Nets can be declared with a description attribute. This information can be used by the simulator to generate
help messages for a module.

Example:

```
(* desc="drain terminal" *) electrical d;
```
If a net is also a module port, the description attribute may also be specified on the port declaration line (in
which the net is declared as **input** , **inout** , or **output** ). If the description attribute is specified for the
same _net_identifier_ in both the net discipline declaration and the port declaration, then the last attribute value
shall be used and the tool can give a warning that a duplicate attribute specification has occurred.

**3.6.3.2 Net Discipline Initial (Nodeset) Values**

Nets with continuous disciplines are allowed to have initializers on their net discipline declarations; how-
ever, nets of non-continuous disciplines are not.

```
electrical a = 5.0;
electrical [0:4] bus = '{2.3,4.5,,6.0};
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
43
```
```
mechanical top.foo.w = 250.0;
```
The initializer shall be a _constant_expression_ and will be used as a nodeset value for the potential of the net
by the analog solver. In the case of analog buses, a constant array expression is used as an initializer. A null
value in the constant array indicates that no nodeset value is being specified for this element of the bus.

If different nets of a node have conflicting initializers, then initializers on hierarchical net declarations win.
If there are multiple hierarchical declarations, then the declaration on the highest level wins. If there are mul-
tiple hierarchical declarations on the highest level, then it is a race condition for which the initializer wins. If
the multiple conflicting initializers are not hierarchical, then it is also a race condition for which the initial-
izer wins.

#### 3.6.4 Ground declaration............................................................................................................

Each ground declaration is associated with an already declared net of continuous discipline. The node asso-
ciated with the net will be the global reference node in the circuit. The net must be assigned a continuous
discipline to be declared ground.

Syntax 3- 7 shows the syntax used for declaring the global reference node ( _ground_ ).

net_declaration ::= _// from A.2.1.3_

```
| ground [ discipline_identifier ] [ range ] list_of_net_identifiers ;
```
```
Syntax 3-7—Syntax for declaring ground
```
Examples:

```
module loadedsrc(in, out);
input in;
output out;
electrical in, out;
electrical gnd;
ground gnd;
parameter real srcval = 5.0;
```
```
resistor #(.r(10K)) r1(out,gnd);
analog begin
V(out) <+ V(in,gnd)*2;
end
endmodule
```
#### 3.6.5 Implicit nets.......................................................................................................................

Nets can be used in structural descriptions without being declared. In this case, the net’s discipline and
domain binding will be determined by discipline resolution (see 7.4 and Annex F).

Examples:

```
module top(i1, i2, o1, o2, o3);
input i1, i2;
output o1, o2, o3;
electrical i1, i2, o1, o2, o3;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
44
```
```
// ab1, ab2, cb1, cb2 are implicit nets, not declared
blk_a a1( i1, ab1 );
blk_a a2( i2, ab2 );
blk_b b1( ab1, cb1 );
blk_b b2( ab2, cb2 );
blk_c c1( o1, o2, o3, cb1, cb2);
endmodule
```
### 3.7 Real net declarations

The **wreal** , or real net data type, represents a real-valued physical connection between structural entities. A
wreal net shall not store its value. A **wreal** net can be used for real-valued nets which are driven by a single
driver, such as a continuous assignment. If no driver is connected to a wreal net, its value shall be zero
(0.0). Unlike other digital nets which have an initial value of ‘z’, wreal nets shall have an initial value of
zero.

wreal nets can only be connected to compatible interconnect and other wreal or real expressions. They can-
not be connected to any other wires, although connection to explicitly declared 64-bit wires can be done via
system tasks $realtobits and $bitstoreal. Compatible interconnect are nets of type **wire** , **tri** , and **wreal**
where the IEEE Std 1364 Verilog net resolution is extended for **wreal**. When the two nets connected by a
port are of net type **wreal** and **wire** / **tri** , the resulting single net will be assigned as **wreal**. Connection
to other net types will result in an error.

Syntax 3- 8 shows the syntax for declaring digital nets.

net_declaration ::= _// from A.2.1.3_
...
| **wreal** [ discipline_identifier ] [ range] list_of_net_identifiers **;**
| **wreal** [ discipline_identifier ] [ range] list_of_net_decl_assignments **;**

```
Syntax 3-8—Syntax for declaring digital nets
```
Examples:

```
module drv(in, out);
input in;
output out;
wreal in;
electrical out;
analog begin
V(out) <+ in;
end
endmodule
```
```
module top();
real stim;
electrical load;
wreal wrstim;
assign wrstim = stim;
drv f1(wrstim, load);
always begin
#1 stim = stim + 0.1;
end
endmodule
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
45
```
**3.8 Default discipline**

Verilog-AMS HDL supports the **`default_discipline** compiler directive. This directive specifies a
default discrete discipline to be applied to any discrete net which does not have an explicit discipline decla-
ration as part of discipline resolution (see 7.4 and Annex F). A description and its syntax is shown in 10.2.

### 3.9 Disciplines of primitives

With internal simulator primitives the discipline of the vpiLoConn to be used in discipline resolution during
a mixed-signal simulation must be known. For digital primitives the domain is discrete and thus the disci-
pline is set via the default_discipline directive as it is for digital modules. If the discipline of digital connec-
tions (vpiLoConn) to a mixed net are unknown then the default_discipline must be specified (via the
directive or other vendor specific method). If not specified, an error will result during discipline resolution.

For analog primitives, the discipline will be defined by the attribute port_discipline on that instance. If no
attribute is found then it will acquire the discipline of other compatible continuous disciplines connected to
that net segment. If no disciplines are connected to that net, then the default discipline is set to electrical.
This is further described in E.3.2.2.

### 3.10 Discipline precedence

While a net itself can be declared only in the module to which it belongs, the discipline of the net can be
specified in a number of ways.

```
— The discipline name can appear in the declaration of the net.
— The discipline name can be used in a declaration which makes an out of context reference to the net
from another module.
```
Discipline conflicts can arise if more than one of these methods is applied to the same net. Discipline con-
flicts shall be resolved using the following order of precedence:

```
1) A declaration from a module other than the module to which the net belongs using an out-of-module
reference, e.g.,
```
```
module example1;
electrical example2.net;
endmodule
```
```
2) The local declaration of the net in the module to which it belongs, e.g.,
```
```
module example2;
electrical net;
endmodule
```
```
3) Discipline resolution (see 7.4 and Annex F)
```
It is not legal to have two different disciplines at the same level of precedence for the same net.

### 3.11 Net compatibility

Certain operations can be done on nets only if the two (or more) nets are compatible. For example, if an
access function has two nets as arguments, they must be compatible. The following rules shall apply to
determine the compatibility of two (or more) nets:


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
46
```
_Discrete Domain Rule:_ Digital nets with the same signal value type (i.e., **real** , **integer** ) are compatible
with each other if their disciplines are compatible (i.e., the discipline has a discrete domain or is empty.

_Signal Domain Rule:_ It shall be an error to connect two ports or nets of different domains unless there is a
connect statement (see 7.4) defined between the disciplines of the nets or ports.

_Signal Connection Rule:_ It shall be an error to connect two ports or nets of the same domain with incompat-
ible disciplines.

#### 3.11.1 Discipline and Nature Compatibility

The following rules shall apply to determine discipline compatibility:

```
— Self Rule (Discipline): A discipline is compatible with itself.
— Natureless Discipline Rule: A natureless discipline is compatible with all other disciplines of the
same domain.
— Domainless Discipline Rule : A domainless discipline is compatible with all disciplines as there is no
nature or domain conflict. Note that domainless disciplines are deprecated.
— Domain Incompatibility Rule: Disciplines with different domain attributes are incompatible.
— Potential Incompatibility Rule: Disciplines with incompatible potential natures are incompatible.
— Flow Incompatibility Rule: Disciplines with incompatible flow natures are incompatible.
```
The following rules shall apply to determine nature compatibility:

```
—S elf Rule (Nature): A nature is compatible with itself.
— Non-Existent Binding Rule: A nature is compatible with a non-existent discipline binding.
— Base Nature Rule: A derived nature is compatible with its base nature.
— Derived Nature Rule: Two natures are compatible if they are derived from the same base nature.
— Units Value Rule: Two natures are compatible if they have the same value for the units attribute.
```
The following examples illustrate these rules.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
47
```
The following compatibility observations can be made from the above examples:

```
— Voltage and highvoltage are compatible natures because they both exist and are derived from
the same base natures.
— electrical and highvolt are compatible disciplines because the natures for both potential and
flow exist and are derived from the same base natures.
— electrical and sig_flow_v are compatible disciplines because the nature for potential is same
for both disciplines and the nature for flow does not exist in sig_flow_v.
— electrical and rotational are incompatible disciplines because the natures for both potential
and flow are not derived from the same base natures.
— electrical and sig_flow_x are incompatible disciplines because the nature for both potentials
are not derived from the same base nature.
```
```
nature Voltage;
access = V;
units = "V";
abstol = 1u;
endnature
```
```
nature Current;
access = I;
units = "A";
abstol = 1p;
endnature
```
```
nature highvoltage: Voltage;
abstol = 1.0;
endnature
```
```
discipline electrical;
potential Voltage;
flow Current;
enddiscipline
```
```
discipline highvolt;
potential highvoltage;
flow Current;
enddiscipline
```
```
discipline sig_flow_v;
potential Voltage;
enddiscipline
```
```
discipline sig_flow_i;
flow Current;
enddiscipline
```
```
nature Position;
access = X;
units = "m";
abstol = 1u;
endnature
```
```
nature Force;
access = F;
units = "N";
abstol = 1n;
endnature
```
```
discipline rotational;
potential Position;
flow Force;
enddiscipline
```
```
discipline sig_flow_x;
potential Position;
enddiscipline
```
```
discipline sig_flow_f;
flow Force;
enddiscipline
```
```
discipline domainless;
enddiscipline
```
```
discipline ddiscrete;
domain discrete ;
enddiscipline
```
```
discipline natureless;
domain continuous ;
enddiscipline
```
```
discipline continuous_elec;
domain continuous ;
potential Voltage;
flow Current;
enddiscipline
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
48
```
```
— The natureless discipline natureless is compatible with all other disciplines of the same domain
(i.e continuous) because it does not have a potential or a flow nature. Without natures, there can be
no conflicting natures.
— domainless is compatible with all other disciplines from the domainless discipline rule.
— electrical and ddiscrete are incompatible disciplines because the domains are different. A
connect statement must be used to connect nets or ports of these disciplines together.
— electrical and continuous_elec are compatible disciplines because the default domain for
discipline electrical is continuous and the specified natures for potential and flow are the same.
```
### 3.12 Branches.........................................................................................................................................

A _branch_ is a path between two nets. If both nets are conservative, then the branch is a _conservative branch_
and it defines a branch potential and a branch flow. If one net is a signal-flow net, then the branch is a _signal-
flow branch_ and it defines either a branch potential or a branch flow, but not both.

Each branch declaration is associated with two nets from which it derives a discipline. These nets are
referred to as the _branch terminals_. Only one net need be specified, in which case the second net defaults to
ground and the discipline for the branch is derived from the specified net. The disciplines for the specified
nets shall be compatible (see 3.11).

Branches can either be explicitly or implicitly declared. Explicitly declared branches are referred to as
named branches. The syntax for declaring named branches is shown in Syntax 3- 9. Unnamed branches are
created by applying an access function (see 4.4 and 5.4.1) to either a net or a pair of nets. If the access func-
tion is applied to a single net, then the branch is formed between that net and the global reference node
(ground). If it is applied to a pair of nets, the branch is formed between the two nets. There shall be at most
one unnamed branch between any two nets or between a net and implicit ground (in addition to any number
of named branches).

branch_declaration ::= _// from A.2.1.3_
**branch (** branch_terminal [ **,** branch_terminal ] **)** list_of_branch_identifiers **;**
| port_branch_declaration

port_branch_declaration ::=
**branch ( <** port_identifier **> )** list_of_branch_identifiers **;**
| **branch ( <** hierarchical_port_identifier **> )** list_of_branch_identifiers **;**

branch_terminal ::=
net_identifier
| net_identifier **[** constant_expression **]**
| net_identifier **[** constant_range_expression **]**
| hierarchical_net_identifier
| hierarchical_net_identifier **[** constant_expression **]**
| hierarchical_net_identifier **[** constant_range_expression **]**

list_of_branch_identifiers ::= _// from A.2.3_
branch_identifier [ range ] { **,** branch_identifier [ range ] }

```
Syntax 3-9—Syntax for branch declaration
```
If one of the terminals of a branch is a vector net, then the other terminal shall either be a scalar net or a vec-
tor net of the same size. In the latter case, the branch is referred to as a _vector branch_. When both terminals
are vectors, the scalar branches that make up the vector branch connect to the corresponding scalar nets of
the vector terminals, as shown in Figure 3- 1.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
49
```
```
Figure 3-1: Two vector terminals
```
When one terminal is a vector and the other is a scalar, a singular scalar branch connects to each scalar net in
the vector terminal and each terminal of the vector branch connects to the scalar terminal, as shown in
Figure 3- 2.

```
Figure 3-2: One vector and one scalar terminal
```
If the range of the _vector branch_ is not specified then the indexing of the _vector branch_ shall start at 0. For
example:

```
electrical [3:5]a;
electrical [1:3]b;
branch (a,b) br1; // Branch br1 is of size 3 and can be indexed from 0 to 2
```
#### 3.12.1 Port Branches

A port branch is a special type of branch used to access the flow into a port of a module (see 5.4.3). It is a
branch between the upper and lower connections of the port. A port branch is a scalar branch if the port iden-
tifier is a scalar port. A port branch is a vector branch if the port identifier is a vector port.

Example:
**module** current_sink(p);
electrical p;
**branch** (<p>) probe_p;
**analog**
$strobe("current probed is %g", I(probe_p));
**endmodule**

### Vector Branch

### Vector Terminal Vector Terminal

### Vector Branch

### Vector Terminal Scalar Terminal


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
50
```
### 3.13 Namespace

The following subsections define the namespace.

#### 3.13.1 Nature and discipline

Natures and disciplines are defined at the same level of scope as modules. Thus, identifiers defined as
natures or disciplines have a global scope, which allows nets to be declared inside any module in the same
manner as an instance of a module.

#### 3.13.2 Access functions

Each access function name, defined before a module is parsed, is automatically added to that module’s
namespace unless there is another identifier defined with the same name as the access function in that mod-
ule’s namespace. Furthermore, the access function of each base nature shall be unique.

#### 3.13.3 Net

The scope rules for net identifiers are the same as the scope rules for any other identifier declarations, except
nets can not be declared anywhere other than in the port of a module or in the module itself. A net can only
be declared inside a module scope; a net can not be declared local to a block.

Access functions are uniquely defined for each net based on the discipline of the net. Each access function is
used with the name of the net as its argument and a net can only be accessed through its access functions.

The hierarchical reference character (**.** ) can be used to reference a net across the module boundary accord-
ing to the rules specified in IEEE Std 1364 Verilog.

#### 3.13.4 Branch

The scope rules for branch identifiers are the same as the scope rules for net identifiers. A branch can only
be declared inside a module scope; a branch can not be declared local to a block.

Access functions are uniquely defined for each branch based on the discipline of the branch. The access
function is used with the name of the branch as its argument and a branch can only be accessed through its
access functions.

The hierarchical reference character (**.** ) can be used to reference a branch across the module boundary
according to the rules specified in IEEE Std 1364 Verilog.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
51
```
