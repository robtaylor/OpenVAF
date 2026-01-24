# Verilog-AMS

# Language Reference Manual

## VAMS-

## Feb 14, 2024


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
ii
```
Copyright© 2024 Accellera Systems Initiative. All rights reserved.

Accellera Systems Initiative Inc., 8698 Elk Grove Blvd. Suite 1, #114, Elk Grove, CA 95624, USA

Verilog® is a registered trademark of Cadence Design Systems, Inc.

```
Notices
```
**Accellera Systems Initiative (Accellera) standards** documents are developed within Accellera by its
Technical Committee. Accellera develops its standards through a consensus development process, approved
by its members and board of directors, which brings together volunteers representing varied viewpoints and
interests to achieve the final product. Volunteers are members of Accellera and serve without compensation.
While Accellera administers the process and establishes rules to promote fairness in the consensus develop-
ment process, Accellera does not independently evaluate, test, or verify the accuracy of any of the informa-
tion contained in its standards.

Use of an Accellera standard is wholly voluntary. Accellera disclaims liability for any personal injury, prop-
erty or other damage, of any nature whatsoever, whether special, indirect, consequential, or compensatory,
directly or indirectly resulting from the publication, use of, or reliance upon this, or any other Accellera stan-
dard document.

Accellera does not warrant or represent the accuracy or content of the material contained herein, and
expressly dis- claims any express or implied warranty, including any implied warranty of merchantability or
suitability for a specific purpose, or that the use of the material contained herein is free from patent infringe-
ment. Accellera standards documents are supplied " **AS IS.** "

The existence of an Accellera standard does not imply that there are no other ways to produce, test, measure,
purchase, market or provide other goods and services related to the scope of an Accellera standard. Further-
more, the viewpoint expressed at the time a standard is approved and issued is subject to change due to
developments in the state of the art and comments received from users of the standard. Every Accellera stan-
dard is subjected to review periodically for revision and update. Users are cautioned to check to determine
that they have the latest edition of any Accellera standard.

In publishing and making this document available, Accellera is not suggesting or rendering professional or
other services for, or on behalf of, any person or entity. Nor is Accellera undertaking to perform any duty
owed by any other person or entity to another. Any person utilizing this, and any other Accellera standards
document, should rely upon the advice of a competent professional in determining the exercise of reasonable
care in any given circumstances.

Interpretations: Occasionally questions may arise regarding the meaning of portions of standards as they
relate to specific applications. When the need for interpretations is brought to the attention of Accellera,
Accellera will initiate action to prepare appropriate responses. Since Accellera standards represent a consen-
sus of concerned interests, it is important to ensure that any interpretation has also received the concurrence
of a balance of interests. For this reason, Accellera and the members of its Technical Committee are not able
to provide an instant response to interpretation requests except in those cases where the matter has previ-
ously received formal consideration.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
iii
```
Comments for revision of Accellera standards are welcome from any interested party, regardless of member-
ship affiliation with Accellera. Suggestions for changes in documents should be in the form of a proposed
change of text, together with appropriate supporting comments. Comments on standards and requests for
interpretations should be addressed to:

```
Accellera Systems Initiative Inc.
8698 Elk Grove Blvd.
Suite 1, #
Elk Grove, CA 95624
USA
```
Accellera is the sole entity that may authorize the use of Accellera-owned certification marks and/or trade-
marks to indicate compliance with the materials set forth herein.

Authorization to reuse portions of any Accellera standard for any purpose other than internal or personal use
must be granted by Accellera, provided that permission is obtained from and any required fee is paid to
Accellera. To arrange for authorization please contact Lynn Garibaldi, Accellera, 8698 Elk Grove Blvd,
Suite 1, #114, Elk Grove, CA, 95624, , e-mail lynn@accellera.org. Permission to copy portions of an Accel-
lera standard for educational or classroom use can also be obtained from Accellera.

Suggestions for improvements to the Verilog-AMS Language Reference Manual are welcome. They should
be sent to the Verilog-AMS e-mail reflector

```
sv-ams@lists.accellera.org
```
The following people contributed to the creation, editing, and review of this document.

