### 11. Using VPI routines

### 11.1 Overview......................................................................................................................................

Clause 11 and Clause 12 specify the Verilog Procedural Interface (VPI) for the Verilog-AMS HDL. This
clause describes how the VPI routines are used and Clause 12 defines each of the routines in alphabetical
order.

### 11.2 The VPI interface

The VPI interface provides routines which allow Verilog-AMS product users to access information con-
tained in a Verilog-AMS design and allow facilities to interact dynamically with a software product. Appli-
cations of the VPI interface can include delay calculators and annotators, connecting a Verilog-AMS
simulator with other simulation and CAE systems, and customized debugging tasks.

The functions of the VPI interface can be grouped into two main areas:

```
— Dynamic software product interaction using VPI callbacks
— Access to Verilog-AMS HDL objects and simulation specific objects
```
#### 11.2.1 VPI callbacks

Dynamic software product interaction shall be accomplished with a registered callback mechanism. VPI
callbacks shall allow a user to request a Verilog-AMS HDL software product, such as a logic simulator, call
a user-defined application when a specific activity occurs. For example, the user can request the user appli-
cation my_monitor() be called when a particular net changes value or my_cleanup() be called when the
software product execution has completed.

The VPI callback facility shall provide the user with the means to interact dynamically with a software prod-
uct, detecting the occurrence of value changes, advancement of time, end of simulation, etc. This feature
allows applications such as integration with other simulation systems, specialized timing checks, complex
debugging features, etc. to be used.

The reasons for providing callbacks can be separated into four categories:

```
— Simulation event (e.g., a value change on a net or a behavioral statement execution)
— Simulation time (e.g., the end of a time queue or after certain amount of time)
— Simulator action/feature (e.g., the end of compile, end of simulation, restart, or enter interactive
mode)
— User-defined system task or function execution
```
VPI callbacks shall be registered by the user with the functions **vpi_register_cb()** , **vpi_regis-
ter_systf()** and **vpi_register_analog_systf()**. These routines indicate the specific reason
for the callback, the application to be called, and what system and user data shall be passed to the callback
application when the callback occurs. A facility is also provided to call the callback functions when a Ver-
ilog-AMS HDL product is first invoked. A primary use of this facility shall be for the registration of user-
defined system tasks and functions.

#### 11.2.2 VPI access to Verilog-AMS HDL objects and simulation objects

Accessible Verilog-AMS HDL objects and simulation objects and their relationships and properties are
described using data model diagrams. These diagrams are presented in 11.6. The data diagrams indicate the


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
275
```
routines and constants which are required to access and manipulate objects within an application environ-
ment. An associated set of routines to access these objects is defined in Clause 12.

The VPI interface also includes a set of utility routines for functions such as handle comparison, file han-
dling, and redirected printing, which are described in Clause 12.

VPI routines provide access to objects in an _instantiated_ Verilog-AMS design. An instantiated design is one
where each instance of an object is uniquely accessible. For instance, if a module m contains wire w and is
instantiated twice as m1 and m2, then m1.w and m2.w are two distinct objects, each with its own set of related
objects and properties.

The VPI interface is designed as a _simulation_ interface, with access to both Verilog-AMS HDL objects and
specific simulation objects. This simulation interface is different from a hierarchical language interface,
which would provide access to HDL information but would not provide information about simulation
objects.

#### 11.2.3 Error handling

To determine if an error occurred, the routine **vpi_chk_error()** shall be provided. The **vpi_chk_er-
ror()** routine shall return a nonzero value if an error occurred in the previously called VPI routine. Call-
backs can be set up for when an error occurs as well. The **vpi_chk_error()** routine can provide detailed
information about the error.

### 11.3 VPI object classifications.............................................................................................................

VPI objects are classified with data model diagrams. These diagrams provide a graphical representation of
those objects within a Verilog-AMS design to which the VPI routines shall provide access. The diagrams
shall show the relationships between objects and the properties of each object. Objects with sufficient com-
monality are placed in groups. Group relationships and properties apply to all the objects in the group.

As an example, the simplified diagram in Figure 11- 1 shows there is a _one-to-many relationships_ from
objects of type module to objects of type net and a _one-to-one relationship_ from objects of type net to
objects of type module. Objects of type net have properties **vpiName** , **vpiVector** , and **vpiSize** , with
the C data types string, Boolean, and integer respectively.

```
Figure 11-1: Object relationships
```
The VPI object data diagrams are presented in 11.6.

## module net

```
-> name
str: vpiName
str: vpiFullName
-> vector
bool: vpiVector
-> size
int: vpiSize
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
276
```
#### 11.3.1 Accessing object relationships and properties

The VPI interface defines the C data type of **vpiHandle**. All objects are manipulated via a **vpiHandle**
variable. Object handles can be accessed from a relationship with another object, or from a hierarchical
name, as the following example demonstrates.

Examples:

```
vpiHandle net;
net = vpi_handle_by_name ("top.m1.w1", NULL);
```
This example call retrieves a handle to wire top.m1.w1 and assigns it to the **vpiHandle** variable _net_. The
NULL second argument directs the routine to search for the name from the top level of the design.

The VPI interface provides generic functions for tasks, such as traversing relationships and determining
property values. One-to-one relationships are traversed with routine **vpi_handle()**.

In the following example, the module containing _net_ is derived from a handle to that net:

```
vpiHandle net, mod;
net = vpi_handle_by_name ("top.m1.w1", NULL);
mod = vpi_handle (vpiModule, net);
```
The call to **vpi_handle()** in the above example shall return a handle to module top.m1.

Properties of objects shall be derived with routines in the **vpi_get** family. The routine **vpi_get()**
returns integer and Boolean properties. The routine **vpi_get_str()** accesses string properties.

To retrieve a pointer to the full hierarchical name of the object referenced by handle _mod_ , the following call
would be made:

```
char *name = vpi_get_str (vpiFullName, mod);
```
In the above example, character pointer name shall now point to the string top.m1.

One-to-many relationships are traversed with an iteration mechanism. The routine **vpi_iterate()** cre-
ates an object of type **vpiIterator** , which is then passed to the routine **vpi_scan()** to traverse the desired
objects.

In the following example, each net in module top.m1 is displayed:

```
vpiHandle itr;
itr = vpi_iterate (vpiNet,mod);
while (net = vpi_scan (itr) )
vpi_printf ("\t%s\n", vpi_get_str (vpiFullName, net) );
```
As the above examples illustrate, the routine naming convention is a _vpi_ prefix with ‘ ___ ’ word delimiters
(with the exception of callback-related defined values, which use the _cb_ prefix). Macro-defined types and
properties have the _vpi_ prefix and they use capitalization for word delimiters.

The routines for traversing Verilog-AMS HDL structures and accessing objects are described in IEEE Std
1364 Verilog, section 22.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
277
```
#### 11.3.2 Delays and values............................................................................................................

