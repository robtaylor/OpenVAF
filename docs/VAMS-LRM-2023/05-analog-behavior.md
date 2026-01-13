## 5. Analog behavior.......................................................................................................................................

### 5.1 Overview........................................................................................................................................

The description of an analog behavior consists of setting up contributions for various signals under certain
procedural or timing control. This section describes an analog procedural block, analog signals, contribution
statements, procedural control statements, and analog timing control functions.

### 5.2 Analog procedural block................................................................................................................

Discrete time behavioral definitions within IEEE Std 1364 Verilog are encapsulated within the **initial**
and **always** procedural blocks. Every **initial** and **always** block starts a separate concurrent activity
flow. For continuous time simulation, the behavioral description is encapsulated within the analog proce-
dural block. The syntax for **analog** block is shown in Syntax 5- 1.

analog_construct ::= _// from A.6.2_
**analog** analog_statement
| **analog initial** analog_function_statement

analog_statement ::= _// from A.6.4_
{ attribute_instance } analog_loop_generate_statement
| { attribute_instance } analog_loop_statement
| { attribute_instance } analog_case_statement
| { attribute_instance } analog_conditional_statement
| { attribute_instance } analog_procedural_assignment
| { attribute_instance } analog_seq_block
| { attribute_instance } analog_system_task_enable
| { attribute_instance } contribution_statement
| { attribute_instance } indirect_contribution_statement
| { attribute_instance } analog_event_control_statement

analog_statement_or_null ::=
analog_statement
| { attribute_instance } **;**

```
Syntax 5-1—Syntax for analog procedural block
```
The analog procedural block defines the behavior as a procedural sequence of statements. The conditional
and looping constructs are available for defining behaviors within the analog procedural block. Because the
description is a continuous-time behavioral description, no blocking event control statements (such as block-
ing delays, blocking events, or waits) are supported.

All the statements within the **analog** block shall be executed sequentially at a given point of time, however
the effects on the analog variables, nets, and branches contained in various modules in a design are consid-
ered concurrently. Analog blocks shall be executed at every point in a simulation. Multiple analog blocks
can also be used within a module declaration. Refer 6.2 for more details on multiple analog blocks.

#### 5.2.1 Analog initial block...........................................................................................................

An _analog initial block_ is a special analog (procedural) block, beginning with the keywords
**analog initial** , for simulation initialization purposes.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
98
```
Like a regular **analog** block, an **analog initial** block is also comprised of a procedural sequence of
statements. If there are multiple **analog initial** blocks, they are executed as if concatenated. However,
statements in **analog initial** blocks are restricted for initialization purposes. So an **analog ini-
tial** block shall not contain the following statements:

```
— statements with access functions or analog operators;
— contribution statements;
— event control statements.
```
This is similar to the restrictions on the statements in analog functions.

This is because an **analog initial** block is executed before a matrix solution is available so state-
ments in an **analog initial** block are restricted to initialization purposes prior to the availability of a
solution of both the digital and the analog modules.

Additionally, digital values cannot be accessed from the **analog initial** block as they have not yet
been assigned when the **analog initial** block is executed.

The **analog initial** block is executed once for each analysis, and can be executed for each sub-task of
parameter sweep analysis (such as dc sweep). The initialization sequence of analog and digital blocks/state-
ments is described in 8.2 and 8.4.1. If a parameter or variable that is referenced from an **analog initial**
block is changed during a sub-task of a parameter sweep analysis, then the **analog initial** block shall
be re-executed so that the new value is taken into account.

### 5.3 Block statements

The _block statements_ , also referred to as _sequential blocks_ , are a means of grouping procedural statements.
The statements within the block shall be executed in sequence, one after another in the given order and the
control shall pass out of the block after the last statement is executed. The block statements are delimited by
the keywords **begin** and **end**.

#### 5.3.1 Sequential blocks

The syntax for sequential blocks is shown in Syntax 5- 2.

analog_seq_block ::= _// from A.6.3_
**begin** [ **:** analog_block_identifier { analog_block_item_declaration } ]
{ analog_statement } **end**

analog_block_item_declaration ::= _// from A.2.8_
{ attribute_instance } parameter_declaration **;**
| { attribute_instance } integer_declaration
| { attribute_instance } real_declaration

```
Syntax 5-2—Syntax for the sequential blocks
```
#### 5.3.2 Block names

A sequential block can be named by adding a **:** _block_identifier_ after the keyword **begin**. The naming of a
block allows local variables to be declared for that block. The block names give a means of uniquely identi-
fying all variables at any simulation time.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
99
```
All named block variables are static—that is, an unique location exists for all variables and leaving or enter-
ing the block do not affect the values stored in them. All identifiers declared within a named sequential
block can be accessed outside the scope in which they are declared. Named block variables cannot be
assigned outside the scope of the block in which they are declared.

Parameters declared within a named block have local scope and cannot be assigned outside the scope.
Named and ordered parameter overrides at module instantiation can only affect parameters declared at mod-
ule scope.

```
module example;
parameter integer p1 = 1;
real moduleVar;
```
```
analog begin
begin : myscope
parameter real p2 = p1;
real localVar = 1.5 * p2;
end
moduleVar = myscope.localVar;
end
endmodule
```
```
module top;
example #(.p1(4)) inst1(); // allowed
example #(.myscope.p2(4)) inst2(); // error
endmodule
```
### 5.4 Analog signals................................................................................................................................

Analog signals are distinguished from digital signals in that an _analog signal_ has a discipline with a continu-
ous domain. Disciplines, nets, nodes, and branches are described in and ports are described in Clause 6.

This section describes analog branch assignments, signal access mechanisms, and operators in Verilog-AMS
HDL.

#### 5.4.1 Access functions

Flows and potentials on nets, ports, and branches are accessed using _access functions_. The name of the
access function is taken from the discipline of the net, port, or branch associated with the signal.

Example 1 — Consider a named electrical branch b where _electrical_ is a discipline with _V_ as the access
function for the potential and _I_ as the access function for the flow. The potential (voltage) is accessed via
V(b) and the flow (current) is accessed via I(b).

```
— There can be any number of named branches between any two signals.
— Unnamed branches are accessed in a similar manner, except the access functions are applied to net
names or port names rather than branch names.
```
Example 2 — If _n1_ and _n2_ are electrical nets or ports, then V(n1, n2) creates an unnamed branch from _n1_
to _n2_ (if it does not already exist) and then accesses the branch potential (or the potential difference between
_n1_ to _n2),_ and V(n1) does the same from _n1_ to the global reference node ( _ground_ ).

```
— In other words, accessing the potential from a net or port to a net or port defines an unnamed branch.
Accessing the potential on a single net or port defines an unnamed branch from that net or port to the
global reference node ( ground ). There can only be one unnamed branch between any two nets or
between a net and implicit ground (in addition to any number of named branches).
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
100
```
```
— An analogous access method is used for flows.
```
Example 3 — I(n1, n2) creates an unnamed branch from _n1_ to _n2_ (if it does not already exist) and then
accesses the branch flow, and I(n1) does the same from _n1_ to the global reference node ( _ground_ ).

```
— Thus, accessing the flow from a net or port to a net or port defines an unnamed branch. Accessing
the potential on a single net or port defines an unnamed branch from that net or port to the global ref-
erence node ( ground ).
— It is also possible to access the flow passing through a port into a module. The name of the access
function is derived from the flow nature of the discipline of the port. In this case, (<>) is used to
delimit the port name rather than ().
```
Example 4 — I(<p1>) is used to access the current flow into the module through the electrical port p1.
This capability is discussed further in 5.4.3.

#### 5.4.2 Probes and sources

An analog component can be represented using a network of probes and controlled sources. The Verilog-
AMS HDL uses the concept of _probes_ and _sources_ as a means of unambiguously representing a network.
The mapping between these representations are defined in following subsections.

**5.4.2.1 Probes**

If no value is specified for either the potential or the flow, the branch is a _probe_. If the flow of the branch
appears in an expression anywhere in the module, the branch is a _flow probe_ , otherwise the branch is a
_potential probe_. Using both the potential and the flow of a probe branch is illegal. The models for probe
branches are shown in Figure 5- 1.

```
Figure 5-1: Equivalent circuit models for probe branches
```
The branch potential of a flow probe is zero ( 0 ). The branch flow of a potential probe is zero ( 0 ).

**5.4.2.2 Sources**

A branch, either named or unnamed, is a _source branch_ if either the potential or the flow of that branch is
assigned a value by a contribution statement (see 5.6) anywhere in the module. It is a _potential source_ if the
branch potential is specified and is a _flow source_ if the branch flow is specified. A branch cannot simultane-
ously be both a potential and a flow source, although it can switch between them (a _switch branch_ ).

Both the potential and the flow of a source branch are accessible in expressions anywhere in the module.
The models for potential and flow sources are shown in Figure 5- 2.

```
p f
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
101
```
```
Figure 5-2: Equivalent circuit models for source branches
```
#### 5.4.3 Accessing flow through a port

The port access function accesses the flow into a port of a module. The name of the access function is
derived from the flow nature of the discipline of the port. However (<>) is used to delimit the port name,
e.g., I(<a>) accesses the current through module port _a_.

Example 1 — Consider the junction diode below, where the total diode current is monitored and a message
is issued if it exceeds a given value.

