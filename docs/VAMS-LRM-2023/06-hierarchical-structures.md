## 6. Hierarchical structures

### 6.1 Overview......................................................................................................................................

Verilog-AMS HDL supports a hierarchical hardware description by allowing modules to be embedded
within other modules. Higher-level modules create instances of lower-level modules and communicate with
them through input, output, and bidirectional ports. These module input/output (I/O) ports can be scalar or
vector.

Verilog-AMS HDL provides a mechanism to customize the behavior of embedded modules using parame-
ters. The embedded module parameter default value can be modified through a higher-level module’s
parameter override or a hierarchy independent defparam statement.

To describe a hierarchy of modules, the user provides textual definitions of various modules. Each module
definition stands alone; the definitions are not nested. Statements within the module definitions create
instances of other modules, thus describing the hierarchy.

### 6.2 Modules........................................................................................................................................

A module definition shall be enclosed between the keywords **module** and **endmodule** , as shown in
Syntax 6- 1. The identifier following the keyword **module** shall be the name of the module being defined.
The optional list of parameter definitions shall specify an ordered list of the parameters for the module. The
optional list of ports or port declarations shall specify an ordered list of the ports of the module. The order
used in defining the list of parameters in the _module_parameter_port_list_ and in the list of ports can be sig-
nificant when instantiating the module (see 6.2.2). The identifiers in this list shall be declared in input, out-
put, or inout declaration statements within the module definition. Ports declared in the list of port
declarations shall not be redeclared within the body of the module. The module items define what consti-
tutes a module, and they include many different types of declarations and definitions, many of which have
already been introduced.

A module definition may have multiple **analog** blocks. The behavior of multiple **analog** blocks shall be
defined by assuming that the multiple **analog** blocks internally combine into a single **analog** block in the
order that the **analog** blocks appear in the module description. In other words, they are concatenated in the
order they appear in the module. Concurrent evaluation of the multiple **analog** blocks is implementation
dependent as long as the behavior in that case is similar to what would happen if they had been concate-
nated.

A module can have a description attribute, which shall be used by the simulator when generating help mes-
sages for the module.

The keyword **macromodule** can be used interchangeably with the keyword **module** to define a module.
An implementation may choose to treat module definitions beginning with the **macromodule** keyword
differently.

module_declaration ::= _// from A.1.2_
{ attribute_instance } module_keyword module_identifier [ module_parameter_port_list ]
list_of_ports **;** { module_item }
**endmodule**
| { attribute_instance } module_keyword module_identifier [ module_parameter_port_list ]
[ list_of_port_declarations ] **;** { non_port_module_item }
**endmodule**


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
135
```
module_keyword ::= **module** | **macromodule** | **connectmodule**

module_parameter_port_list ::= **# (** parameter_declaration { **,** parameter_declaration } **)** _// from A.1.3_

list_of_ports ::= **(** port { **,** port } **)**

list_of_port_declarations ::=
**(** port_declaration { **,** port_declaration } **)**

port ::=
[ port_expression ]
|**.** port_identifier **(** [ port_expression ] **)**

port_expression ::=
port_reference
| **{** port_reference { **,** port_reference } **}**

port_reference ::=
port_identifier [ **[** constant_range_expression **]** ]

port_declaration ::=
{attribute_instance} inout_declaration
| {attribute_instance} input_declaration
| {attribute_instance} output_declaration

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
| specify_block
| { attribute_instance } parameter_declaration **;**


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
136
```
```
| { attribute_instance } specparam_declaration
| aliasparam_declaration
```
parameter_override ::= **defparam** list_of_defparam_assignments **;**

```
Syntax 6-1—Syntax for module
```
#### 6.2.1 Top-level modules and $root

_Top-level modules_ are modules that are included in the source text, but do not appear in any module instanti-
ation statement, as described in 6.2.2. This applies even if the module instantiation appears in a generate
block that is not itself instantiated (see Syntax 6.6).

Verilog-AMS incorporates hierarchical identifier prefix **$root** from IEEE Std 1800 SystemVerilog. The
name **$root** is used to unambiguously refer to a top-level instance or to an instance path starting from the
root of the instantiation tree. **$root** is the root of the instantiation tree.

For example:

```
$root .A.B // item B within top instance A
$root .A.B.C // item C within instance B within top instance A
```
**$root** allows explicit access to the top of the instantiation tree. This is useful to disambiguate a local path
(which takes precedence) from the rooted path. If **$root** is not specified, a hierarchical path is ambiguous.

For example, A.B.C can mean the local A.B.C or the top-level A.B.C (assuming there is an instance A that
contains an instance B at both the top level and in the current module). The ambiguity is resolved by giving
priority to the local scope and thereby preventing access to the top-level path. **$root** allows explicit access
to the top level in those cases in which the name of the top-level module is insufficient to uniquely identify
the path.

#### 6.2.2 Module instantiation

Instantiation allows one module to incorporate a copy of another module into itself. Module definitions do
not nest. That is, one module definition does not contain the text of another module definition within its
**module** ... **endmodule** keyword pair. A module definition nests another module by _instantiating_ it. The
_module instantiation statement_ creates one or more named _instances_ of a defined module.

Syntax 6- 2 gives the syntax for specifying instantiations of modules.

module_instantiation ::= _// from A.4.1_
module_or_paramset_identifier [ parameter_value_assignment ]
module_instance { **,** module_instance } **;**

parameter_value_assignment ::= **# (** list_of_parameter_assignments **)**

list_of_parameter_assignments ::=
ordered_parameter_assignment { **,** ordered_parameter_assignment }
| named_parameter_assignment { **,** named_parameter_assignment }

ordered_parameter_assignment ::= expression

named_parameter_assignment ::=

**.** parameter_identifier **(** [ mintypmax_expression ] **)**
|**.** system_parameter_identifier **(** [ constant_expression ] **)**