Properties are of type integer, boolean, real or string. Delay and logic value properties, however, are more
complex and require specialized routines and associated structures. The routines **vpi_get_delays()**
and **vpi_put_delays()** use structure pointers, where the structure contains the pertinent information
about delays. Similarly, simulation values are also handled with the routines **vpi_get_value()** and
**vpi_put_value()** , along with an associated set of structures. For analog tasks and functions,
**vpi_handle_multi()** and **vpi_put_value()** support declaration and assignment of derivatives for
the task arguments and function return values.

The routines and C structures for handling delays, derivatives, and logic values are presented in IEEE Std
1364 Verilog, section 22.

### 11.4 List of VPI routines by functional category.................................................................................

The VPI routines can be divided into groups based on primary functionality.

```
— VPI routines for simulation-related callbacks
— VPI routines for system task/function callbacks
— VPI routines for traversing Verilog-AMS HDL hierarchy
— VPI routines for accessing properties of objects
— VPI routines for accessing objects from properties
— VPI routines for delay processing
— VPI routines for logic and strength value processing
— VPI routines for task parameters derivatives processing
— VPI routines for analysis and simulation time processing
— VPI routines for miscellaneous utilities
```
Table 11- 1 through Table 11- 9 list the VPI routines by major category. IEEE Std 1364-2005 Verilog HDL,
Section 22 defines each of the VPI routines, listed in alphabetical order.

```
Table 11-1—VPI routines for simulation related callbacks
```
```
To Use
Register a simulation-related callback vpi_register_cb()
Remove a simulation-related callback vpi_remove_cb()
Get information about a simulation-related callback vpi_get_cb_info()
```
```
Table 11-2—VPI routines for system task/function callbacks
```
```
To Use
Register a system task/function callback vpi_register_systf()
Get information about a system task/function callback vpi_get_systf_info()
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
278
```
```
Table 11-3—VPI routines for analog system task/function callbacks
```
```
To Use
```
Register an analog system task/function callback **vpi_register_analog_systf()**

Get information about an analog system task/function callback **vpi_get_analog_systf_info()**

```
Table 11-4—VPI routines for traversing Verilog-AMS HDL hierarchy
```
```
To Use
```
Obtain a handle for an object with a one-to-one relationship **vpi_handle()**

Obtain handles for objects in a one-to-many relationship **vpi_iterate()
vpi_scan()**

