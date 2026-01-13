## Annex G (informative) Change history

This annex lists the changes made to the document for each revision.

### G.1 Changes from previous LRM versions

This subclause highlights some of the key differences between versions of the Verilog-AMS HDL reference
manual. The syntax and semantics of this document supersede any syntax, semantics, or interpretations of
previous revisions.

```
Table G.1—Changes from v1.0 to v2.0 syntax
```
```
Feature OVI Verilog-A v1.0 OVI Verilog-AMS v2.0 Change type
```
```
Analog time $realtime $abstime new
Ceiling operator N/A ceil (expr) new
Floor operator N/A floor (expr) new
Circular integrator N/A idtmod (expr) new
Expression looping N/A genvar new
Distribution functions $dist_ functions ()
Integer based functions
```
```
$rdist_ functions ()
Real value equivalents to
$dist_ functions ()
```
```
new
```
```
Empty discipline predefined as type wire type not defined default definition
Implicit nodes ‘default_nodetype disci-
pline_identifier
default: wire
```
```
default type: empty disci-
pline, no domain type
```
```
default definition
```
```
initial_step default = TRAN default = ALL default definition
final_step default = TRAN default = ALL default definition
Analog ground no definition now a declaration state-
ment
```
```
definition
```
```
$realtime $realtime :timescale =1
sec
```
```
$realtime :timescale=
’timescale def=1n,
see $abstime
```
```
definition
```
```
Array setting aa[0:1] = {2.1 = (1), 4.5 =
(2)
```
```
aa[0:1] = {2.1,4.5} syntax
```
```
Discontinuity function discontinuity ( x ) $discontinuity (x) syntax
Limiting exponential func-
tion
```
```
$limexp( expression ) limexp ( expression ) syntax
```
```
Port branch access I(a,a) I(< a >) syntax
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
414
```
Timestep control (maxi-
mum stepsize)

```
bound_step ( const_ex-
pression )
```
```
$bound_step( expr ) syntax
```
Continuous waveform
delay

```
delay() absdelay() syntax
```
User-defined analog func-
tions

```
function analog function syntax
```
Discipline domain N/A, assumed continuous now continuous(default)
and discrete

```
Extension
```
k scalar (10^3 ) N/A, only “K” supported now supported Extension

Module keyword **module module** or **macro-
module**

```
Extension
```
Modulus operator integers only now supports integer
and reals

```
Extension
```
Time tolerance on timer
functions

```
N/A supports additional time
tolerance argument for
timer ()
```
```
Extension
```
Time tolerance on transi-
tion filter

```
N/A supports additional time
tolerance argument for
transition ()
```
```
Extension
```
‘default_nodetype ‘ **default_node-
type**

```
‘ default_disci-
pline
```
```
Obsolete
```
Forever statement **forever** N/A Obsolete

Generate statement **generate** N/A Obsolete

Null statement ; Limited to case, condi-
tional, and event state-
ments (see syntax)

```
Obsolete
```
```
Table G.2—Changes from v2.0 to v2.1
```
```
Item Description/Issue Clause
```
1 Clarification on when range checking for parameters is done. Range check will
be done only on the final value of the parameter for that instance.

```
3.4.2
```
2 Not to use “max” and use “maxval” instead since **max** is a keyword 3.6.1.1, 3.6.2.6

3 Support of user-defined attributes to disciplines similar to natures has been
added. This would be a useful way to pass information to other tools reading
the Verilog-AMS netlist

```
3.6.2, 3.6.1.3
```
4 LRM specifies TRI and WIRE as aliases. The existing AMS LRM forces nets
with wiretypes other than wire or tri to become digital, but in many cases these
are really interconnect also. If they are tied to behavioral code they will
become digital but if they are interconnected, we should not force them until
after discipline resolution. This is needed if you have configs where the blocks
connected to the net can change between analog and digital. If we force these
nets to be digital we force unneeded CMs when blocks are switched to analog.

```
3.6.2.4, 3.7
```
```
Table G.1—Changes from v1.0 to v2.0 syntax (continued)
```
```
Feature OVI Verilog-A v1.0 OVI Verilog-AMS v2.0 Change type
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
415
```
5 Setting an initial value on net as part of the net declaration. 3.6.3, Syntax 3- 6 ,
3.6.3.2

6 Initial value of **wreal** to be set to 0.0 if the value has not been determined at
t = 0.

