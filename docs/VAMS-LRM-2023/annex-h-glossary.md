## Annex H (informative) Glossary

##### A

##### AMS

```
See also, Verilog-AMS.
```
##### B

**behavioral description**

```
A mathematical mapping of inputs to outputs for a module, including intermediate variables and
control flow.
```
**behavioral model**

```
A version of a module with a unique set of parameters designed to model a specific component.
```
**block**

```
A level within the behavioral description of a module, delimited by begin and end.
```
**branch**

```
A relationship between two nodes and their attached quantities within the behavioral description of a
module. Each branch has two quantities, a value and a flow, with a reference direction for each.
```
##### C

**compact model**

```
A behavioral model or description of a semiconductor device.
```
**component**

```
A fundamental unit within a system which encapsulates behavior and/or structure. Modules and
models can represent a single component or a subcircuit with many components.
```
**constitutive relationships**

```
The essential relationships ( expressions and statements ) between the outputs of a module and its
inputs and parameters, which define the nature of the module. These relationships constitute a
behavioral description.
```
**control flow**

```
The conditional and iterative statements controlling the behavior of a module. These statements
evaluate arbitrary variables (counters, flags, and tokens) to control the operation of different sections
of a behavioral description.
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
427
```
**child module**

```
A module instantiated inside another, “parent” module. A complete definition of the child module
needs to be defined somewhere. A child module is also known as instantiated module.
```
##### F

**flow**

```
One of the two fundamental quantities used to simulate the behavior of a system. In electrical sys-
tems, the flow is the current.
```
##### I

**instance**

```
Any named occurrence of an component created from a module definition. One module can occur in
multiple instances.
```
**instantiation**

```
The process of creating an instance from a module definition or simulator primitive and defining the
connectivity and parameters of that instance. (Placing the instance in the circuit or system.)
```
##### K

**Kirchhoff’s Laws**

```
The physical laws defining the interconnection relationships of nodes, branches, values, and flows.
They specify a conservation of flow in and out of a node and a conservation of value around a loop
of branches.
```
##### L

**level**

```
One block within a behavioral description, delimited by a pair of matching keywords such as
begin - end or discipline - enddiscipline.
```
##### M

**model**

```
A named instance with a unique group of parameters specifying the behavior of one particular ver-
sion of a module. Models can be used to instantiate elements with parametric specifications different
from those in the original module definition.
```
**module**

```
A definition of the interfaces and behavior of a component.
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
428
```
##### N

**net declaration**

```
The statement in a module definition identifying the names of the nets associated with the module
ports or local to the module. A net declaration also identifies the discipline of the net, which in turn
identifies the access functions.
```
**node**

```
A connection point in the system, with access functions for potential and/or flow through an under-
lying discipline.
```
**NR method**

```
Newton-Raphson method. A generalized method for solving systems of nonlinear algebraic equa-
tions by breaking them into a series of many small linear operations ideally suited for computer pro-
cessing.
```
##### P

**parameter**

```
A constant for characterizing the behavior of an instance of a module. Parameters are defined via a
parameter declaration statement in the module definition, and can be specified each time a module is
called in a netlist instance statement.
```
**parameter declaration**

```
The statement in a module definition which defines the parameters of that module.
```
**port**

```
An external connection point for a module (also known as a terminal ).
```
**potential**

```
One of the two fundamental quantities used to simulate the behavior of a system. In electrical sys-
tems, the potential is the voltage.
```
**primitive**

```
A basic component defined entirely in terms of behavior, without reference to any other primitives.
A primitive is the smallest and simplest portion of a simulated circuit or system.
```
**probe**

```
A branch in a circuit (or system), which does not alter its behavior, but lets the simulator read out the
potential or flow at that point.
```
##### R

### reference direction

```
A convention for determining whether the value of a node, the flow through a branch, the value
across a branch, or the flow in or out of a terminal, is positive or negative.
```
**reference node**

```
A globalanalog node (which equals zero ( 0 )) against whose potentials all node values are measured.
Nets declared as ground shall be bound to the reference node.
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
429
```
##### S

**scope**

```
The current level of a block statement, which includes all lines of code within one set of braces in a
module definition.
```
### structural definitions

```
Instantiating modules inside other modules through the use of module definitions and declarations to
create a hierarchical structure in the module’s behavioral description.
```
##### T

**terminal**

```
See also, port.
```
##### V

**Verilog-A**

```
A subset of Verilog-AMS detailing the analog version of IEEE Std 1364 Verilog (see Annex C).
This is a language for the behavioral description of continuous-time systems, which uses a syntax
similar to the IEEE Std 1364 Verilog specification.
```
**Verilog-AMS**

```
Mixed-signal version of IEEE Std 1364 Verilog. A language for the behavioral description of con-
tinuous-time and discrete-time systems based on the IEEE Std 1364 Verilog specification.
```