```
module diode (a, c);
inout a, c;
electrical a, c;
branch (a, c) i_diode, junc_cap;
parameter real is = 1e-14, tf = 0, cjo = 0, imax = 1, phi = 0.7;
analog begin
I(i_diode) <+ is*( limexp (V(i_diode)/ $vt ) – 1);
I(junc_cap) <+
ddt (tf*I(i_diode) - 2*cjo* sqrt (phi*(phi*V(junc_cap))));
if (I(<a>) > imax)
$strobe ( "Warning: diode is melting!" );
end
endmodule
```
The expression V(<a>) is invalid for ports and nets, where V is a potential access function. The port access
function shall not be used on the left side of a contribution operator <+.

Example 2 — An ideal relay (a controlled switch) can be implemented as:

```
module relay (p, n, ps, ns);
inout p, n, ps, ns;
electrical p, n, ps, ns;
parameter vth=0.5;
integer closed;
analog begin
closed = (V(ps,ns) >vth? 1 : 0);
if (closed)
```
```
f is a probe which measures the flow through the branch and p is a probe which
measures the potential across the branch.
```
```
f
```
```
p
```
```
f
```
```
p
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
102
```
```
V(p,n) <+ 0;
else
I(p,n) <+ 0;
end
endmodule
```
A discontinuity of order zero ( 0 ) is assumed to occur when the branch switches and so it is not necessary to
use the **$discontinuity** function with switch branches.

#### 5.4.4 Unassigned sources

If a value is not assigned to a branch, and it is not a probe branch, the branch flow is set to zero ( 0 ).

Examples:

```
if (closed)
V(p,n) <+ 0;
```
is equivalent to

```
if (closed)
V(p,n) <+ 0;
else
I(p,n) <+ 0;
```
### 5.5 Accessing net and branch signals and attributes..........................................................................

The methods for accessing signal and attributes of nets and branches are described in this section.

#### 5.5.1 Accessing net and branch signals....................................................................................

Signals on nets and branches can be accessed only by either the access functions of the discipline associated
with them or by the generic potential or flow access functions. The name of the net or the branch shall be
specified as the argument to the access function. The syntax for analog signal access is shown in Syntax 5- 3.

nature_access_function ::= _// from A.8.2_
nature_attribute_identifier
| **potential**
| **flow**

branch_probe_function_call ::=
nature_access_function **(** branch_reference **)**
| nature_access_function **(** analog_net_reference [ **,** analog_net_reference ] **)**

port_probe_function_call ::= nature_access_function **( <** analog_port_reference **> )**

branch_reference ::= _// from A.8.9_
hierarchical_branch_identifier
| hierarchical_branch_identifier **[** constant_expression **]**
| hierarchical_unnamed_branch_reference

hierarchical_unnamed_branch_reference ::=
hierarchical_inst_identifier **.branch (** branch_terminal [ **,** branch_terminal ] **)**
| hierarchical_inst_identifier **.branch ( <** port_identifier **> )**
| hierarchical_inst_identifier **.branch ( <** hierarchical_port_identifier **> )**

analog_net_reference ::=


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
103
```
```
port_identifier
| port_identifier [ constant_expression ]
| net_identifier
| net_identifier [ constant_expression ]
| hierarchical_port_identifier
| hierarchical_port_identifier [ constant_expression ]
| hierarchical_net_identifier
| hierarchical_net_identifier [ constant_expression ]
```
analog_port_reference ::=
port_identifier
| port_identifier **[** constant_expression **]**
| hierarchical_port_identifier
| hierarchical_port_identifier **[** constant_expression **]**

```
Syntax 5-3—Syntax for analog signal access
```
Branch or port probe function calls shall only reference nets and ports that have been declared to belong to a
continuous discipline; references to branches require that the branch terminals belong to a continuous disci-
pline. The nature attribute identifier for a branch probe function call must be the access function name for
the potential or flow nature defined for the discipline associated with the nets or branches. For a port probe
function call, the nature attribute identifier must be the access function name for the flow nature associated
with the port, and the port reference many not use hierarchical specifications, i.e., it must be a declared port
of the module in which the port access function is used.

The examples below use the electrical discipline defined in 3.6.2.1 and its associated natures and their
access functions defined in 3.6.1.

```
module transamp(out, in);
inout out, in;
electrical out, in;
parameter real gm = 1;
analog
I(out) <+ gm*V(in);
endmodule
```
```
module resistor(p, n);
inout p, n;
electrical p, n;
branch (p,n) res;
parameter real R = 50;
analog
V(res) <+ R*I(res);
endmodule
```
The **potential** and **flow** access functions can also be used to access the potential or flow of a named or
unnamed branch. The example below demonstrates the potential access functions being used. Note V cannot
be used as an access function because there is a parameter called V declared in the module.

```
module measure1(p);
output p;
electrical p;
parameter real V = 1.1;
analog begin
$strobe ("%M: voltage ratio is %g", potential (p) / V);
end
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
104
```
```
endmodule
```
When the potential and flow access functions are used on an unnamed branch composed of two nets – the
discipline of both nets must be the same.

#### 5.5.2 Signal access for vector branches

Verilog-AMS HDL allows ports, nets, and branches to be arranged as vectors, however, the access functions
can only be applied to scalars or individual elements of a vector. The scalar element of a vector is selected
with an index, e.g., V(in[1]) accesses the voltage in[1].

The index must be a constant expression, though it may include genvar variables. Genvar variables can only
be assigned to as the iteration index of for loops; they allow signal access within looping constructs.

The following examples illustrate applications of access functions to elements of a an analog signal vector or
bus. In the N-bit DAC example, the analog vector in is accessed within an analog for-loop containing the
genvar variable i. In the following fixed-width DAC8 example, literal values are used to access elements of
the bus directly.

```
//
// N-bit DAC example.
//
```
```
module dac(out, in, clk);
parameter integer width = 8 from [2:24];
parameter real fullscale = 1.0, vth = 2.5, td = 1n, tt = 1n;
output out;
input [0:width-1] in;
input clk;
electrical out;
electrical [0:width-1] in;
electrical clk;
```
```
real aout;
genvar i;
```
```
analog begin
@( cross (V(clk) - vth, +1)) begin
aout = 0;
for (i = width - 1; i >= 0; i = i - 1) begin
if (V(in[i]) > vth) begin
aout = aout + fullscale/ pow (2, width - i);
end
end
end
V(out) <+ transition (aout, td, tt);
end
endmodule
```
```
//
// 8-bit fixed-width DAC example.
//
```
```
module dac8(out, in, clk);
parameter real fullscale = 1.0, vth = 2.5, td = 1n, tt = 1n;
output out;
input [0:7] in;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
105
```
```
input clk;
electrical out;
electrical [0:7] in;
electrical clk;
```
```
real aout;
```
```
analog begin
@( cross (V(clk) - 2.5, +1)) begin
aout = 0;
aout = aout + ((V(in[7]) > vth)? fullscale/2.0 : 0.0);
aout = aout + ((V(in[6]) > vth)? fullscale/4.0 : 0.0);
aout = aout + ((V(in[5]) > vth)? fullscale/8.0 : 0.0);
aout = aout + ((V(in[4]) > vth)? fullscale/16.0 : 0.0);
aout = aout + ((V(in[3]) > vth)? fullscale/32.0 : 0.0);
aout = aout + ((V(in[2]) > vth)? fullscale/64.0 : 0.0);
aout = aout + ((V(in[1]) > vth)? fullscale/128.0 : 0.0);
aout = aout + ((V(in[0]) > vth)? fullscale/256.0 : 0.0);
end
```
```
V(out) <+ transition (aout, td, tt);
end
```
```
endmodule
```
#### 5.5.3 Accessing attributes

Attributes are attached to the nature of a potential or flow. Therefore, the attributes for a net or a branch can
be accessed by using the hierarchical referencing operator (**.** ) to the potential or flow for the net or branch.

Example:

```
module twocap(a, b, n1, n2);
inout a, b, n1, n2;
electrical a, b, n1, n2;
branch (n1, n2) cap;
parameter real c= 1p;
analog begin
I(a,b) <+ c* ddt (V(a,b), a. potential. abstol );
I(cap) <+ c* ddt (V(cap), n1. potential. abstol );
end
endmodule
```
The syntax for referencing access attributes is shown in Syntax 5- 4. This syntax shall not be used for the
**access** , **ddt_nature** , or **idt_nature** attributes of a nature, nor any other attribute whose value is not
a constant expression.

nature_attribute_reference ::= _// from A.8.9_
net_identifier**.** potential_or_flow**.** nature_attribute_identifier

potential_or_flow ::= **potential** | **flow** _// from A.1.7_

```
Syntax 5-4—Syntax for referencing attributes of a net
```
The **abstol** attribute of a nature may also be accessed simply by using the nature’s identifier as the appro-
priate argument to the **ddt()** , **idt()** , or **idtmod()** operators described in 4.5.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
106
```
#### 5.5.4 Creating unnamed branches using hierarchical net references

An access function in a module can have one or more hierarchical net references to nets in other module
instances. In these cases, a new unnamed branch is created in the module containing the access function call.

Example:

```
module signal_monitor;
parameter refv = 2.3;
electrical a;
analog begin
V(a) <+ refv;
```
```
// Creates an unnamed branch in module signal_monitor between
// nets top.drv.a and implicit ground.
$strobe ("voltage at top.drv.a = %g volts", V(top.drv.a));
```
```
// Creates an unnamed branch in module signal_monitor between
// nets top.drv.a and top.drv.b
$strobe ("voltage diff in top.drv = %g volts", V(top.drv.a, top.drv.b));
```
```
// Creates an unnamed branch in module signal_monitor between
// local net a and top.drv.a
$strobe ("voltage diff from ref in top.drv = %g volts", V(a,top.drv.a));
```
```
// References the unnamed branch created in the first $strobe()
// statement
if (V(top.drv.a) > 10.0) $strobe ("voltage limit exceeded at top.drv.a");
```
```
end
endmodule
```
Note that even if the instance _top.drv_ already has an unnamed branch between nodes _a_ and _ground_ and _a_
and _b_ , the new unnamed branches are created in the module _signal_monitor_.

#### 5.5.5 Accessing nets and branch signals hierarchically

A module is allowed to access the potential and flow of a branch in another module instance using an access
function providing that value is available in the other instance. If it is not available, then an error shall be
reported. Reasons why it would be unavailable are:

```
— The branch does not exist in the other instance
— The access function is not the valid access function for that named branch
```
An example of a hierarchical access of the potential of a named branch is:

```
module top;
A a1();
B b1();
endmodule
```
```
module A;
electrical n,p;
branch (n,p) b;
analog V(b) <+ 1.34;
endmodule
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
107
```
```
module B;
analog $strobe ("voltage == %g", V(top.a1.b));
endmodule
```
To access an existing unnamed branch in another module instance, the _hierarchical_unnamed_branch_ref-
erence_ syntax is used.

Example:

```
analog begin
// strobes the voltage of the unnamed branch between
// nets a and b in top.drv.
$strobe ("Voltage == %g", V(top.drv .branch (a,b)));
```
```
// strobes the current flowing through the unnamed port
// branch for the port p in top.drv
$strobe( "Current == %g", I(top.drv .branch (<p>)));
end
```
### 5.6 Contribution statements

The branch contribution statement is used in the **analog** block to describe continuous-time behavior
between a module’s analog nets and ports. Contribution statements may be described in direct or indirect
form.

#### 5.6.1 Direct branch contribution statements

The direct contribution statement uses the _branch contribution operator_ **<+** to describe the mathematical
relationship between one or more analog nets within the module. The mapping is done with contribution
statements using the form shown in Syntax 5- 5 :

contribution_statement ::= branch_lvalue **<+** analog_expression **;** _// from A.6.10_

branch_lvalue ::= branch_probe_function_call _// from A.8.5_

branch_probe_function_call ::= _// from A.8.2_
nature_access_function **(** branch_reference **)**
| nature_access_function **(** analog_net_reference [ **,** analog_net_reference ] **)**

```
Syntax 5-5—Syntax for branch contribution
```
In general, a branch contribution statement consists of two parts, a left-hand side and a right-hand side, sep-
arated by a branch contribution operator. The right-hand side can be _analog_expression_ can be any combi-
nation of linear, nonlinear, or differential expressions of module signals, constants, and parameters which
evaluates to or can be promoted to a real value. The left-hand side specifies the source branch signal where
the right-hand side shall be assigned. It shall consist of a signal access function applied to a branch.

If the branch contribution statement is conditionally executed, the expression shall not include an analog fil-
ter function, as described in 4.5, unless the conditional expression is a constant expression.

Electrical behavior can be described using:

```
V(n1, n2) <+ expression;
```
or


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
108
```
```
I(n1, n2) <+ expression;
```
where (n1,n2) represents an unnamed source branch and V(n1,n2) refers to the potential on the branch,
while I(n1,n2) refers to the flow through the branch. The ‘V’ and ‘I’ functions (access attributes of the
nature) are obtained from the discipline’s potential and flow bindings of the electrical net (refer 3.6 for fur-
ther details on disciplines and natures).

Implementations may issue a warning if a contribution is made to an analog port declared with an input
direction. There are no restrictions on the probing of an analog port declared with an output direction.

There shall be no contributions to an implicit net; contributions shall be done only on analog nets declared
with a continuous discipline.

For example, the following modules model a resistor and a capacitor.

```
module resistor(p, n);
inout p, n;
electrical p, n;
branch (p,n) path; // named branch
parameter real r = 0;
```
```
analog
V(path) <+ r*I(path);
endmodule
```
```
module capacitor(p, n);
inout p, n;
electrical p, n;
parameter real c = 0;
```
```
analog
I(p,n) <+ c* ddt (V(p, n)); // unnamed branch p,n
endmodule
```
The **potential** and **flow** access functions can also be used to contribute to the potential or flow of a
named or unnamed branch. The example below demonstrates the **potential** access functions being used
to contribute to a branch and the **flow** and **potential** access functions being used to probe branches.
Note V and I cannot be used as access functions because there are parameters called V and I declared in the
module.

```
module measure2(p);
output p;
electrical p;
parameter real V = 1.1;
parameter real I = 1u;
parameter real R = 10k;
analog begin
potential (p) <+ flow (p) * R; // create a resistor
$strobe ("voltage ratio at port 'p' is %g", potential (p) / V);
$strobe ("current ratio through port 'p' is %g", flow (<p>) / I);
end
endmodule
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
109
```
**5.6.1.1 Relations**

Branch contribution statements implicitly define source branch relations. The branch is directed from the
first net of the access function to the second net. If the second net is not specified, the global reference node
( _ground_ ) is used as the reference net.

A branch relation is a path of the flow between two nets in a module. Each net has two quantities associated
with it—the potential of the net and the flow out of the net. In electrical circuits, the potential of a net is its
voltage, whereas the flow out of the net is its current. Similarly, each branch has two quantities associated
with it—the potential across the branch and the flow through the branch.

For example, the following module models a simple single-ended amplifier.

```
module amp(out, in);
input in;
output out;
electrical out, in;
parameter real Gain = 1;
```
```
analog
V(out) <+ Gain*V(in);
endmodule
```
**5.6.1.2 Evaluation**

A statement is evaluated as follows for source branch contributions:

```
1) The simulator evaluates the right-hand side.
2) The simulator adds the value of the right-hand side to any previously retained value of the branch for
later assignment to the branch. If there are no previously retained values, the value of the right-hand
side itself is retained.
3) At the end of the simulation cycle, the simulator assigns the retained value to the source branch.
```
Parasitics are added to the amplifier shown in 5.6.1.1 by simply adding additional contribution statements to
model the input admittance and output impedance.

Examples:

```
module amp(out, in);
inout out, in;
electrical out, in;
parameter real Gain = 1, Rin = 1, Cin = 1, Rout = 1, Lout = 1;
```
```
analog begin
// gain of amplifier
V(out) <+ Gain*V(in);
```
```
// model input admittance
I(in) <+ V(in)/Rin;
I(in) <+ Cin* ddt (V(in));
```
```
// model output impedance
V(out) <+ Rout*I(out);
V(out) <+ Lout* ddt (I(out));
end
endmodule
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
110
```
**5.6.1.3 Value retention**

When solving an **analog** block during an iteration, multiple contributions to the same potential branch or
same flow branch will be additive. However, contributing a flow to a branch which already has a value
retained for the potential results in the potential being discarded and the branch being converted to a flow
source. Conversely, contributing a potential to a branch which already has a value retained for the flow
results in the flow being discarded and the branch being converted into a potential source.

Unlike variables, the contributed value for a branch is only valid for the current iteration. If a branch is not
contributed to, directly or indirectly, for any particular iteration, and it is not a branch probe, it shall be
treated as a flow source with a value of 0.

Example 1:

```
if (closed)
V(p,n) <+ 0;
```
is equivalent to

```
if (closed)
V(p,n) <+ 0;
else
I(p,n) <+ 0;
```
Example 2:

The value retention rules specify that the example below will result in an assignment of 7.0 to the potential
source for the unnamed branch between ports p and n.

```
module value_ret(p, n);
inout p, n;
electrical p, n;
analog begin
V(p,n) <+ 1.0; // no previously-retained value, 1 is retained
I(p,n) <+ 2.0; // potential discarded; flow of 2 retained
V(p,n) <+ 3.0; // flow discarded; potential of 3 retained
V(p,n) <+ 4.0; // 4 added to previously-retained 3
end
endmodule
```
Example 3:

The following module defines a current-controlled current source. Because the branch flow I(ps,ns) appears
in an expression on the right-hand side, 5.4.2.1 states that this unnamed branch is a probe and its potential is
zero (0).
**module** cccs (p, n, ps, ns);
**inout** p, n, ps, ns;
electrical p, n, ps, ns;
**parameter real** A = 1.0;
**analog begin**
I(p,n) <+ A * I(ps,ns);
**end
endmodule**

The value retention rules are used to model switches, as described in 5.6.5.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
111
```
#### 5.6.2 Examples

The following examples demonstrate how to formulate models and the correspondence between the behav-
ioral description and the equivalent probe/source model.

**5.6.2.1 The four controlled sources**

The following example is used with each of the four behavioral statements listed below. Each statement cre-
ates a unique controlled source when inserted into this example.

```
module control_source (p, n, ps, ns);
inout p, n, ps, ns;
electrical p, n, ps, ns;
parameter A=1;
branch (ps,ns) in;
branch (p,n) out;
```
```
analog begin
// add behavioral statement here
end
endmodule
```
The model for a voltage controlled voltage source is

```
V(out) <+ A * V(in);
```
The model for a voltage controlled current source is

