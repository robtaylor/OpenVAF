## Annex C (normative) Analog language subset

This annex defines a working subset of Verilog-AMS HDL for analog-only products.

### C.1 Verilog-A overview

This Verilog-A subset defines a behavioral language for analog only systems. Verilog-A is derived from the
IEEE Std 1364 Verilog specification using a minimum number of constructs for analog and mixed-signal
behavioral descriptions. This Annex is intended to cover the definition and semantics of Verilog-A.

The intent of Verilog-A is to let designers of analog systems and integrated circuits create and use modules
which encapsulate high-level behavioral descriptions of systems and components. The behavior of each
module can be described mathematically in terms of its terminals and external parameters applied to the
module. These behavioral descriptions can be used in many disciplines such as electrical, mechanical, fluid
dynamics, and thermodynamics.

Verilog-A has been defined to be applicable to both electrical and non-electrical systems description. It sup-
ports conservative and signal-flow descriptions by using the terminology for these descriptions using the
concepts of nodes, branches, and terminals. The solution of analog behaviors which obey the laws of conser-
vation fall within the generalized form of Kirchhoff’s Potential and Flow Laws (KPL and KFL). Both of
these are defined in terms of the quantities associated with the analog behaviors.

### C.2 Verilog-A language features

The Verilog-A subset provides access to a salient set of features of the full modeling language that allow
analog designers the ability to model analog systems:

```
— Verilog-A modules are compatible with Verilog-AMS HDL.
— Analog behavioral modeling descriptions are contained in separate analog blocks.
— Branches can be named for easy selection and access.
— Parameters can be specified with valid range limits.
— Systems can be modeled by using expressions consisting of operators, variables, and signals:
— a full set of operators including trigonometric functions, integrals, and derivatives;
— a set of waveform filters to modify the waveform results for faster and more accurate simulation
like transition, slew, Laplace, and Z-domain;
— a set of events to control when certain code is simulated;
— selection of the simulation time step for simulation control;
— support for accessing SPICE primitives from within the language.
```
### C.3 Lexical conventions

With the exception of certain keywords required for Verilog-AMS HDL, Clause 2 shall be applicable to
both Verilog-A and Verilog-AMS HDL. All Verilog-AMS HDL keywords shall be supported by Verilog-A


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
390
```
as reserved words, but IEEE Std 1364 Verilog and Verilog-AMS HDL specific keywords are not used in
Verilog-A. The following Verilog-AMS HDL keywords are not required to be supported for a fully compli-
ant Verilog-A subset:

From 2.6, Numbers: support for X and Z values is limited in the **analog** block to the mixed signal context,
as defined in 7.3.2. In the same paragraph, the use of the question mark character as an alternative for z is
also limited to the mixed signal context.

From 2.8.2, Keywords: certain keywords are not applicable in Verilog-A, as defined in Annex C.16.

### C.4 Data types.....................................................................................................................................

The data types of Clause 3 are applicable to both Verilog-AMS HDL and Verilog-A with the following
exceptions:

```
— From 3.6.2.2, Domain binding: the domain binding type discrete shall be an error in Verilog-A.
— From 3.7, Real net declarations: the wreal data type is not supported in Verilog-A.
— From 3.8, Default discipline: the `default_discipline compiler directive is not supported in
Verilog-A. All Verilog-A modules shall have a discipline defined for each module.
```
This feature allows the use of digital modules in Verilog-AMS HDL without editing them to add a disci-
pline.

### C.5 Expressions

The expressions defined in Clause 4 are applicable to both Verilog-AMS HDL and Verilog-A with the fol-
lowing exception:

The case equality operators ( **===** , **!==** ) are not supported in Verilog-A.

### C.6 Analog signals..............................................................................................................................

The signals defined in 5.4 are applicable to both Verilog-AMS HDL and Verilog-A.

### C.7 Analog behavior

The analog behavior defined in Clause 5 are applicable to both Verilog-AMS HDL and Verilog-A with the
following exceptions:

```
— No digital behavior or events are supported in Verilog-A.
— casex and casez are not supported in Verilog-A.
```
### C.8 Hierarchical structures

The hierarchical structure defined in Clause 6 is applicable to both Verilog-AMS HDL and Verilog-A,
except support for _real value ports_ is only applicable to Verilog-AMS HDL and IEEE Std 1364 Verilog (see
6.5.3).


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
391
```
### C.9 Mixed signal.................................................................................................................................

Clause 7 only applies to Verilog-AMS HDL.

### C.10 Scheduling semantics...................................................................................................................

The analog simulation cycle is applicable to both Verilog-AMS HDL and Verilog-A. The mixed-signal sim-
ulation cycle from 8.2 is only applicable to Verilog-AMS HDL.

### C.11 System tasks and functions

All system tasks and functions in Clause 9 that are applicable in the analog context are applicable to Ver-
ilog-A.

### C.12 Compiler directives

The compiler directives of Clause 10 are applicable to both Verilog-AMS HDL and Verilog-A.

### C.13 Using VPI routines.......................................................................................................................

The analog behavior defined in Clause 11 is applicable to both Verilog-AMS HDL and Verilog-A.

### C.14 VPI routine definitions.................................................................................................................

The analog behavior defined in Clause 12 is applicable to both Verilog-AMS HDL and Verilog-A.

### C.15 Analog language subset

This annex (Annex C) defines the differences between Verilog-AMS HDL and Verilog-A. Annex A defines
the BNF for Verilog-AMS HDL.

### C.16 List of keywords

The keywords in Annex B are the complete set of Verilog-AMS HDL keywords, including those from IEEE
Std 1364 Verilog. The following keywords as defined in this LRM are not used by Verilog-A:

```
connect
connectmodule
connectrules
driver_update
endconnectrules
merged
resolveto
split
wreal
```
NOTE—All keywords of Verilog-AMS HDL are reserved words for Verilog-A.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
392
```
### C.17 Standard definitions

The definitions of Annex D are applicable to both Verilog-AMS HDL and Verilog-A, with the exception of
those disciplines with a domain of **discrete**. A Verilog-A implementation shall silently ignore any defi-
nition of a discipline with a domain of **discrete**.

### C.18 SPICE compatibility

Annex E defines the SPICE compatibility for both Verilog-A and Verilog-AMS HDL.

### C.19 Changes from previous Verilog-A LRM versions.......................................................................

Annex G describes the changes from previous LRM versions for both Verilog-A and Verilog-AMS HDL.

### C.20 Obsolete functionality

Annex G also describes the statements that are no longer supported in the current version of Verilog-AMS
HDL as well as the analog language subset.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
393
```
