## Annex E (normative) SPICE compatibility

### E.1 Introduction..................................................................................................................................

Analog simulation has long been performed with SPICE and SPICE-like simulators. As such, there is a huge
legacy of SPICE netlists. In addition, SPICE provides a rich set of predefined models and it is considered nei-
ther practical nor desirable to convert these models into a Verilog-AMS HDL behavioral description. In
order for Verilog-AMS HDL to be embraced by the analog design community, it is important Verilog-AMS
HDL provide an appropriate degree of SPICE compatibility. This annex describes the degree of compatibility
which Verilog-AMS HDL provides and the approach taken to provide that compatibility.

**E.1.1 Scope of compatibility**

SPICE is not a single language, but rather is a family of related languages. The first widely used version of
SPICE was SPICE2g6 from the University of California at Berkeley. However, SPICE has been enhanced and
distributed by many different companies, each of which has added their own extensions to the language and
models. As a result, there is a great deal of incompatibility even among the SPICE languages themselves.

Verilog-AMS HDL makes no judgment as to which of the various SPICE languages should be supported.
Instead, it states if a simulator which supports Verilog-AMS HDL is also able to read SPICE netlists of a par-
ticular flavor, then certain objects defined in that flavor of SPICE netlist can be referenced from within a Ver-
ilog-AMS HDL structural description. In particular, SPICE models and subcircuits can be instantiated within
a Verilog-AMS HDL module. This is also true for any SPICE primitives which are built into the simulator. In
general, anything that can be instantiated in the particular flavor of SPICE can also be instantiated within a
Verilog-AMS HDL module.

**E.1.2 Degree of incompatibility**

There are four primary areas of incompatibility between versions of SPICE simulators.

```
1) The version of the SPICE language accepted by various simulators is different and to some degree
proprietary. This issue is not addressed by Verilog-AMS HDL. So whether a particular Verilog-
AMS simulator is SPICE compatible, and with which particular variant of SPICE it is compatible, is
solely determined by the authors of the simulator.
2) Not all SPICE simulators support the same set of component primitives. Thus, a particular SPICE net-
list can reference a primitive which is unsupported. Verilog-AMS HDL offers no alternative in this
case other than the possibility that if the model equations are known, the primitive can be rewritten
as a module.
3) The names of the built-in SPICE primitives, their parameters, or their ports can differ from simulator
to simulator. This is particularly true because many primitives, parameters, and ports are unnamed in
SPICE. When instantiating SPICE primitives in Verilog-AMS HDL, the primitives shall, and parame-
ters and ports can, be named. Since there are no established standard names, there is a high likeli-
hood of incompatibility cropping up in these names. To reduce this, a list of what names shall be
used for the more common components is shown in Table E.1. However, it is not possible to antici-
pate all SPICE primitives and parameters which could be supported; so different implementations can
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
402
```
```
end up using different names. This level of incompatibility can be overcome by using wrapper mod-
ules to map names.
4) The mathematical description of the built-in primitives can differ. As with the netlist syntax, incom-
patible enhancements of the models have crept in through the years. Again, Verilog-AMS HDL
offers no solution in this case other than the possibility that if the model equations are known, the
primitive can be rewritten as a module.
```
### E.2 Accessing Spice objects from Verilog-AMS HDL......................................................................

If an implementation of a Verilog-AMS tool supports SPICE compatibility, it is expected to provide the basic
set of SPICE primitives (see Annex E.3) and be able to read SPICE netlists which contain models and subcir-
cuit statements.

SPICE primitives built into the simulator shall be treated in the same manner in Verilog-AMS HDL as built-
in primitives of gate- and switch-level modeling. However, while the Verilog-AMS HDL built-in primitives
are standardized, the SPICE primitives are not. All aspects of SPICE primitives are implementation dependent.

In addition to SPICE primitives, it shall also be possible to access subcircuits and models defined within
SPICE netlists. The subcircuits and models contained within the SPICE netlist are treated as module defini-
tions.

**E.2.1 Case sensitivity**