```
I(out) <+ A * V(in);
```
The model for a current controlled voltage source is

```
V(out) <+ A * I(in);
```
The model for a current controlled current source is

```
I(out) <+ A * I(in);
```
#### 5.6.3 Resistor and conductor

Figure 5- 3 shows the model for a linear conductor.

```
Figure 5-3: Linear conductor model
```
### v

```
module my_conductor(p,n);
inout p, n;
electrical p,n;
parameter real G=1;
branch (p,n) cond;
analog begin
I(cond) <+ G * V(cond);
end
endmodule
```
### Gv

### G


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
112
```
The assignment to I(cond) makes cond a current source branch and V(cond) simply accesses the poten-
tial probe built into the current source branch.

Figure 5- 4 shows the model for a linear resistor.

```
Figure 5-4: Linear resistor model
```
The assignment to V(res) makes res a potential source branch and I(res) simply accesses the optional
flow probe built into the potential source branch.

#### 5.6.4 RLC circuits

A series RLC circuit is formulated by summing the voltage across its three components,

which can be defined as

```
V(p, n) <+ R*I(p, n) + L* ddt (I(p, n)) + idt (I(p, n))/C;
```
A parallel RLC circuit is formulated by summing the currents through its three components,

which can be defined as

```
I(p, n) <+ V(p, n)/R + C* ddt (V(p, n)) + idt (V(p, n))/L;
```
#### 5.6.5 Switch branches

Contribution to a branch may be switched between potential and a flow during a simulation. This type of
branch is useful when modeling ideal switches and mechanical stops. As a result, contribution statements are
allowed within conditional statements but are not allowed within event control statements. Note that the
contribution statements shall not use _analog operators_ when the condition can change during the course of a
simulation.

### i

### Ri

```
module my_resistor(p,n);
inout p,n;
electrical p,n;
parameter real R=1;
branch (p,n) res;
analog begin
V(res) <+ R * I(res);
end
endmodule
```
### R

```
vt () Ri t () L
dt
```
```
dit () 1
C
```
```
---- i () d 
```
- 

```
t
```
## = ++

```
it () vt ()
R
```
##### -------- C

```
dt
```
```
dvt () 1
L
```
```
--- v () d 
```
- 

```
t
```
## = + + 


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
113
```
```
Figure 5-5: Circuit model for a switched source branch
```
For example, an ideal relay (a controlled switch) can be implemented using a switch branch as follows:

```
module relay (p, n, cp, cn);
inout p, n, cp, cn;
electrical p, n, cp, cn;
branch (p,n) out;
branch (cp,cn) ctrl;
parameter real thresh = 0;
```
```
analog begin
@( cross (V(ctrl) - thresh, 0))
; // acts only to resolve threshold crossings
```
```
if (V(ctrl) > thresh)
V(out) <+ 0;
else
I(out) <+ 0; // optional due to value retention
end
endmodule
```
A discontinuity of order zero ( 0 ) is assumed to occur when the branch switches and so it is not necessary to
use the **$discontinuity** function with switch branches. Usage of contribution statements inside event
control statements is disallowed as these statements may not be executed at every time point.

#### 5.6.6 Implicit Contributions

An important feature of contribution statements is that the value of the target may be expressed in terms of
itself. This is referred to as an implicit or fixed-point formulation.

Example:

```
I(diode) <+ is*( limexp ((V(diode) - r*I(diode))/ $vt ) - 1);
```
```
Position of the switch depends on whether a potential or flow is assigned to the branch.
```
```
f
```
```
p
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
114
```
Notice that I(diode) is found on both sides of the contribution operator. The underlying implementation of
the simulator will find the value of I(diode) that equals the sum of the contributions made to it, even if the
contributions are a function of I(diode) itself.

#### 5.6.7 Indirect branch contribution statements

Direct contribution statements are not the only way that values can be assigned to analog signals. Indirect
branch contributions provide an alternative approach that is useful in cases where direct contributions do not
behave as needed. One such case is the ideal opamp (or nullor). In this model, the output is driven to the
voltage that results in the input voltage being zero. The constitutive equation is

```
vin = 0, (1)
```
which can be formulated with a contribution statement as

```
V(out) <+ V(out) + V(in);
```
This statement defines the output of the opamp to be a controlled voltage source by assigning to V(out) and
defines the input to be high impedance by only probing the input voltage. That the desired behavior is
achieved can be seen by subtracting V(out) from both sides of the contribution operator, resulting in (1).
However, this approach does not result in the right tolerances being applied to the equation if out and in have
different disciplines. In this situation the tolerances for the equations would come from V(out) because it is
the target of the contribution, but the final equation does not contain V(out). It would be better if the toler-
ances for the equation were taken from V(in).

The indirect branch assignment should be used in this situation.

```
V(out): V(in) == 0;
```
which reads “drive V(out) so that V(in) == 0”. This indicates out is driven with a voltage source and the
source voltage needs to be adjusted so that the given equation is satisfied. Any branches referenced in the
equation are only probed and not driven. In particular, V(in) acts as a voltage probe.

The left-hand side of the equality operator must either be an access function, or **ddt** , **idt** or **idtmod**
applied to an access function. The tolerance for the equation is taken from the argument on the left side of
the equality operator, in this case V(in) as desired. Syntax 5- 6 shows the syntax for an indirect assignment
statement.

indirect_contribution_statement ::= _// from A.6.10_
branch_lvalue **:** indirect_expression **==** analog_expression **;**

indirect_expression ::= _// from A.8.3_
branch_probe_function_call
| port_probe_function_call
| **ddt (** branch_probe_function_call [ **,** abstol_expression ] **)**
| **ddt (** port_probe_function_call [ **,** abstol_expression ] **)**
| **idt (** branch_probe_function_call [ **,** analog_expression
[ **,** analog_expression [ **,** abstol_expression ] ] ] **)**
| **idt (** port_probe_function_call [ **,** analog_expression [ **,** analog_expression
[ **,** abstol_expression ] ] ] **)**
| **idtmod (** branch_probe_function_call [ **,** analog_expression [ **,** analog_expression
[ **,** analog_expression [ **,** abstol_expression ] ] ] ] **)**
| **idtmod (** port_probe_function_call [ **,** analog_expression [ **,** analog_expression
[ **,** analog_expression [ **,** abstol_expression ] ] ] ] **)**


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
115
```
branch_lvalue ::= branch_probe_function_call _// from A.8.5_

branch_probe_function_call ::= _// from A.8.2_
nature_attribute_identifier **(** branch_reference **)**
| nature_attribute_identifier **(** analog_net_reference [ **,** analog_net_reference ] **)**

```
Syntax 5-6—Syntax for indirect branch assignment
```
Indirect branch contributions shall not be used in conditional or looping statements, unless the conditional
expression is a constant expression. The constant expression shall not include the **analysis()** function
with an argument that can result in different return values during a single analysis, such as the "ic" or
"nodeset" arguments.

For example, a complete description of an ideal opamp is:

```
module opamp(out, pin, nin);
inout out, pin, nin;
electrical out, pin, nin;
analog
V(out):V(pin,nin) == 0;
endmodule
```
**5.6.7.1 Multiple indirect contributions**

For multiple indirect contribution statements, the targets frequently can be paired with any equation.

For example, the following ordinary differential equation,

can be written as

```
V(x): ddt (V(x)) == f(V(x), V(y), V(z));
V(y): ddt (V(y)) == g(V(x), V(y), V(z));
V(z): ddt (V(z)) == h(V(x), V(y), V(z));
```
or

```
V(y): ddt (V(x)) == f(V(x), V(y), V(z));
V(z): ddt (V(y)) == g(V(x), V(y), V(z));
V(x): ddt (V(z)) == h(V(x), V(y), V(z));
```
or

```
V(z): ddt (V(x)) == f(V(x), V(y), V(z));
V(x): ddt (V(y)) == g(V(x), V(y), V(z));
V(y): ddt (V(z)) == h(V(x), V(y), V(z));
```
without affecting the results.

```
dt
```
```
dx = fxyz 
```
```
dt
```
```
dy = gxyz 
```
```
dt
```
```
dz = hx yz 
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
116
```
**5.6.7.2 Indirect and direct contribution**

Indirect contribution statements is incompatible with direct contribution statements across the same pair of
analog nets (or any of its parallel branches). Once a value is indirectly assigned to a branch, it cannot be con-
tributed to using the branch contribution operator <+.

#### 5.6.8 Contributing hierarchically

**5.6.8.1 Contributions to branches between hierarchical nets**

Direct contribution statements can contribute to a branch between combinations of local and hierarchical
nets.

In these cases, a new unnamed branch is created in the module containing the direct contribution statements.

Example:
**module** source_driver();
electrical m;
**parameter real** vref = 0.0;
**analog begin**
V(m) <+ vref;

```
// creates an unnamed voltage source branch of 1.8
// volts between the net top.drv.x and implicit ground.
V(top.drv.x) <+ 1.8;
```
```
// creates an unnamed voltage source branch of 1.2
// volts between nets top.drv.x and top.drv.y
V(top.drv.x, top.drv.y) <+ 1.2;
```
```
// creates an unnamed voltage source branch of 0.9
// volts between the local net m and top.drv.y
V(m, top.drv.y) <+ 0.9;
```
```
end
endmodule
```
The simulator shall check if the contribution produces a solvable set of equations, e.g. no voltage source
loops created.

**5.6.8.2 Hierarchical direct contributions to branches**

Hierarchical direct contributions to named and unnamed branches is allowed provided that the branch is
suitable for such contributions. Reasons that a hierarchical branch contribution would not be allowed are:

```
— Invalid access function used for the contribution to the particular branch
— The hierarchical contribution changes the branch into a switch branch
```
The simulator shall check if a hierarchical contribution produces a solvable set of equations, e.g. no voltage
source loops created.

Example:
**module** source_driver();
**analog begin**
// contributes 1.8 volts to the named branch br_v in top.drv
V(top.drv.br_v) <+ 1.8;


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
117
```
```
// contributes 1.2 volts to the unnamed branch between
// nets x and y in top.drv
V(top.drv.branch(x,y)) <+ 1.2;
```
```
// contributes 1mA to the named branch br_i in top.drv
I(top.drv.br_i) <+ 1m;
```
```
end
endmodule
```
Hierarchical contributions are not allowed to branches that have been indirectly contributed to (see 5.6.7)