```
Note: Attention is called to the possibility that implementation of this standard may require use of subject
matter covered by patent rights. By publication of this standard, no position is taken with respect to the
existence or validity of any patent rights in connection therewith. Accellera shall not be responsible for
identifying patents for which a license may be required by an Accellera standard or for conducting inquiries
into the legal validity or scope of those patents that are brought to its attention.
```
```
Peter Grove , Renesas Electronics, Chair
David Miller , NXP Semiconductor, Technical Editor, Secretary
```
```
Boon Chong Ang, Intel Corporation
Adnan Assar, Siemens EDA
Lakshmanan Balasubramanian, Texas Instruments
Luis Humberto Rezende Barbosa, Cadence Design
Systems, Inc.
Shalom Bresticker , Accellera
Jerry Chang, Texas Instruments
Shekar Chetput , Cadence Design Systems, Inc.
```
```
Geoffrey Coram , Analog Devices
Dave Cronauer , Synopsys
Hardik Parekh, NXP Semiconductor
Bob Pau, Qualcomm
Rock Shi, Allegro Microsystems
Aaron Spratt, Cadence Design Systems, Inc.
Evgeny Vlasov , Synopsys
Mina Zaki, Siemens EDA
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
iv
```
The following people have made contributions to previous versions of this document.

```
Ramana Aisola
Andre Baguenier
Kenneth Bakalar
Jim Barby
Martin Barnasconi
Graham Bell
William Bell
Xavier Bestel
Kevin Cameron
James Cavanaugh
Srikanth Chandrasekaran
Ed Chang
Chandrashekar Chetput
Joe Daniels
Jonathan David
Al Davis
Raphael Dorado
John Downey
Dan FitzPatrick
Bob Floyd
Paul Floyd
Vassilios Gerousis
Ian Getreu
Kim Hailey
Steve Hamm
Graham Helwig
William Hobson
Junwei Hou
Robert Hughes
Dick Klaassen
Marq Kole
Abhi Kolpekwar
Ken Kundert
Laurent Lemaitre
Top Lertpanyavit
Oskar Leuthold
S. Peter Liebmann
```
```
Scott Little
Colin McAndrew
Steve Meyer
Marek Mierzwinski
Ira Miller
Michael Mirmak
John Moore
Scott Morrison
Arpad Muranyi
Patrick O’Halloran
Don O'Riordan
Jeroen Paasschens
Rick Poore
Farzin Rasteh
Tom Reeder
Steffen Rochel
Jon Sanders
David Sharrit
John Shields
James Spoto
Stuart Sutherland
Prasanna Tamhankar
George Tipple
Richard Trihy
Yatin Trivedi
Boris Troyanovsky
Alessandro Valerio
Martin Vlach
Don Webber
Frank Weiler
Ian Wilson
Ilya Yusim
Alex Zamfirescu
Amir Zarkesh
David Zweidinger
```

## Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL

```
i
```

Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL


Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL


Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL


Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL


Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL


Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL


Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL


Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL

- 1. Verilog-AMS introduction......................................................................................................................... Contents
   - 1.1 Overview..........................................................................................................................................
   - 1.2 Mixed-signal language features
   - 1.3 Systems
      - 1.3.1 Conservative systems
      - 1.3.2 Kirchhoff’s Laws
      - 1.3.3 Natures, disciplines, and nets
      - 1.3.4 Signal-flow systems
      - 1.3.5 Mixed conservative/signal flow systems
   - 1.4 Conventions used in this document
   - 1.5 Contents
- 2. Lexical conventions
   - 2.1 Overview........................................................................................................................................
   - 2.2 Lexical tokens
   - 2.3 White space
   - 2.4 Comments
   - 2.5 Operators........................................................................................................................................
   - 2.6 Numbers
      - 2.6.1 Integer constants
      - 2.6.2 Real constants
   - 2.7 String literals
   - 2.8 Identifiers, keywords, and system names ......................................................................................
      - 2.8.1 Escaped identifiers
      - 2.8.2 Keywords
      - 2.8.3 System tasks and functions
      - 2.8.4 Compiler directives
   - 2.9 Attributes........................................................................................................................................
      - 2.9.1 Syntax................................................................................................................................
      - 2.9.2 Standard attributes.............................................................................................................
