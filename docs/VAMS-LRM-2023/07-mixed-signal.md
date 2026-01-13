## 7. Mixed signal...........................................................................................................................................

### 7.1 Overview......................................................................................................................................

With the mixed use of digital and analog simulators, a common terminology is needed. This clause provides
the core terminology used in this LRM and highlights the behavior of the mixed-signal capabilities of Ver-
ilog-AMS HDL.

Verilog-AMS HDL provides the ability to accurately model analog, digital, and mixed-signal blocks.
_Mixed-signal blocks_ provide the ability to access data and be controlled by events from the other domain. In
addition to providing mixed-signal interaction directly through behavioral descriptions, Verilog-AMS HDL
also provides a mechanism for the mixed-signal interaction between modules.

Verilog-AMS HDL is a hierarchical language which enables top-down design of mixed-signal systems.
Connect modules are used in the language to resolve the mixed-signal interaction between modules. These
modules can be manually inserted (by the user) or automatically inserted (by the simulator) based on rules
provided by the user.

_Connect rules_ and the discipline of the mixed signals can be used to control auto-insertion throughout the
hierarchy. Prior to insertion, all net segments of a mixed signal shall first be assigned a discipline. This is
commonly needed for interconnect, which often does not have a discipline declared for it. Once a discipline
has been assigned (usually through use of a discipline resolution algorithm), _connect modules_ shall be
inserted based on the specified connect rules. _Connect rules_ control which connect modules are used and
where are they inserted.

_Connect modules_ are a special form of a mixed-signal module which allow accurate modeling of the inter-
faces between analog and digital blocks. They help ensure the drivers and receivers of a connect module are
correctly handled so the simulation results are not impacted.

This clause also details a feature which allows analog to accurately model the effects the digital receivers for
mixed signals containing both drivers and receivers. In addition, special functions provide access to driver
values so a more accurate connect module can be created.

The following subclauses define these capabilities in more detail.

### 7.2 Fundamentals

The most important feature of Verilog-AMS HDL is that it combines the capabilities of both analog and dig-
ital modeling into a single language. This subclause describes how the continuous (analog) and discrete
(digital) domains interact together, as well as the mixed-signal-specific features of the language.

#### 7.2.1 Domains

The domain of a value refers to characteristics of the computational method used to calculate it. In Verilog-
AMS HDL, a variable is calculated either in the _continuous_ (analog) _domain_ or the _discrete_ (digital) _domain_
every time. The potentials and flows described in natures are calculated in the continuous domain, while reg-
ister contents and the states of gate primitives are calculated in the discrete domain. The values of real and
integer variables can be calculated in either the continuous or discrete domain depending on how their val-
ues are assigned.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
165
```
Values calculated in the discrete domain change value instantaneously and only at integer multiples of a
minimum resolvable time. For this reason, the derivative with respect to time of a digital value is always
zero ( 0 ). Values calculated in the continuous domain, on the other hand, are continuously varying.

#### 7.2.2 Contexts

The domain of a variable is determined based upon the context in which it is assigned. Assignment state-
ments which appear within an **analog** block or **analog initial** block are considered to be in the _con-
tinuous_ (analog) context. Assignment statements within an **initial** or **always** block, or within a _contin-
uous_assign_ statement, are said to be within the _discrete_ (digital) context. Module-level variable declaration
assignments are considered to be context free and do not associate the variable with a particular domain. It
shall be an error to assign to a given variable in both contexts. The domain of an unassigned variable is
undefined and left up to implementations to determine. The implementation may issue a warning if an unas-
signed variable is referenced.

#### 7.2.3 Nets, nodes, ports, and signals

In Verilog-AMS HDL, hierarchical structures are created when higher-level modules create instances of
lower level modules and communicate with them through input, output, and bidirectional ports. A _port_ rep-
resents the physical connection between an expression in the instantiating or parent module and an expres-
sion in the instantiated or child module. The expressions involved are referred to as _nets_ , although they can
include registers, variables, and nets of both continuous and discrete disciplines. A port of an instantiated
module has two nets, the upper connection (vpiHiConn) which is a net in the instantiating module and the
lower connection (vpiLoConn) which is a net in the instantiated module, as shown in Figure 7- 1. The vpi-
LoConn and vpiHiConn connections to a port are frequently referred to as the _formal_ and _actual connec-
tion_ s respectively.

```
Figure 7-1: Signal “out” hierarchy of net segments
```
A net can be declared with either a discrete or analog _discipline_ or no _discipline_ (neutral interconnect).
Within the Verilog-AMS language, only digital blocks and primitives can drive a discrete net ( _drivers_ ), and
only **analog** blocks can contribute to an analog net ( _contributions_ ). A _signal_ is a hierarchical collection of
nets which, because of port connections, are contiguous. If all the nets that make up a signal are in the dis-
crete domain, the signal is a _digital signal_. If all the nets that make up a signal are in the continuous domain,
the signal is an _analog signal_. A signal that consists of nets from both domains is called a _mixed signal_.

```
Module
```
```
Module D
```
```
Module C
```
```
Module B
```
```
Module A
```
```
Module Top
```
```
out
```
```
a_out
```
```
b_out
```
### d_out c_out

### Signal out := Net Top.out

### + Net A.a_out

### + Net B.b_out

### + Net C.c_out

### + Net D.d_out

```
Port
vpiLoConn
```
```
vpiHiConn
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
166
```
Similarly, a port whose connections are both analog is an _analog port_ , a port whose connections are both
digital is a _digital port_ , and a port whose connections are analog and digital is a _mixed port_.

Since it is physically one wire in the design, Kirchhoff’s current law applies to the whole signal, and it forms
one node in analog simulation (see 3.6). Drivers in the digital domain are converted to contributions in the
analog domain using auto-inserted digital-to-analog connection modules (D2As), and the signal value is cal-
culated in the analog domain. Instead of determining the final digital receiver value of the signal by resolv-
ing all the digital drivers, the resolved analog signal is converted back to a digital value. A digital behavioral
block that reads the value of a signal is a _receiver_ , but since Verilog-AMS has no syntax that identifies mul-
tiple receivers within a module as distinct, the associated net can be viewed as a single receiver for the pur-
poses of analog to digital conversion. Drivers are created by declaring a reg, instantiating a digital primitive
or using a continuous assign statement. Since it is only possible to insert connect modules at port boundaries,
when multiple continuous assign statements exist in a module, they are handled by a single connect module.

The drivers and receivers of a mixed signal are associated with their locally-declared net; the discipline of
that net is used to determined which connection modules to use. The discipline of the whole signal is found
by discipline resolution, as described in 7.4, and is used to determine the attributes of the node in simulation.

#### 7.2.4 Mixed-signal and net disciplines.....................................................................................

One job of the discipline of a continuous net is to specify the tolerance ( **abstol** ) for the potential of the
associated node. A mixed signal can have a number of compatible continuous nets, with different continuous
disciplines and different abstols. In this case, the abstol of the associated node shall be the smallest of the
_abstols_ specified in the disciplines associated with all the continuous nets of the signal.

If an undeclared net segment has multiple compatible disciplines connected to it, a connect statement shall
specify which discipline to use during discipline resolution.

### 7.3 Behavioral interaction

Verilog-AMS HDL supports several types of block statements for describing behavior, such as **analog**
blocks, **initial** blocks, and **always** blocks. Typically, non-analog behavior is described in **initial**
and **always** blocks, assignment statements, or **assign** declarations. There can be any number of **ini-
tial** , **always** and **analog** blocks in a particular Verilog-AMS HDL module.

Nets and variables in the continuous domain are termed _continuous nets_ and _continuous variables_ respec-
tively. Likewise nets, regs and variables in the discrete domain are termed _discrete nets_ , _discrete regs_ , and
_discrete variables_. In Verilog-AMS HDL, the nets and variables of one domain can be referenced in the
other’s context. This is the means for passing information between two different domains (continuous and
discrete). Read operations of nets and variables in both domains are allowed from both contexts. Write oper-
ations of nets and variables are only allowed from the context of their domain.

Verilog-AMS HDL provides ways to:

```
— access discrete primaries (e.g., nets, regs, or variables) from a continuous context
— access continuous primaries (e.g., flows, potentials, or variables) from a discrete context
— detect discrete events in a continuous context
— detect continuous events in a discrete context
```
The specific time when an event from one domain is detected in the other domain is subject to the synchro-
nization algorithm described in 7.3.6 and Clause 8. This algorithm also determines when changes in nets and
variables of one domain are accessible in the other domain.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
167
```
#### 7.3.1 Accessing discrete nets and variables from a continuous context