### 5.7 Analog procedural assignments

Analog procedural assignments are used for modifying analog **integer** , **real** , and **string** variables
including array variables. The syntax for procedural assignments shown in Syntax 5- 7.

analog_procedural_assignment ::= analog_variable_assignment **;** _// from A.6.2_

analog_variable_assignment ::=
scalar_analog_variable_assignment
| array_analog_variable_assignment

scalar_analog_variable_assignment ::= scalar_analog_variable_lvalue **=** analog_expression

analog_variable_lvalue ::= _// from A.8.5_
variable_identifier
| variable_identifier **[** analog_expression **]** { **[** analog_expression **]** }

array_analog_variable_assignment ::= array_analog_variable_lvalue **=** array_analog_variable_rvalue **;**

array_analog_variable_rvalue ::=
array_variable_identifier
| array_ variable_identifier **[** analog_expression **]** { **[** analog_expression **]** }
| assignment_pattern

```
Syntax 5-7—Syntax for procedural assignments
```
For scalar variable assignments the following requirements hold;

```
— The left-hand side of a procedural assignment shall be scalar, either an integer , real , or
string identifier or an element of an integer , real or string array.
— The right-hand side expression can be any arbitrary expression constituted from legal operands and
operators as described in Clause 4 that evaluates to a scalar.
—A scalar_analog_variable_assignment is defined as a variable assignment whose right-hand side
expression is an analog_expression involving analog operators.
— The following semantic restrictions are applicable to the analog_expression in the scalar_analog_-
variable_assignment syntax:
— Concatenation expressions cannot be used as part of the analog_expression (assigning to list of
values in the analog context is not allowed).
— Analog filter functions cannot be used as part of the analog_expression syntax if the statement
is conditionally executed during simulation.
— Hierarchical assignment of a variable from another scope/module is not allowed
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
118
```
Verilog-AMS supports both packed arrays and unpacked arrays of data. The term packed array is used to
refer to the dimensions declared before the data identifier name. The term unpacked array is used to refer to
the dimensions declared after the data identifier name.

Examples:

```
wire [7:0] c1; // packed array of scalar wire types
real u [7:0]; // unpacked array of real types
```
The requirements of unpacked array variable assignments are a subset of the requirements of IEEE Std 1800
SystemVerilog.

```
— The array on the LHS of the assignment shall be an array variable, a slice of an array variable or an
array parameter (when the default value of the parameter is assigned).
— The arrays on the LHS and the RHS of the assignment must be unpacked.
— Array assignments shall only be done with arrays that are compatible. An array, or a slice of such an
array, shall be assignment compatible with any other such array or slice if all the following condi-
tions are satisfied:
— The element types of source and target shall be equivalent.
— Every dimension of the source array shall have the same number of elements as the target array.
Example:
int A[10:1]; // fixed-size array of 10 elements
int B[0:9]; // fixed-size array of 10 elements
int C[24:1]; // fixed-size array of 24 elements
A = B; // ok. Compatible type and same size
A = C; // type check error: different sizes
```
### 5.8 Analog conditional statements

There are two types of conditional statement allowed in analog behavior:

```
— if-else-if statements
— case statements
```
#### 5.8.1 if-else-if statement...........................................................................................................

The _if-else statement_ is used to determine whether a statement is executed or not. The syntax of an _analog
conditional statement_ is shown in Syntax 5- 8. If any of the conditionally-executed statements ( _analog_state-
ment_or_null_ ) contains an _analog operator_ , the conditional expression ( _analog_expression_ ) shall be a _anal-
ysis_or_constant_expression_. (See the discussion in 4.5.15 regarding restrictions on the usage of analog
operators.)

analog_conditional_statement ::= _// from A.6.6_
**if (** analog_expression **)** analog_statement_or_null
{ **else if (** analog_expression **)** analog_statement_or_null }
[ **else** analog_statement_or_null ]

```
Syntax 5-8—Syntax of conditional statement
```
If the expression evaluates to _True_ (that is, has a non-zero value), the _analog statements specified as part of
the true conditional_ shall be executed. If it evaluates to _False_ (has a zero value ( 0 )), the _analog statements_


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
119
```
_specified as part of the true conditional_ shall not be executed. If analog statements are specified as part of
the false condition using **else** and expression is _False_ , these statements shall be executed.

Since the numeric value of the **if** expression is tested for being zero ( 0 ), certain shortcuts are possible (see
4.2).

#### 5.8.2 Examples

For example, the following two statements express the same logic:

```
if (expression)
if (expression != 0)
```
Because the **else** part of an _if-else_ is optional, there can be confusion when an **else** is omitted from a
nested **if()** sequence. This is resolved by always associating the **else** with the closest previous **if()**
which lacks an **else**.

In the example below, the **else** goes with the inner **if()** , as shown by indentation.

```
if (index > 0)
if (i > j)
result = i;
else // else applies to preceding if
result = j;
```
If that association is not desired, a _begin-end_ shall be used to force the proper association, as shown below.

```
if (index > 0) begin
if (i > j)
result = i;
end
else result = j;
```
Nesting of _if_ statements (known as an _if-else-if_ construct) is the most general way of writing a multi-way
decision. The expressions are evaluated in order; if any expression is _True_ , the statement associated with it
shall be executed and this action shall terminate the whole chain. Each statement is either a single statement
or a sequential block of statements.

#### 5.8.3 Case statement.................................................................................................................

The _case statement_ is a multi-way decision statement which tests if an expression matches one of a number
of other expressions, and if so, branches accordingly. The case statement has the syntax shown in
Syntax 5- 9.

analog_case_statement ::= _// from A.6.7_
**case (** analog_expression **)** analog_case_item { analog_case_item } **endcase**
| **casex (** analog_expression **)** analog_case_item { analog_case_item } **endcase**
| **casez (** analog_expression **)** analog_case_item { analog_case_item } **endcase**

analog_case_item ::=
analog_expression { **,** analog_expression } **:** analog_statement_or_null
| **default** [ **:** ] analog_statement_or_null

```
Syntax 5-9—Syntax for case statement
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
120
```
The **default** statement is optional. Use of multiple default statements in one case statement is illegal.

The _analog_expression_ and the _analog_case_item_ expression can be computed at runtime; neither expres-
sion is required to be a constant expression.

The _analog_case_item_ expressions are evaluated and compared in the exact order in which they are given.
During this linear search, if one of the _analog_case_item_ expressions matches the _analog_expression_ given
in parentheses, then the statement associated with that _analog_case_item_ is executed. If all comparisons fail,
and the default item is given, then the default item statement is executed; otherwise none of the _analog_-
case_item_ statements are executed.

The **casex** and the **casez** versions of the _case_ statement are described in 7.3.2 and IEEE Std 1364 Ver-
ilog.

#### 5.8.4 Restrictions on conditional statements............................................................................

Since analog filter functions have to be evaluated at every time point these are restricted to be used inside
conditional statements (if-else-if and case) unless the conditional expression is a _constant expression._ Also,
for the use of analog filter functions, the conditional statements cannot be conditionally executed (nested
conditional statements). Contributions statements are allowed as part of the conditional analog statements
(refer 5.6.5 for details on switch branches).

Event control statements (e.g.: **timer** , **cross** ) cannot be used inside conditional statements unless the
conditional expression is a constant expression.

### 5.9 Looping statements

There are several types of looping statements: **repeat** , **while** , and **for**. These statements provide a
means of controlling the execution of a statement zero ( 0 ), one ( 1 ), or more times.

The **for** looping statements can be used to describe analog behaviors using analog operators.

The following restrictions are applied to looping statements ( **repeat** , **while** and **for** ) except for
_analog_for_ statements, refer 5.9.3

```
— Analog filter functions are not allowed
— Event control statements are not allowed
— Contribution statements are not allowed
```
#### 5.9.1 Repeat and while statements

**repeat()** executes a statement a fixed number of times. Evaluation of the expression decides how many
times a statement is executed.

**while()** executes a statement until an expression becomes _False_. If the expression starts out _False_ , the
statement is not executed at all.

The _repeat_ and _while_ expressions shall be evaluated once before the execution of any statement in order to
determine the number of times, if any, the statements are executed. The syntax for **repeat()** and
**while()** statements is shown in Syntax 5- 10.

analog_loop_statement ::= _// from A.6.8_
**repeat (** analog_expression **)** analog_statement


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
121
```
```
| while ( analog_expression ) analog_statement
```
```
Syntax 5-10—Syntax for repeat and while statements
```
#### 5.9.2 For statements