module_instance ::= name_of_module_instance **(** [ list_of_port_connections ] **)**


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
137
```
name_of_module_instance ::= module_instance_identifier [ range ]

list_of_port_connections ::=
ordered_port_connection { **,** ordered_port_connection }
| named_port_connection { **,** named_port_connection }

ordered_port_connection ::= { attribute_instance } [ expression ]

named_port_connection ::= { attribute_instance }**.** port_identifier **(** [ expression ] **)**

```
Syntax 6-2—Syntax for module instantiation
```
The instantiations of modules can contain a range specification. This allows an array of instances to be cre-
ated.

One or more module instances (identical copies of a module definition) can be specified in a single module
instantiation statement.

The list of module connections shall be provided only for modules defined with ports. The parentheses,
however, are always required. When a list of port connections is given using the ordered port connection
method, the first element in the list shall connect to the first port declared in the module, the second to the
second port, and so on. See 6.5 for a more detailed discussion of ports and port connection rules.

A connection can be a simple reference to a variable or a net identifier, an expression or a blank. An expres-
sion can be used for supplying a value to a module input port if it is a digital port. A blank port connection
shall represent the situation where the port is not to be connected.

When connecting ports by name, an unconnected port can be indicated either by omitting it in the port list or
by providing no expression in the parentheses [i.e., .port_name ()]

The example below illustrates a comparator and an integrator (lower-level modules) which are instantiated
in sigma-delta A/D converter module (the higher-level module).

```
module comparator(cout, inp, inm);
output cout;
input inp, inm;
electrical cout, inp, inm;
parameter real td = 1n, tr = 1n, tf = 1n;
real vcout;
analog begin
@( cross (V(inp) - V(inm), 0))
vcout = ((V(inp) > V(inm))? 1 : 0);
V(cout) <+ transition (vcout, td, tr, tf);
end
endmodule
```
```
module integrator(out, in);
output out;
input in;
electrical in, out;
parameter real gain = 1.0;
parameter real ic = 0.0;
analog begin
V(out) <+ gain* idt (V(in), ic);
end
endmodule
```
```
module sigmadelta(out, aref, in);
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
138
```
```
output out;
input aref, in;
electrical out, aref, in;
electrical gnd; ground gnd;
```
```
comparator C1(.cout(aa0), .inp(in), .inm(aa2));
integrator #(1.0) I1(.out(aa1), .in(aa0));
comparator C2(out, aa1, gnd);
d2a #(.width(1)) D1(aa2, aref, out); // a D/A converter
endmodule
```
The comparator instance C1 and the integrator instance I1 in Figure 6- 1 use named port connections,
whereas the comparator instance C2 and the d2a (not described here) instance D1 use ordered port connec-
tions. Note the integrator instance I1 overrides gain parameter positionally, whereas the d2a instance D1
overrides width parameter by named association.

```
Figure 6-1: Comparator and integrator modules
```
**6.3 Overriding module parameter values**

When one module instantiates another module, it can alter the values of any parameters declared within the
instantiated module, as well as the values of various system parameters that are implicitly declared for all
modules. There are three ways to alter parameter values: the _defparam statement_ , which allows assignment
to parameters using their hierarchical names, the _module instance parameter value assignment_ , which
allows values to be assigned inline during module instantiation, and the _paramset_ , which is described in 6.4.
If a defparam assignment conflicts with a module instance parameter, the parameter in the module shall take
the value specified by the defparam. If a defparam assignment conflicts with a paramset instance parameter,
the paramset selection will occur with the parameter value specified by the defparam.

The module instance parameter value assignment comes in two forms, by ordered list or by name. The first
form is _module instance parameter value assignment by order_ , which allows values to be assigned in-line
during module instantiation in the order of their declaration. The second form is _module instance parameter
value assignment by name_ , which allows values to be assigned in-line during module instantiation by explic-
itly associating parameter names with the overriding values.

#### 6.3.1 Defparam statement

Using the defparam statement, parameter values can be changed in any module instance throughout the
design using the hierarchical name of the parameter. See 6.7 for details about hierarchical names.

However, a defparam statement in a hierarchy in or under a generate block instance (see 6.6) or an array of
instances (see 6.2.2) shall not change a parameter value outside that hierarchy. Additionally, a defparam
statement is not allowed in a hierarchy in or under a paramset instance (see 6.4).

```
in out
C1 I1 C2
```
##### D1

```
aa0 aa1
```
```
ground
aa2
```
```
aref
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
139
```
Each instantiation of a generate block is considered to be a separate hierarchy scope. Therefore, this rule
implies that a defparam statement in a generate block may not target a parameter in another instantiation of
the same generate block, even when the other instantiation is created by the same loop generate construct.

For example, the following code is not allowed:

```
genvar i;
```
```
generate
for (i = 0; i < 8; i = i + 1) begin : somename
flop my_flop(in[i], in1[i], out1[i]);
defparam somename[i+1].my_flop.xyz = i ;
end
endgenerate
```
Similarly, a defparam statement in one instance of an array of instances may not target a parameter in
another instance of the array.

The expression on the right-hand side of a defparam assignments shall be a constant expression involving
only constant numbers and references to parameters. The referenced parameters (on the right-hand side of a
defparam) shall be declared in the same module as the defparam statement.

The defparam statement is particularly useful for grouping all of the parameter value override assignments
together in one module. Its syntax is shown in Syntax 6- 3.

parameter_override ::= **defparam** list_of_defparam_assignments **;** _// from A.1.4_

list_of_defparam_assignments ::= defparam_assignment { **,** defparam_assignment } _// from A.2.3_

defparam_assignment ::= hierarchical_parameter_identifier **=** constant_mintypmax_expression _// from A.2.4_

```
Syntax 6-3—Syntax for defparam
```
Examples:

```
module tgate ();
electrical io1,io2,control,control_bar;
mosn m1 (io1, io2, control);
mosp m2 (io1, io2, control_bar);
endmodule
```
```
module mosp (drain,gate,source);
inout drain, gate, source;
electrical drain, gate, source;
parameter gate_length = 0.3e-6,
gate_width = 4.0e-6;
```
```
spice_pmos #(.l(gate_length),.w(gate_width)) p (drain, gate, source);
endmodule
```
```
module mosn (drain,gate,source);
inout drain, gate, source;
electrical drain, gate, source;
parameter gate_length = 0.3e-6,
gate_width = 4.0e-6;
spice_nmos #(.l(gate_length),.w(gate_width)) n (drain, gate, source);
endmodule
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
140
```
```
module annotate ();
defparam
tgate.m1.gate_width = 5e-6,
tgate.m2.gate_width = 10e-6;
endmodule
```
#### 6.3.2 Module instance parameter value assignment by order

The order of the assignments in module instance parameter value assignment shall follow the order of decla-
ration of the parameters within the module. Local parameter declarations are not considered when assigning
by order; parameter alias declarations are also skipped. It is not necessary to assign values to all of the
parameters within a module when using this method. However, the left-most parameter assignment(s) can
not be skipped. Therefore, to assign values to a subset of the parameters declared within a module, the decla-
rations of the parameters which make up this subset shall precede the declarations of the remaining
(optional) parameters. An alternative is to assign values to all of the parameters, but use the default value
(the same value assigned in the declaration of the parameter within the module definition) for those parame-
ters which do not need new values.

Consider the following example, where the parameters within module instance weakp are changed during
instantiation.

```
module m ();
electrical clk;
electrical out_a, in_a;
electrical out_b, in_b;
```
```
// create an instance and set parameters
mosp #(2e-6,1e-6) weakp (out_a, in_a, clk);
```
```
// create an instance leaving default values
mosp plainp (out_b, in_b, clk);
endmodule
```
#### 6.3.3 Module instance parameter value assignment by name

Parameter assignment by name consists of explicitly linking the parameter name and its value. The name of
the parameter shall be the name specified in the instantiated module. It is not necessary to assign values to
all the parameters within a module when using this method. Only those parameters which are assigned new
values need to be specified.

The parameter expression is optional so the instantiating module can document the existence of a parameter
without assigning anything to it. The parentheses are required and in this case the parameter retains its
default value. Once a parameter is assigned a value, there shall not be another assignment to this parameter
name.

In the following example of instantiating a voltage-controlled oscillator, the parameters are specified on a
named-association basis much as they are for ports.

```
module n (lo_out, rf_in);
output lo_out;
input rf_in;
electrical lo_out, rf_in;
```
```
//create an instance and set parameters
vco #(.centerFreq(5000), .convGain(1000)) vco1(lo_out, rf_in);
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
141
```
```
endmodule
```
Here, the name of the instantiated vco module is vco1. The centerFreq parameter is passed a value of
5000 and the convGain parameter is passed a value of 1000. The positional assignment mechanism for
ports assigns lo_out as the first port and rf_in as the second port of vco1.

#### 6.3.4 Parameter dependence.....................................................................................................

A parameter (for example, gate_cap) can be defined with an expression containing another parameter (for
example, gate_width or gate_length). Since gate_cap depends on the value of gate_width and
gate_length, a modification of gate_width or gate_length changes the value of gate_cap.

In the following parameter declaration, an update of gate_width, whether by a defparam statement or in an
instantiation statement for the module which defined these parameters, automatically updates gate_cap.

```
parameter
gate_width = 0.3e-6,
gate_length = 4.0e-6,
gate_cap = gate_length * gate_width * ‘COX;
```
#### 6.3.5 Detecting parameter overrides

In some cases, it is important to be able to determine whether a parameter value was obtained from the
default value in its declaration statement or if that value was overridden. In such a case, the **$param_-
given()** function described in 9.19 can be used.

#### 6.3.6 Hierarchical system parameters

In addition to the parameters explicitly declared in a module’s header, there are six system parameters that
are implicitly declared for every module: **$mfactor** , **$xposition** , **$yposition** , **$angle** , **$hflip** ,
and **$vflip**. The values of these parameters may be accessed in a module or paramset using these names,
as described in 9.18. The value of these parameters may be overridden using the defparam statement, mod-
ule instance parameter value assignment by name, or a paramset; in all three methods, the system parameter
identifier is prefixed by a period (**.** ), just as for explicitly-declared parameters.

If an instance of a module has a non-unity value of **$mfactor** , then the following rules are applied auto-
matically by the simulator:

```
— All contributions to a branch flow quantity in the analog block shall be multiplied by $mfactor
— The value returned by any branch flow probe in the analog block, including those used in indirect
assignments, shall be divided by $mfactor
— Contributions to a branch flow quantity using the noise functions of 4.6.4 ( white_noise ,
flicker_noise , and noise_table ) shall have the noise power multiplied by $mfactor
— Contributions to a branch potential quantity using the noise functions of 4.6.4 shall have the noise
power divided by $mfactor
— The module’s value of $mfactor is also propagated to any module instantiated by the original
module, according to the rules found in 9.18.
```
Application of these rules guarantees that the behavior of the module in the design is identical to the behav-
ior of a quantity **$mfactor** of identical modules with the same connections; however, the simulator only
has to evaluate the module once.

Verilog-AMS does not provide a method to disable the automatic **$mfactor** scaling. The simulator shall
issue a warning if it detects a misuse of the **$mfactor** in a manner that would result in double-scaling.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
142
```
The two resistor modules below show ways that the **$mfactor** might be used in a module. The first exam-
ple, badres, misuses the **$mfactor** such that the contributed current would be multiplied by **$mfactor**
twice, once by the explicit multiplication and once by the automatic scaling rule. The simulator will generate
an error for this module.