Obtain a handles for an object in a many-to-one relationship **vpi_handle_multi()**

```
Table 11-5—VPI routines for accessing properties of objects
```
```
To Use
```
Get the value of objects with types of int or bool **vpi_get()**

Get the value of objects with types of string **vpi_get_str()**

Get the value of objects with types of real **vpi_get_real()**

```
Table 11-6—VPI routines for accessing objects from properties
```
```
To Use
```
Obtain a handle for a named object **vpi_handle_by_name()**

Obtain a handle for an indexed object **vpi_handle_by_index()**

```
Table 11-7—VPI routines for delay processing
```
```
To Use
```
Retrieve delays or timing limits of an object **vpi_get_delays()**

Write delays or timing limits to an object **vpi_put_delays()**


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
279
```
### 11.5 Key to object model diagrams

This clause contains the keys to the symbols used in the object model diagrams. Keys are provided for
objects and classes, traversing relationships, and accessing properties.

```
Table 11-8—VPI routines for logic, real, strength and analog value processing
```
```
To Use
Retrieve logic value or strength value of an object vpi_get_value()
Write logic value or strength value to an object vpi_put_value()
Retrieve values of an analog object vpi_get_analog_value()
```
```
Table 11-9—VPI routines for analysis and simulation time processing
```
```
To Use
Find the current simulation time or the scheduled time of future events vpi_get_time()
Find the current simulation time value in the continuous domain. vpi_get_analog_time()
Find the current simulation time delta value in continuous domain. vpi_get_analog_delta()
Find the current simulation frequency in the small-signal domain. vpi_get_analog_freq()
```
```
Table 11-10—VPI routines for miscellaneous utilities
```
```
To Use
Write to stdout and the current log file vpi_printf()
Open a file for writing vpi_mcd_open()
Close one or more files vpi_mcd_close()
Write to one or more files vpi_mcd_printf()
Retrieve the name of an open file vpi_mcd_name()
Retrieve data about product invocation options vpi_get_vlog_info()
See if two handles refer to the same object vpi_compare_objects()
Obtain error status and error information about the previous call to a
VPI routine
```
```
vpi_chk_error()
```
```
Free memory allocated by VPI routines vpi_free_object()
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
280
```
#### 11.5.1 Diagram key for objects and classes

#### 11.5.2 Diagram key for accessing properties

## class defn

## obj defn

## class

## object

## obj defn

## object

## class

## obj1

## obj2

```
Object Definition :
```
```
Bold letters in a solid enclosure indicate an object definition. The
properties of the object are defined in this location.
```
```
Unnamed Class:
```
```
A dotted enclosure with no name is an unnamed class. It is sometimes
convenient to group objects although they shall not be referenced as a
group elsewhere, so a name is not indicated.
```
```
Object Reference:
```
```
Normal letters in a solid enclosure indicate an object reference.
```
```
Class Definition:
```
```
Bold italic letters in a dotted enclosure indicate a class definition,
where the class groups other objects and classes. Properties of the
class are defined in this location. The class definition can contain an
object definition.
```
```
Class Reference:
```
```
Italic letters in a dotted enclosure indicate a class reference.
```
## obj

## obj

## object

```
String properties are accessed with routine vpi_get_str().
```
```
Example:
```
```
char name[nameSize];
vpi_get_str(vpiName, obj_h);
```
```
Integer and Boolean properties are accessed with the routine
vpi_get().
```
```
Example: Given a vpiHandle obj_h to an object of type vpiObj , get
the size of the object.
```
```
bool vect_flag = vpi_get(vpivector, obj_h);
int size = vpi_get_size(vpiSize, obj_h);
```
```
Complex properties for time and logic value are accessed with the
indicated routines. See the descriptions of the routines for usage.
```
```
-> vector
bool: vpiVector
-> size
int: vpiSize
```
```
-> complex
func1()
func2()
```
```
-> name
str: vpiName
str: vpiFullName
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
281
```
#### 11.5.3 Diagram key for traversing relationships

## ref

## obj

## ref

## obj

### vpiTag

## ref

## obj

## ref

## obj

### vpiTag

## obj

## obj

```
A single arrow indicates a one-to-one relationship accessed
with the routine vpi_handle().
```
```
Example: Given vpiHandle variable ref_h of type ref, access
obj_h of type vpiObj :
```
```
obj_h = vpi_handle(vpiObj, ref_h);
```
```
A tagged one-to-one relationship is traversed similarly, using
vpiTag instead of vpiObj :
```
```
Example:
```
```
obj_h = vpi_handle(vpiTag, ref_h);
```
```
A top-level one-to-one relationship is traversed similarly, using
NULL instead of ref_h:
```
```
Example:
```
```
obj_h = vpi_handle(vpiObj, NULL);
```
```
A double arrow indicates a one-to-many relationship accessed
with the routine vpi_scan().
```
```
Example: Given vpiHandle variable ref_h of type ref, scan
objects of type vpiObj :
```
```
itr = vpi_iterate(vpiObj, ref_h);
while (obj_h = vpi_scan(itr) )
/* process ‘obj_h’ */
```
```
A tagged one-to-many relationship is traversed similarly, using
vpiTag instead of vpiObj :
```
```
Example:
```
```
itr = vpi_iterate(vpiTag, ref_h);
while (obj_h = vpi_scan(itr) )
/* process ‘obj_h’ */
```
```
A top-level one-to-many relationship is traversed similarly,
using NULL instead of ref_h:
```
```
Example:
```
```
itr = vpi_iterate(vpiObj, NULL);
while (obj_h = vpi_scan(itr) )
/* process ‘obj_h’ */
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
282
```
### 11.6 Object data model diagrams.........................................................................................................

Subclauses in 11.6.1 through 11.6.25 contain the data model diagrams that define the accessible objects and
groups of objects, along with their relationships and properties.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
283
```
#### 11.6.1 Module