The **for()** statement is a looping construct which controls execution of its associated statement(s) using an
index variable. In the case of _analog_for_ statement the control mechanism shall consist of _genvar_initializa-
tion_ and _genvar_expression_ s to adhere to the restrictions associated with the use of analog operators.
Syntax 5- 11 shows the syntax for the looping statements that can be used in analog behavior.

analog_loop_statement ::= _// from A.6.8_

```
| for ( analog_variable_assignment ; analog_expression ; analog_variable_assignment )
analog_statement
```
```
Syntax 5-11—Syntax for the for statements
```
The **for()** statement controls execution of its associated statement(s) by a three-step process:

```
1) it executes an assignment normally used to initialize an integer which controls the number of loops
executed.
2) it evaluates an expression—if the result is zero ( 0 ), the for-loop exits; otherwise, the for-loop exe-
cutes its associated statement(s) and then performs Step 3.
3) it executes an assignment normally used to modify the value of the loop-control variable and repeats
Step 2.
```
#### 5.9.3 Analog For Statements

The _analog_for_ statements are syntactically equivalent to **for()** statements except the associated analog
statement can contain analog operators. The _analog_loop_generate_statement_ puts the additional restriction
upon the procedural assignment and conditional expressions of the _for-loop_ to be statically evaluatable. Ver-
ilog-AMS HDL provides genvar-derived expressions for this purpose. Syntax 5- 12 shows the syntax for the
_analog_for_ statement.

analog_loop_generate_statement ::= _// from A.4.2_
**for (** genvar_initialization **;** genvar_expression **;** genvar_iteration **)**
analog_statement

```
Syntax 5-12—Syntax for the analog_for statements
```
Examples:

```
module genvarexp(out, dt);
parameter integer width = 1;
output out;
input [1:width] dt;
electrical out;
electrical [1:width] dt;
genvar k;
real tmp;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
122
```
```
analog begin
tmp = 0.0;
for (k = 1; k <= width; k = k + 1) begin
tmp = tmp + V(dt[k]);
V(out) <+ ddt (V(dt[k]));
end
end
endmodule
```
See the discussion in 4.5.15 regarding other restrictions on the usage of analog operators.

### 5.10 Analog event control statements

The analog behavior of a component can be controlled using _events_. _events_ have the following characteris-
tics:

```
— events have no time duration
— events can be triggered and detected in different parts of the behavioral model
— events do not block the execution of an analog block
— events can be detected using the @ operator
— events do not hold any data
— there can be both digital and analog events
```
There are three types of analog events, _global events_ (5.10.2), _monitored events_ (5.10.3), and _named events_
(5.10.4). Null arguments are not allowed in analog events. Analog event detection consist of an event
expression followed by a procedural statement, as shown in Syntax 5- 13.

analog_event_control_statement ::= analog_event_control analog_event_statement _// from A.6.5_

analog_event_control ::=
**@** hierarchical_event_identifier
| **@ (** analog_event_expression **)**

analog_event_expression ::=
expression
| **posedge** expression
| **negedge** expression
| hierarchical_event_identifier
| **initial_step** [ **( "** analysis_identifier **"** { **, "** analysis_identifier **"** } **)** ]
| **final_step** [ **( "** analysis_identifier **"** { **, "** analysis_identifier **"** } **)** ]
| analog_event_functions
| analog_event_expression **or** analog_event_expression
| analog_event_expression **,** analog_event_expression

analog_event_functions ::=
**cross (** analog_expression [ **,** analog_expression_or_null
[ **,** analog_expression_or_null [ **,** analog_expression_or_null [ **,** analog_expression ] ] ] ] **)**
| **above (** analog_expression [ **,** analog_expression_or_null
[ **,** analog_expression_or_null [ **,** analog_expression ] ] ] **)**
| **timer (** analog_expression [ **,** analog_expression_or_null
[ **,** analog_expression_or_null [ **,** analog_expression ] ] ] **)**
| **absdelta (** analog_expression **,** analog_expression


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
123
```
```
[ , analog_expression_or_null [ , analog_expression_or_null [ , analog_expression ] ] ] )
```
analog_event_statement ::= _// from A.6.4_
{ attribute_instance } analog_loop_statement
| { attribute_instance } analog_case_statement
| { attribute_instance } analog_conditional_statement
| { attribute_instance } analog_procedural_assignment
| { attribute_instance } analog_event_seq_block
| { attribute_instance } analog_system_task_enable
| { attribute_instance } disable_statement
| { attribute_instance } event_trigger
| { attribute_instance } **;**

```
Syntax 5-13—Syntax for event detection in analog context
```
The procedural statements following the event expression is executed whenever the event described by the
expression changes. The analog event detection is non-blocking, meaning the execution of the procedural
statement is skipped unless the analog event has occurred. The event expression consists of one or more sig-
nal names, global events, or monitored events separated by the **or** operator.

The following restrictions applies to the statements that can be specified within an event control block:

```
— Analog filter functions cannot be used as part of the event control statement. This statement cannot
maintain its internal state since it is only executed intermittently when the corresponding event is
triggered
— Contribution statements cannot be used inside an event control block because it can generate discon-
tinuity in analog signals
— Nested event control statements are not allowed
```
The parentheses around the event expression are required.

Analog events can also be detected within digital blocks. Syntax 5- 14 shows the usage of analog event con-
trol statements inside digital to monitor analog values in the digital context. The usage of _initial_step_ and
_final_step_ analog events are not allowed in the digital context. Refer 7.3.4 for further details on detecting
continuous events in a discrete context.

event_expression ::= _// from A.6.5_
expression
| **posedge** expression
| **negedge** expression
| hierarchical_event_identifier
| event_expression **or** event_expression
| event_expression **,** event_expression
| analog_event_functions
| **driver_update** expression
| analog_variable_lvalue

```
Syntax 5-14—Syntax for analog event detection in digital context
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
124
```
#### 5.10.1 Event OR operator

The “OR-ing” of events indicates the occurrence of any one of the events specified shall trigger the execu-
tion of the procedural statement following the event. The keyword **or** is used as an event OR operator. A
comma ( **,** ) can be used interchangeably with the keyword **or** to OR event expressions.

Examples:

```
analog begin
@( initial_step or cross (V(smpl)-2.5,+1)) begin
vout = (V(in) > 2.5);
end
V(out) <+ vout;
end
```
Here, **initial_step** is a global event and **cross()** returns a monitored event. The variable vout is set
to zero ( 0 ) or one ( 1 ) whenever either event occurs.

#### 5.10.2 Global events...................................................................................................................

Global events are generated by a simulator at various stages of simulation. The user model cannot generate
these events. These events are detected by using the name of the global event in an event expression with the
**@** operator.

Global events are pre-defined in Verilog-AMS HDL. These events cannot be redefined in a model.

The pre-defined global events are shown in Syntax 5- 15.

analog_event_expression ::= _// from A.6.5_
...
| **initial_step** [ **( "** analysis_identifier **"** { **, "** analysis_identifier **"** } **)** ]
| **final_step** [ **( "** analysis_identifier **"** { **, "** analysis_identifier **"** } **)** ]
...

```
Syntax 5-15—Global events
```
**initial_step** and **final_step** generate global events on the first and the last point in an analysis
respectively. **final_step** will also generate a global event upon the termination of the simulation due to a
**$finish()** simulation control task (see 9.7.1). They are useful when performing actions which should
only occur at the beginning or the end of an analysis. Both global events can take an optional argument, con-
sisting of an analysis list for the active global event.

Examples:

```
@( initial_step ("ac", "dc")) // active for dc and ac only
@( initial_step ("tran")) // active for transient only
```
Table 5- 1 describes the return value of **initial_step** and **final_step** for standard analysis types.
Each column shows the return-on-event status. One ( 1 ) represents _Yes_ and zero ( 0 ) represents _No_. A Ver-


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
125
```
ilog-AMS HDL simulator can use any or all of these typical analysis types. Additional analysis names can
also be used as necessary for specific implementations. (See 4.6.1 for further details.)

The following example measures the bit-error rate of a signal and prints the result at the end of the simula-
tion.

```
module bitErrorRate (in, ref);
input in, ref;
electrical in, ref;
parameter real period=1, thresh=0.5;
integer bits, errors;
```
```
analog begin
@( initial_step ) begin
bits = 0;
errors = 0;
end
```
```
@( timer (0, period)) begin
if ((V(in) > thresh) != (V(ref) > thresh))
errors = errors + 1;
bits = bits + 1;
end
```
```
@( final_step )
$strobe ("bit error rate = %f%%", 100.0 * errors / bits );
end
endmodule
```
```
Table 5-1—Return value of initial_step and final_step
```
```
Analysisa
```
```
apX designates frequency/time analysis point X, X = 1 to N; OP designates the Operating Point.
```
```
DCOP
OP
```
```
Sweepb
d1 d2 dN
```
```
bSweep refers to a dc analysis in which a parameter is swept through multiple values and an operating point analysis
is performed for each value. d1 refers to the first point in the sweep; d2 through dN are subsequent points.
```
```
TRAN
OP p1 pN
```
```
AC
OP p1 pN
```
```
NOISE
OP p1 pN
```
```
initial_step 1 1 0 0 1 0 0 1 0 0 1 0 0
initial_step ("ac") 0 0 0 0 0 0 0 1 0 0 0 0 0
initial_step ("noise") 0 0 0 0 0 0 0 0 0 0 1 0 0
initial_step ("tran") 0 0 0 0 1 0 0 0 0 0 0 0 0
initial_step ("dc") 1 1 0 0 0 0 0 0 0 0 0 0 0
initial_step ( unknown ) 0 0 0 0 0 0 0 0 0 0 0 0 0
final_step 1 0 0 1 0 0 1 0 0 1 0 0 1
final_step ("ac") 0 0 0 0 0 0 0 0 0 1 0 0 0
final_step ("noise") 0 0 0 0 0 0 0 0 0 0 0 0 1
final_step ("tran") 0 0 0 1 0 0 1 0 0 0 0 0 0
final_step ("dc") 1 0 0 1 0 0 0 0 0 0 0 0 0
final_step ( unknown ) 0 0 0 0 0 0 0 0 0 0 0 0 0
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
126
```
**initial_step** and **final_step** take a list of quoted strings as optional arguments. The strings are
compared to the name of the analysis being run. If any string matches the name of the current analysis name,
the simulator generates an event on the first point and the last point of that particular analysis, respectively.