```
module badres(a, b);
inout a, b;
electrical a, b;
parameter real r = 1.0 from (0: inf );
analog begin
I(a,b) <+ V(a,b) / r * $mfactor ; // ERROR
end
endmodule
```
In this second example, parares, **$mfactor** is used only in the conditional expression and does not scale
the output. No error will be generated for this module. In cases where the effective resistance r/ **$mfactor**
would be too small, the resistance is simply shorted out, and the simulator may collapse the node to reduce
the size of the system of equations.

```
module parares(a, b);
inout a, b;
electrical a, b;
parameter real r = 1.0 from (0: inf );
analog begin
if (r / $mfactor < 1.0e-3)
V(a,b) <+ 0.0;
else
I(a,b) <+ V(a,b) / r;
end
endmodule
```
The values of the five geometrical system parameters, **$xposition** , **$yposition** , **$angle** , **$hflip** ,
and **$vflip** , do not have any automatic effect on the simulation. The paramset or module may use these
values to compute geometric layout-dependent effects, as shown in the following example.

In the next example, it is assumed that a top-level module named processinfo contains values for the
properties of polysilicon resistors in the manufacturing process, including the nominal value pro-
cessinfo.rho and the gradients processinfo.drho_dx and processinfo.drho_dy, in the x and y
direction respectively.

```
module polyres(a, b);
inout a, b;
electrical a, b;
parameter real length = 1u from (0: inf );
parameter real width = 1u from (0: inf );
real rho, reff;
analog begin
rho = processinfo.rho
+ $xposition * processinfo.drho_dx
+ $yposition * processinfo.drho_dy;
reff = rho * length / width;
I(a,b) <+ V(a,b) / reff;
end
endmodule
```
The resistor just defined could be instantiated in the following manner so as to cancel out the process gradi-
ents:


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
143
```
```
module matchedres(a, b);
inout a, b;
electrical a, b;
parameter real length = 1u from (0: inf );
parameter real width = 1u from (0: inf );
polyres #(.width(width/4.0), .length(length),
.$xposition(-1u), .$yposition(-1u)) R1 (a, b);
polyres #(.width(width/4.0), .length(length),
.$xposition(+1u), .$yposition(-1u)) R2 (a, b);
polyres #(.width(width/4.0), .length(length),
.$xposition(-1u), .$yposition(+1u)) R3 (a, b);
polyres #(.width(width/4.0), .length(length),
.$xposition(+1u), .$yposition(+1u)) R4 (a, b);
endmodule
```
Unfortunately, if the module matchedres is itself instantiated off-center, then the process gradients will not
be canceled.

### 6.4 Paramsets

A _paramset definition_ is enclosed between the keywords **paramset** and **endparamset** , as shown in
Syntax 6- 4. The first identifier following the keyword **paramset** is the name of the paramset being
defined. The second identifier will usually be the name of a module with which the paramset is associated.
The second identifier may instead be the name of a second paramset. A chain of paramsets may be defined
in this way, but the last paramset in the chain shall reference a module.

paramset_declaration ::= _// from A.1.9_
{ attribute_instance } **paramset** paramset_identifier module_or_paramset_identifier **;**
paramset_item_declaration { paramset_item_declaration }
paramset_statement { paramset_statement }
**endparamset**

paramset_item_declaration ::=
{ attribute_instance } parameter_declaration **;**
| { attribute_instance } local_parameter_declaration **;**
| aliasparam_declaration
| { attribute_instance } integer_declaration
| { attribute_instance } real_declaration

paramset_statement ::=

**.** module_parameter_identifier **=** paramset_constant_expression **;**
|**.** module_output_variable_identifier **=** paramset_constant_expression **;**
|**.** system_parameter_identifier **=** paramset_constant_expression **;**
| analog_function_statement

paramset_constant_expression ::=
constant_primary
| hierarchical_parameter_identifier
| unary_operator { attribute_instance } constant_primary
| paramset_constant_expression binary_operator { attribute_instance } paramset_constant_expression
| paramset_constant_expression**?** { attribute_instance } paramset_constant_expression **:** {
attribute_instance } paramset_constant_expression

```
Syntax 6-4—Syntax for paramset
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
144
```
The paramset itself contains no behavioral code; all of the behavior is determined by the associated module.
The restrictions on statements in the paramset are described in 6.4.1.

The paramset provides a convenient way to collect parameter values for a particular module, such that an
instance need only provide overrides for a smaller number of parameters. A simulator can use this informa-
tion to optimize data storage for the instances: multiple instances may share a paramset, and the simulator
can share storage of parameters of the underlying module. The shared storage of paramsets makes them sim-
ilar to the SPICE model card. Also like the SPICE model card, paramsets may be overloaded, as described in
6.4.2.

The only restriction on the associated module for a paramset is that it does not contain a defparam statement
in or under its hierarchy, see 6.3.1.

A paramset can have a description attribute, which shall be used by the simulator when generating help mes-
sages for the paramset.

The following example shows how one might convert a SPICE model card into a Verilog-AMS paramset.
Suppose one has the following lines in a SPICE netlist:

```
m1 d1 g 0 0 nch l=1u w=10u
m2 d2 g 0 0 nch l=1u w=5u
.model nch nmos (level=3 kp=5e-5 tox=3e-8 u0=650 nsub=1.3e17
+ vmax=0 tpg=1 nfs=0.8e12)
```
These lines could be written in Verilog-AMS as follows, assuming that nmos3 is a behavioral module that
contains the same equations as the SPICE primitive.

```
nch #(.l(1u), .w(10u)) m1 (.d(d1), .g(g), .s(0), .b(0));
nch #(.l(1u), .w(5u)) m2 (.d(d2), .g(g), .s(0), .b(0));
```
```
paramset nch nmos3; // default paramset
parameter real l=1u from [0.25u: inf );
parameter real w=1u from [0.2u: inf );
.l=l; .w=w; .ad=w*0.5u; .as=w*0.5u;
.kp=5e-5; .tox=3e-8; .u0=650; .nsub=1.3e17;
.vmax=0; .tpg=1; .nfs=0.8e12;
endparamset
```
Note that the paramset has only two parameters, l and w; an instance of the paramset that attempts to over-
ride any of the other parameters of the underlying module nmos3 would generate an error. Analog simula-
tors are expected to optimize the storage of paramset values in a manner similar to the way SPICE optimizes
model parameter storage.

#### 6.4.1 Paramset statements

The restrictions on statements or assignments allowed in a paramset are similar to the restrictions for analog
functions. Specifically, a paramset:

```
— can use any statements available for conditional execution (see 5.2);
— shall not use access functions;
— shall not use contribution statements or event control statements; and
— shall not use named blocks.
```
The special syntax


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
145
```
**.** module_parameter_identifier **=** paramset_constant_expression **;**

is used to assign values to the parameters of the associated module. The expression on the right-hand side
can be composed of numbers, parameters and hierarchical out-of-module references to local parameters of a
different module. Hierarchical out-of-module references to non-local parameters of a different module is
disallowed. The expression may also use the **$arandom** function from 9.13.1 and the **$rdist_** functions
from 9.13.2, so long as the arguments to these functions are constant.