```
NOTES
1— Top-level modules shall be accessed using vpi_iterate() with a NULL reference object.
2— Passing a NULL handle to vpi_get() with types vpiTimePrecision or vpiTimeUnit shall return the smallest time
precision of all modules in the instantiated design.
```
## net

## reg

## variables

## mod path

## tchk

## memory

## scope

## process

## module

## cont assign

## port

## module

## io decl

```
vpiInternalScope
```
## def param

## param assign

## primitive

## parameter

## spec param

```
-> cell
bool: vpiCellInstance
-> decay time
int: vpiDefDecayTime
-> default net type
int: vpiDefNetType
-> definition location
int: vpiDefLineNo
str: vpiDefFile
-> definition name
str: vpiDefName
-> delay mode
int: vpiDefDelayMode
-> location
int: vpiLineNo
str: vpiFile
-> name
str: vpiName
str: vpiFullName
-> protected
bool: vpiProtected
-> timeprecision
int: vpiTimePrecision
-> timeunit
int: vpiTimeUnit
-> top module
bool: vpiTopModule
-> unconnected drive
int: vpiUnconnDrive
```
## named event

## branches

## nodes


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
284
```
#### 11.6.2 Nature, discipline

## discipline param assign

```
vpiPotentialNature
```
```
vpiFlowNature
```
## nature

## nature

## nature param assign

## nature

```
vpiParent
```
```
-> name
str: vpiName
str: vpiFullName
```
```
-> name
str: vpiName
str: vpiFullName
```
## discipline

## nature

```
vpiChild
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
285
```
#### 11.6.3 Scope, task, function, IO declaration

## scope

## module

## named event

## variables

## memory

## taskfunc

## scope

## def param

## taskfunc

## task

## function

## expr

## io decl

```
vpiInternalScope
```
## reg

## named begin

## named fork

## stmt

## expr

```
vpiRightRange
```
```
vpiLeftRange
```
## udp defn

## module

## reg

## net

## variables

```
vpiExpr
```
```
-> name
str: vpiName
str: vpiFullName
```
```
-> location
int: vpiLineNo
str: vpiFile
```
```
-> direction
int: vpiDirection
-> location
int: vpiLineNo
str: vpiFile
-> name
str: vpiName
-> scalar
bool: vpiScalar
-> size
int: vpiSize
-> vector
bool: vpiVector
```
## parameter

```
NOTE—A Verilog-AMS HDL function shall contain an object with the same name, size, and type as the function.
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
286
```
#### 11.6.4 Ports

```
vpiHighConn
```
```
vpiBit
```
```
vpiParent
```
## port vpiLowConn

## port bit

## ports

```
NOTES
1— vpiHighConn shall indicate the hierarchically higher (closer to the top module) port connection.
2— vpiLowConn shall indicate the lower (further from the top module) port connection.
3— Properties scalar and vector shall indicate if the port is 1 bit or more than 1 bit. They shall not indicate anything
about what is connected to the port.
4— Properties index and name shall not apply for port bits.
5— If a port is explicitly named, then the explicit name shall be returned. If not, and a name exists, that name shall be
returned. Otherwise, NULL shall be returned.
6— vpiPortIndex can be used to determine the port order.
```
## expr

## expr

```
-> connected by name
bool: vpiConnByName
-> delay (mipd)
vpi_get_delays()
vpi_put_delays()
-> direction
int: vpiDirection
-> explicitly named
bool: vpiExplicitName
-> index
int: vpiPortIndex
-> location
int: vpiLineNo
str: vpiFile
-> name
str: vpiName
str: vpiFullName
-> scalar
bool: vpiScalar
-> size
int: vpiSize
-> vector
bool: vpiVector
```
## nodes

## module


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
287
```
#### 11.6.5 Nodes