Some SPICE netlists are case insensitive, whereas Verilog-AMS HDL descriptions are case-sensitive. From
within Verilog-AMS HDL, a mixed-case name matches the same name with an identical case (if one is
defined in a Verilog-AMS HDL description). However, if no exact match is found, the mixed-case name
shall match the same name defined within SPICE regardless of the case.

**E.2.2 Examples**

This subsection shows some examples.

**E.2.2.1 Accessing SPICE models**

Consider the following SPICE model file being read by a Verilog-AMS HDL simulator.

```
.MODEL VERTNPN NPN BF=80 IS=1E-18 RB=100 VAF=50
+ CJE=3PF CJC=2PF CJS=2PF TF=0.3NS TR=6NS
```
This model can be instantiated in a Verilog-AMS HDL module as shown in Figure E.1.

```
Figure E.1—Instantiated module
```
### e

### c1 c2

### b1 b2


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
403
```
```
module diffPair (c1, b1, e, b2, c2);
electrical c1, b1, e, b2, c2;
```
```
vertNPN Q1 (c1, b1, e);
vertNPN Q2 (.c(c2), .b(b2), .e(e));
endmodule
```
Unlike with SPICE, the first letter of the instance name, in this case Q1 and Q2, is not constrained by the prim-
itive type. For example, they can just as easily be T1 and T2.

The ports and parameters of the bjt are determined by the bjt primitive itself and not by the model state-
ment for the bjt. See E.3 for more details. This bjt primitive has 3 mandatory ports ( _c_ , _b_ , and _e_ ) and one
optional port ( _s_ ). In the instantiation of Q1, the ports are passed by order. With Q2, the ports are passed by
name. In both cases, the optional substrate port _s_ is defaulted by simply not giving it.

**E.2.2.2 Accessing SPICE subcircuits**

As an example of how a SPICE subcircuit is referenced from Verilog-AMS HDL, consider the following
SPICE subcircuit definition of an oscillator.

```
.SUBCKT ECPOSC (OUT GND)
VA VCC GND 5
IEE E GND 1MA
Q1 VCC B1 E VCC VERTNPN
Q2 OUT B2 E OUT VERTNPN
L1 VCC OUT 1UH
C1 VCC OUT 1P IC=1
C2 OUT B1 272.7PF
C3 B1 GND 3NF
R1 B1 GND 10K
C4 B2 GND 3NF
R2 B2 GND 10K
.ENDS ECPOSC
```
This oscillator can be referenced from Verilog-AMS HDL as:

```
module osc (out, gnd);
electrical out, gnd;
ecpOsc Osc1 (out, gnd);
endmodule
```
NOTE—In Verilog-AMS HDL the name of the subcircuit instance is not constrained to start with X as it is in SPICE.

**E.2.2.3 Accessing SPICE primitives**

To show how various SPICE primitives can be accessed from Verilog-AMS HDL, the subcircuit in E.2.2.2 is
translated to native Verilog-AMS HDL.

```
module ecpOsc (out, gnd);
electrical out, gnd;
```
```
vsine #(.dc(5)) Vcc (vcc, gnd);
isine #(.dc(1m)) Iee (e, gnd);
vertNPN Q1 (vcc, b1, e, vcc);
vertNPN Q2 (out, b2, e, out);
inductor #(.l(1u)) L1 (vcc, out);
capacitor #(.c(1p), .ic(1)) C1 (vcc, out);
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
404
```
```
capacitor #(.c(272.7p)) C2 (out, b1);
capacitor #(.c(3n)) C3 (b1, gnd);
resistor #(.r(10k)) R1 (b1, gnd);
capacitor #(.c(3n)) C4 (b2, gnd);
resistor #(.r(10k)) R2 (b2, gnd);
endmodule
```
### E.3 Preferred primitive, parameter, and port names...........................................................................

```
Table E.1 shows the required names for primitives, parameters, and ports which are otherwise unnamed in
SPICE. For connection by order instead of by name, the ports and parameters shall be given in the order
listed. The default discipline of the ports for these primitives shall be electrical and their descriptions
shall be inout.
```
```
Table E.1—Names for primitives, parameters, and ports in SPICE
```
```
Primitive Port name Parameter name Behavior
```
resistor p, n r, tc1, tc2 V =

capacitor p, n c, ic V =

inductor p, n l, ic I =

iexp p, n dc, mag,
phase, val0,
val1, td0,
tau0, td1,
tau1

```
I =
```
```
with the value of I at time.
```
ipulse p, n dc, mag,
phase, val0,
val1, td,
rise, fall,
width, period

```
I =
```
```
with the following definitions ( n is a non-negative integer):
```
```
I  r  1 ++ tc 1  T tc 2  T^2
```
```
1
c
```
### --- 0 tId + ic

### lV  0 t d + ic

```
val 0 ttd  0
```
```
val 1  val 1 – dc e
```
```
td 0 – t
```
-  ---------------- _tau_^0 _td_ 0  _ttd_  1

```
val 0  val 0 – Itd 1 e
```
```
td 1 – t
```
-  ---------------- _tau_^1 _td_ 1  _t_


```