- 3. Data types.................................................................................................................................................
   - 3.1 Overview........................................................................................................................................
   - 3.2 Integer and real data types
      - 3.2.1 Output variables
   - 3.3 String data type
   - 3.4 Parameters......................................................................................................................................
      - 3.4.1 Type specification
      - 3.4.2 Value range specification ..................................................................................................
      - 3.4.3 Parameter units and descriptions.......................................................................................
      - 3.4.4 Parameter arrays................................................................................................................
      - 3.4.5 Local parameters
      - 3.4.6 String parameters
      - 3.4.7 Parameter aliases
      - 3.4.8 Multidimensional parameter array examples
   - 3.5 Genvars
   - 3.6 Net_discipline
      - 3.6.1 Natures
      - 3.6.2 Disciplines.........................................................................................................................
      - 3.6.3 Net_discipline declaration.................................................................................................
      - 3.6.4 Ground declaration............................................................................................................
      - 3.6.5 Implicit nets.......................................................................................................................
   - 3.7 Real net declarations
   - 3.8 Default discipline ii
   - 3.9 Disciplines of primitives
   - 3.10 Discipline precedence
   - 3.11 Net compatibility
      - 3.11.1 Discipline and Nature Compatibility
   - 3.12 Branches.........................................................................................................................................
      - 3.12.1 Port Branches
   - 3.13 Namespace
      - 3.13.1 Nature and discipline
      - 3.13.2 Access functions
      - 3.13.3 Net
      - 3.13.4 Branch
- 4. Expressions
   - 4.1 Overview........................................................................................................................................
   - 4.2 Operators........................................................................................................................................
      - 4.2.1 Operators with real operands
      - 4.2.2 Operator precedence
      - 4.2.3 Expression evaluation order
      - 4.2.4 Arithmetic operators
      - 4.2.5 Relational operators
      - 4.2.6 Case equality operators
      - 4.2.7 Logical equality operators.................................................................................................
      - 4.2.8 Logical operators...............................................................................................................
      - 4.2.9 Bitwise operators...............................................................................................................
      - 4.2.10 Reduction operators
      - 4.2.11 Shift operators
      - 4.2.12 Conditional operator
      - 4.2.13 Concatenations
      - 4.2.14 Assignment patterns
   - 4.3 Built-in mathematical functions.....................................................................................................
      - 4.3.1 Standard mathematical functions
      - 4.3.2 Transcendental functions
   - 4.4 Signal access functions
   - 4.5 Analog operators
      - 4.5.1 Vector or array arguments to analog operators
      - 4.5.2 Analog operators and equations
      - 4.5.3 Time derivative operator ...................................................................................................
      - 4.5.4 Time integral operator.......................................................................................................
      - 4.5.5 Circular integrator operator...............................................................................................
      - 4.5.6 Derivative operator
      - 4.5.7 Absolute delay operator ....................................................................................................
      - 4.5.8 Transition filter
      - 4.5.9 Slew filter
      - 4.5.10 last_crossing function
      - 4.5.11 Laplace transform filters
      - 4.5.12 Z-transform filters
      - 4.5.13 Limited exponential
      - 4.5.14 Constant versus dynamic arguments
      - 4.5.15 Restrictions on analog operators
   - 4.6 Analysis dependent functions
      - 4.6.1 Analysis.............................................................................................................................
      - 4.6.2 DC analysis
      - 4.6.3 AC stimulus.......................................................................................................................
      - 4.6.4 Noise
   - 4.7 User-defined functions................................................................................................................... iii
      - 4.7.1 Defining an analog user-defined function.........................................................................
      - 4.7.2 Returning a value from an analog user-defined function..................................................
      - 4.7.3 Calling an analog user-defined function