```
vpiLeftRange
```
```
vpiBit
```
```
vpiParent
```
## node vpiRightRange

## node bit

## nodes

```
NOTES
1— Properties scalar and vector shall indicate if the node is 1 bit or more than 1 bit.
```
## expr

## expr

```
-> implicitly declared
bool: vpiImplicitDecl
-> location
int: vpiLineNo
str: vpiFile
-> name
str: vpiName
str: vpiFullName
-> scalar
bool: vpiScalar
-> size
int: vpiSize
-> vector
bool: vpiVector
```
## branches

## nets

```
vpiIndex
```
## expr

## module

## discipline


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
288
```
#### 11.6.6 Branches

```
vpiLeftRange
```
```
vpiBit
```
```
vpiParent
```
## branch vpiRightRange

## branch

## branches

```
NOTE—Properties scalar and vector shall indicate if the branch is 1 bit or more than 1 bit.
```
## expr

## expr

```
-> location
int: vpiLineNo
str: vpiFile
-> name
str: vpiName
str: vpiFullName
-> scalar
bool: vpiScalar
-> size
int: vpiSize
-> vector
bool: vpiVector
```
## nodes

## nodes

```
vpiIndex
```
## Quantity

## module

```
vpiPosNode
```
```
vpiNegNode
```
## expr

## Quantity

## Discipline

```
vpiFlow
```
```
vpiPotential
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
289
```
#### 11.6.7 Quantities

```
vpiLeftRange
```
```
vpiBit
```
```
vpiParent
```
## Quantity vpiRightRange

## Quantity

## Quantities

## expr

## expr

```
-> implicitly declared
bool: vpiImplicitDecl
-> real value
vpi_get_analog_value()
-> imaginary value
vpi_get_analog_value()
-> scalar
bool: vpiScalar
-> size
int: vpiSize
-> vector
bool: vpiVector
-> source
bool: vpiSource
-> equation target
bool: vpiEquationTarget
```
## Branches

```
vpiIndex
```
```
vpiBranch
```
## expr

## Nature

## Contribs


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
290
```
#### 11.6.8 Nets

```
NOTES
1— For vectors, net bits shall be available regardless of vector expansion.
2— Continuous assignments and primitive terminals shall be accessed regardless of hierarchical boundaries.
3— Continuous assignments and primitive terminals shall only be accessed from scalar nets or bit selects.
4— For vpiPortInst and vpiPort , if the reference handle is a bit or the entire vector, the relationships shall return a
handle to either a port bit or the entire port, respectively.
5— For implicit nets, vpiLineNo shall return 0 , and vpiFile shall return the filename where the implicit net is first
referenced.
6— Only active forces and assign statements shall be returned for vpiLoad.
7— Only active forces shall be returned for vpiDriver.
8— vpiDriver shall also return ports which are driven by objects other than nets and net bits.
```
```
vpiBit
```
```
vpiParent
```
## nets

## net

## net bit

## module

```
vpiPortInst
```
```
vpiHighConn
```
## ports

```
vpiLowConn
```
## prim term

## path term

## tchk term

```
vpiDriver
```
```
vpiLoad
```
```
vpiDelay
```
```
vpiLeftRange
```
```
vpiRightRange
```
```
vpiIndex
```
## cont assign

## expr

## expr

## expr

## expr

## ports

## ports

## force

## assign stmt

```
-> delay
vpi_get_delays()
-> expanded
bool: vpiExpanded
-> implicitly declared
bool: vpiImplicitDecl
-> location
int: vpiLineNo
str: vpiFile
-> name
str: vpiName
str: vpiFullName
```
```
-> net decl assign
bool: vpiNetDeclAssign
-> net type
int: vpiNetType
-> scalar
bool: vpiScalar
-> scalared declaration
bool: vpiExplicitScalared
-> size
int: vpiSize
-> domain
int vpiDomain
```
```
-> strength
int: vpiStrength0
int: vpiStrength1
int: vpiChargeStrength
-> value
vpi_get_value()
vpi_put_value()
-> vector
bool: vpiVector
-> vectored declaration
bool: vpiExplicitVectored
```
## discipline