Paramset statements may assign values to variables declared in the paramset; the values need not be constant
expressions. However, these variables shall not be used to assign values to the module’s parameters. Param-
set variables may be used to provide output variables for instances that use the paramset; see 6.4.3.

The following example shows how to use the **$rdist_normal** function of 9.13.2 to model two kinds of
statistical variation.

```
module semicoCMOS ();
localparam real tox = 3e-8;
localparam real dtox_g = $rdist_normal (1,0,1n,"global");
localparam real dtox_mm = $rdist_normal (2,0,5n,"instance");
endmodule
```
```
paramset nch nmos3; // mismatch paramset
parameter real l=1u from [0.25u: inf );
parameter real w=1u from [0.2u: inf );
parameter integer mm=0 from (0:1];
.l=l; .w=w; .ad=w*0.5u; .as=w*0.5u;
.kp=5e-5; .u0=650; .nsub=1.3e17;
.vmax=0; .tpg=1; .nfs=0.8e12;
.tox = semicoCMOS.tox + semicoCMOS.dtox_g + semicoCMOS.dtox_mm;
endparamset
```
```
module top ();
electrical d1, d2, g, vdd, gnd;
ground gnd;
nch #(.l(1u), .w(5u), .mm(1)) m1(.d(d1), .g(g), .s(gnd), .b(gnd));
nch #(.l(1u), .w(5u), .mm(1)) m2(.d(d2), .g(g), .s(gnd), .b(gnd));
resistor #(.r(1k)) R1 (vdd, d1);
resistor #(.r(1k)) R2 (vdd, d2);
vsine #(.dc(2.5)) Vdd (vdd, gnd);
vsine #(.dc(0), .ampl(1.0), .offset(1.5), .freq(1k)) Vg (g, gnd);
endmodule
```
Because the local parameter dtox_mm is obtained from **$rdist_normal** with the string "instance", the
instances m1 and m2 will get different values of tox. Though the local variation has a smaller standard devi-
ation than the global variation, only the local variation will affect the differential voltage between nodes d1
and d2.

#### 6.4.2 Paramset overloading

Paramset identifiers need not be unique: multiple paramsets can be declared using the same _paramset_iden-
tifier_ , and they may refer to different modules. During elaboration, the simulator shall choose an appropriate
paramset from the set that shares a given name for every instance that references that name.

When choosing an appropriate paramset, the following rules shall be enforced:

```
— All parameters overridden on the instance shall be parameters of the paramset
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
146
```
```
— The parameters of the paramset, with overrides and defaults, shall be all within the allowed ranges
specified in the paramset parameter declaration.
— The local parameters of the paramset, computed from parameters, shall be within the allowed ranges
specified in the paramset.
— The underlying module shall have a port declared for each port connected in the instance line.
```
The rules above may not be sufficient for the simulator to pick a unique paramset, in which case the follow-
ing rules shall be applied in order until a unique paramset has been selected:

```
— The paramset with the fewest number of un-overridden parameters shall be selected.
— The paramset with the greatest number of local parameters with specified ranges shall be selected.
— The paramset with the fewest ports not connected in the instance line shall be selected.
```
It shall be an error if there are still more than one applicable paramset for an instance after application of
these rules.

If a paramset assigns a value to a module parameter and this value is outside the range specified for that
module parameter, it shall be an error. The simulator shall consider only the ranges of the paramset’s own
parameters when choosing a paramset.

The following example illustrates some of the rules for paramset selection. Consider a design that includes
the two paramsets defined previously (in the examples of 6.4 and 6.4.1) as well as the following paramsets:

```
paramset nch nmos3; // short-channel paramset
parameter real l=0.25u from [0.25u:1u);
parameter real w=1u from [0.2u: inf );
parameter real ad=0.5*w from (0: inf );
parameter real as=0.5*w from (0: inf );
.l=l; .w=w; .ad=ad; .as=as;
.kp=5e-5; .tox=3e-8; .u0=650; .nsub=1.3e17;
.vmax=0; .tpg=1; .nfs=0.8e12;
endparamset
```
```
paramset nch nmos3; // long-channel paramset
parameter real l=1u from [1u: inf );
parameter real w=1u from [0.2u: inf );
parameter real ad=0.4*w from (0: inf );
parameter real as=0.4*w from (0: inf );
.l=l; .w=w; .ad=ad; .as=as;
.kp=5e-5; .tox=3e-8; .u0=640; .nsub=1.3e17;
.vmax=0; .tpg=1; .nfs=0.7e12;
endparamset
```
The following instances might exist in the design:

```
nch #(.l(1u), .w(5u), .mm(1)) m1(.d(d1), .g(g), .s(0), .b(0));
nch #(.l(1u), .w(5u), .mm(1)) m2(.d(d2), .g(g), .s(0), .b(0));
nch #(.l(1u), .w(10u)) m3 (.d(g), .g(g), .s(0), .b(0));
nch #(.l(3u), .w(5u), .ad(1.2p), .as(1.3p))
m4 (.d(d1), .g(g2), .s(d2), .b(0));
```
The instances m1 and m2 will use the mismatch paramset from 6.4.1, because it is the only one for which mm
is a parameter. The instance m4 will use the long-channel paramset defined in this example, because while
the short-channel paramset also has ad and as as parameters, the length of m4 is only allowed by the range
for l in the long-channel paramset. The instance m3 will use the default paramset defined in 6.4; it cannot
use the mismatch paramset because the default value of mm for that paramset is not allowed by the range, and


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
147
```
it discriminates against the long-channel paramset because that paramset would have two un-overridden
parameters.

#### 6.4.3 Paramset output variables

As with modules, integer or real variables in the paramset that are declared with descriptions are considered
output variables; see 3.2.1. A few special rules apply to paramset output variables and output variables of
modules referenced by a paramset:

```
— If a paramset output variable has the same name as an output variable of the module, the value of the
paramset output variable is the value reported for any instance that uses the paramset.
— If a paramset variable without a description has the same name as an output variable of the module,
the module output variable of that name shall not be available for instances that use the paramset.
— A paramset output variable’s value may be computed from values of any output parameters of the
module by using the special syntax
```
**.** module_output_variable_identifier

The following example declares an output variable ft for instances of the paramset smnpn.The module is
assumed to have output variables named gm, cpi, and cmu. If the module npn had an output variable named
ft, the paramset’s output variable would replace it.

```
paramset smnpn npn; // small npn paramset
(*desc="cut-off frequency"*) real ft;
.is=2.0e-17; .bf=120.0; .br=10; rb=145; .rc=75; .re=12;
.cje=2.0e-14; .vje=0.9; .mje=0.4;
.cjc=3.0e-14; .vjc=0.6; .mjc=0.3; .xcjc=0.2;
ft = .gm/(‘M_TWO_PI*(.cpi + .cmu));
endparamset
```
### 6.5 Ports

Ports provide a means of interconnecting instances of modules. For example, if a module A instantiates mod-
ule B, the ports of module B are associated with either the ports or the internal nets of module A.

#### 6.5.1 Port definition

The syntax for a port association is shown in Syntax 6- 5.

port ::= _// from A.1.3_
[ port_expression ]
|**.** port_identifier **(** [ port_expression ] **)**

port_expression ::=
port_reference
| **{** port_reference { **,** port_reference } **}**

```
Syntax 6-5—Syntax for port
```
The port expression in the port definition can be one of the following:

```
— a simple net identifier
— a scalar member of a vector net or port declared within the module
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
148
```
```
— a sub-range of a vector net or port declared within the module
— a vector net formed as a result of the concatenation operator
```
The port expression is optional because ports can be defined which do not connect to anything internal to the
module.

#### 6.5.2 Port declarations..............................................................................................................

The type and direction of each port listed in the module definition’s list of ports are declared in the body of
the module.

**6.5.2.1 Port type**

The type of a port is declared by giving its discipline, as shown in Syntax 6- 6. If the type of a port is not
declared, the port can only be used in a structural description. (It can be passed to instances of modules, but
cannot be accessed in a behavioral description.)

net_declaration ::= _// from A.2.1.3_
...
| discipline_identifier [ range ] list_of_net_identifiers **;**
| discipline_identifier [ range ] list_of_net_decl_assignments **;**
...

range ::= **[** msb_constant_expression **:** lsb_constant_expression **]** _// from A.2.5_

list_of_net_decl_assignments ::= net_decl_assignment { **,** net_decl_assignment } _// from A.2.3_

list_of_net_identifiers ::= ams_net_identifier { **,** ams_net_identifier }

net_decl_assignment ::= ams_net_identifier **=** expression _// from A.2.4_

```
Syntax 6-6—Syntax for port type declarations
```
**6.5.2.2 Port direction**

Each port listed in the list of ports for the module definition shall be declared in the body of the module as an
**input** , **output** , or **inout** (bidirectional). This is in addition to any other declaration for a particular
port—for example, a _net_discipline_ , **reg** , or **wire**. The syntax for port declarations is shown in Syntax 6- 7.

inout_declaration ::= _// from A.2.1.2_

```
inout [ discipline_identifier ] [ net_type | wreal ] [ signed ] [ range ] list_of_port_identifiers
```
input_declaration ::=
**input** [ discipline_identifier ] [ net_type | **wreal** ] [ **signed** ] [ range ] list_of_port_identifiers

output_declaration ::=
**output** [ discipline_identifier ] [ net_type | **wreal** ] [ **signed** ] [ range ] list_of_port_identifiers
| **output** [ discipline_identifier ] **reg** [ **signed** ] [ range ] list_of_variable_port_identifiers
| **output** output_variable_type list_of_variable_port_identifiers

```
Syntax 6-7—Syntax for port direction declarations
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
149
```
A port can be declared in both a _port type_ declaration and a _port direction_ declaration. If a port is declared as
a vector, the range specification between the two declarations of a port shall be identical. For example:

```
input [0:3] in;
electrical [0:3] in; // valid, MSB and LSB in both the port type and port
// direction declaration evaluate to the same value
```
```
input [0:3] in;
electrical [0:4-1] in; // valid, MSB and LSB in both the port type and port
// direction declaration evaluate to the same value
```
```
input [3:0] in;
electrical [0:3] in; // error, MSB and LSB in the port type declaration does
// not evaluate to the same value as the port direction
// declaration.
```
Implementations can limit the maximum number of ports in a module definition, but this shall be a mini-
mum of 256 ports per implementation.

#### 6.5.3 Real valued ports.............................................................................................................

Verilog-AMS HDL supports ports which are declared to be real-valued and have a discrete-time discipline.
This is done using the net type **wreal** (defined in 3.7). There can be a maximum of one driver of a real-val-
ued net.

Examples:

```
module top();
wreal stim;
reg clk;
wire [1:8] out;
```
```
testbench tb1 (stim, clk);
a2d dut (out, stim, clk);
```
```
initial clk = 0;
always #1 clk = ~clk;
endmodule
```
```
module testbench(wout, clk);
output wout;
input clk;
real out;
wire clk;
wreal wout;
```
```
assign wout = out;
```
```
always @( posedge clk) begin
out = out + $abstime ;
end
endmodule
```
```
module a2d(dout, in, clk);
output [1:8] dout;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
150
```
```
input in, clk;
wreal in;
wire clk;
reg [1:8] dout;
real residue;
integer i;
```
```
always @( negedge clk) begin
residue = in;
for (i = 8; i >= 1; i = i - 1) begin
if (residue > 0.5) begin
dout[i] = 1'b1;
residue = residue - 0.5;
end
else begin
dout[i] = 1'b0;
end
residue = residue*2;
end
end
endmodule
```
#### 6.5.4 Connecting module ports by ordered list

One way to connect the ports listed in a module instantiation with the ports defined by the instantiated mod-
ule is via an ordered list—that is, the ports listed for the module instance shall be in the same order as the
ports listed in the module definition.

Examples:

```
module adc4 (out, rem, in);
output [3:0] out; output rem;
input in;
electrical [3:0] out;
electrical in, rem, rem_chain;
```
```
adc2 hi2 (out[3:2], rem_chain, in);
adc2 lo2 (out[1:0], rem, rem_chain);
endmodule
```
```
module adc2 (out, remainder, in);
output [1:0] out; output remainder;
input in;
electrical [1:0] out;
electrical in, remainder, r;
```
```
adc hi1 (out[1], r, in);
adc lo1 (out[0], remainder, r);
endmodule
```
```
module adc (out, remainder, in);
output out, remainder;
input in;
electrical out, in, remainder;
integer d;
```
```
analog begin
d = (V(in) > 0.5);
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
151
```
```
V(out) <+ transition (d);
V(remainder) <+ 2.0 * V(in);
if (d)
V(remainder) <+ -1.0;
end
endmodule
```
#### 6.5.5 Connecting module ports by name..................................................................................

The second way to connect module ports consists of explicitly linking the two names for each side of the
connection — specify the name used in the module definition, followed by the name used in the instantiating
module. This compound name is then placed in the list of module connections.

The following rules apply:

```
— The name of port shall be the name specified in the module definition.
— The name of port cannot be a bit select or a part select.
— The port expression shall be the name used by the instantiating module and can be one of the follow-
ing:
— a simple net identifier
— a scalar member of a vector net or port declared within the module
— a sub-range of a vector net or port declared within the module
— a vector net formed as a result of the concatenation operator
— The port expression is optional so the instantiating module can document the existence of the port
without connecting it to anything. The parentheses are required.
— The two types of module port connections can not be mixed; connections to the ports of a particular
module instance shall be all by order or all by name.
```
Examples:

```
module adc4 (out, rem, in);
input in;
output [3:0] out; output rem;
electrical [3:0] out;
electrical in, rem, rem_chain;
```
```
adc2 hi (.in(in), .out(out[3:2]), .remainder(rem_chain));
adc2 lo (.in(rem_chain), .out(out[1:0]), .remainder(rem));
endmodule
```
```
module adc2 (out, in, remainder);
output [1:0] out; output remainder;
input in;
electrical [1:0] out;
electrical in, remainder, r;
```
```
// adc is same as defined in 6.5.4
adc hi1 (out[1], r, in);
adc lo1 (out[0], remainder, r);
endmodule
```
Since these connections were made by port name, the order in which the connections appear is irrelevant.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
152
```
#### 6.5.6 Detecting port connections..............................................................................................

When a module is instantiated, all of its ports need not be connected. For example, a clock module may pro-
vide outputs clk and clkbar, but a design may only need clk. In some cases, it may be important to know
whether a particular port is connected. For example, if the **transition()** filter of 4.5.8 is used on the
outputs, it might speed up the simulation if the filter is only used when the port is connected. The **$port_-
connected()** function described in 9.19 can be used to determine whether a port is connected.

#### 6.5.7 Port connection rules.......................................................................................................

All digital ports connected to a net shall be of compatible disciplines, as shall all analog ports connected to a
net. Ports of both analog and digital discipline may be connected to a net provided the appropriate connect
statements exist (see 7.7).

**6.5.7.1 Matching size rule**

A scalar port can be connected to a scalar net and a vector port can be connected to a vector net or concate-
nated net expression of the matching width. In other words, the sizes of the ports and net need to match.

**6.5.7.2 Resolving discipline of undeclared interconnect signal**

Verilog-AMS HDL supports undeclared interconnects between module instances when describing hierarchi-
cal structures. That is, a signal appearing in the connection list of a module instantiation need not appear in
any port declaration or discipline declaration (see 7.4).

#### 6.5.8 Inheriting port natures

A net of continuous discipline shall have a potential nature and may have a flow nature. Because of hierar-
chical connections, an analog node may be associated with a number of analog nets, and thus, a number of
continuous disciplines. The node shall be treated as having a **potential abstol** with a value equal to
the smallest **abstol** of all the potential natures of all the disciplines with which it is associated. The node
shall be treated as having a **flow abstol** with a value equal to the smallest **abstol** of all the flow
natures, if any, of all the disciplines with which it is associated.

### 6.6 Generate constructs