- 5. Analog behavior.......................................................................................................................................
   - 5.1 Overview........................................................................................................................................
   - 5.2 Analog procedural block................................................................................................................
      - 5.2.1 Analog initial block...........................................................................................................
   - 5.3 Block statements
      - 5.3.1 Sequential blocks
      - 5.3.2 Block names
   - 5.4 Analog signals................................................................................................................................
      - 5.4.1 Access functions
      - 5.4.2 Probes and sources
      - 5.4.3 Accessing flow through a port
      - 5.4.4 Unassigned sources
   - 5.5 Accessing net and branch signals and attributes..........................................................................
      - 5.5.1 Accessing net and branch signals....................................................................................
      - 5.5.2 Signal access for vector branches
      - 5.5.3 Accessing attributes
      - 5.5.4 Creating unnamed branches using hierarchical net references
      - 5.5.5 Accessing nets and branch signals hierarchically
   - 5.6 Contribution statements
      - 5.6.1 Direct branch contribution statements
      - 5.6.2 Examples
      - 5.6.3 Resistor and conductor
      - 5.6.4 RLC circuits
      - 5.6.5 Switch branches
      - 5.6.6 Implicit Contributions
      - 5.6.7 Indirect branch contribution statements
      - 5.6.8 Contributing hierarchically
   - 5.7 Analog procedural assignments
   - 5.8 Analog conditional statements
      - 5.8.1 if-else-if statement...........................................................................................................
      - 5.8.2 Examples
      - 5.8.3 Case statement.................................................................................................................
      - 5.8.4 Restrictions on conditional statements............................................................................
   - 5.9 Looping statements
      - 5.9.1 Repeat and while statements
      - 5.9.2 For statements
      - 5.9.3 Analog For Statements
   - 5.10 Analog event control statements
      - 5.10.1 Event OR operator
      - 5.10.2 Global events...................................................................................................................
      - 5.10.3 Monitored events.............................................................................................................
      - 5.10.4 Named events
      - 5.10.5 Digital events in analog behavior....................................................................................
   - 5.11 Jump statements
- 6. Hierarchical structures
   - 6.1 Overview......................................................................................................................................
   - 6.2 Modules........................................................................................................................................
      - 6.2.1 Top-level modules and $root
      - 6.2.2 Module instantiation
   - 6.3 Overriding module parameter values........................................................................................... iv
      - 6.3.1 Defparam statement
      - 6.3.2 Module instance parameter value assignment by order
      - 6.3.3 Module instance parameter value assignment by name
      - 6.3.4 Parameter dependence.....................................................................................................
      - 6.3.5 Detecting parameter overrides
      - 6.3.6 Hierarchical system parameters
   - 6.4 Paramsets
      - 6.4.1 Paramset statements
      - 6.4.2 Paramset overloading
      - 6.4.3 Paramset output variables
   - 6.5 Ports
      - 6.5.1 Port definition
      - 6.5.2 Port declarations..............................................................................................................
      - 6.5.3 Real valued ports.............................................................................................................
      - 6.5.4 Connecting module ports by ordered list
      - 6.5.5 Connecting module ports by name..................................................................................
      - 6.5.6 Detecting port connections..............................................................................................
      - 6.5.7 Port connection rules.......................................................................................................
      - 6.5.8 Inheriting port natures
   - 6.6 Generate constructs
      - 6.6.1 Loop generate constructs
      - 6.6.2 Conditional generate constructs
      - 6.6.3 External names for unnamed generate blocks.................................................................
   - 6.7 Hierarchical names
      - 6.7.1 Usage of hierarchical references
   - 6.8 Scope rules
   - 6.9 Elaboration
      - 6.9.1 Concatenation of analog blocks
      - 6.9.2 Elaboration and paramsets
      - 6.9.3 Elaboration and connectmodules
      - 6.9.4 Order of elaboration