If no analysis list is specified, the **initial_step** global event is active during the solution of the first
point (or initial DC analysis) of every analysis. The **final_step** global event, without an analysis list, is
only active during the solution of the last point of every analyses.

#### 5.10.3 Monitored events.............................................................................................................

Monitored events are detected using event functions with the **@** operator. The triggering of a monitored event
is implicit due to change in signals, simulation time, or other runtime conditions.

**5.10.3.1 cross function**

The **cross()** function is used for generating a monitored analog event to detect threshold crossings in ana-
log signals when the expression crosses zero (0) in the specified direction. In addition, **cross()** controls
the timestep to accurately resolve the crossing.

analog_event_functions ::= _// from A.6.5_
**cross (** analog_expression [ **,** analog_expression_or_null
[ **,** analog_expression_or_null [ **,** analog_expression_or_null [ **,** analog_expression ] ] ] ] **)**
...

```
Syntax 5-16—The cross analog event function
```
The expressions in this syntax have the following meanings:

```
cross ( expr [ , dir [ , time_tol [ , expr_tol [ , enable ] ] ] ] )
```
where _expr_ is required, and _dir_ , _time_tol_ , _expr_tol_ , and _enable_ are optional. The _expr_ , _dir_ , and _enable_ argu-
ments are specified as _analog_expression_ .The tolerances ( _time_tol_ and _expr_tol_ ) are specified as _analog_ex-
pression_ and shall be non-negative.If a value of zero (0.0) is specified, the simulator shall apply a suitable
value. The _dir_ and _enable_ arguments, if specified, shall evaluate to integers. If the tolerances are not speci-
fied, then the tool (e.g., the simulator) sets them. If either or both tolerances are defined, then the direction
shall also be defined.

If the direction indicator is set to 0 or is not specified, the event and timestep control occur on both positive
and negative crossings of the signal. If _dir_ is +1, the event and timestep control only occur on rising edge
transitions of the signal. If _dir_ is –1, the event and timestep control only occur on falling edge transitions of
the signal. For any other values of _dir_ , the **cross()** function does not generate an event and does not act to
control the timestep.