Generate constructs are used to either conditionally or multiply instantiate generate blocks into a model. A
generate block is a collection of one or more module items. A generate block may not contain port declara-
tions, parameter declarations, specify blocks, or specparam declarations. All other module items, including
other generate constructs, are allowed in a generate block. Generate constructs provide the ability for param-
eter values to affect the structure of the model. They also allow for modules with repetitive structure to be
described more concisely, and they make recursive module instantiation possible.

There are two kinds of generate constructs: _loops_ and _conditionals_. Loop generate constructs allow a single
generate block to be instantiated into a model multiple times. Conditional generate constructs, which include
if-generate and case-generate constructs, instantiate at most one generate block from a set of alternative gen-
erate blocks. The term _generate scheme_ refers to the method for determining which or how many generate
blocks are instantiated. It includes the conditional expressions, case alternatives, and loop control statements
that appear in a generate construct.

Generate schemes are evaluated during elaboration of the model. Elaboration occurs after parsing the HDL
and before simulation; and it involves expanding module instantiations, computing parameter values, resolv-
ing hierarchical names (see 6.7), establishing net connectivity and in general preparing the model for simula-


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
153
```
tion. Although generate schemes use syntax that is similar to behavioral statements, it is important to
recognize that they do not execute at simulation time. They are evaluated at elaboration time, and the result
is determined before simulation begins. Therefore, all expressions in generate schemes shall be constant
expressions, deterministic at elaboration time. For more details on elaboration, see 6.9.

The elaboration of a generate construct results in zero or more instances of a generate block. An instance of
a generate block is similar in some ways to an instance of a module. It creates a new level of hierarchy. It
brings the objects, behavioral constructs, and module instances within the block into existence. These con-
structs act the same as they would if they were in a module brought into existence with a module instantia-
tion, except that object declarations from the enclosing scope can be referenced directly (see 6.8). Names in
instantiated named generate blocks can be referenced hierarchically as described in 6.7.

The keywords **generate** and **endgenerate** may be used in a module to define a _generate region_. A
generate region is a textual span in the module description where generate constructs may appear. Use of
generate regions is optional. There is no semantic difference in the module when a generate region is used. A
parser may choose to recognize the generate region to produce different error messages for misused generate
construct keywords. Generate regions do not nest, and they may only occur directly within a module. If the
generate keyword is used, it shall be matched by an endgenerate keyword.

The syntax for generate constructs is given in Syntax 6- 8.

module_or_generate_item ::= _// from A.1.4_
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

generate_region ::= _// from A.4.2_
**generate** { module_or_generate_item } **endgenerate**

genvar_declaration ::=
**genvar** list_of_genvar_identifiers **;**

list_of_genvar_identifiers ::=


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
154
```
```
genvar_identifier { , genvar_identifier }
```
analog_loop_generate_statement ::=
**for (** genvar_initialization **;** genvar_expression **;** genvar_iteration **)**
analog_statement

loop_generate_construct ::=
**for (** genvar_initialization **;** genvar_expression **;** genvar_iteration **)**
generate_block

genvar_initialization ::=
genvar_identifier **=** constant_expression

genvar_expression ::=
genvar_primary
| unary_operator { attribute_instance } genvar_primary
| genvar_expression binary_operator { attribute_instance } genvar_expression
| genvar_expression**?** { attribute_instance } genvar_expression **:** genvar_expression

genvar_iteration ::=
genvar_identifier **=** genvar_expression

genvar_primary ::=
constant_primary
| genvar_identifier

conditional_generate_construct ::=
if_generate_construct
| case_generate_construct

if_generate_construct ::=
**if (** constant_expression **)** generate_block_or_null
[ **else** generate_block_or_null ]

case_generate_construct ::=
**case (** constant_expression **)** case_generate_item { case_generate_item } **endcase**

case_generate_item ::=
constant_expression { **,** constant_expression } **:** generate_block_or_null
| **default** [ **:** ] generate_block_or_null

generate_block ::=
module_or_generate_item
| **begin** [ **:** generate_block_identifier ] { module_or_generate_item } **end**

generate_block_or_null ::=
generate_block
| **;**

```
Syntax 6-8—Syntax for generate constructs
```
#### 6.6.1 Loop generate constructs

A loop generate construct permits a generate block to be instantiated multiple times using syntax that is sim-
ilar to a for loop statement. The loop index variable shall be declared in a genvar declaration prior to its use
in a loop generate scheme.