- 7. Mixed signal...........................................................................................................................................
   - 7.1 Overview......................................................................................................................................
   - 7.2 Fundamentals
      - 7.2.1 Domains
      - 7.2.2 Contexts
      - 7.2.3 Nets, nodes, ports, and signals
      - 7.2.4 Mixed-signal and net disciplines.....................................................................................
   - 7.3 Behavioral interaction
      - 7.3.1 Accessing discrete nets and variables from a continuous context
      - 7.3.2 Accessing X and Z bits of a discrete net in a continuous context...................................
      - 7.3.3 Accessing continuous nets and variables from a discrete context
      - 7.3.4 Detecting discrete events in a continuous context
      - 7.3.5 Detecting continuous events in a discrete context
      - 7.3.6 Concurrency
      - 7.3.7 Function calls
   - 7.4 Discipline resolution
      - 7.4.1 Compatible discipline resolution
      - 7.4.2 Connection of discrete-time disciplines
      - 7.4.3 Connection of continuous-time disciplines
      - 7.4.4 Resolution of mixed signals
      - 7.4.5 Discipline resolution of continuous signals
   - 7.5 Connect modules..........................................................................................................................
   - 7.6 Connect module descriptions v
   - 7.7 Connect specification statements
      - 7.7.1 Connect module auto-insertion statement
      - 7.7.2 Discipline resolution connect statement
      - 7.7.3 Parameter passing attribute
      - 7.7.4 connect_mode
   - 7.8 Automatic insertion of connect modules
      - 7.8.1 Connect module selection
      - 7.8.2 Signal segmentation
      - 7.8.3 connect_mode parameter
      - 7.8.4 Rules for driver-receiver segregation and connect module selection and insertion........
      - 7.8.5 Instance names for auto-inserted instances
      - 7.8.6 Supply sensitive connect module examples
   - 7.9 Driver-receiver segregation
- 8. Scheduling semantics.............................................................................................................................
   - 8.1 Overview......................................................................................................................................
   - 8.2 Simulation initialization...............................................................................................................
   - 8.3 Analog simulation cycle
      - 8.3.1 Nodal analysis
      - 8.3.2 Transient analysis............................................................................................................
      - 8.3.3 Convergence....................................................................................................................
   - 8.4 Mixed-signal simulation cycle
      - 8.4.1 Circuit initialization
      - 8.4.2 Mixed-signal DC analysis
      - 8.4.3 Mixed-signal transient analysis.......................................................................................
      - 8.4.4 The synchronization loop
      - 8.4.5 Synchronization and communication algorithm
      - 8.4.6 absdelta interpolated A2D events
      - 8.4.7 Assumptions about the analog and digital algorithms
   - 8.5 Scheduling semantics for the digital engine
      - 8.5.1 The stratified event queue
      - 8.5.2 The Verilog-AMS digital engine reference model
      - 8.5.3 Scheduling implication of assignments...........................................................................
- 9. System tasks and functions
   - 9.1 Overview......................................................................................................................................
   - 9.2 Categories of system tasks and functions
   - 9.3 System tasks/functions executing in the context of the Analog Simulation Cycle......................
   - 9.4 Display system tasks
      - 9.4.1 Behavior of the display tasks in the analog context
      - 9.4.2 Escape sequences for special characters
      - 9.4.3 Format specifications
      - 9.4.4 Hierarchical name format................................................................................................
      - 9.4.5 String format
      - 9.4.6 Behavior of the display tasks in the analog block during iterative solving
      - 9.4.7 Extensions to the display tasks in the digital context......................................................
   - 9.5 File input-output system tasks and functions...............................................................................
      - 9.5.1 Opening and closing files................................................................................................
      - 9.5.2 File output system tasks
      - 9.5.3 Formatting data to a string
      - 9.5.4 Reading data from a file
      - 9.5.5 File positioning
      - 9.5.6 Flushing output
      - 9.5.7 I/O error status
      - 9.5.8 Detecting EOF................................................................................................................. vi
      - 9.5.9 Behavior of the file I/O tasks in the analog block during iterative solving
   - 9.6 Timescale system tasks
   - 9.7 Simulation control system tasks
      - 9.7.1 $finish..............................................................................................................................
      - 9.7.2 $stop
      - 9.7.3 $fatal, $error, $warning, and $info
   - 9.8 PLA modeling system tasks.........................................................................................................
   - 9.9 Stochastic analysis system tasks
   - 9.10 Simulator time system functions..................................................................................................
   - 9.11 Conversion system functions
   - 9.12 Command line input.....................................................................................................................
   - 9.13 Probabilistic distribution system functions
      - 9.13.1 $random and $arandom...................................................................................................
      - 9.13.2 Distribution functions
      - 9.13.3 Algorithm for probabilistic distribution
   - 9.14 Math system functions
   - 9.15 Analog kernel parameter system functions..................................................................................
   - 9.16 Dynamic simulation probe function.............................................................................................
   - 9.17 Analog kernel control system tasks and functions.......................................................................
      - 9.17.1 $discontinuity
      - 9.17.2 $bound_step task
      - 9.17.3 $limit
   - 9.18 Hierarchical parameter system functions.....................................................................................
   - 9.19 Explicit binding detection system functions
   - 9.20 Analog node alias system functions.............................................................................................
   - 9.21 Table based interpolation and lookup system function................................................................
      - 9.21.1 Table data source
      - 9.21.2 Control string
      - 9.21.3 Example control strings
      - 9.21.4 Interpolation algorithms
      - 9.21.5 Example
   - 9.22 Connectmodule driver access system functions and operator
      - 9.22.1 $driver_count
      - 9.22.2 $receiver_count
      - 9.22.3 $driver_state
      - 9.22.4 $driver_strength
      - 9.22.5 driver_update
      - 9.22.6 Receiver net resolution....................................................................................................
      - 9.22.7 Connect module example using driver access functions
   - 9.23 Supplementary connectmodule driver access system functions
      - 9.23.1 $driver_delay
      - 9.23.2 $driver_next_state
      - 9.23.3 $driver_next_strength
      - 9.23.4 $driver_type