Discrete nets and variables can be accessed from a continuous context. However, because the data types
which are supported in continuous contexts are more restricted than those supported in discrete contexts,
certain discrete types can not be accessed in a continuous context.

Table 7- 1 lists how the various discrete net/variable types can be accessed from a continuous context.

The syntax for a Verilog-AMS HDL primary is defined in Syntax 7- 1.

primary ::= _// from A.8.4_
number
| hierarchical_identifier [ { **[** expression **]** } **[** range_expression **]** ]
| concatenation
| multiple_concatenation
| function_call
| system_function_call
| **(** mintypmax_expression **)**
| string_literal
| branch_probe_function_call
| port_probe_function_call

```
Syntax 7-1—Syntax for primary
```
The following example accesses the discrete primary in from a continuous context.

```
module onebit_dac (in, out);
input in;
inout out;
wire in;
electrical out;
```
```
Table 7-1—Discrete net/variable access from continuous context
```
```
Discrete net/reg/
variable type Examples
```
```
Equivalent
continuous
variable type
```
```
Access to this discrete net/reg/variable type
from a continuous context
```
```
real real r;
real rm[0:8];
```
```
real Discrete reals are accessed in the continuous
context as real numbers.
integer integer i;
integer im[0:4];
```
```
integer Discrete integers are accessed in continuous
context as integer numbers.
bit reg r1;
wire w1;
reg [0:9] r[0:7];
reg r[0:66];
reg [0:34] rb;
```
```
integer Discrete bit and bit groupings (buses and part
selects) are accessed in the continuous context
as integer numbers.
The sign bit (bit 31) of the integer is always set
to zero (0). The lowest bit of the bit grouping
is mapped to the zeroth bit of the integer. The
next bit of the bus is mapped to the first bit of
the integer and so on.
If the bus width is less than 31 bits, the higher
bits of the integer are set to zero (0).
Access of discrete bit groupings with greater
than 31 bits is illegal.
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
168
```
```
real x;
```
```
analog begin
if (in == 0)
x = 0.0;
else
x = 3.0;
V(out) <+ x;
end
endmodule
```
#### 7.3.2 Accessing X and Z bits of a discrete net in a continuous context...................................

Discrete nets can contain bits which are set to x ( _unknown_ ) or z ( _high impedance_ ). Verilog-AMS HDL sup-
ports accessing of 4-state logic values within the analog context. The x and z states must be translated to
equivalent analog real or integer values before being used within the analog context. The language supports
the following specific features, which provide a mechanism to perform this conversion.

```
— the case equality operator (===)
— the case inequality operator (!==)
— the case , casex , and casez statements
— binary, octal and hexadecimal numeric constants which can contain x and z as digits.
```
The case equality and case inequality operators have the same precedence as the equality operator.

Example:

```
module a2d(dnet, anet);
input dnet;
output anet;
wire dnet;
ddiscrete dnet;
electrical anet;
real avar;
```
```
analog begin
if (dnet === 1'b1)
avar = 5;
else if (dnet === 1'bx)
avar = avar; // hold value
else if (dnet === 1'b0)
avar = 0;
else if (dnet === 1'bz)
avar = 2.5; // high impedance - float value
```
```
V(anet) <+ avar;
end
endmodule
```
A **case** statement could also have been used as an alternative to the above _if-else-if_ statement to perform the
4-state logic value comparisons.

Example:

```
case (dnet)
1'b1: avar = 5;
1'bx: avar = avar; // hold value
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
169
```
```
1'b0: avar = 0;
1'bz: avar = 2.5; // high impedance - float value
endcase
```
Accessing digital net and digital binary constant operands are supported within analog context expressions.
It is an error if these operands return x or z bit values when solved. It will be an error if the value of the dig-
ital variable being accessed in the analog context goes either to x or z.

Example:

```
module converter(dnet, anet);
output dnet;
inout anet;
reg dnet;
electrical anet;
integer var1;
real var2;
```
```
initial begin
dnet = 1'b1;
#50 dnet = 1'bz;
$finish ;
end
```
```
analog begin
var1 = 1'bx; // error
var2 = 1'bz; // error
var1 = 1 + dnet; // error after #50
```
```
if (dnet === 1'bx) // error
$display ("Error to access x bit in continuous context");
```
```
V(anet) <+ 1'bz; // error
V(anet) <+ dnet; // error after #50
end
endmodule
```
The syntax for the features that support x and z comparisons in a continuous context is defined in 2.6 and
5.8.3. Support for x and z is limited in the **analog** blocks as defined above.

NOTE—Consult section 5.1.8 in IEEE Std 1364 Verilog for a description of the semantics of these operators.

**7.3.2.1 Special floating point values**

Floating point arithmetic can produce special values representing plus and minus infinity and Not-a-Number
(NaN) to represent a bad value. While use of these special numbers in digital expressions is not an error, it is
illegal to assign these values to a branch through contribution in the analog context.

#### 7.3.3 Accessing continuous nets and variables from a discrete context

All continuous nets can be probed from a discrete context using access functions. All probes which are legal
in a continuous context of a module are also legal in the discrete context of a module.

The following example accesses the continuous net V(in) from the discrete context is.

```
module sampler (in, clk, out);
inout in;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
170
```
```
input clk;
output out;
electrical in;
wire clk;
reg out;
```
```
always @( posedge clk)
out = V(in);
```
```
endmodule
```
Continuous variables can be accessed for reading from any discrete context in the same module where these
variables are declared. Because the discrete domain can fully represent all continuous types, a continuous
variable is fully visible when it is read in a discrete context. If the current time in the continuous and discrete
kernels differ, interpolation is used to determine the value to be used in the discrete context for the continu-
ous variable unless the value of the continuous variable was last assigned in an analog event statement. In
this case, the value used in the digital context is exactly the same as the last value assigned to the continuous
variable.

#### 7.3.4 Detecting discrete events in a continuous context

Discrete events can be detected in a Verilog-AMS HDL continuous context. The arguments to discrete
events in continuous contexts are in the discrete context. A discrete event in a continuous context is non-
blocking like the other event types allowed in continuous contexts. The syntax for events in a continuous
context is shown in Syntax 7- 2.

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

analog_event_functions ::=
**cross (** analog_expression [ **,** analog_expression_or_null
[ **,** analog_expression_or_null [ **,** analog_expression_or_null [ **,** analog_expression ] ] ] ] **)**
| **above (** analog_expression [ **,** analog_expression_or_null
[ **,** analog_expression_or_null [ **,** analog_expression ] ] ] **)**
| **timer (** analog_expression [ **,** analog_expression_or_null
[ **,** analogt_expression_or_null [ **,** analog_expression ] ] ] **)
|absdelta (** analog_expression **,** analog_expression
[ **,** analog_expression_or_null [ **,** analog_expression_or_null [ **,** analog_expression ] ] ] **)**

analog_event_statement ::=
{ attribute_instance } analog_loop_statement
| { attribute_instance } analog_case_statement
| { attribute_instance } analog_conditional_statement
| { attribute_instance } analog_procedural_assignment


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
171
```
```
| { attribute_instance } analog_event_seq_block
| { attribute_instance } analog_system_task_enable
| { attribute_instance } disable_statement
| { attribute_instance } event_trigger
| { attribute_instance } ;
```
```
Syntax 7-2—Syntax for event control statement
```
The following example shows a discrete event being detected in an **analog** block.

```
module sampler3 (in, clk1, clk2, out);
input in, clk1, clk2;
output out;
wire clk1;
electrical in, clk2, out;
real vout;
```
```
analog begin
@( posedge clk1 or cross (V(clk2), 1))
vout = V(in);
V(out) <+ vout;
end
endmodule
```
#### 7.3.5 Detecting continuous events in a discrete context

In Verilog-AMS HDL, monitored continuous events can be detected in a discrete context. The arguments to
these events are in the continuous context. A continuous event in a discrete context is blocking like other
discrete events. The syntax for analog events in a discrete context is shown in Syntax 7- 3.

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
Syntax 7-3—Syntax for analog event detection in digital context
```
The following example detects a continuous event in an always block.