The genvar is used as an integer during elaboration to evaluate the generate loop and create instances of the
generate block, but it does not exist at simulation time. A genvar shall not be referenced anywhere other than
in a loop generate scheme.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
155
```
Both the initialization and iteration assignments in the loop generate scheme shall assign to the same genvar.
The initialization assignment shall not reference the loop index variable on the right-hand side.

Within the generate block of a loop generate construct, there is an implicit localparam declaration. This is an
integer parameter that has the same name and type as the loop index variable, and its value within each
instance of the generate block is the value of the index variable at the time the instance was elaborated. This
parameter can be used anywhere within the generate block that a normal parameter with an integer value can
be used. It can be referenced with a hierarchical name.

Because this implicit localparam has the same name as the genvar, any reference to this name inside the loop
generate block will be a reference to the localparam, not to the genvar. As a consequence, it is not possible to
have two nested loop generate constructs that use the same genvar.

Generate blocks in loop generate constructs can be named or unnamed, and they can consist of only one
item, which need not be surrounded by begin/end keywords. Even if the begin/end keywords are absent, it is
still a generate block, which, like all generate blocks, comprises a separate scope and a new level of hierar-
chy when it is instantiated.

If the generate block is named, it is a declaration of an array of generate block instances. The index values in
this array are the values assumed by the genvar during elaboration. This can be a sparse array because the
genvar values do not have to form a contiguous range of integers. The array is considered to be declared
even if the loop generate scheme resulted in no instances of the generate block. If the generate block is not
named, the declarations within it cannot be referenced using hierarchical names other than from within the
hierarchy instantiated by the generate block itself.

It shall be an error if the name of a generate block instance array conflicts with any other declaration, includ-
ing any other generate block instance array. It shall be an error if the loop generate scheme does not termi-
nate. It shall be an error if a genvar value is repeated during the evaluation of the loop generate scheme. It
shall be an error if any bit of the genvar is set to x or z during the evaluation of the loop generate scheme.

For example, this module implements a continuously running (unclocked) analog-to-digital converter.

```
module adc (in, out);
parameter bits=8, fullscale=1.0, dly=0.0, ttime=10n;
input in;
output [0:bits-1] out;
electrical in;
electrical [0:bits-1] out;
```
```
real sample, thresh;
genvar i;
```
```
analog begin
thresh = fullscale/2.0;
sample = V(in);
end
```
```
generate
for (i=bits-1; i>=0; i=i-1)
analog begin
V(out[i]) <+ transition (sample > thresh, dly, ttime);
if (sample > thresh) sample = sample - thresh;
sample = 2.0*sample;
end
endgenerate
endmodule
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
156
```
The model in the next two examples are parametrized modules that use a loop to generate SPICE primitive
instances. The second of these examples makes a net declaration inside of the generate loop to generate the
nodes needed to connect the analog primitives for each iteration of the loop.

This module implements an interconnect line constructed from RC sections.

```
module rcline (n1, n2);
inout n1, n2;
electrical n1, n2, gnd;
ground gnd;
parameter integer N = 10 from (0: inf );
electrical [0:N] n;
parameter Cap = 1p, Res = 1k;
localparam Csec = Cap/N, Rsec = Res/N;
```
```
genvar i;
```
```
// "generate" and "endgenerate" keywords are not required.
for (i=0; i <N; i=i+1) begin
resistor #(.r(Rsec)) R(n[i], n[i+1]);
capacitor #(.c(Csec)) C(n[i+1], gnd);
end
```
```
analog begin
V(n1, n[0]) <+ 0.0;
V(n2, n[N]) <+ 0.0;
end
endmodule
```
This module also implements an interconnect line constructed from RC sections, but the sections are now
symmetric. Additionally, the capacitor is now implemented by an **analog** block.

```
module rcline2 (n1, n2);
inout n1, n2;
electrical n1, n2, gnd;
ground gnd;
parameter integer N = 10 from (0: inf );
electrical [0:N] n;
parameter Cap = 1p, Res = 1k;
localparam Csec = Cap/N, Rsec = Res/(2*N);
```
```
genvar i;
```
```
for (i=0; i <N; i=i+1) begin : section
electrical n_int;
```
```
resistor #(.r(Rsec)) R1(n[i], n_int);
resistor #(.r(Rsec)) R2(n_int, n[i+1]);
analog
I(n_int, gnd) <+ Csec * ddt(V(n_int));
end
```
```
analog begin
V(n1, n[0]) <+ 0.0;
V(n2, n[N]) <+ 0.0;
end
endmodule
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
157
```
In the above example the block inside the generate loop is a named block. For each block instance created
by the generate loop, the generate block identifier for the loop is indexed by adding the "[genvar value]" to
the end of the generate block identifier. These names can be used in hierarchical path names (see 6.7).

#### 6.6.2 Conditional generate constructs

The conditional generate constructs, _if-generate_ and _case-generate_ , select at most one generate block from a
set of alternative generate blocks based on constant expressions evaluated during elaboration. The selected
generate block, if any, is instantiated into the model.

Generate blocks in conditional generate constructs can be named or unnamed, and they may consist of only
one item, which need not be surrounded by **begin** - **end** keywords. Even if the **begin** - **end** keywords are
absent, it is still a generate block, which, like all generate blocks, comprises a separate scope and a new level
of hierarchy when it is instantiated.

Because at most one of the alternative generate blocks is instantiated, it is permissible for there to be more
than one block with the same name within a single conditional generate construct. It is not permissible for
any of the named generate blocks to have the same name as generate blocks in any other conditional or loop
generate construct in the same scope, even if the blocks with the same name are not selected for instantia-
tion. It is not permissible for any of the named generate blocks to have the same name as any other declara-
tion in the same scope, even if that block is not selected for instantiation.

If the generate block selected for instantiation is named, then this name declares a generate block instance
and is the name for the scope it creates. Normal rules for hierarchical naming apply. If the generate block
selected for instantiation is not named, it still creates a scope; but the declarations within it cannot be refer-
enced using hierarchical names other than from within the hierarchy instantiated by the generate block itself.

If a generate block in a conditional generate construct consists of only one item that is itself a conditional
generate construct and if that item is not surrounded by begin/end keywords, then this generate block is not
treated as a separate scope. The generate construct within this block is said to be directly nested. The gener-
ate blocks of the directly nested construct are treated as if they belong to the outer construct. Therefore, they
can have the same name as the generate blocks of the outer construct, and they cannot have the same name
as any declaration in the scope enclosing the outer construct (including other generate blocks in other gener-
ate constructs in that scope). This allows complex conditional generate schemes to be expressed without cre-
ating unnecessary levels of generate block hierarchy.

The most common use of this would be to create an if-else-if generate scheme with any number of else-if
clauses, all of which can have generate blocks with the same name because only one will be selected for
instantiation. It is permissible to combine if-generate and case-generate constructs in the same complex gen-
erate scheme. Direct nesting applies only to conditional generate constructs nested in conditional generate
constructs. It does not apply in any way to loop generate constructs.

The following module implements a non-linear resistor that internally uses the SPICE resistor primitive if
the non-linear coefficients are not given or a short if the resistance value is 0.

```
module nlres ( inout electrical a, inout electrical b);
parameter real res = 1k from (0: inf );
parameter real coeff1 = 0.0;
```
```
generate
if ( $param_given (coeff1) && coeff1 != 0.0)
analog
V(a, b) <+ res * (1.0 + coeff1 * I(a, b)) * I(a, b);
else if (res == 0.0)
analog
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
158
```
```
V(a, b) <+ 0.0;
else
resistor #(.r(res)) R1(a, b);
endgenerate
endmodule
```
For compact modeling of semiconductor devices where the delay time of signals through the device needs to
be taken into account (non-quasi-static models) introduction of extra nodes and branches can be controlled
through a module parameter.

```
module nmosfet (d, g, s, b);
inout electrical d, g, s, b;
parameter integer nqsMod = 0 from [0:1];
```
```
// "generate" and "endgenerate" keywords are not required.
if (nqsMod) begin : nqs
electrical GP;
electrical BP;
electrical BI;
electrical BS;
electrical BD;
...
end
endmodule
```
Conditional generate constructs make it possible for a module to contain an instantiation of itself. The same
can be said of loop generate constructs, but it is more easily done with conditional generates. With proper
use of parameters, the resulting recursion can be made to terminate, resulting in a legitimate model hierar-
chy. Because of the rules for determining top-level modules, a module containing an instantiation of itself
will not be a top-level module.

The following example is a continuously running (unclocked) pipeline analog-to-digital converter that
instantiates a lower resolution version of itself as part of its structure.

```
module pipeline_adc (in, out);
parameter bits=8, fullscale=1.0;
inout in;
inout [0:bits-1] out;
electrical in;
electrical [0:bits-1] out;
```
```
comparator #(.ref(fullscale/2)) cmp (in, out[bits-1]);
```
```
generate
if (bits > 1) begin
electrical n1, n2;
subtractor #(.level(fullscale)) sub (in, out[bits-1], n1);
amp2x amp (n1, n2);
pipeline_adc #(.bits(bits-1)) section (n2, out[0:bits-2]);
end
endgenerate
endmodule
```
Some of the functionality of conditional generate constructs can also be achieved using paramset overload-
ing, see 6.4.2. For instance, selection of a particular module based on the value or presence of a parameter
can also be handled by constructing appropriate paramsets.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
159
```
**6.6.2.1 Dynamic parameters**

A special case exists for dc sweep simulations: a series of operating point analyses where one or more
parameters of the circuit change value between each analysis, see also 4.6.1 on the Analysis function. Digital
simulations do not normally allow a parameter to vary during simulation; in analog simulation it is quite
common to sweep a parameter during simulation to get information on how the parameter values influence
the circuit behavior and hence the simulation results.

In connection with the conditional generate construct, an implementation may choose to limit the possible
parameters to sweep to those that do not influence the structure of the circuit.

#### 6.6.3 External names for unnamed generate blocks.................................................................

Although an unnamed generate block has no name that can be used in a hierarchical name, it needs to have a
name by which external interfaces can refer to it. A name will be assigned for this purpose to each unnamed
generate block as described in the next paragraph.

Each generate construct in a given scope is assigned a number. The number will be 1 for the construct that
appears textually first in that scope and will increase by 1 for each subsequent generate construct in that
scope. All unnamed generate blocks will be given the name "genblk<n>" where <n> is the number assigned
to its enclosing generate construct. If such a name would conflict with an explicitly declared name, then
leading zeroes are added in front of the number until the name does not conflict.

Each generate construct is assigned its number as described in the previous paragraph even if it does not
contain any unnamed generate bocks.

Example:

```
module top ();
parameter genblk2 = 0;
genvar i;
```
```
// The following generate block is implicitly named genblk1
if (genblk2) electrical a; // top.genblk1.a
else electrical b; // top.genblk1.b
```
```
// The following generate block is implicitly named genblk02
// as genblk2 is already a declared identifier
if (genblk2) electrical a; // top.genblk02.a
else electrical b; // top.genblk02.b
```
```
// The following generate block would have been named genblk3
// but is explicitly named g1
for (i = 0; i < 1; i = i + 1) begin : g1 // block name
// The following generate block is implicitly named genblk1
// as the first nested scope inside of g1
if (1) electrical a; // top.g1[0].genblk1.a
end
```
```
// The following generate block is implicitly named genblk4 since
// it belongs to the fourth generate construct in scope "top".
// The previous generate block would have been named genblk3
// if it had not been explicitly named g1
for (i = 0; i < 1; i = i + 1)
// The following generate block is implicitly named genblk1
// as the first nested generate block in genblk4
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
160
```
```
if (1) electrical a; // top.genblk4[0].genblk1.a
```
```
// The following generate block is implicitly named genblk5
if (1) electrical a; // top.genblk5.a
```
```
endmodule
```
### 6.7 Hierarchical names