- 10. Compiler directives
   - 10.1 Overview......................................................................................................................................
   - 10.2 `default_discipline........................................................................................................................
   - 10.3 `default_transition
   - 10.4 `define and `undef
   - 10.5 Predefined macros........................................................................................................................
   - 10.6 `begin_keywords and `end_keywords
   - 10.7 `__FILE__ and `__LINE__..........................................................................................................
- 11. Using VPI routines................................................................................................................................. vii
   - 11.1 Overview......................................................................................................................................
   - 11.2 The VPI interface
      - 11.2.1 VPI callbacks
      - 11.2.2 VPI access to Verilog-AMS HDL objects and simulation objects
      - 11.2.3 Error handling
   - 11.3 VPI object classifications.............................................................................................................
      - 11.3.1 Accessing object relationships and properties
      - 11.3.2 Delays and values............................................................................................................
   - 11.4 List of VPI routines by functional category.................................................................................
   - 11.5 Key to object model diagrams
      - 11.5.1 Diagram key for objects and classes
      - 11.5.2 Diagram key for accessing properties
      - 11.5.3 Diagram key for traversing relationships
   - 11.6 Object data model diagrams.........................................................................................................
      - 11.6.1 Module
      - 11.6.2 Nature, discipline
      - 11.6.3 Scope, task, function, IO declaration
      - 11.6.4 Ports
      - 11.6.5 Nodes
      - 11.6.6 Branches
      - 11.6.7 Quantities
      - 11.6.8 Nets
      - 11.6.9 Regs.................................................................................................................................
      - 11.6.10 Variables, named event
      - 11.6.11 Memory
      - 11.6.12 Parameter, specparam
      - 11.6.13 Primitive, prim term
      - 11.6.14 UDP.................................................................................................................................
      - 11.6.15 Module path, timing check, intermodule path
      - 11.6.16 Task and function call
      - 11.6.17 Continuous assignment
      - 11.6.18 Simple expressions..........................................................................................................
      - 11.6.19 Expressions
      - 11.6.20 Contribs
      - 11.6.21 Process, block, statement, event statement
      - 11.6.22 Assignment, delay control, event control, repeat control
      - 11.6.23 If, if-else, case
      - 11.6.24 Assign statement, deassign, force, release, disable
      - 11.6.25 Callback, time queue