```
module sampler2 (in, clk, out);
input in, clk;
output out;
wire in;
reg out;
electrical clk;
```
```
always @( cross (V(clk) - 2.5, 1))
out = in;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
172
```
```
endmodule
```
#### 7.3.6 Concurrency

Verilog-AMS HDL provides synchronization between the continuous and discrete domains. Simulation in
the discrete domain proceeds in integer multiples of the digital tick. This is the smallest value of the second
argument of the **`timescale** directive (see 19.8 in IEEE Std 1364 Verilog).

Simulation in the continuous domain appears to proceed continuously. Thus, there is no time granularity
below which continuous values can be guaranteed to be constant.

The rest of this subclause describes synchronization semantics for each of the four types of mixed-signal
behavioral interaction. Any synchronization method can be employed, provided the semantics preserved. A
typical synchronization algorithm is described in 8.2.

**7.3.6.1 Analog event appearing in a digital event control**

In this case, an analog event, such as **cross** or **timer** , appears in an @() statement in the digital context.

Example:

```
always begin
@( cross (V(x) - 5.5,1))
n = 1;
end
```
Besides using analog event functions, one can also use analog variables that are only assigned values in ana-
log event statements in a digital event control statement. An event occurs whenever a value is assigned to the
variable, regardless of whether the variable changes value or not. This might be done when one wants to
sample a value in the continuous time domain to avoid jitter being created by the discrete nature of time in
the digital context, but wish to process the sample in the digital context.

Example:

```
analog @(timer(0,100n))
smpl = V(in);
```
```
always @(smpl) begin
...
```
When it is determined the event has occurred in the analog domain, the statements under the event control
shall be scheduled in the digital domain at the nearest digital time tick to the time of the analog event. This
event shall not be schedule in the digital domain earlier than the last or current digital event (see 8.3.3), how-
ever it may appear to be in a delta cycle belonging to a tick started at an earlier or later time.

Zero-delay scheduling is not rounded, so in the case where the digital event causes another event on the dig-
ital to analog boundary with zero delay, it will be handled at the current analog time.

**7.3.6.2 Digital event appearing in an analog event control**

Example:

```
analog begin
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
173
```
```
@( posedge n)
r = 3.14;
end
```
In this case, a digital event, such as **posedge** or **negedge** , appears in an @() statement in the analog con-
text.

When it is determined the event has occurred in the digital domain, the statements under the event control
shall be executed in the analog domain at the time corresponding to a real promotion of the digital time (e.g.,
27ns to 27.0e-9).

**7.3.6.3 Analog primary appearing in a digital expression**

In this case, an analog primary (variable, potential, or flow) whose value is calculated in the continuous
domain appears in a expression which is in the digital context; thus the analog primary is evaluated in the
digital domain.

The expression shall be evaluated using the analog value calculated for the time corresponding to a real pro-
motion of the digital time at which the expression is evaluated.

If the current time in the continuous and discrete kernels differ, interpolation is used to determine the value
to be used in the discrete context for the continuous variable unless the value of the continuous variable was
last assigned in an analog event statement. In this case, the value used in the digital context is exactly the
same as the last value assigned to the continuous variable.

**7.3.6.4 Analog variables appearing in continuous assigns**

Analog variables that are only assigned values within analog event statements can be used in the expressions
that drive continuous assigns, both when the target of the continuous assign is a **wreal** or a traditional Ver-
ilog wire type ( **wire** , **trireg** , **wor** , **wand** , etc.).

**7.3.6.5 Digital primary appearing in an analog expression**

In this case, a digital primary ( **reg** , **wire** , **integer** , etc.) whose value is calculated in the discrete domain
appears in an expression which is in the analog context; thus the analog primary is evaluated in the continu-
ous domain.

The expression shall be evaluated using the digital value calculated for the greatest digital time tick which is
less than or equal to the analog time when the expression is evaluated.

#### 7.3.7 Function calls

Digital functions cannot be called from within the analog context. Analog functions cannot be called from
within the digital context.

### 7.4 Discipline resolution

In general a mixed signal is a contiguous collection of nets, some with discrete discipline(s) and some with
continuous discipline(s). A continuous signal is a contiguous collection of nets where all the nets are in the
continuous domain (see 1.3). Additionally, some of the nets can have undeclared discipline(s). Discipline
resolution assigns disciplines and domains to those nets whose discipline is undeclared. This is done to (1)
control auto-insertion of connect modules, according to the rules embodied in _connect statements_ and (2) to
ensure that the nets of all mixed signals and continuous signals have a known discipline and domain.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
174
```
The assignments are based on: discipline declarations, **`default_discipline** directives (see 3.8), and
the hierarchical connectivity of the design. Once all net segments of every mixed signal has been resolved,
insertion of connect modules shall be performed.

#### 7.4.1 Compatible discipline resolution

One factor which influences the resolved discipline of a net whose discipline is undeclared is the disciplines
of nets to which it is connected via ports; i.e., if multiple compatible disciplines are connected to the same
net via multiple ports only one discipline can be assigned to that net. This is controlled by the **resolveto**
form of the connect statement; the syntax of this form is described in 7.7.2.

If disciplines at the lower connections of ports (where the undeclared net is an upper connection) are among
the disciplines in discipline_list, the result_discipline is the discipline which is assigned to the undeclared
net. If all the nets are of the same discipline, no rule is needed; that discipline becomes the resolved disci-
pline of the net.

In the example shown in Figure 7- 2 , NetA and NetB are undeclared interconnects. NetB has cmos3 and
cmos4 at the lower connection ports, while it is an upper connection.

```
Figure 7-2: Compatible discipline resolution
```
The first connect statement resolves NetB to be assigned the discipline cmos3.

NetA has cmos1, cmos2 and the resulting cmos3 from module twoblks at the lower connection ports;
based on the second connect statement, it resolves to be assigned the discipline cmos1.

#### 7.4.2 Connection of discrete-time disciplines

Ports of discrete-time disciplines (ports where digital signals appear at both upper (vpiHiConn) and lower
(vpiLoConn) connections) shall obey the rules imposed by IEEE Std 1364 Verilog on such connections.

In addition, the real-value nets shall obey the rules imposed by 3.7.

```
connect cmos3 cmos4 resolveto cmos3;
connect cmos1 cmos2 cmos3 resolveto cmos1;
```
```
cmos1
```
```
cmos2
```
```
cmos3
```
```
cmos4
```
```
NetA
```
```
NetB
```
```
module blk (out);
```
```
module blk (out);
```
```
module blk (out);
```
```
module blk (out);
```
```
module digital_blk (out); module twoblks (out);
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
175
```
#### 7.4.3 Connection of continuous-time disciplines

Ports of continuous-time disciplines (ports where analog signals appear at both upper (vpiHiConn) and
lower (vpiLoConn) connections) shall obey the rules imposed in 3.11. It shall be an error to connect incom-
patible continuous disciplines together.

#### 7.4.4 Resolution of mixed signals

Once discipline declarations have been applied, if any mixed-signal or continuous-signal nets don’t have a
discipline and domain assigned additional resolution is needed. This section provides an additional method
for discipline resolution of remaining undeclared nets.

There are two modes for this method of resolution, _basic_ (the default) and _detail_ , which determine how
known disciplines are used to resolve these undeclared nets. For the entire design, undeclared nets shall be
resolved at each level of the hierarchy where continuous (analog) has precedence over discrete (digital). The
selection of these discipline resolution modes shall be vendor-specific.

More than one conflicting discipline declaration from the same context (in or out of context) for the same
hierarchical segment of a signal is an error. In this case, _conflicting_ simply means an attempt to declare more
than one discipline regardless of whether the disciplines are compatible or not.

Sample algorithms for the complete discipline resolution process are listed in Annex F.

**7.4.4.1 Basic discipline resolution algorithm**

In this mode (the default), both continuous and discrete disciplines propagate up the hierarchy to meet one
another. At each level of the hierarchy where continuous and discrete meet for an undeclared net that net
segment is declared continuous. This typically results in connect modules being inserted higher up the
design hierarchy.

In the example shown in Figure 7- 3 , NetA, NetB, NetC, and NetD are undeclared interconnects.

```
Figure 7-3: Discipline resolution mode: basic
```
```
cmos1
```
```
cmos2
```
```
cmos3
```
```
cmos4
```
```
NetA
```
```
NetB
```
```
connect cmos3 cmos4 resolveto cmos3;
connect cmos1 cmos2 cmos3 resolveto cmos1;
```
```
module blk2 (out);
```
```
module blk1 (out);
```
```
module blk3 (out);
```
```
module blk4 (out);
```
```
module digital_blk (out); module twoblks (out);
```
```
cmos2
```
```
electrical
```
```
NetC
```
```
module ablk (out);
```
```
module blk2 (out);
```
```
module mix (out);
```
```
NetD
module top;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
176
```
Using the basic mode of discipline resolution and the specified **resolveto** connect statements for this
example results in the following:

```
— NetB resolves to cmos3 based on the first resolveto connect statement.
— NetA resolves to cmos1 based on the second resolveto connect statement.
— NetC resolves to electrical based on continuous (electrical) winning over discrete (cmos2).
— NetD resolves to electrical based on continuous (electrical) winning over discrete (cmos1).
```
**7.4.4.2 Detail discipline resolution algorithm**

In this mode continuous disciplines propagate up and then back down to meet discrete disciplines. Discrete
disciplines do not propagate up the hierarchy. This can result in more connect modules being inserted lower
down into discrete sections of the design hierarchy for added accuracy.

In the example shown in Figure 7- 4 , NetA, NetB, NetC, and NetD are undeclared interconnects.

```
Figure 7-4: Discipline resolution mode: detail
```
Using the detail mode of discipline resolution for this example results in the following:

```
— Continuous up : NetC resolves to electrical based on continuous (electrical) winning over discrete
(cmos2).
— Continuous up : NetD resolves to electrical based on continuous (electrical) winning over unde-
clared.
— Continuous down : NetA resolves to electrical based on continuous (electrical) winning over unde-
clared.
— Continuous down : NetB resolves to electrical based on continuous (electrical) winning over unde-
clared.
```
The specified **resolveto** connect statements are ignored in this mode unless coercion (see 7.8.1) is used.

```
cmos1
```
```
cmos2
```
```
cmos3
```
```
cmos4
```
```
NetA
```
```
NetB
```
```
connect cmos3 cmos4 resolveto cmos3; // discrete resolveto’s ignored
connect cmos1 cmos2 cmos3 resolveto cmos1; // discrete resolveto’s ignored
```
```
module blk2 (out);
```
```
module blk1 (out);
```
```
module blk3 (out);
```
```
module blk4 (out);
```
```
module digital_blk (out); module twoblks (out);
```
```
cmos2
```
```
electrical
```
```
NetC
```
```
module ablk (out);
```
```
module blk2 (out);
```
```
module mix (out);
```
### module top; NetD


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
177
```
**7.4.4.3 Coercing discipline resolution**