```
3.7
```
7 Clarification on the usage of **`default_discipline** and default disci-
pline for analog and digital primitives. Analog primitives will have default dis-
cipline as electrical, whereas digital primitives shall use the
**`default_discipline** declaration.
**`default_discipline** explanation moved to the section along with
other compiler directives and clarification of impact on **‘reset_all** on
this. The usage of word ‘scope’ is clarified to be used as the scope of the appli-
cation of the compiler directive, and not as a scope argument.

```
3.8, 3.9, 3.10, 10.2, 10.3
```
8 Reference to derived disciplines to be removed as current BNF does not sup-
port the syntax

```
3.1 1
```
9 Reworked discipline and nature compatibility rules for better clarity. 3.1 1

10 Removed the reference to neutral discipline since wire can be used in the same
context.

```
3.1 1
```
11 **absdelay** instead of **delay** 4.5.14

12 Array declaration wrongly specified before the variable identifier. For vari-
ables, array specification is written after the name of the variable.

```
3.2
```
13 **@(final_step)** without arguments should not have parenthesis 5.10.2, Table 5- 1

15 **@(final_step)** for DCOP should be 1 5.10.2, Table 5- 1

16 Examples to be fixed to use assign for **wreal** and use **wreal** in instantia-
tion, and also add a top level block for example in 7.3.3, and the testbench use
**wreal**.

```
6.5.3, 3.7
```
17 Clarification on the port bound semantics in explaining the hierarchical struc-
ture for a port with respect to vpiLoConn and vpiHiConn and clarification on
driver and receiver segregation

```
7.2.3
```
18 Figure should have NetC.c_out instead of NetC.b_out 7.2.3

19 Mixed-signal module examples to use case syntax with X & Z instead of “==”
for value comparison

```
7.2.3
```
20 Clarification on accessing discrete nets and variables and X & Z bits in the
analog context.

```
7.2.3
```
21 Adding Support for ‘NaN & X’ into Verilog-AMS. Contribution of these val-
ues to a branch would be an error; however, analog variables should be able to
propagate this value. Added a section regarding NaN

```
7.2.3, 7.3.2.1
```
22 The diagram corresponding to the bidir model has been reworked, and the
example module shown for bidir will match the corresponding figure.

```
7.6
```
23 Rework on _connect-resolveto_ syntax section to clarify the rules 7.7.2.1

24 Use **merged** instead of **merge** 7.8.1

25 Support for digital primitive instantiation in an **analog** block. Port names are
created for the ports of the digital primitives, and these digital port names can-
not be used in child instantiations.

```
7.8.5.1
```
```
Table G.2—Changes from v2.0 to v2.1 (continued)
```
```
Item Description/Issue Clause
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
416
```
26 Net resolution function has been removed and replaced with ‘Receiver Net
Resolution’. Reintroduced the **assign** statement syntax.

```
7.10.5 (subclause
deleted in v2.3)
```
27 Corrections to the connect module example using the driver access function.
The errors in the example have been corrected to make it syntactically and
semantically correct

```
7.10.6 (subclause
deleted in v2.3)
```
28 The constraints for supplementary drivers and delays ar clearly stated. 7.11 (subclause deleted
in v2.3)

29 Driver Type function: There should be a driver access function for finding type
of driver.
driver_type_function ::= $driver_type(signal_name, signal_index)

```
7.11.4 (subclause
deleted in v2.3),
Annex D
```
30 Clarification on the MS synchronization algorithm: Includes a more detailed
explanation on the analog-digital synchronization mechanism.

```
Clause 8
```
31 Truncation versus Rounding mechanism for converting from analog to digital
times.

```
8.4.3.3
```
32 Spelling mistake on “boltzmann” and “planck” in constants file Annex D

33 Units for charge, angle and other definitions in disciplines.vams have been
changed to adhere to SI standards.

```
Annex D
```
34 Values specified in constants file for charge, light, Boltzmann constant, and so
forth have been changed to adhere to the standard definitions.

```
Annex D
```
```
Table G.3—Changes from v2.1 to v2.2
```
```
Item Description/Issue Clause
```
1 Attributes were added following syntax in 1364-2001. 2.9

2 Output variables were defined. 3.2.1

3 Parameters were extended to include units and descriptions, **localparam** ,
**aliasparam** , and **string** parameters.

```
3.4, 3.4.3, 3.4.5, 3.4.6,
3.4.7, Syntax 6- 1
```
4 Net descriptions allowed by attributes. 3.6.3.1

5 Additional bitwise operators were added. Table 4- 2 , 4.2.9

6 Modifications to the domains of functions. Table 4- 9 , Table 4- 10

7 Changes to the descriptions of access function examples. Table 4- 11

8 Added symbolic derivative operators **ddx** () 4.5.6

9 Added references to limiting algorithms, cross-reference to **$limit()** 4.5.13

10 Added entries for **above()** , **ddx()** , and **$limit()** 4.5.14

11 Clarified dc sweep behavior for **analysis** (), **initial_step** , and
**final_step** ;added section describing dc analysis.

```
4.2.1, 4.5.2, Table 5- 1
```
12 Allow multiple return values for analog functions. 4.7

14 Add above event 5.10.3, 5.10.3.2

```
Table G.2—Changes from v2.0 to v2.1 (continued)
```
```
Item Description/Issue Clause
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
417
```
15 Module descriptions allowed by attributes. 6.2, Syntax 6- 1

16 Allow attributes for module item declarations Syntax 6- 1

17 Add **$param_given()** and **$port_connected()** 6.3.5, 6.5.6, 9.19

18 Added hierarchical system parameters **$mfactor** , **$xposition** ,
**$yposition** , **$angle** , **$hflip** , **$vflip**

```
6.3.6, Syntax 6- 2 ,
Syntax 6- 3 , 9.18,
Annex E.4.1
```
19 Add paramsets 6.4

20 Add **$simparam()** 9.15

21 Add support for Monte-Carlo analysis to **$random** and **$rdist_** func-
tions; clarify descriptions of arguments.

```
9.13
```
22 Add **$debug()** 9.4

23 Add format specifiers %r and $R Table 9- 23

24 Add support for limiting (damped Newton-Raphson) with **$limit()** and
**$discontinuity(-1)**

```
9.17, Annex E.3.4
```
25 Add interpolation function **$table_model()** 9.20

26 Add **__VAMS_COMPACT_MODELING__** 10.5

27 New keywords: **above** , **aliasparam** , **ddx** , **endparamset** ,
**localparam** , **paramset** , **string**

```
Annex B
```
28 Corrected the value of ‘M_TWO_PI, defined Planck’s constant as‘P_H (not
‘P_K, which is Boltzmann’s constant), removed parenthetical value after
‘P_U0

```
D.3
```
```
Table G.4—Changes from v2.2 to v2.3
```
```
Item Description/Issue Clause
```
```
1 Add string data type and applicable operations 3.3^
```
```
2 Add apostrophe before opening { in list of values (to distinguish a list of values
from the concatenation operator)
```
```
3.4.2
```
```
3 Add Verilog function style versions of standard mathematical functions
$ln() , $log10() , $exp() , $sqrt() , $pow() , $floor() and
$ceil()
```
```
Table 4- 14
```
```
4 Add Verilog function style versions of trigonometric and hyperbolic functions
$sin() , $cos() , $tan() , $asin() , $acos() , $atan() ,
$atan2() , $hypot() , $sinh() , $cosh() , $tanh() ,
$asinh() , $acosh() and $atanh()
```
```
Table 4- 15
```
```
5 Specify atan2(0, 0) as equal to 0 Table 4-^15
```
```
6 Disallow V(n1, n1) as legal access function usage Table 4-^16
```
```
7 Add conversion from real to integer 4.2.1.1^
```
```
Table G.3—Changes from v2.1 to v2.2 (continued)
```
```
Item Description/Issue Clause
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
418
```
```
8 More strict definition of time-integral operator 4.5.4
```
```
9 The noise_table() function accepts a file name as argument to read table
data from file.
```
```
4.6.4.3
```
```
10 Define use of locally defined parameters and module-level parameters inside
user-defined analog functions
```
```
4.7
```
```
11 Define semantics of inout arguments for user-defined analog functions 4.7.2.4
```
```
12 Allow a user-defined analog function to be called from within another user-
defined analog function
```
```
4.7.3
```
```
13 Support for the analog initial block 5.2.1^
```
```
14 Detailed restrictions on conditional statements 5.8^
```
```
15 Detailed restrictions on looping statements 5.9^
```
```
16 The cross , above and timer monitored event functions have been
extended with an enable argument
```
```
5.10.3, Syntax 5- 13
```
```
17 Support for null arguments in the cross , above and timer monitored event
functions
```
```
5.10.3, Syntax 5- 13
```
```
18 Support for multiple analog blocks within a single module 6.2
```
```
19 Added extra rule on connected ports for paramset selection 6.3.3
```
```
20 Support for loop generate constructs and conditional generate constructs 6.6.2
```
```
21 Restricted use of out-of-module references (OOMRs) 6.7.1
```
```
22 Extended scope definitions to generate blocks 6.8^
```
```
23 Added elaboration rules for analog and mixed-signal hierarchies 6.9^
```
```
24 Support for discipline incompatibility declaration 7.7.2
```
```
25 Description of mixed-signal DC analysis process 8.4.2
```
```
26 Extended definition of $fopen() 9.5.1^
```
```
27 Support for $fdebug() system task 9.2, 9.5.2^
```
```
28 Support for the $swrite() and $sformat() system tasks 9.5.3
```
```
29 Support for $fatal, $error , $warning , and $info system tasks 9.7.3
```
```
30 Renamed the former $random system task to $arandom^ 9.13.1
```
```
31 Added the $simprobe() system task 9.16
```
```
32 Extended $table_model() system function to support isoline data,
tables with multiple dependent values, and higher-order data interpolation.
```
```
9.20
```
```
33 Support for `begin_keywords and `end_keywords compiler direc-
tives; added "VAMS-2.3" version specifier for keywords compiler directive
```
```
10.6
```
```
34 Support for port declarations in module header A.1.2, Syntax 6-^1
```
```
35 Optional semicolon following the nature identifier in a nature declaration A.1.6, Syntax 3-^4
```
```
Table G.4—Changes from v2.2 to v2.3 (continued)
```
**Item Description/Issue Clause**


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
419
```
```
36 Optional semicolon following the discipline identifier in a discipline declara-
tion
```
```
A.1.7, Syntax 3- 5
```
```
37 Annex C of LRM v2.2 has been split and the section describing the changes
from previous LRM versions has been documented in this Annex
```
```
38 Introduced guard clauses to the driver_access.vams standard definitions D.3
```
```
39 Corrected syntax of port_discipline attribute E.3.2.1
```
```
40 Added name scoping of analog primitives E.3.3
```
```
41 Annex G of version 2.2, Open Issues, removed; this information is now in the
Verilog Mantis data base
```
```
42 The keywords in Annex B.2 and Annex B.3 have been merged into the single
table in Annex B.1
```
```
Table G.5—Changes from v2.3 to v2.3.1
```
**Mantis
Item Description/Issue Clause**

2266 The signal flow discipline for current now uses the flow nature and not poten-
tial

```
D.1
```
2391 Clarified semantics for when a branch is treated as a flow source of value zero
(0)

```
5.4.4, 5.6.1.3
```
2453 Corrected summation formula for the analog filter function **laplace_nd()** 4.5.11.4

2458 Added **$simparam$str** to syntax box Syntax 9-^10

2498 Added in keywords: **wire** , **wor** , **wreal** , **xnor** , **xor** , **zi_nd** , **zi_np** ,
**zi_zd** , and **zi_zp** which were accidentally deleted in LRM v2.3

```
Annex B
```
2535 Corrected definition for multiline strings A.8.8

2536 Corrected examples that were using invalid real numbers 3.6.2.1

2538 Removed redundant _string_parameter_declaration_ and _local_string_parame-
ter_declaration_ syntax items

```
A.1.9
```
2391 Clarified definition of a switch branch 5.6.1, 5.8.1

2581 Clarified restrictions on unnamed branches 3.12

2589 Removed multiple definitions of _net_assignment_ A.2.1.3, A.2.3, A.2.4,
A.8.4

2497 Added in definition of _nature_access_identifier_ syntax item A.9.3

2497 Added in definition of _text_macro_ syntax item Syntax 10-^3

2497 Syntax item _analog_variable_lvalue_ was missing in certain places Syntax 5-^14 , Syntax 7-^3

2497 Mathematical function, **pow()** , was missing from _analog_built_in_func-
tion_name_ syntax item definition

```
A.8.2
```
```
Table G.4—Changes from v2.2 to v2.3 (continued)
```
```
Item Description/Issue Clause
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
420
```
2497 A new syntax item, _analog_or_constant_expression_ , has been created to allow
the use of the analog **analysis()** function as part of the constant condi-
tional expression of an **if-else** statement

```
5.8.1, A.8.3
```
2537 Corrected example where the _parameter_type_ was specified before the
**parameter** keyword

```
5.10.3.1
```
2537 Missing trailing ";" in _analog_function_item_declaration_ for the _input_decla-
ration_ , _output_declaration_ , and _inout_declaration_

```
A.2.6
```
2538 Missing trailing ";" in the _analog_block_item_declaration_ for _parameter_dec-
laration_

```
A.2.8
```
2537 Added in a new syntax item definition, _paramset_constant_expression_ , which
is used as the RHS expresson in the _paramset_statement_

```
6.4, A.1.9
```
```
Table G.6—Changes from v2.3.1 to v2.4
```
**Mantis
Item Description/Issue Clause**

831 Clarified ambiguity in named vector branch indexing 3.12

874 Corrected example to use spice _vsine_ primitive 3.6.2.1

875 Added support for **string** parameters to **$fopen()** 9.5.1

876 Clarified ambiguity regarding vector port range specification 6.5.2.2

1638 Modified standard definition file for physical constants to allow for backward
compatibility

```
D.2
```
1854 Added support for parameter aliases to the hierarchical system parameters 3.4.7, 9.18

2266 Added additional clarification for signal flownodes 1.3.4

2331 Changed rule from _hierarchical_system_parameter_functions_ to _hierarchi-
cal_parameter_system_functions_

```
9.18
```
2792 Corrected table to indicate that **$monitor** is supported in the analog context Table 9-^1

2806 Corrected rule for **last_crossing** to allow the _direction_ argument to be
optional

```
4.5.10
```
2836 Added support for _.module_output_variable_identifier_ to the **paramset** defi-
nition

```
6.4, A.1.9, A.9.3
```
2843 Modified **$monitor** description to specify that input arguments **$abstime**^
and **$realtime** don’t cause it to fire

```
9.4.1
```
2860 Re-formatted _connect_resolution_ rule to remove ambiguity 7.7.2, A.1.8

2921 Clarified $random and $arandom support for when the seed argument is a **reg**^
or **time** variable

```
9.13.1
```
2922 Clarified the formal argument requirements for analog user-defined functions 4.7.1

```
Table G.5—Changes from v2.3 to v2.3.1 (continued)
```
**Mantis
Item Description/Issue Clause**


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
421
```
3343 Corrected table cross reference 9.5.1

3371 Corrected example with incorrect range specification 4.5.8

3435 Allow $sscanf to also accept string parameters and literals 9.5.4.2

3461 Corrected error with incorrect capitalization of the reference to the _ttl_ disci-
pline

```
3.6.1.1, 3.6.2.6
```
3462 Modified description of when short-circuit evaluation occurs 4.2.3

3464 Corrected example for **$simprobe** where the declared nets where separated
by '.' characters and not ','

```
9.16
```
3465 Corrected default value for **parameter**^ slewrate 8.4.3.3

3466 Corrected range definition for **parameter integer**^ dir 5.10.3.1

3527 Removed ambiguities for usage of **$table_model** 9.20

3570 Corrected _net_decl_assignment_ syntax rule 3.6.3, 6.5.2.1, A.2.4

4064 Removed redundant paragraph 6.7

4170 Corrected examples 6.6.1

4193 Clarified behavior for multiple **$bound_step** tasks 9.17.2

4259 Corrected syntax for the random seed argument to allow negative numbers 9.13.1, 9.13.2

4308 Added examples showing how to support multiple power regions in a mixed
signal simulation

```
7.8.6
```
4320 Added explicit default for the _type_string_ argument to the distribution func-
tions

```
9.13.1, 9.13.2
```
4339 Removed several restrictions on the use of out-of-module references 3.12, 5.5.1, 5.5.4,
5.6.8.1, 6.7.1, A.2.1.3,
A.8.9

4348 Specified the minimum data requirements for **$table_model** 9.20

4349 Added support for **$noise_table_log** 4.5.1, 4.6.4, 4.6.4.4,
A.8.2

4350 Specified the behavior of the severity system functions when called from an
**analog initial** block

```
9.7.1, 9.7.2, 9.7.3
```
4355 Corrected missing variable declaration in example 7.3.4

4356 Corrected missing ground declaration in example 3.6.2.1, 6.2.2

4441 Added examples for multidimensional arrays 3.2, 3.3, 3.4, 3.4.8,
4.2.14, 4.5.1, 5.7, A.6.2,
A.8.1, A.8.5

4473 Corrected error in timer description with the _start_time_ argument 5.10.3.3

4484 Specified behavior of **final_step** in conjunction with **$finish** 5.10.2, 9.7.1

4543 Added support for the anaog node alias system functions
**$analog_node_alias()** and **$analog_port_alias()**

```
7.8.6, Table 9- 17 , 9.20
```
```
Table G.6—Changes from v2.3.1 to v2.4 (continued)
```
**Mantis
Item Description/Issue Clause**


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
422
```
4582 Added additional standard attributes: _op_ and _multiplicity_ 2.9, 2.9.2

4689 Added copyright notice for the standard header files to allow distribution Annex D

4713 Clarified probe branch semantics 1.3.1, 5.4.2.1

4754 Added support for the branch access functions: **potential()** and **flow()** 4.4, 5.5.1, 5.6.1, A.8.2

4792 Corrected problems with discipline propagation algorithm F.2

4795 Added in a chart outlining the analog simulation initialization flow 8.2

4803 Added in support for the **absdelta** event function 5.10, 5.10.3.4, 8.4.6,
8.4.7, A.6.5

4815 Deprecated support for empty disciplines 3.6.2, 3.6.3, 3.6.5, 3.8,
3.10, 3.11.1, 7.4, 7.4.4,
7.4.5, 10.2

4826 Added in support for the hierarchical identifier prefix **$root** 6.2.1, 6.7, A.9.3

4833 Corrected description for **cross()** which was missing the _enable_ argument 5.10.3.1

4834 Ensure that document consistently refers to “dc sweep” in all lower case 5.2.1, 6.6.2.1

4849 Modified document contributor’s table to acknowledge people who have con-
tributed to previous versions of the standard

```
Table G.7—Changes from v2.4 to VAMS-2023
```
**Mantis
Item Description/Issue Clause**

830 Support _jump_statements_^ **return** , **break** , and **continue** 5.1^1 , A.6.5

2594 Resolved redundancy for _list_of_port_declarations_ A.1.3

4848 Updated description for Laplace and Zi transform filters to support vector
parameters

```
4.5.1 1 , 4.5.12
```
4926 Fixed incorrect grammar production for _analog_function_case_item_ A.6.7

4935 Incorrect font for ternary operator A.1.9, A.8.3

5027 Removed unused keyword **net_resolution** B.1, C.16

5036 Fixed incorrect reference to _net_ with _node_ 6.7

7754 Clarified description of the behavior for the system function **$limit()** 9.17.3

7780 Added support for math functions expm1() and ln1p() 4-^14 , 9-^11 , A.8.2, B.1

7791 Clarified description of the behavior for the analog operator **limexp()** 4.5.13

7792 Clarified description of **last_crossing()** operator 4.5.10

7793 Support **$receiver_count()** function, 9.22.2

```
Table G.6—Changes from v2.3.1 to v2.4 (continued)
```
**Mantis
Item Description/Issue Clause**


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
423
```
7794 Clarified description on how **analog initial** blocks impact identifica-
tion of a variable domains

```
7.2.2
```
7795 Support the alternative Verilog style for **$min()** , **$max()** , and **$abs()** 4.3.1, 9-^11

7808 Allow string return type for analog user defined functions
Support **return** statement for analog user defined functions

```
4.7.1
4.7.2.2
```
7809 Explicitly describe how analog named events can be triggered and detected in
the analog context

```
5.10.4
```
7810 Allow tolerance arguments to **transition()** , **timer()** , **cross()** ,
**above()** and **absdelta()** to be dynamic expressions

```
4- 20 , 5.10.3.1, 5.10.3.2,
5.10.3.3, 5.10.3.4, A.6.5
```
7811 Rework for description of the **transition()** filter to clearly define behav-
ior when interrupted

```
4.5.8
```
7812 Clarify how linear interpolation is used for the **absdelay()** filter 4.5.7

7817 Fixed typography error in syntax for **idtmod()** 4-^19

7886 Fixed example for sized literal decimal number 2.6.1

7888 Fixed example to use case equality operator ( **===** ) instead of logical equality
operator ( **==** )

```
7.3.2
```
7891 **string** was being used as both a keyword and a non-terminal. Renamed the
non-terminal to _string_literal_

```
A.8.8
```
##### 7893

7897 Aligned wording for when short-circuit evaluation is applied with that
described in IEEE Std 1364 Verilog

```
4.2.3
```
7898 Added example of arithmetic operator ** to table 4.2.4

7900 Corrected description for domain of pow() to not use undefined reference
_int(y)_ , but instead explicitly state _for all integer y_

```
4- 14
```
7901 Moved definition for analog_filter_function_arg non-terminal from A.8.1 to
A.8.2

```
A.8.2
```
7903 In example, discrete discipline should be \logic^ 10.6

7909 In the non-terminal _analog_filter_function_arg_ replaced reference to _con-
stant_optional_arrayinit_ with newly defined _constant_assignment_pat-
tern_or_null_

```
A.8.1, A.8.2
```
7912 Add support for **__FILE__** and **__LINE__** compiler directives 10.7

7920 Add support for **$roi()** and **$itor()** in the analog context 9-^8 , 9.1^1

7921 Added _VAMS-2023_ as a keywords specifier 10.6

7922 Remove error for contributing to a port declared with out **input** direction.
This is now a warning

```
5.6.1
```
```
Table G.7—Changes from v2.4 to VAMS-2023 (continued)
```
**Mantis
Item Description/Issue Clause**


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
424
```
### G.2 Obsolete functionality

The following statements are not supported in the current version of Verilog-AMS HDL; they are only noted
for backward compatibility.

**G.2.1 Forever**

This statement is no longer supported.

**G.2.2 NULL**

This statement is no longer supported. Certain functions such as case, conditionals and the event statement
do allow null statements as defined by the syntax.

**G.2.3 Generate**

The _generate statement_ is a looping construct which is unrolled at elaboration time. It is the only looping
statement that can contain analog operators. The syntax of generate statement is shown in Figure G- 1.

```
Figure G-1—Syntax for generate statement
```
The index shall not be assigned or modified in any way inside the loop. In addition, it is local to the loop and
is expanded when the loop is unrolled. Even if there is a local variable with the same name as the index and
the variable is modified as a side effect of a function called from within the loop, the loop index is unaf-
fected.

The start and end bounds and the increment are constant expressions. They are only evaluated at elaboration
time. If the expressions used for the increment and bounds change during the simulation, it does not affect
the behavior of the generate statement.

If the lower bound is less than the upper bound and the increment is negative, or if the lower bound is greater
than the upper bound and the increment is positive, then the generate statement does not execute.

If the lower bound equals the upper bound, the increment is ignored and the statement execute once. If the
increment is not given, it is taken to be +1 if the lower bound is less than the upper bound, and -1 if the
lower bound is greater than the upper bound.

The statement, which can be a sequential block, is replicated with all occurrences of index in the statement
replaced by a constant. In the first instance of the statement, the index is replaced with the lower bound. In

```
generate_statement ::=
generate index_ identifier ( start_expr , end_expr [ , incr_expr ] )
statement
start_expr ::=
constant_expression
end_expr ::=
constant_expression
incr_expr ::=
constant_expression
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
425
```
the second, it is replaced by the lower bound plus the increment. In the third, it is replaced by the lower
bound plus two times (2x) the increment. This pattern is repeated until the lower bound plus a multiple of the
increment is greater than the upper bound.

Example: This module implements a continuously running (unclocked) analog-to-digital converter.

```
module adc(in,out) ;
parameter bits=8, fullscale=1.0, dly=0.0, ttime=10n;
input in;
output [0:bits-1] out;
electrical in;
electrical [0:bits-1] out;
real sample, thresh;
analog begin
thresh = fullscale/2.0;
generate i (bits-1,0) begin
V(out[i]) <+ transition (sample > thresh, dly, ttime);
if (sample > thresh) sample = sample - thresh;
sample = 2.0*sample;
end
end
endmodule
```
**G.2.4 `default_function_type_analog**

The **`default_function_type_analog** directive is no longer supported. this compiler directive
allowed user-defined functions to be treated as analog functions in Verilog-A if they did not have the key
word **analog** as part of the definition.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
426
```