```
```

```
```

```
```

```
```

```
```

```
```

```
```
Itd 1 ttd = 1
```
```
val 0 tt  0
val 0  val 1 – val 0 tt –^0
rise
+ ------------- t 0  t  t 1
val 1 t 1  t  t 2
val 1  val 0 – val 1 tt –^2
fall
+ ------------- t 2  t  t 3
 val^0 t^3  t  t^4
```
```

```
```

```
```

```
```

```
```

```
```


```
```

```
```

```
```
t 0 = td + n period 
t 1 = rise td ++ n period 
t 2 = width rise td +++ n period 
t 3 = fall ++++ width rise td n period 
t 4 = td + n + 1  period
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
405
```
ipwl p, n dc, mag,
phase, wave

```
I =
```
```
for and ,
```
```
I =
```
```
for
```
isine p, n dc, mag,
phase, off-
set, ampl,
freq, td,
damp,
sinephase,
ammodindex,
ammodfreq,
ammodphase,
fmmodindex,
fmmodfreq

```
I =
```
```
with = ammodindex, = ammodfreq, =
```
```
ammodphase, = fmmodindex, = fmmodfreq, and
```
```
=sinephase.
```
vexp p, n dc, mag,
phase, val0,
val1, td0,
tau0, td1,
tau1

```
V =
```
```
with the value of V at time.
```
vpulse p, n dc, mag,
phase, val0,
val1, td,
rise, fall,
width, period

```
V =
```
```
with the following definitions ( n is a non-negative integer):
```
```
Table E.1—Names for primitives, parameters, and ports in SPICE (continued)
```
```
Primitive Port name Parameter name Behavior
```
```
wave i + 1
 wave i + 3 – wave i + 1 twavei – 
wave i + 2 – wave i 
--------------------------------------------------------
```
```
+
```
```
wave i  t  wave i + 2 0  i  n nlenwave = ()
```
```
wave n – 1
```
```
wave n – 2  t
```
```
offset ampl
1 – FAM cos 2   fAM  ttd – – AM 
1 – damp  t td – 
2  freq
1 – FFM cos 2  fFM  ttd –   ttd –
```
```

– SIN
```
```


```
```
cos
```
```



```
```
+
```
```
FAM fAM  AM
```
```
FFM fAM
```
```
 SIN
```
```
dc t td  0
```
```
val 1  val 1 – dc e
```
```
td 0 – t
```
-  ---------------- _tau_^0 _td_ 0  _ttd_  1

```
val 0  val 0 – Vtd 1 e
```
```
td 1 – t
```
-  ---------------- _tau_^1 _td_ 1  _t_


```

```
```

```
```
Vtd 1 ttd = 1
```
```
val 0 tt  0
val 0  val 1 – val 0 tt –^0
rise
+ ------------- t 0  t  t 1
val 1 t 1  t  t 2
val 1 + val 0 – val 1  tt ------------ fall –^2 - t 2  t  t 3
 val^0 t^3  t  t^4
```
```

```
```

```
```

```
```

```
```


```
```

```
```

```
```

```
```