Connect module insertion can be affected by _coercion_ i.e., declaring disciplines for the interconnect in the
hierarchy. If an interconnect is assigned a discipline, that discipline shall be used unless the **resolveto**
connect statement overrides the discipline.

The example in Figure 7- 5 shows several effects of coercion on auto-insertion.

```
Figure 7-5: Coercion effects on auto- insertion
```
Case1: NetB is declared as cmos3 (the others are undeclared)

```
cmos3 top.digital_blk.twoblks.NetB
discipline resolution basic : Same as without coercion.
discipline resolution detail: NetB stays cmos3; NetA, NetC, and NetD become electrical.
```
Case2: NetA is declared as cmos1 (the others are undeclared)

```
discipline resolution basic : NetA stays cmos1, NetB is assigned cmos3, and NetC and NetD
become electrical.
discipline resolution detail : Same as basic mode.
```
Case3: NetC is declared as cmos2 (the others are undeclared)

```
discipline resolution basic : NetC stays cmos2, NetB is assigned cmos3, NetA is assigned cmos1,
and NetD is assigned cmos1.
discipline resolution detail : Same as basic mode.
```
#### 7.4.5 Discipline resolution of continuous signals

The discipline of nets, without a declared discipline, of a continuous signal shall also be determined by using
the discipline resolution algorithms listed in Annex F. Both algorithms will give the same result as there are
no discrete disciplines to propagate upwards.

```
cmos1
```
```
cmos2
```
```
cmos3
```
```
cmos4
```
```
NetA
```
```
NetB
```
```
connect cmos3 cmos4 resolveto cmos3; // discrete resolveto’s ignored
connect cmos1 cmos2 cmos3 resolveto cmos1; // discrete resolveto’s ignored
```
```
module blk2 (out);
```
```
module blk1 (out);
```
```
module blk3 (out);
```
```
module blk4 (out);
```
```
module digital_blk (out); module twoblks (out);
```
```
cmos2
```
```
electrical
```
```
NetC
```
```
module ablk (out);
```
```
module blk2 (out);
```
```
module mix (out);
```
```
NetD
```
### module top;


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
178
```
### 7.5 Connect modules..........................................................................................................................

Connect modules are automatically inserted to connect the continuous and discrete disciplines (mixed nets)
of the design hierarchy together. The continuous and discrete disciplines of the ports of the connect modules
and their directions are used to determine the circumstances in which the module can be automatically
inserted.

The connect module is a special form of a module; its definition is shown in Syntax 7- 4.

module_declaration ::= _// from A.1.2_
{ attribute_instance } module_keyword module_identifier [ module_parameter_port_list ]
list_of_ports **;** { module_item }
**endmodule**
| { attribute_instance } module_keyword module_identifier [ module_parameter_port_list ]
[ list_of_port_declarations ] **;** { non_port_module_item }
**endmodule**

module_keyword ::= **module** | **macromodule** | **connectmodule**

```
Syntax 7-4—Syntax for connect modules
```
**7.6 Connect module descriptions**

The disciplines of mixed nets are determined prior to the connect module insertion phase of elaboration.
Connect module declarations with matching port discipline declarations and directions are instantiated to
connect the continuous and discrete domains of the mixed net.

The port disciplines define the default type of disciplines which shall be bridged by the connect module. The
directional qualifiers of the discrete port determine the default scenarios where the module can be instanti-
ated. The following combinations of directional qualifiers are supported for the continuous and discrete dis-
ciplines of a connect module:

Example 1:

```
connectmodule d2a (in, out);
input in;
output out;
ddiscrete in;
electrical out;
// insert connect module behavioral here
endmodule
can bridge a mixed input port whose upper connection is compatible with discipline ddiscrete and
whose lower connection is compatible with electrical, or a mixed output port whose upper con-
```
```
Table 7-2—Connect module directional qualifier combinations
```
```
continuous discrete
```
```
input output
output input
inout inout
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
179
```
```
nection is compatible with discipline electrical and whose lower connection is compatible with
ddiscrete.
```
Example 2:

```
connectmodule a2d (out, in);
output out;
input in;
ddiscrete out;
electrical in;
// insert connect module behavioral here
endmodule
can bridge a mixed output port whose upper connection is compatible with discipline ddiscrete
and whose lower connection is compatible with electrical, or a mixed input port whose upper
connection is compatible with discipline electrical and whose lower connection is compatible
with ddiscrete.
```
Example 3:

```
connectmodule bidir (out, in);
inout out;
inout in;
ddiscrete out;
electrical in;
// insert connect module behavioral here
endmodule
can bridge any mixed port whose one connection is compatible with discipline ddiscrete and
whose connection is compatible with electrical.
```
### 7.7 Connect specification statements

Any number of connect modules can be defined. The designer can choose and specialize those in the design
via the connect specification statements. The connect specification statements allow the designer to define:

```
— specification of which connect module is used, including parameterization, for bridging given dis-
crete and continuous disciplines
— overrides for the connect module default disciplines and port directions
— resolution of incompatible disciplines
```
The syntax for connect specifications is shown in Syntax 7- 5.

connectrules_declaration ::= _// from A.1.8_
**connectrules** connectrules_identifier **;**
{ connectrules_item }
**endconnectrules**

connectrules_item ::=
connect_insertion
| connect_resolution

```
Syntax 7-5—Syntax for connect specification statements
```
The two forms of the connect specification statements and their syntaxes are detailed in the following sub-
clauses.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
180
```
#### 7.7.1 Connect module auto-insertion statement