- 12. VPI routine definitions...........................................................................................................................
   - 12.1 Overview......................................................................................................................................
   - 12.2 vpi_chk_error()
   - 12.3 vpi_compare_objects().................................................................................................................
   - 12.4 vpi_free_object()..........................................................................................................................
   - 12.5 vpi_get()
   - 12.6 vpi_get_cb_info().........................................................................................................................
   - 12.7 vpi_get_analog_delta()
   - 12.8 vpi_get_analog_freq()..................................................................................................................
   - 12.9 vpi_get_analog_time()
   - 12.10 vpi_get_analog_value()................................................................................................................
   - 12.11 vpi_get_delays()...........................................................................................................................
   - 12.12 vpi_get_str()
   - 12.13 vpi_get_analog_systf_info()
   - 12.14 vpi_get_systf_info() viii
   - 12.15 vpi_get_time()..............................................................................................................................
   - 12.16 vpi_get_value()
   - 12.17 vpi_get_vlog_info()
   - 12.18 vpi_get_real()
   - 12.19 vpi_handle()
   - 12.20 vpi_handle_by_index()
   - 12.21 vpi_handle_by_name()
   - 12.22 vpi_handle_multi().......................................................................................................................
      - 12.22.1 Derivatives for analog system task/functions
      - 12.22.2 Examples
   - 12.23 vpi_iterate()..................................................................................................................................
   - 12.24 vpi_mcd_close()...........................................................................................................................
   - 12.25 vpi_mcd_name()
   - 12.26 vpi_mcd_open()
   - 12.27 vpi_mcd_printf()
   - 12.28 vpi_printf()
   - 12.29 vpi_put_delays()
   - 12.30 vpi_put_value()
   - 12.31 vpi_register_cb()
      - 12.31.1 Simulation-event-related callbacks
      - 12.31.2 Simulation-time-related callbacks...................................................................................
      - 12.31.3 Simulator analog and related callbacks...........................................................................
      - 12.31.4 Simulator action and feature related callbacks
   - 12.32 vpi_register_analog_systf()..........................................................................................................
      - 12.32.1 System task and function callbacks
      - 12.32.2 Declaring derivatives for analog system task/functions
      - 12.32.3 Examples
   - 12.33 vpi_register_systf()
      - 12.33.1 System task and function callbacks
      - 12.33.2 Initializing VPI system task/function callbacks
   - 12.34 vpi_remove_cb()
   - 12.35 vpi_scan().....................................................................................................................................
   - 12.36 vpi_sim_control().........................................................................................................................
- Annex A (normative) Formal syntax definition
   - A.1 Source text
   - A.2 Declarations
   - A.3 Primitive instances
   - A.4 Module instantiation and generate construct
   - A.5 UDP declaration and instantiation
   - A.6 Behavioral statements
   - A.7 Specify section
   - A.8 Expressions
   - A.9 General
   - A.10 Details
- Annex B (normative) List of keywords
- Annex C (normative) Analog language subset
   - C.1 Verilog-A overview
   - C.2 Verilog-A language features
   - C.3 Lexical conventions
   - C.4 Data types.....................................................................................................................................
   - C.5 Expressions
   - C.6 Analog signals..............................................................................................................................
   - C.7 Analog behavior........................................................................................................................... ix
   - C.8 Hierarchical structures
   - C.9 Mixed signal.................................................................................................................................
   - C.10 Scheduling semantics...................................................................................................................
   - C.11 System tasks and functions
   - C.12 Compiler directives
   - C.13 Using VPI routines.......................................................................................................................
   - C.14 VPI routine definitions.................................................................................................................
   - C.15 Analog language subset
   - C.16 List of keywords
   - C.17 Standard definitions
   - C.18 SPICE compatibility
   - C.19 Changes from previous Verilog-A LRM versions.......................................................................
   - C.20 Obsolete functionality
- Annex D (normative) Standard definitions..................................................................................................
   - D.1 The disciplines.vams file
   - D.2 The constants.vams file................................................................................................................
   - D.3 The driver_access.vams file.........................................................................................................
- Annex E (normative) SPICE compatibility
   - E.1 Introduction..................................................................................................................................
   - E.2 Accessing Spice objects from Verilog-AMS HDL......................................................................
   - E.3 Preferred primitive, parameter, and port names...........................................................................
   - E.4 Other issues
- Annex F (normative) Discipline resolution methods...................................................................................
   - F.1 Discipline resolution
   - F.2 Resolution of mixed signals.........................................................................................................
- Annex G (informative) Change history
   - G.1 Changes from previous LRM versions
   - G.2 Obsolete functionality
- Annex H (informative) Glossary


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
1
```