```
```
t 0 = td + n period 
t 1 = rise td ++ n period 
t 2 = width rise td +++ n period 
t 3 = fall ++++ width rise td n period 
t 4 = td + n + 1  period
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
406
```
```
Although in a SPICE context the primitives for diode, bjt, mosfet, jfet, and mesfet can be used only
in a model definition, in Verilog-AMS they may be used directly in a paramset statement as described in 7.5.
```
vpwl p, n dc, mag,
phase, wave

```
V =
```
```
for and ,
```
```
I =
```
```
for.
```
vsine p, n dc, mag,
phase, off-
set, ampl,
freq, td,
damp,
sinephase,
ammodindex,
ammodfreq,
ammodphase,
fmmodindex,
fmmodfreq

```
V =
```
```
with = ammodindex, = ammodfreq, =
```
```
ammodphase, = fmmodindex, = fmmodfreq, and
```
```
=sinephase.
```
tline t1, b1,
t2, b2

```
z0, td, f, nl
```
vccs sink, src,
ps, ns

```
gm I(sink, src) =
```
vcvs p, n,
ps, ns

```
gain V(p, n) =
```
diode a, c area

bjt c, b, e, s area

mosfet d, g, s, b w, l, ad, as,
pd, ps, nrd,
nrs

jfet d, g, s area

mesfet d, g, s area

```
Table E.1—Names for primitives, parameters, and ports in SPICE (continued)
```
```
Primitive Port name Parameter name Behavior
```
```
wave i + 1
 wave i + 3 – wave i + 1 twavei – 
wave i + 2 – wave i 
--------------------------------------------------------
```
```
+
```
```
wave i  t  wave i + 2 0  i  n nlenwave = ()
```
```
wave n – 1
```
```
wave n – 2  t
```
```
offset ampl
1 – FAM cos 2   fAM  ttd – – AM 
1 – damp  t td – 
2  freq
1 – FFM cos 2  fFM  ttd –   ttd –
```
```

– SIN
```
```


```
```
cos
```
```



```
```
+
```
```
FAM fAM  AM
```
```
FFM fAM
```
```
 SIN
```
```
gm V ps ns  ()
```
```
gain V ps ns  ()
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
407
```
**E.3.1 Unsupported primitives**

Verilog-AMS HDL does not support the concept of passing an instance name as a parameter. As such, the
following primitives are not supported: ccvs, cccs, and mutual inductors; however, these primitives can be
instantiated inside a SPICE subcircuit that itself is instantiated in Verilog-AMS.

**E.3.2 Discipline of primitives**

To afford the ability to use analog primitive in any design, including mixed disciplines, the default discipline
override is provided. The discipline of analog primitives will be resolved based on instance specific attri-
butes, the disciplines of other instances on the same net, or default to electrical if it cannot be determined.

The precedence for the discipline of analog primitives is as follows:

```
1) A port_discipline attribute on the analog primitive;
2) The resolution of the discipline;
3) The default analog primitive of electrical.
```
**E.3.2.1 Setting the discipline of analog primitives**

A new optional attribute shall be provided called _port_discipline_ , which shall have as a value the desired dis-
cipline for the port of the analog primitive. It shall only apply to either the analog primitive itself or the port
to which it is attached. The value shall be of type string and the value must be a valid discipline of domain
continuous. This attribute shall only apply to analog primitives or the ports of analog primitives; for other
modules as well as the ports of all other modules it shall be ignored.

The following provides an example of this attribute applied to an analog primitive.

```
(* port_discipline="electrical" *) resistor #(.r(1k))
r1 (node1, node2); // not needed as default
(* port_discipline="rotational" *) resistor #(.r(1k))
r2 (node1, node2);
```
The following provides an example of this attribute applied to the ports of an analog primitive.

```
resistor #(.r(1k)) r3
((* port_discipline="rotational" *) node1,
(* port_discipline="rotational" *) node2);
```
The use of these attributes can be combined to change the basic discipline of all ports for the analog primi-
tive, but overriding the discipline for specific ports. The following provides an example of this use

```
(* port_discipline="electrical" *) vcvs #(.gain(1.45e-3))
motor1 (n1, gnd_e,
(* port_discipline="rotational_omega" *) shaft1,
(* port_discipline="rotational_omega" *) gnd_rot);
```
The above model uses a voltage-controlled voltage source to model a motor as a converter from electrical
potential to rotational velocity.

Attributes are described in 2.9 of this document.

**E.3.2.2 Resolving the disciplines of analog primitives**


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
408
```
If no attribute exists on the instance of an analog primitive, then the discipline may be determined by the dis-
ciplines of other instances connected to the same net segment. The disciplines of the vpiLoConn of all other
instances on the net segment shall be evaluated to determine if they are of domain continuous and compati-
ble with each other. If they are, then the discipline of the analog primitive shall be set to the same discipline.
If they are not compatible, then an error will occur as defined in 3.11. If there are no continuous disciplines
defined on the net segment, then the discipline shall default to electrical.