## node


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
291
```
#### 11.6.9 Regs.................................................................................................................................

```
vpiBit
```
```
vpiParent
```
## regs

## reg

## reg bit

## scope

```
vpiPortInst
```
## ports

```
vpiLowConn vpiHighConn
```
```
NOTES
1— Continuous assignments and primitive terminals shall be accessed regardless of hierarchical boundaries.
2— Continuous assignments and primitive terminals shall only be accessed from scalar regs and bit selects.
3— Only active forces and assign statements shall be returned for vpiLoad and vpiDriver.
```
```
vpiLeftRange
```
```
vpiRightRange
```
```
vpiIndex
```
## expr

## ports

## prim term

## cont assign

## force

## assign stmt

```
vpiLoad
```
```
vpiDriver
```
## expr

## expr

## tchk term

```
-> location
int: vpiLineNo
str: vpiFile
-> name
str: vpiName
str: vpiFullName
-> scalar
bool: vpiScalar
-> size
int: vpiSize
-> value
vpi_get_value()
vpi_put_value()
-> vector
bool: vpiVector
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
292
```
#### 11.6.10 Variables, named event

```
vpiParent
```
## variables

## integer var

## var select

## real var

## time var

## scope

```
vpiPortInst
```
## ports

```
vpiLowConn vpiHighConn
```
```
vpiParent
```
## scope named event

## expr

## expr

```
vpiLeftRange
```
```
vpiRightRange
```
## expr

```
vpiIndex
```
## ports

```
NOTE— vpiLeftRange and vpiRightRange shall be invalid for reals, since there can not be arrays of reals.
```
```
-> array
bool: vpiArray
-> location
int: vpiLineNo
str: vpiFile
-> name
str: vpiName
str: vpiFullName
-> size
int: vpiSize
-> value
vpi_get_value()
vpi_put_value()
-> domain
int: vpiDomain
```
```
-> location
int: vpiLineNo
str: vpiFile
-> value
vpi_get_value()
vpi_put_value()
```
```
-> location
int: vpiLineNo
str: vpiFile
-> name
str: vpiName
str: vpiFullName
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
293
```
#### 11.6.11 Memory

```
NOTES
1— vpiSize for a memory shall return the number of words in the memory.
2— vpiSize for a memory word shall return the number of bits in the word.
```
## scope

## memory

```
vpiParent
```
## memory word

```
vpiLeftRange
```
```
vpiRightRange
```
```
vpiLeftRange
```
```
vpiRightRange
```
## expr

## expr

## expr

## expr

```
-> location
int: vpiLineNo
str: vpiFile
-> name
str: vpiName
str: vpiFullName
-> size
int: vpiSize
-> value
vpi_get_value()
vpi_put_value()
```
```
-> location
int: vpiLineNo
str: vpiFile
-> name
str: vpiName
str: vpiFullName
-> size
int: vpiSize
```
## expr

```
vpiIndex
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
294
```
#### 11.6.12 Parameter, specparam

## module

## scope parameter

## module def param

## module param assign

```
vpiRhs
```
## expr

```
vpiLhs
```
## parameter

## spec param

```
vpiRhs
```
## expr

```
vpiLhs
```
## parameter

## expr

## expr

```
NOTES
1— Obtaining the value from the object parameter shall return the final value of the parameter after all module
instantiation overrides and defparams have been resolved.
2— vpiLhs from a param assign object shall return a handle to the overridden parameter.
```
```
-> constant type
int: vpiConstType
-> location
int: vpiLineNo
str: vpiFile
-> name
str: vpiName
str: vpiFullName
-> value
vpi_get_value()
```
```
-> constant type
int: vpiConstType
-> location
int: vpiLineNo
str: vpiFile
-> name
str: vpiName
str: vpiFullName
-> value
vpi_get_value()
```
```
-> location
int: vpiLineNo
str: vpiFile
```
```
-> location
int: vpiLineNo
str: vpiFile
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
295
```
#### 11.6.13 Primitive, prim term

## prim term

## module

## primitive

## gate

## switch

## udp defn udp

```
vpiDelay
```
## expr

## expr

```
-> definition name
str: vpiDefName
-> delay
vpi_get_delays()
vpi_put_delays()
-> location
int: vpiLineNo
str: vpiFile
-> name
str: vpiName
str: vpiFullName
-> primitive type
int: vpiPrimType
-> number of inputs
int: vpiSize
->strength
int: vpiStrength0
int: vpiStrength1
-> value
vpi_get_value()
vpi_put_value()
-> domain
int: vpiDomain
```
```
-> direction
int: vpiDirection
-> index
int: vpiTermIndex
-> location
int: vpiLineNo
str: vpiFile
-> value
vpi_get_value()
```
```
NOTES
1— vpiSize shall return the number of inputs.
2— For primitives, vpi_put_value() shall only be used with sequential UDP primitives.
```
## device


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
296
```
#### 11.6.14 UDP.................................................................................................................................

