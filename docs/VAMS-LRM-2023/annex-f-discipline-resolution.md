## Annex F (normative) Discipline resolution methods...................................................................................

### F.1 Discipline resolution

Discipline resolution is described in 7.4; it provides the semantics for two methods of resolving the disci-
pline of undeclared interconnect. This annex provides a possible algorithm for achieving the semantics of
each method. It is also possible to develop and use other algorithms to match the semantics.

### F.2 Resolution of mixed signals.........................................................................................................

The following algorithms for discipline resolution of undeclared nets provide users with the ability to con-
trol the auto-insertion of connection modules. The undeclared nets are resolved at each level of the hierarchy
in which _continuous_ (analog) has precedence over _discrete_ (digital). In both algorithms, the _continuous_
domain is passed up the hierarchy from lower levels to the top level.

The algorithms traverse the hierarchy of a signal composed of nets (also known as net segments of the sig-
nal) in order to determine the discipline of all nets of undeclared discipline. See 7.2.3 for a description of
how a signal consists of a set of net segments.

A net segment of a signal on the upper connection of a port shall be considered as the parent to a net segment
on the lower connection of the port. The net segment on the lower connection of a port shall be considered as
a child net segment of that parent net segment.

When a signal is being traversed _depth-first_ , this means that the traversal shall start at the bottom (leaf) net
segments of the signal – these are net segments which have no children net segments. It further means that
all the children net segments of a parent net segment shall be traversed before that parent net segment is tra-
versed. This type of depth first traversal is more precisely termed a _post-order depth-first traversal_.

When a signal is being traversed _top-down_ , this means that the traversal shall start at the top net segment(s)
of the signal – these are net segments which have no parent net segments. It further means that that all the
parent net segments of a child net segment shall be traversed before that child net segment is traversed.

**F.2.1 Default discipline resolution algorithm**

This default algorithm propagates both continuous and discrete disciplines up the hierarchy to meet one
another. Insertion of interface elements shall occur at each level of the hierarchy where both continuous and
discrete disciplines meet. This results in connection modules being inserted higher up the design hierarchy.
The algorithm is described as follows.

```
1) Elaborate the design
After this step, every port in the design has both its upper (actual) connection and its lower (formal)
connection defined.
2) Apply all in-context node and signal declarations
For example, electrical sig; makes all instances of sig electrical, unless they have been over-
ridden by an out-of-context declaration.
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
411
```
```
3) Apply all out-of-context node and signal declarations.
For example, electrical top.middle.bottom.sig; overrides any discipline which may be
declared for sig in the module where sig was declared.
More than one conflicting in-context discipline declaration or more than one conflicting out-of-con-
text discipline declaration for the same hierarchical segment of a signal is an error. In this case, con-
flicting simply means an attempt to declare more than one discipline regardless of whether the
disciplines are compatible or not.
4) Traverse each signal hierarchically (depth-first) when a net is encountered which still has not been
assigned a discipline:
a) It shall be determined whether the net is analog or digital. Any net which is used in digital
behavioral code shall be considered digital. Any net whose child nets are all digital shall be con-
sidered digital (discrete domain), any others shall be considered analog (continuous domain).
b) If the net has not yet been assigned a discipline, examine all the child nets of that net and con-
struct a list of all disciplines of the child nets whose domains match the domain of the segment:
— If there are no disciplines in the list apply any `default_discipline directives to the net,
provided their domain is the same as the domain of the net. This is done according to the rules
of precedence for `default_discipline (see 3.8).
— If there is only a single discipline in the list, the signal is of that discipline
— If there is more than one discipline in the list and the contents of the list match the discipline list
of a resolution connect statement, the net is of the resolved discipline given by the statement.
— Otherwise the discipline is unknown. This is legal provided the net has no mixed-port connec-
tions (i.e., it does not connect through a port to a segment of a different domain). Otherwise this
is an error
```
At this point, connection module selection and insertion can be performed. Insert converters applying the
rules and semantics of the connect statement (7.7) and auto-insertion sections (7.8).

**F.2.2 Alternate expanded analog discipline resolution algorithm**

This algorithm propagates continuous disciplines up and then back down to meet discrete disciplines. This
may result in more connection modules being inserted lower down into discrete sections of the design hier-
archy for added accuracy. The selection of this algorithm instead of the default shall be controlled by a sim-
ulator option. The algorithm is described as follows.

```
1) Elaborate the design
After this step, every port in the design has both its upper (actual) connection and its lower (formal)
connection defined.
2) Apply all in-context node and signal declarations
For example, electrical sig; makes all instances of sig electrical, unless they have been over-
ridden by an out-of-context declaration.
3) Apply all out-of-context node and signal declarations.
For example, electrical top.middle.bottom.sig; overrides any discipline which may be
declared for sig in the module where sig was declared.
More than one conflicting in-context discipline declaration or more than one conflicting out-of-con-
text discipline declaration for the same hierarchical segment of a signal is an error. In this case, con-
flicting simply means an attempt to declare more than one discipline regardless of whether the
disciplines are compatible or not.
4) Traverse each signal hierarchically (depth-first) when a net is encountered which has still not been
assigned a discipline:
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
412
```
```
a) It shall be determined whether the net is analog or digital. Any net which is used in digital
behavioral code shall be considered digital. Any net whose child nets are all digital shall be con-
sidered digital. If any of the connections are analog, the net shall be considered analog. Any oth-
ers shall still be considered unknown.
b) If the net has not yet been assigned a discipline, examine all the child nets of that net and con-
struct a list of all disciplines of these child nets whose domains match the domain of the seg-
ment:
— If there are no disciplines in the list apply any `default_discipline directives to the net
segment, provided their domain is the same as the domain of the net. This is done according to
the rules of precedence for `default_discipline (see 3.8).
— If there is only a single discipline in the list, the signal is of that discipline
— If there is more than one discipline in the list and the contents of the list match the discipline list
of a resolution connect statement, the net is of the resolved discipline given by the statement.
— Otherwise the discipline is unknown. This is legal provided the net has no mixed-port connec-
tions (i.e., it does not connect through a port to a segment of a different domain). Otherwise this
is an error.
5) Traverse each signal hierarchically (top-down) when a net is encountered which still has not been
assigned a discipline or which has been assigned a digital domain from step 4:
a) It shall be re-determined whether the net is analog or digital. Any net which is used in digital
behavioral code shall be considered digital. Any net whose parent nets are digital shall be con-
sidered digital. Any others shall be considered analog.
b) If the net has not yet been assigned a discipline, examine all the parent nets of that net and con-
struct a list of all disciplines of these parent nets whose domains match the domain of the seg-
ment:
— If there are no disciplines in the list apply any `default_discipline directives to the net,
provided their domain is the same as the domain of the net. This is done according to the rules
of precedence for `default_discipline (see 3.8).
— If there is only a single discipline in the list, the signal is of that discipline
— If there is more than one discipline in the list and the contents of the list match the discipline list
of a resolution connect statement, the net is of the resolved discipline given by the statement.
— Otherwise the discipline is unknown. This is legal provided the net has no mixed-port connec-
tions (i.e., it does not connect through a port to a segment of a different domain). Otherwise this
is an error.
```
At this point, connection module selection and insertion can be performed. Insert converters applying the
rules and semantics of the connect statement (7.7) and auto-insertion sections (7.8).


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
413
```
