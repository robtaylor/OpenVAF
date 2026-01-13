## 10. Compiler directives

### 10.1 Overview......................................................................................................................................

All Verilog-AMS HDL compiler directives are preceded by the ( **` )** character. This character is called
accent grave (ASCII 0x60). It is different from the character ( **’ )** , which is the apostrophe character
(ASCII 0x27). The scope of compiler directives extends from the point where it is processed, across all files
processed, to the point where another compiler directive supersedes it or the processing completes.

The following compiler directives are supported:

```
`__FILE__ [10.7]
`__LINE__ [10.7]
`begin_keywords [10.6]
`celldefine [IEEE Std 1364 Verilog]
`default_discipline [10.2]
`default_nettype [IEEE Std 1364 Verilog]
`default_transition [10.3]
`define [10.4]
`else [IEEE Std 1364 Verilog]
`elsif [IEEE Std 1364 Verilog]
`end_keywords [10.6]
`endcelldefine [IEEE Std 1364 Verilog]
`endif [IEEE Std 1364 Verilog]
`ifdef [IEEE Std 1364 Verilog]
`ifndef [IEEE Std 1364 Verilog]
`include [IEEE Std 1364 Verilog]
`line [IEEE Std 1364 Verilog]
`nounconnected_drive [IEEE Std 1364 Verilog]
`pragma [IEEE Std 1364 Verilog]
`resetall [IEEE Std 1364 Verilog]
`timescale [IEEE Std 1364 Verilog]
`unconnected_drive [IEEE Std 1364 Verilog]
`undef [10.4]
```
### 10.2 `default_discipline........................................................................................................................

The default discipline is applied by discipline resolution (see 7.4 and Annex F)to all discrete signals without
a discipline declaration that appear in the text stream following the use of the **`default_discipline**
directive, until either the end of the text stream or another **`default_discipline** directive with the
qualifier (if applicable) is found in the subsequent text, even across source file boundaries. Therefore, more
than one **`default_discipline** directive can be in force simultaneously, provided each differs in
qualifier.

In addition to **`resetall** , if this directive is used without a discipline name, discipline resolution will not
use a default discipline for nets declared after this directive is encountered in the text stream. Syntax 10- 1
shows the syntax for this directive.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
269
```
default_discipline_directive ::=
**`default_discipline** [discipline_identifier [ qualifier ] ]

qualifier ::=
**integer** | **real** | **reg** | **wreal** | **wire** | **tri** | **wand** | **triand**
| **wor** | **trior** | **trireg** | **tri0** | **tri1** | **supply0** | **supply1**

```
Syntax 10-1—Syntax for the default discipline compiler directive
```
Example:

```
`default_discipline ddiscrete
module behavnand(in1, in2, out);
input in1, in2;
output out;
reg out;
always begin
out = ~(in1 && in2);
end
endmodule
```
This example illustrates the usage of the **`default_discipline** directive. The nets in1, in2, and out
all have discipline ddiscrete by default.

There is a precedence of compiler directives; the more specific directives have higher precedence over gen-
eral directives.

### 10.3 `default_transition

The scope of this directive is similar to the scope of the **`define** compiler directive although it can be used
only outside of module definitions. This directive specifies the default value for rise and fall time for the
transition filter (see 4.5.8). There are no scope restrictions for this directive. The syntax for this directive is
shown in Syntax 10- 2.

default_transition_compiler_directive ::=
**`default_transition** transition_time

transition_time ::=
constant_expression

```
Syntax 10-2—Syntax for default transition compiler directive
```
_transition_time_ is a real value.

For all transition filters which follow this directive and do not have rise time and fall time arguments speci-
fied, _transition_time_ is used as the default rise and fall time values. If another **`default_transition**
directive is encountered in the subsequent source description, the transition filters following the newly
encountered directive derive their default rise and fall times from the transition time value of the newly
encountered directive. In other words, the default rise and fall times for a transition filter are derived from
the _transition_time_ value of the directive which immediately precedes the transition filter.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
270
```
If a **`default_transition** directive is not used in the description, _transition_time_ is controlled by the
simulator.

### 10.4 `define and `undef

The **‘define** and **‘undef** compiler directives are described in IEEE Std 1364 Verilog.

To avoid conflicts with predefined Verilog-AMS macros (10.5), the **‘define** compiler directive’s macro
text shall not begin with __VAMS_. The **‘undef** compiler directive shall have no effect on predefined Ver-
ilog-AMS macros; the simulator may issue a warning for an attempt to undefine one of these macros.

The syntax for text macro definitions is given in Syntax 10- 3

text_macro_definition ::=
**‘define** text_macro_name macro_text

text_macro_name ::=
text_macro_identifier [ **(** list_of_formal_arguments **)** ]

list_of_formal_arguments ::=
formal_argument_identifier { **,** formal_argument_identifier }

formal_argument_identifier ::=
simple_identifier

text_macro_identifier ::=
identifier

```
Syntax 10-3—Syntax for text macro definition (not in Annex A)
```
### 10.5 Predefined macros........................................................................................................................

Verilog-AMS HDL supports a predefined macro to allow modules to be written that work with both IEEE
Std 1364 Verilog and Verilog-AMS HDL.The predefined macro is called **__VAMS_ENABLE__**.

This macro shall always be defined during the parsing of Verilog-AMS source text. Its purpose is to support
the creation of modules which are both legal Verilog and Verilog-AMS. The Verilog-AMS features of such
modules are made visible only when the **__VAMS_ENABLE__** macro has previously been defined.

Example:

```
module not_gate(in, out);
input in;
output out;
reg out;
`ifdef __VAMS_ENABLE__
parameter integer del = 1 from [1:100];
`else
parameter del = 1;
`endif
always @ in
out = #del !in;
endmodule
```
Verilog-AMS HDL version 2.2 introduced a number of extensions to support compact modeling. A pre-
defined macro allows modules to add functionality if these extensions are supported, or to generate warnings


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
271
```
or errors if they are not. This predefined macro is called **__VAMS_COMPACT_MODELING__** and shall be
defined during the parsing of Verilog-AMS source text if and only if all the compact modeling extensions
are supported by the simulator.

The following example computes noise of a nonlinear resistor only if the extensions, specifically ddx, are
supported.

```
module nonlin_res(a, b);
input a, b;
electrical a, b;
parameter real rnom = 1;
parameter real vc1 = 0;
real reff, iab;
analog begin
iab = V(a,b) / (rnom * (1.0 + vc1 * V(a,b)));
I(a,b) <+ iab;
`ifdef __VAMS_COMPACT_MODELING__
reff = ddx( iab, V(a) );
I(a,b) <+ white_noise( 4.0*‘P_K* $temperature *reff, "thermal");
`else
if (analysis( "noise" ))
$strobe( "Noise not computed." );
`endif
end
endmodule
```
Verilog-AMS simulators shall also provide a predefined macro so that the module can conditionally include
(or exclude) portions of the source text specific to a particular simulator. This macro shall be documented in
the Verilog-AMS section of the simulator manual.

### 10.6 `begin_keywords and `end_keywords

Verilog-AMS HDL extends the **`begin_keywords** and **`end_keywords** compiler directives from
IEEE Std 1364 Verilog as well existing extensions made in previous versions of this standard by adding a
"VAMS-2023" version specifier.

The version_specifier specifies the valid set of reserved keywords in effect when a design unit is parsed by
an implementation. The **`begin_keywords** and **`end_keywords** directives can only be specified out-
side of a design element (module, primitive, configuration, paramset, connectrules or connectmodule). The
**`begin_keywords** directive affects all source code that follows the directive, even across source code
file boundaries, until the matching **`end_keywords** directive is encountered.

The version_specifier, "VAMS-2023" specifies that only the identifiers listed as reserved keywords in the
Verilog-AMS HDL are considered to be reserved words. These identifiers are listed in Annex B.

The **`begin_keywords** and **`end_keywords** directives only specify the set of identifiers that are
reserved as keywords. The directives do not affect the semantics, tokens, and other aspects of the Verilog-
AMS language.

The version specifiers "1364-1995", "1364-2001", "1364-2005" and "VAMS-2.3" must also be sup-
ported. "1364-1995" represents the keywords for IEEE Std 1364-1995. "1364-2001" represents the key-
words for IEEE Std 1364-2001. "1364-2005" represents the keywords for IEEE Std 1364-2005. "VAMS-
2.3" represents the keywords for Verilog-AMS 2.3 HDL.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
272
```
In the example below, it is assumed that the definition of module m1 does not have a **`begin_keywords**
directive specified prior to the module definition. Without this directive, the set of reserved keywords in
effect for this module shall be the implementation’s default set of reserved keywords.

```
module m1; // module definition with no `begin_keywords directive
...
endmodule
```
The following example specifies a **`begin_keywords** "1364-2005" directive. The source code within
the module uses the identifier sin as a port name. The **`begin_keywords** directive would be necessary
in this example if an implementation uses Verilog-AMS as its default set of keywords because **sin** is a
reserved keyword in Verilog-AMS but not in 1364-2005. Specifying the "1364-1995" or "1364-2001"
Verilog keyword lists would also work with this example.

```
`begin_keywords "1364-2005" // use IEEE Std 1364-2005 Verilog keywords
module m2 (sin ...);
input sin; // OK since sin is not a keyword in 1364-2005
...
endmodule
`end_keywords
```
The next example is the same code as the previous example, except that it explicitly specifies that the Ver-
ilog-AMS keywords should be used. This example shall result in an error because **sin** is reserved as a key-
word in this standard.

```
`begin_keywords "VAMS-2023" // use Verilog-AMS LRM 2023 keywords
module m2 (sin, ... ); // ERROR: "sin" is a keyword in Verilog-AMS
input sin;
...
endmodule
`end_keywords
```
The following example uses several Verilog-AMS constructs, and designates that the Verilog-AMS version
2023 set of keywords should be used. Note that the word “logic” is not a keyword in Verilog-AMS 2023,
whereas it is a keyword in the IEEE Std 1800 SystemVerilog.

```
`begin_keywords "VAMS-2023"
discipline \logic;
domain discrete;
enddiscipline
module a2d(dnet, anet);
input dnet;
wire dnet;
logic dnet;
output anet;
electrical anet;
real avar;
analog begin
if (dnet === 1'b1)
avar = 5;
else if (dnet === 1'bx)
avar = avar; // hold value
else if (dnet === 1'b0)
avar = 0;
else if (dnet === 1'bz)
avar = 2.5; // high impedance - float value
V(anet) <+ avar;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
273
```
```
end
endmodule
```
**`end_keywords**

### 10.7 `__FILE__ and `__LINE__..........................................................................................................

**`__FILE__** expands to the name of the current input file, in the form of a string literal. This is the path by
which a tool opened the file, not the short name specified in `include or as a tool’s input file name argument.
The format of this path name may be implementation dependent.

**`__LINE__** expands to the current input line number, in the form of a simple decimal number.

**`__FILE__** and **`__LINE__** are useful in generating an error message to report a problem; the message
can state the source line at which the problem was detected.

For example:

```
$display ("Internal error: null handle at %s, line %d.",
`__FILE__, `__LINE__);
```
An **`include** directive changes the expansions of **`__FILE__** and **`__LINE__** to correspond to the
included file. At the end of that file, when processing resumes on the input file that contained the
**`include** directive, the expansions of **`__FILE__** and **`__LINE__** revert to the values they had before
the `include (but **`__LINE__** is then incremented by one as processing moves to the line after the
**`include** ).

A **`line** directive (as defined in IEEE Std 1364 Verilog) changes **`__LINE__** and may change
**`__FILE__** as well.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
274
```