## udp defn

## udp

## table entry

## initial

```
NOTE—Only string (decompilation) and vector (ASCII values) shall be obtained for table entry objects using
vpi_get_value(). Refer to the definition of vpi_get_value() for additional details.
```
## io decl

```
-> definition name
str: vpiDefName
-> location
int: vpiLineNo
str: vpiFile
-> number of inputs
int: vpiSize
-> protected
bool: vpiProtected
-> type
int: vpiPrimType
```
```
-> location
int: vpiLineNo
str: vpiFile
-> number of symbol entries
int: vpiSize
-> value
vpi_get_value()
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
297
```
#### 11.6.15 Module path, timing check, intermodule path

```
NOTES
1— The vpiTchkRefTerm is the first terminal for all tchks except $setup , where vpiTchkDataTerm is the first
terminal and vpiTchkRefTerm is the second terminal.
2— To get to an intermodule path, vpi_handle_multi(vpiInterModPath, port1, port2) can be used.
```
## path term

```
vpiModPathIn
vpiModPathOut
```
## module

## expr

## expr

```
vpiModDataPathIn
```
## mod path

## module

## tchk vpiTchkRefTerm tchk term

```
vpiTchkNotifier
```
## regs

## expr

## expr vpiCondition

```
vpiTchkDataTerm
```
## expr

```
vpiDelay
```
## expr

```
vpiDelay
```
```
-> delay
vpi_get_delays()
vpi_put_delays()
-> location
int: vpiLineNo
str: vpiFile
-> path type
int: vpiPathType
-> polarity
int: vpiPolarity
int: vpiDataPolarity
-> hasIfNone
bool: vpiModPathHasIfNone
```
```
-> direction
int: vpiDirection
-> edge
int: vpiEdge
-> location
int: vpiLineNo
str: vpiFile
```
```
-> limit
vpi_get_delays()
vpi_put_delays()
-> location
int: vpiLineNo
str: vpiFile
-> tchk type
int: vpiTchkType
```
```
-> edge
int: vpiEdge
-> location
int: vpiLineNo
str: vpiFile
```
## inter mod path ports

```
-> delay
vpi_get_delay()
vpi_put_delay()
```
```
vpiCondition
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
298
```
#### 11.6.16 Task and function call

## tf call

## sys task call

## sys func call

## task call

## func call

## expr

## task

## function

```
vpiArgument
```
## user systf

```
NOTES
1— The system task or function which invoked an application shall be accessed with vpi_handle(vpiSysTfCall,
NULL)
2— vpi_get_value() shall return the current value of the system function.
3— If the vpiUserDefn property of a system task or function call is true, then the properties of the corresponding systf
object shall be obtained via vpi_get_systf_info().
4— All user-defined system tasks or functions shall be retrieved using vpi_iterate() , with vpiUserSystf as the type
argument, and a NULL reference argument.
```
```
vpiSysTfCall
```
```
-> tf name
str: vpiName
-> location
int: vpiLineNo
str: vpiFile
```
```
-> systf info
p_vpi_systf_data:
-> user-defined vpi_get_systf_info()
bool: vpiUserDefn
-> value
vpi_put_value()
vpi_get_value()
-> sys func type
int: vpiSysFuncType
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
299
```
#### 11.6.17 Continuous assignment

## cont assign

```
vpiRhs
```
## expr

```
vpiLhs
```
## expr

## module

## expr

```
vpiDelay
```
```
-> delay
vpi_get_delays()
-> location
int: vpiLineNo
str: vpiFile
-> net decl assign
bool: vpiNetDeclAssign
-> strength
int: vpiStrength0
int: vpiStrength1
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
300
```
#### 11.6.18 Simple expressions..........................................................................................................

## simple expr

## variables

## expr

## nets

## regs

## memory word

## var select

## vpiUse prim term

## stmt

## port

## path term

## tchk term

```
NOTES
1— For vectors, the vpiUse relationship shall access any use of the vector or part-selects or bit-selects thereof.
2— For bit-selects, the vpiUse relationship shall access any specific use of that bit, any use of the parent vector, and
any part-select which contains that bit.
```
## cont assign

```
vpiIndex
```
## parameter