The connect module insertion statement declares which connect modules are automatically inserted when
mixed nets of the appropriate types are encountered, as shown in Syntax 7- 6.

This specifies the connect module _connect_module_identifier_ is used to determine the mixed nets of the type
used in the declaration of the connect module.

There can be multiple connect module declarations of a given ( **discrete** — **continuous** ) discipline
pair and the connect module specification statement specifies which is to be used in the auto-insertion pro-
cess. In addition, parameters of the connect module declaration can be specified via the _connect_attributes_.

connect_insertion ::= **connect** connectmodule_identifier [ connect_mode ] _// from A.1.8_
[ parameter_value_assignment ] [ connect_port_overrides ] **;**

connect_mode ::= **merged** | **split**

connect_port_overrides ::=
discipline_identifier **,** discipline_identifier
| **input** discipline_identifier **, output** discipline_identifier
| **output** discipline_identifier **, input** discipline_identifier
| **inout** discipline_identifier **, inout** discipline_identifier

```
Syntax 7-6—Syntax for connect configuration statements
```
Connect modules can be reused for different, but compatible disciplines by specifying different discipline
combinations in which the connect module can be used. The form is

```
connect connect_module _identifier connect_attributes discipline _identifier , discipline _identifier ;
```
where the specified disciplines shall be compatible for both the continuous and discrete disciplines of the
given connect module.

It is also possible to override the port directions of the connect module, which allows a module to be used
both as a unidirectional and bidirectional connect module. This override also aids library based designs by
allowing the user to specify the connect rules, rather than having to search the entire library. The form is

```
connect connect_module _identifier connect_attributes direction discipline _identifier ,
direction discipline _identifier ;
```
where the specified disciplines shall be compatible for both the continuous and discrete disciplines of the
given connect module and the specified directions are used to define the type of connect module.

#### 7.7.2 Discipline resolution connect statement

The discipline resolution connect statement specifies a single discipline to use during the discipline resolu-
tion process when multiple nets with compatible disciplines are part of the same mixed net, as shown in
Syntax 7- 7.

connect_resolution ::= **connect** discipline_identifier { **,** discipline_identifier } **resolveto** _// from A.1.8_
discipline_identifier_or_exclude **;**

discipline_identifier_or_exclude ::=
discipline_identifier
| **exclude**


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
181
```
```
Syntax 7-7—Syntax for connect configuration resolveto statements
```
where the discipline identifiers before the _resolveto_ keyword are the list of compatible disciplines and the
discipline identifier after is the discpline to be used. If the keyword _exclude_ follows _resolveto_ rather than a
discipline identifier, then the otherwise compatible disciplines are deemed to be incompatible and an error is
indicated if they are found on the same net.

Example:
**connect** logic18 logic32 **resolveto exclude** ;
**connect** electrical18 electrical32 **resolveto exclude** ;

In the first case, two discrete disciplines, and the second case two continuous disciplines, are declared to be
incompatible. In both cases, the discipline ending in 18 is associated with 1.8V logic and the discipline end-
ing in 32 is associated with 3.2V logic. These connect statements prevent ports associated with one supply
voltage to be connected to nets associated with the other.

**7.7.2.1 Connect rule resolution mechanism**

When there is an exact match for the set of disciplines specified as part of the discipline_list, the resolved
discipline would be as per the rule specified in the exact match. When more than one specified rule applies
to a given scenario a warning message shall be issued by the simulator and the first match would be used.

When there is no exact fit, then the resolved discipline would be based on the subset of the rules specified. If
there is more than one subset matching a set of disciplines, the simulator shall give a warning message and
apply the first subset rule that satisfies the current scenario.

The resolved discipline need not be one of the disciplines specified in the discipline list.

The **connect** ... **resolveto** shall not be used as a mechanism to set the disciplines of simulator primitives
but used only for discipline resolution.

Example 1:

```
connect x,y,a resolveto a;
connect x,y resolveto x;
```
For the above set of connect rule specifications:

```
— disciplines x,y would resolve to discipline x.
— disciplines x,y,a would resolve to discipline a.
— disciplines y,a would resolve to discipline a.
```
Example 2:

```
connect x,y,a resolveto y;
connect x,y,a resolveto a;
connect x,y,b resolveto b;
```
For the above set of connect rule specifications:

```
— disciplines x,y would resolve to discipline y with a warning.
— disciplines x,y,a would resolve to discipline y with a warning.
— disciplines y,b would resolve to b.
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
182
```
#### 7.7.3 Parameter passing attribute

An attribute method can be used with the connect statement to specify parameter values to pass into the Ver-
ilog-AMS HDL connect module and override the default values. Any parameters declared in the connect
module can be specified.

Example:

```
connect a2d_035u #(.tt(3.5n), .vcc(3.3));
```
Here each parameter is listed with the new value to be used for that parameter.

#### 7.7.4 connect_mode

This can be used to specify additional segregation of connect modules at each level of the hierarchy. Setting
_connect_mode_ to **split** or **merged** defines whether all ports of a common discrete discipline and port
direction share an connect module or have individual connect modules.

Example:

```
connect a2d_035u split #(.tt(3.5n), .vcc(3.3));
```
Here each digital port has a separate connect module.

### 7.8 Automatic insertion of connect modules

Automatic insertion of connect modules is performed when signals and ports with continuous time domain
and discrete time domain disciplines are connected together. The connect module defines the conversion
between these different disciplines.

An instance of the connect module shall be inserted across any mixed port that matches the rule specified by
a **connect** statement. Rules for matching connect statements with ports take into account the port direction
(see 7.8.1) and the disciplines of the signals connected to the port.

Each **connect** statement designates a module to be a connect module. When two disciplines are specified
in a connect statement, one shall be discrete and the other continuous.

Example:

```
module dig_inv(in, out);
input in;
output out;
reg out;
ddiscrete in, out;
always begin
out = #10 ~in;
end
endmodule
```
```
module analog_inv(in, out);
input in;
output out;
electrical in, out;
parameter real vth = 2.5;
real outval;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
183
```
```
analog begin
if (V(in) > vth)
outval = 0;
else
outval = 5 ;
V(out) <+ transition(outval);
end
endmodule
```
```
module ring;
dig_inv d1 (n1, n2);
dig_inv d2 (n2, n3);
analog_inv a3 (n3, n1);
endmodule
```
```
connectmodule elect_to_logic(el,cm);
input el;
output cm;
reg cm;
electrical el;
ddiscrete cm;
always
@( cross (V(el) - 2.5, 1))
cm = 1;
always
@( cross (V(el) - 2.5, -1))
cm = 0;
endmodule
```
```
connectmodule logic_to_elect(cm,el);
input cm;
output el;
ddiscrete cm;
electrical el;
analog
V(el) <+ transition ((cm == 1)? 5.0 : 0.0);
endmodule
```
```
connectrules mixedsignal;
connect elect_to_logic;
connect logic_to_elect;
endconnectrules
```
Here two modules, elect_to_logic and logic_to_elect, are specified as the connect modules to be
automatically inserted whenever a signal and a module port of disciplines electrical and ddiscrete are
connected.

Module elect_to_logic converts signals on port out of instance a3 to port in of instance d1. Module
logic_to_elect converts the signal on port out of instance d2 to port in of instance a3.

#### 7.8.1 Connect module selection

The selection of a connect module for automatic insertion depends upon the disciplines of nets connected
together at ports. It is, therefore, a post elaboration operation since the signal connected to a port is only
known when the module in which the port is declared has been instantiated.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
184
```
Auto-insertion of connect modules is done hierarchically. The connect modules are inserted based on the net
disciplines and ports at each level of the hierarchy. The _connect_mode_ **split** and **merged** are applied at
each level of the hierarchy. This insertion supports the ability to coerce the placement of connect modules by
declaring the disciplines of interconnect.