**E.3.3 Name scoping of SPICE primitives**

In the resolution hierarchy of names during elaboration a module or paramset defined in the Verilog-AMS
will always be selected in favor of a SPICE primitive, model, or subcircuit using exactly the same name.

In case of a name match with differences in case, the module or paramset does not interfere with the SPICE
primitive, model, or subcircuit, but the resolution method described in E.2.1 shall apply.

In case of a SPICE primitive which is always available in the Verilog-AMS simulator, a Verilog-AMS mod-
ule or paramset whose name exactly matches that of the primitive will be used in module instantiations. The
Verilog-AMS simulator may issue a warning stating that the Verilog-AMS module or paramset is used
instead of the SPICE primitive. In case of a Verilog-AMS module or paramset whose name exactly matches
that of a SPICE model or subcircuit, the Verilog-AMS simulator shall issue an warning message stating that
the Verilog-AMS module or paramset is used instead of the SPICE model or subcircuit.

**E.3.4 Limiting algorithms**

Many SPICE simulators use limiting algorithms to improve convergence in Newton-Raphson iterations.
Table E.2 lists the preferred names for three functions that may be available in a simulator, their arguments,
and their intended uses. The function name, enclosed in quotation marks, can be used in the **$limit()** func-
tion of 9.17.3. This allows a Verilog-AMS module to use the same limiting algorithms available to built-in
SPICE primitives. The arguments are described in 9.17.3.

### E.4 Other issues

This section highlights some other issues

**E.4.1 Multiplicity factor on subcircuits**

Some SPICE simulators support a multiplicity factor (M) parameter on subcircuits without the parameter
being explicitly being declared. This factor is typically used to indicate the subcircuit should be modeled as
if there are a specified number of copies in parallel. In previous versions of Verilog-AMS HDL, subcircuits
defined as modules could not support automatic M factors.

```
Table E.2—SPICE limiting functions
```
```
Function name Arguments Meant for limiting:
```
```
fetlim vth gate-to-source voltage of field-effect transistors
pnjlim vte, vcrit voltage across diodes and pn junctions in other devices
vdslim (none) drain-to-source voltage of field-effect transistors
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
409
```
Starting with LRM Version 2.2, the multiplicity factor is supported for subcircuits defined as modules in
Verilog-AMS using the hierarchical system parameter **$mfactor** , as described in 6.3.6.

**E.4.2 Binning and libraries**

Some SPICE netlists provide mechanisms for mapping an instance to a group of models, with the final deter-
mination of which model to use being based on rules encapsulated in the SPICE netlist. Examples include
model binning or corners support. From within an instance statement, it appears as if the instance is refer-
encing a simple SPICE model; supporting these additional capabilities in Verilog-AMS HDL is supported via
the instance line by default. Support of SPICE model cards is implementation specific (including those using
these mechanisms).

Similar functionality for Verilog-AMS is supported through use of the paramset, as described in 6.4. Instead
of referencing a specific module, and instance may refer to a paramset identifier, and there may be several
paramsets with the same identifier (name). The final determination of which paramset to use is made accord-
ing to rules specified in 6.4.2.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
410
```