```
-> name
str: vpiName
str: vpiFullName
```
## memory


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
301
```
#### 11.6.19 Expressions

## expr

## operation

## constant

## simple expr

## part select

```
vpiParent
```
```
vpiOperand
```
## func call

## sys func call

## expr

## expr

```
vpiLeftRange
```
```
vpiRightRange
```
## expr

```
NOTE—For an operator whose type is vpiMultiConcat , the first operand shall be the multiplier expression.
```
```
-> location
int: vpiLineNo
str: vpiFile
```
```
-> size
int: vpiSize
-> value
vpi_get_value()
```
```
-> operation type
int: vpiOpType
-> location
int: vpiLineNo
str: vpiFile
```
```
-> constant type
int:
vpiConstType
-> location
int: vpiLineNo
```
## accessfunc

## analog oper

## discipline

## branches


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
302
```
#### 11.6.20 Contribs

## contribs

## potential

## ind flow

## flow

```
vpiRhs
```
## expr

## expr

```
vpiLhs
```
```
vpiRhs
```
## expr

```
-> value
vpi_get_value()
-> direct
bool:vpiDirect
-> flow
bool: vpiFlow
```
## ind potential

## branches


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
303
```
#### 11.6.21 Process, block, statement, event statement

## module

## initial

## process

## always

## block

## stmt

## atomic stmt

## block stmt

## atomic stmt

## assignment

## deassign

## case

## for

## delay control

## event control

## event stmt

## assign stmt

## if

## if else

## while

## repeat

## wait

## tf call

## disable

## force

## release

## null stmt

## forever

## begin

## fork

## named begin

## named fork

## scope

```
-> location
int: vpiLineNo
str: vpiFile
```
```
-> location
int: vpiLineNo
str: vpiFile
```
## event stmt ‘->’ named event

```
-> location
int: vpiLineNo
str: vpiFile
```
## analog

## contribs


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
304
```
#### 11.6.22 Assignment, delay control, event control, repeat control

## assignment

```
vpiRhs
```
## expr

```
vpiLhs
```
## expr

## delay control

## event control

## repeat control

## delay control ‘#’ stmt

```
vpiCondition
```
## expr

## stmt

## event control ‘@’

## named event

## expr

```
vpiDelay
```
```
NOTE—For delay control and event control associated with assignment, the statement shall always be NULL.
```
## repeat control expr

## event control

```
-> location
int: vpiLineNo
str: vpiFile
```
```
-> blocking
bool: vpiBlocking
-> location
int: vpiLineNo
str: vpiFile
```
```
-> delay
vpi_get_delays()
-> location
int: vpiLineNo
str: vpiFile
```
```
-> location
int: vpiLineNo
str: vpiFile
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
305
```
While, repeat, wait, for, forever

```
vpiCondition
```
## expr

## stmt

## while

## repeat

## wait

## stmt

## for

```
vpiForInitStmt
```
## stmt

```
vpiCondition
```
## expr

```
vpiForIncStmt
```
## stmt

## forever stmt

```
-> location
int: vpiLineNo
str: vpiFile
```
```
-> location
int: vpiLineNo
str: vpiFile
```
```
-> location
int: vpiLineNo
str: vpiFile
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
306
```
#### 11.6.23 If, if-else, case

```
vpiElseStmt
```
## stmt

## if

## if else

```
vpiCondition
```
## expr

## stmt

## case

```
vpiCondition
```
## expr

## case item expr

```
vpiStmt
```
## stmt

```
NOTES
1— The case item shall group all case conditions which branch to the same statement.
2— vpi_iterate() shall return NULL for the default case item since there is no expression with the default case.
```
```
-> location
int: vpiLineNo
str: vpiFile
```
```
-> location
int: vpiLineNo
str: vpiFile
```
```
-> case type
int: vpiCaseType
-> location
int: vpiLineNo
str: vpiFile
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
307
```
#### 11.6.24 Assign statement, deassign, force, release, disable

## deassign

```
vpiLhs
```
## expr

```
vpiRhs
```
## expr

```
vpiLhs
```
## expr

## function

## task

## named fork

## disable

```
vpiScope
```
## named begin

## release

## force

## assign stmt

```
-> location
int: vpiLineNo
str: vpiFile
```
```
-> location
int: vpiLineNo
str: vpiFile
```
```
-> location
int: vpiLineNo
str: vpiFile
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
308
```
#### 11.6.25 Callback, time queue

## callback

## prim term

## time queue

```
vpiParent
```
```
NOTES
1— To get information about the callback object, the routine vpi_get_cb_info() can be used.
2— To get callback objects not related to the above objects, the second argument to vpi_iterate() shall be NULL.
3— The time queue objects shall be returned in increasing order of simulation time.
4— vpi_iterate() shall return NULL if there is nothing left in the simulation queue.
5— If any events after read only sync remain in the current queue, then it shall not be returned as part of the iteration.
```
## stmt

## expr

```
-> cb info
p_cb_data:
vpi_get_cb_info()
```
## time queue

```
-> time
vpi_get_time()
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
309
```