Figure 7- 6 shows an example of auto-insertion with coercion.

```
Figure 7-6: Auto-insertion with coercion
```
Case1: All interconnects are undeclared

```
— discipline resolution basic :
— merged : d2a at top.mix.blk2 and d2a at top.digital_blk (two connect modules).
— split : Same as merged.
— discipline resolution detail :
— merged : d2a at top.mix.blk2, d2a at top.digital_blk.(blk1-blk2), and d2a at
top.digital_blk.twoblks (three connect modules).
— split : d2a at each of the five cmos1 blocks.
```
Case2: If NetB is declared as cmos1 and the remaining interconnect is undeclared

```
— discipline resolution basic :
— merged : d2a at top.mix.blk2 and d2a at top.digital_blk (two connect modules).
— split : Same as merged.
— discipline resolution detail :
— merged : d2a at top.mix.blk2, d2a at top.digital_blk.(blk1-blk2), and d2a at
top.digital_blk.twoblks (three connect modules).
— split : d2a at top.mix.blk2, d2a at top.digital_blk.blk1, d2a at top.digi-
tal_blk.blk2, and d2a at top.digital_blk.twoblks (four connect modules).
```
```
cmos1
```
```
cmos1
```
```
cmos1
```
```
cmos1
```
```
NetA
```
```
NetB
```
```
// All digital modules have only output ports of discipline cmos1
```
```
module blk2 (out);
```
```
module blk1 (out);
```
```
module blk3 (out);
```
```
module blk4 (out);
```
```
module digital_blk (out); module twoblks (out);
```
```
cmos1
```
```
electrical
```
```
NetC
```
```
module ablk (out);
```
```
module blk2 (out);
```
```
module mix (out);
```
```
NetD
module top;
```
```
connect cmos_d2a input cmos1 output electrical;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
185
```
#### 7.8.2 Signal segmentation

Once a connect module has been selected it can not be inserted until it can be determined whether there
should be one connect module per port or one connect module for all the ports on the net of a signal which
match a given **connect** statement. Inserting multiple copies of the same connect module on one signal
(i.e., between the signal and the multiple ports) has the effect of creating distinct segments of the signal with
the same discipline at that level of the hierarchy.

This segmentation of the signal which connects ports is only performed in the case of digital ports (i.e., ports
with discrete-time domain or digital discipline). For analog (or continuous-time domain) disciplines, it is not
desirable to segment the signal between the ports; i.e, there shall never be more than one analog node repre-
senting a signal. However, it can be desirable for the simulator’s internal representation of the signal to con-
sist of various separate digital segments, each with its own connect module.

Figure 7- 7 shows how to model the loading effect of each individual digital port on the analog node.


Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL

```
186
```
```
Figure 7-7: Signal segmentation by connect modules
```
```
Analog
```
##### LOGIC

```
Analog
```
##### LOGIC

##### LOGIC

```
Analog
```
##### LOGIC

```
Analog
```
##### LOGIC

##### LOGIC

```
Analog
```
##### LOGIC

```
Analog
```
##### LOGIC

##### LOGIC

```
one LOGIC segment for all LOGIC ports
```
```
two LOGIC segments
(one for inputs, one for outputs)
```
```
connect instance
```
```
a separate LOGIC segment for each LOGIC port
```
```
in
```
```
out
```
```
out
```
```
in
```
```
in
```
### Insertion of connect instances creates distinct

### segments in a signal


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
187
```
#### 7.8.3 connect_mode parameter

This parameter can be used in the connect statement to direct the segmentation of the signal at each level
of the hierarchy, which can occur while inserting a connect module. It can be one of two predefined values,
**split** or **merged.** The default is **merged**.

The _connect_mode_ indicates how input, output, or inout ports of the given discipline shall be combined for
the purpose of inserting connect modules. It is applied when there is more than one port of discrete disci-
pline on a net of a signal where the **connect** statement applies.

**7.8.3.1 merged**

This instructs the simulator to try to group all ports (whether they are input, output, or inout) and to use just
one connector module, provided the module is the same.

Figure 7- 9 illustrates the effect of the **merged** attribute.

Connection of the electrical signal to the ttl inout ports and ttl input ports results in a single connec-
tor module, bidir, being inserted between the ports and the electrical signal. The ttl output ports are
merged, but with a different connect module; i.e., there is one connector module inserted between the elec-
trical signal and all of the ttl output ports.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
188
```
```
Figure 7-8:
```
```
Figure 7-9: Connector insertion using merged
```
**7.8.3.2 split**

If more than one input port is connected at a net of a signal, using **split** forces there to be one connect
module for each port which converts between the net discipline and the port discipline. In this way, the net
connecting to the ports is segmented by the insertion of one connect module for each port.

Example 1:

```
connect elect_to_logic split ;
```
This **connect** statement specifies the module elect_to_logic shall be split across the discrete module
ports:

```
— if an input port has ddiscrete discipline and the signal connecting to the port has electrical
discipline, or
```
##### TTL

```
inputs
```
```
outputs
```
```
inouts
```
##### TTL

```
inputs
```
```
outputs
```
```
inouts
```
```
electrical
```
```
connect d2a merged input ttl, output electrical ;
connect bidir merged output electrical, input ttl ;
connect bidir merged inout ttl, inout electrical ;
```
```
bidir
```
```
d2a
```
```
endconnectrules
```
```
connectrules example;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
189
```
```
— if an output port has electrical discipline and the signal connecting to the port has ddiscrete
discipline.
```
Example 2:

In Figure 7- 10 , the connections of an electrical signal to ttl output ports results in a distinct instance of
the d2a connect module being inserted for each output port. This is mandated by the **split** parameter.

Connection of the electrical signal to ttl input ports results in a single instance of the a2d connect
module being inserted between the electrical signal and all the ttl input ports. This is mandated by
merged parameter. This behavior is also seen for the ttl inout ports where the **merged** parameter is used.

```
Figure 7-10: Connect module insertion with signal segmentation
```
##### TTL

```
inputs
```
```
outputs
```
```
inouts
```
##### TTL

```
inputs
```
```
outputs
```
```
inouts
```
```
electrical
```
```
connect d2a split input ttl, output electrical;
connect a2d merged output electrical, input ttl ;
connect bidir merged inout electrical, inout ttl ;
```
### a2d

### d2a

### d2a

### d2a

### d2a

### bidir

```
connectrules example;
```
```
endconnectrules
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
190
```
Example 3

```
connect cmosA2d split #(.r(30k) input electrical, output cmos02u;
```
performs three functions:

```
1) Connects an instance of cmosA2d module between a signal with electrical discipline and the
input port with cmos02u discipline, or an output port with electrical discipline and the signal
with cmos02u discipline;
2) Sets the value of the parameter r to 30k; and
3) Uses one module instance for each input port.
```
If there are many output ports where this rule applies, by definition there is no segmentation of the signal
between these ports, since the ports have discipline electrical (an analog discipline).

Example 4

```
connect cmosA2d merged #(.r(15k) input electrical, output cmos04u;
```
does three things:

```
1) Connects an instance of cmosA2d module between a signal with electrical discipline and an
input port with cmos04u discipline, or an output port with electrical discipline and a signal with
cmos4u discipline;
2) Sets the value of the parameter r to 15k; and
3) Uses one module instance regardless of the number of ports.
```
#### 7.8.4 Rules for driver-receiver segregation and connect module selection and insertion........