Every identifier in Verilog-AMS HDL has a unique _hierarchical path name_. The hierarchy of modules and
the definition of items such as named blocks within the modules define these names. The hierarchy of names
can be viewed as a tree structure, where each module instance or a named begin-end block defines a new
hierarchical level, or as a scope (of a particular branch of the tree).

At the top of the name hierarchy are the names of modules where no instances have been created. This is the
_root_ of the hierarchy. Inside any module, each module instance and named begin-end block define a new
branch of the hierarchy. Named blocks within named blocks also create new branches.

Each node in the hierarchical name tree is treated as a separate scope with respect to identifiers. A particular
identifier can be declared only once in any scope.

Any named object can be referenced uniquely in its full form by concatenating the names of the module
instance or named blocks that contain it. The period character (**.** ) is used to separate names in the hierarchy.
The complete path name to any object starts at a top-level module. This path name can be used from any
level in the description. The first name in a path name can also be the top of a hierarchy which starts at the
level where the path is being used.

The syntax for hierarchical path names is given in Syntax 6- 9.

hierarchical_identifier ::= [ **$root.** ] { identifier [ **[** constant_expression **]** ]**.** } identifier

```
Syntax 6-9—Syntax for hierarchical path name
```
Hierarchical names consist of instance names separated by periods, where an instance name can be an array
element. The instance name **$root** refers to the top of the instantiated design and is used to unambigu-
ously gain access to the top of the design.

```
$root .mymodule.u1 // absolute name
u1.struct1.field1 // u1 must be visible locally or above, including globally
adder1[5].sum
```
Examples:

```
module samplehold (in, cntrl, out);
input in, cntrl;
output out;
electrical in, cntrl, out;
electrical store, sample;
parameter real vthresh = 0.0;
parameter real cap = 10e-9;
amp op1 (in, sample, sample);
amp op2 (store, out, out);
```
```
analog begin
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
161
```
```
I(store) <+ cap * ddt (V(store));
if (V(cntrl) > vthresh)
V(store, sample) <+ 0;
else
I(store, sample) <+ 0;
end
endmodule
```
```
module amp(inp, inm, out);
input inp, inm;
output out;
electrical inp, inm, out;
parameter real gain=1e5;
```
```
analog begin
V(out) <+ gain*V(inp,inm);
end
endmodule
```
Figure 6- 2 illustrates the hierarchy implicit in the example code.

```
Figure 6-2: Hierarchy in a model
```
Figure 6- 3 is a list of the hierarchical forms of the names of all the objects defined in the example code.

```
Figure 6-3: Hierarchical path names in a design
```
#### 6.7.1 Usage of hierarchical references

The following usage rules and semantic restrictions shall be applied to analog identifiers referred hierarchi-
cally using an _out-of-module reference_ (OOMR) in a mixed signal module:

```
— Potential and flow access for named and unnamed branches (including port branches) can be done
hierarchically.
— Hierarchical reference of an implicit net is allowed when the referenced net is first coerced to a spe-
cific discipline.
— Access of parameters can be done hierarchically. However, parameter declaration statements shall
not make out-of-module references (e.g., for setting default values).
— Analog user-defined functions can be accessed hierarchically.
— It shall be an error to access analog variables hierarchically.
— Potential and flow contributions to named and unnamed branches can be done hierarchically.
```
### op1 op2

### samplehold

```
samplehold in, cntrl, out, sample, store, vthresh, cap
op1 op1.inp, op1.inm, op1.out, op1.gain
op2 op2.inp, op2.inm, op2.out, op2.gain
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
162
```
```
— It shall be an error to assign to an analog variable using hierarchical notation.
```
Hierarchical references to analog branches and nets can be done in both analog as well as digital blocks.

Verilog-AMS HDL follows the rules for hierarchical upward referencing as described in 12.6 of IEEE Std
1364 Verilog with the addition that the _scope_name_ shall be restricted to a _hierarchical_inst_identifier_.

### 6.8 Scope rules

The following elements define a new scope in Verilog-AMS HDL:

```
— modules
— tasks
— named blocks
— functions
— generate blocks
— analog functions
```
An identifier shall be used to declare only one item within a scope. This rule means it is illegal to declare
two or more variables which have the same name, or to name a task the same as a variable within the same
module, or to give an instance the same name as the name of the net connected to its output. For generate
blocks, this rule applies regardless of whether the generate block is instantiated. An exception to this is made
for generate blocks in a conditional generate construct. See 6.6.3 for a discussion of naming conditional gen-
erate blocks.

If an identifier is referenced directly (without a hierarchical path) within a named block, or generate block it
shall be declared either within the named block, or generate block locally or within a module, or within a
named block, or generate block that is higher in the same branch of the name tree containing the named
block, or generate block. If it is declared locally, the local item shall be used; if not, the search shall continue
upward until an item by that name is found or until a module boundary is encountered. If the item is a vari-
able, it shall stop at a module boundary; if the item is a named block, or generate block, it continues to
search higher level modules until found.

Because of the upward searching process, path names which are not strictly on a downward path can be
used.

### 6.9 Elaboration

Elaboration is the process that occurs between parsing and simulation. It binds modules to module instances,
builds the model hierarchy, computes parameter values, selects paramsets, resolves hierarchical names,
establishes net connectivity, resolves disciplines and inserts connect modules, and prepares all of this for
simulation. With the addition of generate constructs, the order in which these tasks occur becomes signifi-
cant.

#### 6.9.1 Concatenation of analog blocks

A module definition may have multiple **analog** blocks. The simulator shall internally combine the multiple
**analog** blocks into a single **analog** block in the order that the **analog** blocks appear in the module
description. In other words, the **analog** blocks shall execute in the order that they are specified in the mod-
ule.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
163
```
Concatenation of the **analog** blocks occurs after all generate constructs have been evaluated, i.e. after the
loop generate constructs have been unrolled, and after the conditional generate constructs have been
selected. If an **analog** block appears in a loop generate statement, then the order in which the loop is
unrolled during elaboration determines the order in which the **analog** blocks are concatenated to the even-
tual single **analog** block after elaboration.

#### 6.9.2 Elaboration and paramsets

If a generate construct contains an instantiation of an overloaded paramset, then the paramset selection is
performed after the generate construct has been evaluated. The evaluation of the generate construct may
influence the values and connections of the paramset instance, and hence the selection of matching paramset
and module.

#### 6.9.3 Elaboration and connectmodules

Automatic insertion of connect modules is a post-elaboration operation, as first the disciplines of the various
nets needs to be resolved. This is described in detail in 7.8.

Discipline resolution can only occur after elaboration of the generate constructs once the connections of all
nets has been resolved. It should also occur after the paramset selection as the choice for a particular module
instantiation may affect the disciplines of the connected nets.

#### 6.9.4 Order of elaboration

Because of generate constructs and paramsets, the model hierarchy can depend on parameter values.
Because defparam statements can alter parameter values from almost anywhere in the hierarchy, the result
of elaboration can be ambiguous when generate constructs are involved. The final model hierarchy can
depend on the order in which defparams and generate constructs are evaluated.

The use of paramsets cannot introduce ambiguity as no defparam inside the hierarchy below a paramset
instantiation is allowed, see 6.3.1 and 6.4.

The following algorithm defines an order that produces the correct hierarchy:

```
1) A list of starting points is initialized with the list of top-level modules.
2) The hierarchy below each starting point is expanded as much as possible without elaborating gener-
ate constructs. All parameters encountered during this expansion are given their final values by
applying initial values, parameter overrides, defparam statements, and paramset selections.
3) In other words, any defparam statement whose target can be resolved within the hierarchy elabo-
rated so far must have its target resolved and its value applied. defparam statements whose target
cannot be resolved are deferred until the next iteration of this step. Because no defparam inside the
hierarchy below a generate construct is allowed to refer to a parameter outside the generate con-
struct, it is possible for parameters to get their final values before going to step 3).
4) Each generate construct encountered in step 2) is revisited, and the generate scheme is evaluated.
The resulting generate block instantiations make up the new list of starting points. If the new list of
starting points is not empty, go to step 2).
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
164
```