_expr_tol_ and _time_tol_ are absolute tolerances and are defined as shown in Figure 5- 6. They represent the
maximum allowable error between the true crossing point and when the event triggers. The event shall occur


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
127
```
after the threshold crossing, and while the signal remains in the box defined by actual crossing and _expr_tol_
and _time_tol_.

```
Figure 5-6: Timing of event relative to threshold crossing.
```
If _expr_tol_ is specified, _time_tol_ shall also be specified and both tolerances shall be satisfied at the crossing.

Specifying a small _time_tol_ ensures a slowly-varying expression triggers an event within a reasonable time
from the actual crossing point, and specifying a small _expr_tol_ ensures a rapidly-varying expression triggers
an event within a reasonable value of the actual crossing point. However, setting either of these tolerances to
unrealistically small values can adversely affect the simulator's performance.

Although changes to either _time_tol_ or _expr_tol_ during the simulation are permitted, care should be taken to
ensure that they do not vary from iteration to iteration. Use of constant tolerances, or changing the tolerances
via an event statement, is desirable.

The following description of a sample-and-hold illustrates how the **cross()** function can be used.

```
module sh (in, out, smpl);
parameter real thresh = 0.0;
parameter integer dir = +1 from [-1:+1] exclude 0;
output out;
input in, smpl;
electrical in, out, smpl;
real state;
```
```
analog begin
@( cross (V(smpl) - thresh, dir))
state = V(in);
V(out) <+ transition (state, 0, 10n);
end
endmodule
```
If _enable_ is specified and nonzero, then **cross()** functions as just described. If _enable_ argument is speci-
fied and it is zero, then **cross()** is inactive, meaning that it does not generate an event at threshold cross-
ings and does not act to control the timestep. Thus, there are two ways to disable the cross function, either by
specifying _enable_ as 0, or giving a value other than –1, 0, or 1 to _dir_. In the following example, the first way
is used to allow the sample and hold to be disabled. Notice that in this example, the tolerances are not speci-
fied, and so take their default values.

```
module sh (in, out, smpl, en);
parameter real thresh = 0.0;
parameter integer dir = +1 from [-1:+1] exclude 0;
output out;
input in, smpl, en;
```
```
expr_tol
```
```
time_tol
```
```
event
threshold
```
```
actual crossing
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
128
```
```
electrical in, out, smpl;
real state;
```
```
analog begin
@( cross (V(smpl) - thresh, dir, , , en === 1'b1))
state = V(in);
V(out) <+ transition (state, 0, 10n);
end
endmodule
```
The **cross()** function maintains its internal state and has the same restrictions as analog operators. In par-
ticular, it shall not be used inside an **if** , **case** , **casex** , or **casez** statement unless the conditional expres-
sion is a genvar expression. In addition, **cross()** is not allowed in the **repeat** and **while** iteration
statements. It is allowed in the _analog_for_ statements.

**5.10.3.2 above function**

The **above()** function is almost identical to the **cross()** function, except that it also triggers during ini-
tialization or dc analysis. It generates a monitored analog event to detect threshold crossings in analog sig-
nals when the expression crosses zero (0) from below. As with the **cross()** function, **above()** controls
the timestep to accurately resolve the crossing during transient analysis.

analog_event_functions ::= _// from A.6.5_
...
| **above (** analog_expression [ **,** analog_expression_or_null
[ **,** analog_expression_or_null [ **,** analog_expression ] ] ] **)**
...

```
Syntax 5-17—The above analog event function
```
The expressions in this syntax have the following meanings:

```
above ( expr [ , time_tol [ , expr_tol [ , enable ] ] ] )
```
where _expr_ is required. The tolerances ( _time_tol_ and _expr_tol_ ) are optional, but if specified shall be non-neg-
ative.If a value of zero (0.0) is specified, the simulator shall apply a suitable value. The _enable_ argument, if
specified, shall evaluate to an integer, all other arguments are real expressions. If the tolerances are not spec-
ified, then the tool (e.g., the simulator) sets them.

The **above()** function can generate an event during initialization. If the expression is positive at the con-
clusion of the initial condition analysis that precedes a transient analysis, the **above()** function shall gener-
ate an event. In contrast, the **cross()** function can only generate an event after the simulation time has
advanced from zero. The **cross()** function will not generate events for non-transient analyses, such as ac,
dc, or noise analyses of SPICE (see 4.6.1), but the **above()** function can. During a dc sweep, the
**above()** function shall also generate an event when the expression crosses zero from below; however, the
step size of the dc sweep is not controlled to accurately resolve the crossing.

The following example uses the **above()** function in place of the **cross()** function in the description of
a simplified version of the sample-and-hold module introduced in the previous section. If the voltage on the
smpl port is above 2.5V initially (at time=0), then use of the **above()** function ensures that the input is
sampled and passed to the output when solving for the initial state of the circuit. If the voltage on the smpl
port never crosses 2.5V in the positive direction, then the **cross()** function of the previous example would
never trigger, even if the voltage on the smpl port is always above 2.5V.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
129
```
```
module sh (in, out, smpl);
output out;
input in, smpl;
electrical in, out, smpl;
real state;
```
```
analog begin
@( above (V(smpl) - 2.5))
state = V(in);
V(out) <+ transition (state, 0, 10n);
end
endmodule
```
If _enable_ is specified and nonzero, then **above()** functions as just described. If _enable_ argument is speci-
fied and it is zero, then **above()** is inactive, meaning that it does not generate an event at threshold cross-
ings and does not act to control the timestep.

The **above()** function maintains its internal state and has the same restrictions on its use as the **cross()**
function.

**5.10.3.3 timer function**

The **timer()** function is used to generate analog events to detect specific points in time.

analog_event_functions ::= _// from A.6.5_
...
| **timer (** analog_expression [ **,** analog_expression_or_null
[ **,** analog_expression_or_null [ **,** analog_expression ] ] ] **)**

```
Syntax 5-18—The timer analog event function
```
The expressions in this syntax have the following meanings:

```
timer ( start_time [ , period [ , time_tol [ , enable ] ] ] )
```
where _start_time_ is required; _period_ , _time_tol_ and _enable_ are optional arguments. The _start_time_ and _period_
arguments are _analog_expressions_. The tolerance ( _time_tol_ ) is an _analog_expression_ and shall be non-nega-
tive.If a value of zero (0.0) is specified, the simulator shall apply a suitable value. The _enable_ argument, if
specified, shall evaluate to an integer.

The **timer()** function schedules an event which occurs at an absolute time ( _start_time_ ). The analog simu-
lator places a time point within _time_tol_ of an event. At that time point, the event evaluates to _True_.

If _time_tol_ is not specified, the default time point is at, or just beyond, the time of the event.Although
changes to _time_tol_ during the simulation are permitted, care should be taken to ensure that it does not vary
from iteration to iteration. Use of a constant _time_tol_ , or changing it via an event statement, is desirable.

If the _period_ is specified as greater than zero ( 0 ), the timer function schedules subsequent events at multiples
of _period_. If the _period_ expression evaluates to a value less than or equal to 0.0, the timer shall trigger only
once at the specified _start_time_ (if the _start_time_ is in the future with respect to the current simulation time).


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
130
```
If the _start_time_ or _period_ expressions change value during the evaluation of the **analog** block, the next
event will be scheduled based on the latest value of the _start_time_ and _period_.

If _enable_ is specified and nonzero, then **timer()** functions as just described. If _enable_ argument is speci-
fied and it is zero, then **timer()** is inactive, meaning that it does not generate events as long as _enable_ is
zero. However, it will start generating events once _enable_ returns to being nonzero as if it had never been
disabled.

A pseudo-random bit stream generator is an example how the timer function can be used.

```
module bitStream (out);
output out;
electrical out;
parameter period = 1.0;
integer x;
```
```
analog begin
@( timer (0, period))
x = $random + 0.5;
V(out) <+ transition ( x, 0.0, period/100.0 );
end
endmodule
```
**5.10.3.4 absdelta function**

The **absdelta()** event function enables efficient and accurate sampling of analog signals for use in digi-
tal behavioral code. The **absdelta()** event function is particularly useful for the conversion of analog-
owned variables to real-typed digital-owned variables just as the **above()** event function is particularly
useful for the conversion of analog-owned variables to logic-typed digital-owned variables.

According to criteria you set, the simulator can generate an **absdelta** event when an analog expression
changes more than a specified amount, a capability that is typically used to discretize analog signals. Use the
**absdelta()** function to specify when the simulator generates an **absdelta** event. This function is only
allowed in an **initial** or **always** block of a Verilog-AMS module.

analog_event_functions ::= _// from A.6.5_
**absdelta (** analog_expression **,** analog_expression
[ **,** analog_expression_or_null [ **,** analog_expression_or_null [ **,** analog_expression ] ] ] **)**
...

```
Syntax 5-19—The absdelta analog event function
```
The expressions in this syntax have the following meanings:

```
absdelta ( expr , delta [ , time_tol [ , expr_tol [ , enable ] ] ] )
```
where _expr_ and _delta_ are required; _time_tol_ , _expr_tol_ and _enable_ are optional arguments. The mandatory
_expr_ and _delta_ arguments are specified as an _analog_expression_ and _delta_ shall be non-negative. Both the
tolerances ( _time_tol_ and _expr_tol_ ) are specified as an _analog_expression_ and shall be non-negative.If a value
of zero (0.0) is specified, the simulator shall apply a suitable value. The _enable_ argument is specified as an
_analog_expression_ and shall evaluate to an integer.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
131
```
A specified _time_tol_ that is smaller than the time precision is ignored and the time precision is used instead.
The _expr_tol_ argument specifies the largest difference in _expr_ that you consider negligible. If the tolerances
are not specified, then the tool (e.g., the simulator) sets them.

The **absdelta()** function does not force timesteps in the analog solver - it just observes the expr argu-
ment and generates events at the appropriate times to meet the requirements above. To avoid forcing analog
timesteps just to determine an event time, **absdelta()** may interpolate the time at which an event
occurred if necessary.

The **absdelta()** function generates events for the following times and conditions.

```
— During initialization or dc sweep analysis.
— When the enable argument changes from zero to non-zero.
— When the expr value changes in absolute value by more than delta, relative to the previous
absdelta() event (but not when the current time is within time_tol of the previous
absdelta() event). The simulator is allowed to schedule this event at any time between the time
corresponding to the interpolated absolute change of ( delta - expr_tol ) and the time corresponding to
the interpolated absolute change of ( delta + expr_tol ) for performance or other reasons.
— When expr changes direction (but not when the amount of the change is less than expr_tol ).
```
If _delta_ is set to zero, an event is generated every timestep the expression value changes with zero tolerances.
_expr_tol_ and _time_tol_ , if specified, will have no effect and be ignored. No events are generated at times cal-
culated by interpolation. Generating events on every value change may severely impact simulation perfor-
mance and so setting _delta_ to zero should only be done with great care.

_expr_tol_ and _time_tol_ can be changed during the simulation. For example, one might change _expr_tol_ when
_delta_ is changed so that the error is proportionate. _time_tol_ can be changed to help filter fast changing signals
to only enforce minimum time-placed events. The values of _delta_ , _expr_tol_ and _time_tol_ do not directly trig-
ger an **absdelta()** event. Instead, they are used by the function to determine when an event should be
generated due to a change in _expr_ ; this could be during any interpolation of _expr_ to satisfy the tolerance
requirements.

If _enable_ is specified and nonzero, then **absdelta()** functions as just described. If _enable_ argument is
specified and it is zero, then the **absdelta()** function is inactive, meaning that it does not generate
events.

The following example describes an event-driven electrical to **wreal** conversion module where the
**absdelta()** function is used to determine when the electrical input signal is converted to a **wreal** output
signal.

```
`include "disciplines.vams"
`timescale 1ns / 100ps
module electrical_sampler (e_in, r_out);
input e_in;
output r_out;
electrical e_in;
wreal r_out;
parameter real vdelta=0.1 from (0: inf ); // voltage delta
parameter real ttol=1n from (0:1m]; // time tolerance
parameter real vtol=0.01 from (0: inf ); // voltage tolerance
real sampled;
```
```
assign r_out = sampled;
always @( absdelta (V(e_in), vdelta, ttol, vtol))
sampled = V(e_in);
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
132
```
```
endmodule
```
#### 5.10.4 Named events

An identifier declared as an event data type is called a _named event_. A named event is triggered explicitly
and is used in an event expression to control the execution of procedural statements in the same manner as
event control described in 5.10. Named events can be triggered from always and initial blocks, or from an
analog event statement. This allows control over the enabling of multiple actions in other procedures.

An event name shall be declared explicitly before it is used. Syntax 5-20 gives the syntax for declaring
events.

event_declaration ::= **event** list_of_event_identifiers **;** _// from A.2.1.3_

list_of_event_identifiers ::= event_identifier { dimension } { **,** event_identifier { dimension } _// from A.2.3_

dimension ::= **[** dimension_constant_expression **:** dimension_constant_expression **]** _// from A.2.5_

### Syntax 5-20—Syntax for event declaration

A declared event is made to occur by the activation of an event triggering statement with the syntax given in
Syntax 5-21. An event is not made to occur by changing the index of an event array in an event control
expression.

event_trigger ::= _// from A.6.5_
**->** hierarchical_event_identifier { **[** expression **]** } **;**

```
Syntax 5-21—Syntax for event trigger
```
An event-controlled statement (for example, @trig rega = regb;) shall cause simulation of its con-
taining procedure to wait until some other procedure executes the appropriate event-triggering statement (for
example, -> trig).

Named events and event control give a powerful and efficient means of describing the communication
between, and synchronization of, two or more concurrently active processes. A basic example of this is a
small waveform clock generator that synchronizes control of a synchronous circuit by signaling the occur-
rence of an explicit event periodically while the circuit waits for the event to occur.

The following example show how an event ( _ana_event_ ) can be generated from an analog event statement,
not only to trigger an event-controlled statement in the **analog** block, but also in an **always** statement.

```
event ana_event;
event dig_event;
```
```
initial #10 -> dig_event;
always @(ana_event) $display(“Event: ana_event detected in digital”);
analog begin
@( timer (1n)) -> ana_event;
@(ana_event) $display(“Event: ana_event detected in analog”);
@(dig_event) $display(“Event: dig_event detected in analog”);
end
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
133
```
#### 5.10.5 Digital events in analog behavior....................................................................................

To model mixed signal functionality, analog behavior can be made sensitive to digital events, including
**posedge** events, **negedge** events, state change events, and named events. In the example above it shows
how a digital event ( **_dig_event_** ) can be detected within the _analog_ context.

### 5.11 Jump statements

jump_statement ::= _// from A.6.5_
**return** [ expression ] **;**
| **break ;**
| **continue ;**

```
Syntax 5-22—Syntax for jump statements
```
Verilog-AMS HDL provides C-like jump statements **break** , **continue** , and **return**.

```
break // break out of loop as in C
continue // skip to end of loop, as in C
return expression // exit from analog user-defined function
```
The **continue** and **break** statements can only be used in a loop. The **continue** statement jumps to the
end of the loop and executes the loop control if present. The **break** statement jumps out of the loop.

The **continue** and **break** statements cannot be used inside an analog for loop. Refer 5.9.3

The **return** statement can only be used from within an analog user-defined function.

In a function returning a value, the **return** statement shall have an expression of the correct type.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
134
```