Driver-receiver segregation and connect module insertion is a post elaboration operation. It depends on a
complete hierarchical examination of each signal in the design, i.e., an examination of the signal in all the
contexts through which it passes. If the complete hierarchy of a signal is digital, i.e., the signal has a digital
discipline in all contexts through which is passes, it is a _digital signal_ rather than a mixed signal. Similarly,
if the complete hierarchy of a signal is analog, it is an _analog signal_ rather than a mixed signal. Rules for
driver-receiver segregation and connect module insertion apply only to _mixed signals_ , i.e., signals which
have an analog discipline in one or more of the contexts through which they pass and a digital discipline in
one or more of the contexts. In this case, _context_ refers to the appearance of a signal in a particular module
instance.

For a particular signal, a module instance has a digital context if the signal has a digital discipline in that
module or an analog context if the signal has an analog discipline. The appearance of a signal in a particular
context is referred to as a _segment_ of the signal. In general, a _signal_ in a fully elaborated design consists of
various segments, some of which can be analog and some of which can be digital.

A _port_ represents a connection between two net segments of a signal. The context of one of the net segments
is an instantiated module and the context of the other is the module which instantiates it. The segment in the
instantiated module is called the _lower_ or _formal connection_ and the segment in the instantiating module is
the _upper_ or _actual connection_. A connection element is selected for each port where one connection is ana-
log and the other digital.

The following rules govern driver-receiver segregation and connect module selection. These rules apply
only to mixed signals.

```
1) A mixed signal is represented in the analog domain by a single node, regardless of how its analog
contexts are distributed hierarchically.
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
191
```
```
2) Digital drivers of mixed signals are segregated from receivers so the digital drivers contribute to the
analog state of the signal and the analog state determines the value seen by the receivers.
3) A connection shall be selected for a port only if one of the connections to the port is digital and the
other is analog. In this case, the port shall match one (and only one) connect statement. The module
named in the connect statement is the one which shall be selected for the port.
```
Once connect modules have been selected, they are inserted according to the connect_mode parameter in
the pertinent connect statements. These rules apply to connect module insertion:

```
1) The connect mode of a port for which a connect module has been selected shall be determined by the
value of the connect_mode parameter of the connect statement which was used to select the
connect module.
2) The connect module for a port shall be instantiated in the context of the ports upper connection.
3) All ports connecting to the same signal (upper connection), sharing the same connect module, and
having merged parameter shall share a single instance of the selected connect module.
4) All other ports shall have an instance of the selected connect module, i.e., one connect module
instance per port.
```
#### 7.8.5 Instance names for auto-inserted instances

Parameters of auto-inserted connect instances can be set on an instance-by-instance basis with the use of the
**defparam** statement. This requires predictable instance names for the auto-inserted modules.

The following naming scheme is employed to unambiguously distinguish the connector modules for the case
of auto-inserted instances.

```
1) merged
In the merged case, one or more ports have a given discipline at their bottom connection, call it
BottomDiscipline, and a common signal, call it SigName, of another discipline at their top con-
nection. A single connect module, call it ModuleName, is placed between the top signal and the bot-
tom signals. In this case, the instance name of the connect module is derived from the signal name,
module name, and the bottom discipline:
```
```
SigName__ModuleName__BottomDiscipline
2) split
In the split case, one or more ports have a given discipline at their bottom connection and a common
signal of another discipline, call it TopDiscipline, at their top connection. One module instance is
instantiated for each such port. In this case, the instance name of the connect module is
```
```
SigName__InstName__PortName
```
```
where InstName and PortName are the local instance name of the port and its instance respec-
tively.
```
NOTE—The __ between the elements of these generated instance names is a double underscore.

**7.8.5.1 Port names for Verilog built-in primitives**

In the cases of instances of modules and instances of UDPs, port names are well defined. In these cases the
port name is the name of the signal at the lower connection of the port. In the case of built-in digital primi-
tives, however, IEEE Std 1364 Verilog does not define port names. In order to support the unique naming of
auto inserted connect modules and the ability to override the parameters of those connect modules, built-in
digital primitives ports will be provided with predictable names. These names are only for the purpose of


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
192
```
naming the connect modules and do not define actual port names. These port names may not be used to
instantiate or to do access of these primitives.

The following naming conventions shall be used when generating connect module instance names that are
connected to built-in digital primitives.

```
1) For N-input gates ( and , nand , nor , or , xnor , xor ) the output will be named out, and the inputs
reading from left to right will be in1, in2, in3, and so forth.
2) For N-output gates ( buf , not ) The input will be named in, and the outputs reading from left to
right will be named out1, out2, out3, and so forth.
3) For 3 port MOS switches ( nmos , pmos , rnmos , rpmos ) the ports reading from left to right will be
named source, drain, gate.
4) For 4 port MOS switches ( cmos , rcmos ) the ports reading from left to right will be named
source, drain, ngate, pgate.
5) For bidirectional pass switches ( tran , tranif1 , tranif0 , rtran , rtranif1 , rtranif) the
ports reading from left to right will be named source, drain, gate.
6) For single port primitives ( pullup , pulldown ) the port will be named out.
```
#### 7.8.6 Supply sensitive connect module examples

The connect modules described so far in Clause 7 use a constant parameter value to set the supply and
threshold voltage levels used in the behavioral blocks of each module. When we need to consider the time
dependent effect of supplies on the switching behavior of connect modules then using elaboration time con-
stants is not sufficient. The following example demonstrates how a string parameter can be used to hierar-
chically access a branch quantity (see 9.20) so that the connect modules are now dependent upon a supply
voltage defined elsewhere in the design,

```
module dig_inv(in, out);
input in;
output out;
reg out;
ddiscrete in, out;
always begin
out = #10 ~in;
end
endmodule
```
```
module analog_inv(in, out, vdd);
input in;
output out;
electrical in, out;
electrical vdd;
real outval;
analog begin
if (V(in) > V(vdd)/2 )
outval = 0;
else
outval = V(vdd) ;
```
```
V(out) <+ transition (outval);
end
endmodule
```
```
module global_supply;
electrical vdd;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
193
```
```
analog V(vdd) <+ 5.0;
endmodule
```
```
module ring;
dig_inv d1 (n1, n2);
dig_inv d2 (n2, n3);
analog_inv a3 (n3, n1, $root .global_supply.vdd);
endmodule
```
```
connectmodule elect_to_logic(el, cm);
input el;
output cm;
reg cm;
electrical el;
ddiscrete cm;
always
@( cross (V(el) - V( $root .global_supply.vdd)/2.0, 1))
cm = 1;
```
```
always
@( cross (V(el) - V( $root .global_supply.vdd)/2.0, -1))
cm = 0;
endmodule
```
```
connectmodule logic_to_elect(cm, el);
input cm;
output el;
ddiscrete cm;
electrical el;
analog
V(el) <+ V( $root .global_supply.vdd) * transition ((cm == 1)? 1 : 0);
endmodule
```
```
connectrules mixedsignal;
connect elect_to_logic;
connect logic_to_elect;
endconnectrules
```
The additional top level module global_supply, now defines a supply voltage vdd. The connect modules
access this supply via a hierarchical reference,

```
$root .global_supply.vdd
```
They are now sensitive to changes in the supply as the simulation proceeds. In this example the name of the
supply is hard coded into the module. Using the analog node alias system functions (see 9.20), a more
generic connect module may be written where the supply name is provided via a string parameter. This
parameter may be set via connect rules.

```
module dig_inv(in, out);
input in;
output out;
reg out;
ddiscrete in, out;
always out = #10 ~in;
endmodule
```
```
module analog_inv(in, out, vdd);
input in;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
194
```
```
output out;
electrical in, out;
electrical vdd;
real outval;
analog begin
if (V(in) > V(vdd)/2.0)
outval = 0;
else
outval = V(vdd);
```
V(out) <+ **transition** (outval);
**end
endmodule**

**module** global_supply;
electrical vdd;
**analog** V(vdd) <+ 5.0;
**endmodule**

**module** ring;
dig_inv d1 (n1, n2);
dig_inv d2 (n2, n3);
analog_inv a3 (n3, n1, **$root** .global_supply.vdd);
**endmodule**

**connectmodule** elect_to_logic(el, cm);
**input** el;
**output** cm;
**reg** cm;
electrical el;
ddiscrete cm;
electrical vdd;
**parameter string** vddname = "(not_given)"; // Set via the CR
**analog initial begin
if** ( **$analog_node_alias** (vdd, vddname) == 0)
**$error** ("Unable to resolve power supply: %s", vddname);
**end**

```
always @( cross (V(el) - V(vdd)/2.0, 1))
cm = 1;
```
```
always @( cross (V(el) - V(vdd)/2.0, -1))
cm = 0;
```
**endmodule**


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
195
```
```
connectmodule logic_to_elect(cm, el);
input cm;
output el;
ddiscrete cm;
electrical el;
electrical vdd;
parameter string vddname = "(not_given)"; // Set via the CR
analog initial begin
if ( $analog_node_alias (vdd, vddname) == 0)
$error ("Unable to resolve power supply: %s", vddname);
end
```
```
analog V(el) <+ V(vdd) * transition ((cm == 1)? 1 : 0);
endmodule
```
```
connectrules mixedsignal;
connect elect_to_logic #(.vddname(" $root .global_supply.vdd"));
connect logic_to_elect #(.vddname(" $root .global_supply.vdd"));
endconnectrules
```
When there are multiple supplies in the design distinct disciplines must be specified for each digital net that
is associated with a given supply. This may be done by explicitly specifying the discipline of the digital nets
or by using remote disciplines. Supply sensitivity in multi supply designs is managed in the same way as
above but with the specific supply hierarchical reference provided on each connect rule as the following
example shows.

```
`include "disciplines.vams"
`timescale 1ns/1ns
```
```
discipline ddiscrete_1v2
domain discrete ;
enddiscipline
```
```
discipline ddiscrete_1v8
domain discrete ;
enddiscipline
```
```
module global_supply;
electrical vdd_1v2;
electrical vdd_1v8;
analog begin
V(vdd_1v2) <+ 1.2;
V(vdd_1v8) <+ 1.8;
end
endmodule
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
196
```
**module** analog_inv(in, out, vdd);
**input** in;
**output** out;
electrical in, out;
electrical vdd;
**real** outval;
**analog begin
if** (V(in) > V(vdd)/2.0)
outval = 0;
**else**
outval = V(vdd);

V(out) <+ **transition** (outval);
**end
endmodule**

// 1.2v supply level
**module** dig_inv_1v2(in, out);
**input** in;
**output** out;
ddiscrete_1v2 in, out;
**reg** out;
**always** out = #10 ~in;
**endmodule**

**module** ring_1v2;
dig_inv_1v2 d1(n1, n2);
dig_inv_1v2 d2(n2, n3);
analog_inv a3 (n3, n1, **$root** .global_supply.vdd_1v2);
**endmodule**

// 1.8v supply level
**module** dig_inv_1v8(in, out);
**input** in;
**output** out;
ddiscrete_1v8 in, out;
**reg** out;
**always** out = #10 ~in;
**endmodule**

**module** ring_1v8;
dig_inv_1v8 d1(n1, n2);
dig_inv_1v8 d2(n2, n3);
analog_inv a3 (n3, n1, **$root** .global_supply.vdd_1v8);
**endmodule**


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
197
```
```
connectmodule elect_to_logic(el, cm);
input el;
output cm;
reg cm;
electrical el;
ddiscrete cm;
electrical vdd;
parameter string vddname = "(not_given)"; // Set via the connect rule
analog initial begin
if ( $analog_node_alias (vdd, vddname) == 0)
$error ("Unable to resolve power supply: %s", vddname);
end
```
```
always @ ( cross (V(el) - V(vdd)/2.0, 1))
cm = 1;
```
```
always @ ( cross (V(el) - V(vdd)/2.0, -1))
cm = 0;
```
```
endmodule
```
```
connectmodule logic_to_elect(cm, el);
input cm;
output el;
ddiscrete cm;
electrical el;
electrical vdd;
parameter string vddname = "(not_given)"; // Set via the connect rule
analog initial begin
if ( $analog_node_alias (vdd, vddname) == 0)
$error ("Unable to resolve power supply: %s", vddname);
end
```
```
analog V(el) <+ V(vdd) * transition ((cm == 1)? 1 : 0);
endmodule
```
```
connectrules mixedsignal;
connect elect_to_logic #(.vddname(" $root .global_supply.vdd_1v2"))
input electrical, output ddiscrete_1v2;
```
```
connect logic_to_elect #(.vddname(" $root .global_supply.vdd_1v2"))
input ddiscrete_1v2, output electrical;
```
```
connect elect_to_logic #(.vddname(" $root .global_supply.vdd_1v8"))
input electrical, output ddiscrete_1v8;
```
```
connect logic_to_elect #(.vddname(" $root .global_supply.vdd_1v8"))
input ddiscrete_1v8, output electrical;
endconnectrules
```
In the above examples the supply hierarchical reference is specified as an absolute name via the **$root**
prefix (see 6.2.1). If **$root** is not supplied then the hierarchical reference search proceeds in the usual way
as defined in 6.7. The hierarchical reference search will start from the connect module insertion point. This
should be taken into consideration when naming the supplies as changes to discipline resolution controls
will affect the connect module location and therefore may change how the supply hierarchical reference
resolves.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
198
```
### 7.9 Driver-receiver segregation

If the hierarchical segments of a signal are all digital or all analog, the signal is not a mixed signal and the
internal representation of the signal does not differ from that of a purely digital or an analog signal.

If the signal has both analog and digital segments in its hierarchy, it is a mixed signal. In this case, the appro-
priate conversion elements are inserted, either manually or automatically, based on the following rules.

```
— All the analog segments of a mixed signal are representations of a single analog node.
— Each of the non-contiguous digital segments of a signal shall be represented internally as a separate
digital signal, with its own state.
— Each non-contiguous digital segment shall be segregated into the collection of drivers of the seg-
ment and the collection of receivers of the segment.
```
In the digital domain, signals can have drivers and receivers. A driver makes a contribution to the state of the
signal. A receiver accesses, or reads, the state of the signal. In a pure digital net, i.e., one without an analog
segment, the simulation kernel resolves the values of the drivers of a signal and it propagates the new value
to the receivers by means of an event when there is a change in state.

In the case of a mixed net, i.e., one with digital segments and an analog segment, it can be useful to propa-
gate the change to the analog simulation kernel, which can then detect a threshold crossing, and then propa-
gate the change in state back to the digital kernel. This, among other things, allows the simulation to account
for rise and fall times caused by analog parasitics.

Within digital segments of a mixed-signal net, drivers and receivers of ordinary modules shall be segregated,
so transitions are not propagated directly from drivers to receivers, but propagate through the analog domain
instead. In this case, the drivers and receivers of connect modules shall be oppositely segregated; i.e., the
connect module drivers shall be grouped with the ordinary module receivers and the ordinary module drivers
shall be grouped with the connect module receivers.

Thus, digital transitions are propagated from drivers to receivers by way of analog (through using connect
module instances). Figure 7- 11 shows driver-receiver segregation in modules having bidirectional and uni-
directional ports, respectively.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
199
```
```
Figure 7-11: Driver-receiver segregation in modules with bidirectional ports
```
```
analog
```
```
digital
```
```
digital
```
```
analog
```
```
inout port
```
```
inout port
```
```
inout port
```
```
drivers
```
drivers

```
receivers
```
```
receivers
```
```
analog
digital
```
```
drivers
```
```
drivers
```
```
receivers
```
```
receivers
```
```
Hierarchical definition Internal representation
```
```
output
port
```
```
input
port
```
```
connection
```
```
digital
```
```
connection
connection
```
```
connection
receiver
```
```
driver
```
```
driver
```
```
receiver
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
200
```
