## 8. Scheduling semantics.............................................................................................................................

### 8.1 Overview......................................................................................................................................

This clause details the simulation cycles for analog simulation and mixed A/D simulations.

A mixed-signal simulator shall contain an analog solver that complies with the analog simulation cycle
described in 8.3. This component of the mixed-signal simulator is termed the analog engine. A mixed signal
simulator shall also contain a discrete event simulator that complies with the scheduling semantics described
in 8.5. This component is termed the digital engine.

In a mixed-signal circuit, an _analog macro-process_ is a set of continuous nodes that must be solved together
because they are joined by analog blocks or analog primitives. A mixed-signal circuit can comprise one or
more analog macro-process separated by digital processes.

### 8.2 Simulation initialization...............................................................................................................

Before simulation of the network or system can be proceed, initialization of the system must first be per-
formed as outlined in Figure 8- 1.

The system initialization is divided into three main processes, compilation, elaboration, and simulation.
Compilation refers to the process where the design artifacts are incorporated into the simulator. Elaboration
is the process where the system is hierarchically instantiated. During this process, as each design element is
elaborated, parameter declaration assignments are evaluated and module-level generate constructs
expanded. Once the system has been elaborated, the simulator will move onto the simulation process. It is
during this process that module-level variable declaration assignments are evaluated followed by the execu-
tion of **analog initial** blocks. Once **analog initial** blocks are evaluated, analog net declaration
assignments and simulator nodeset values are then applied.

At this point, the system is initialized and the simulation cycle will proceed as detailed in 8.3.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
201
```
```
Figure 8-1: System initialization flow
```
It is important to note that for parametric sweep type analyses like dc sweep, the tool shall re-evaluate the
Elaboration and Pre-simulation steps as outlined in Figure 8- 1 for each sweep point to ensure that all param-
eter value changes are captured.

### 8.3 Analog simulation cycle

Simulation of a network, or system, starts with an analysis of each node to develop equations which define
the complete set of values and flows in a network. Through transient analysis, the value and flow equations
are solved incrementally with respect to time. At each time increment, equations for each signal are itera-
tively solved until they converge on a final solution.

### Compilation

```
Compilation of
design data
```
### Elaboration

```
Hierarchical instantiation
of design
```
```
Evaluation of parameter
declaration assignments
```
```
Execution of generate
constructs
```
### Pre-simulation

```
Variable declaration
assignments
```
```
Execution of analog
initial blocks
```
```
Analog net declaration
assignments and simulator
nodeset evaluation
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
202
```
#### 8.3.1 Nodal analysis

To describe a network, simulators combine constitutive relationships with Kirchhoff’s Laws in _nodal analy-
sis_ to form a system of differential-algebraic equations of the form

These equations are a restatement of Kirchhoff’s Flow Law (KFL).

```
v is a vector containing all node values
t is time
q and i are the dynamic and static portions of the flow
f( ) is a vector containing the total flow out of each node
v 0 is the vector of initial conditions
```
This equation was formulated by treating all nodes as being conservative (even signal flow nodes). In this
way, signal-flow and conservative terminals can be connected naturally. However, this results in unneces-
sary KFL equations for those nodes with only signal-flow terminals attached. This situation is easily recog-
nized and those unnecessary equations are eliminated along with the associated flow unknowns, which shall
be zero ( 0 ) by definition.

#### 8.3.2 Transient analysis............................................................................................................

The equation describing the network is differential and non-linear, which makes it impossible to solve
directly. There are a number of different approaches to solving this problem numerically. However, all
approaches discretize time and solve the nonlinear equations iteratively, as shown in Figure 8- 2.

The simulator replaces the time derivative operator ( _dq/dt_ ) with a discrete-time finite difference approxima-
tion. The simulation time interval is discretized and solved at individual time points along the interval. The
simulator controls the interval between the time points to ensure the accuracy of the finite difference approx-
imation. At each time point, a system of nonlinear algebraic equations is solved iteratively. Most circuit sim-
ulators use the Newton-Raphson (NR) method to solve this system.

```
fvt  dq v t 
dt
```
```
==-------------------+0 ivt 
```
```
v  0 = v 0
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
203
```
```
Figure 8-2: Simulation flowchart (transient analysis)
```
#### 8.3.3 Convergence....................................................................................................................

In the analog kernel, the behavioral description is evaluated iteratively until the NR method converges. On
the first iteration, the signal values used in expressions are approximate and do not satisfy Kirchhoff’s Laws.

In fact, the initial values might not be reasonable, so models need to be written so they do something reason-
able even when given unreasonable signal values.

```
No
```
```
System initializa-
tion (see 8.2)
```
```
Update time
```
### t <- t +  t

```
Update values
```
### v <- v +  v

```
Evaluate equations
f(v,t) = residue
```
### Converged?

```
residue < e
 v < 
Yes
```
```
No
time step?
```
```
Accept the
```
```
$Display
```
```
Start Analysis
```
```
Done? (T = t )
```
```
Yes
```
```
No
```
```
Yes
End
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
204
```
For example, the log or square root of a signal value is being computed, some signal values cause the argu-
ments to these functions to become negative, even though a real-world system never exhibits negative val-
ues.

As the iteration progresses, the signal values approach the solution. Iteration continues until two conver-
gence criteria are satisfied. The first criterion is the proposed solution on this iteration, _v(j)(t)_ , shall be close
to the proposed solution on the previous iteration, _v(j-1)(t)_ , and

where _reltol_ is the relative tolerance and _abstol_ is the absolute tolerance.

_reltol_ is set as a simulator option and typically has a value of 0.001. There can be many absolute tolerances,
which one is used depends on the quantity the signal represents (volts, amps, etc.). The absolute tolerance is
important when _vn_ is converging to zero ( 0 ). Without _abstol_ , the iteration never converges.

The second criterion ensures Kirchhoff's Flow Law is satisfied:

where _fni(v(j))_ is the flow exiting node _n_ from branch _i_.

Both of these criteria specify the absolute tolerance to ensure convergence is not precluded when _vn_ or _fn(v)_
go to zero ( 0 ). The relative tolerance can be set once in an options statement to work effectively on any node
in the circuit, but the absolute tolerance shall be scaled appropriately for its associated signal. The absolute
tolerance shall be the largest signal value which is considered negligible on all the signals where it is associ-
ated.

The simulator uses absolute tolerance to get an idea of the scale of signals. Absolute tolerances are typically
1,000 to 1,000,000 times smaller than the largest typical value for signals of a particular quantity. For exam-
ple, in a typical integrated circuit, the largest potential is about 5 volts, so the default absolute tolerance for
voltage is 1V. The largest current is about 1mA, so the default absolute tolerance for current is 1pA.

### 8.4 Mixed-signal simulation cycle

This section describes the semantics of the initialization, the process of mixed-signal DC analysis, and the
synchronization of analog and digital in transient analysis for Verilog-AMS simulation.

#### 8.4.1 Circuit initialization

The initialization phase of mixed-signal simulation is the process of initializing the circuit state for analysis
tasks such as DC, transient, and AC. It is a one time execution of nodeset statements (3.6.3.2), then the pro-
cedural statements in **analog initial** block, and then the procedural statements in the Verilog initial
block for time zero. These procedures can also be used for assertion of circuit/module parameters and initial
state.

#### 8.4.2 Mixed-signal DC analysis

Mixed-signal DC analysis is the process of finding the steady state of the circuit, which is the DC operating
point for transient and AC analysis. The steady state of the digital circuit is defined as the final state at time
0 when all analog and digital events are executed. For mixed-signal DC analysis, the processes of the analog

```
| vn(j) - vn(j-1) | < reltol ( max (| vn(j) | , | vn(j-1) |)) + abstol
```
```
fn  v  j
n
```
##  reltol max f

```
i
nv
  j + abstol
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
205
```
DC analysis and the digital simulation at time 0 are executed iteratively, starting with the initialization phase
(including analog and digital) defined in circuit initialization (8.4.1), until all signals at the A/D boundaries
reach steady state. The signal propagation at the A/D boundaries follows the same scheduling semantics as
are defined in transient analysis in the following sections.

#### 8.4.3 Mixed-signal transient analysis.......................................................................................

A Verilog-AMS simulation consists of a number of analog and digital processes communicating via events,
shared memory and conservative nodes. Analog processes that share conservative nodes are “solved” jointly
and can be viewed as a “macro” process, there may be any number “macro” processes, and it is left up to the
implementation whether it solves them in a single matrix, multiple matrices or uses other techniques but it
should abide by the accuracy stipulated in the disciplines and analog functions.

**8.4.3.1 Concurrency**

Most (current) simulators are single-threaded in execution, meaning that although the semantics of Verilog-
AMS imply processes are active concurrently, the reality is that they are not. If an implementation is genu-
inely multi-threaded, it should not evaluate processes that directly share memory concurrently, as there are
no data locking semantics in Verilog-AMS.

**8.4.3.2 Analog macro process scheduling semantics**

The internal evaluation of an analog macro process is described in 8.3.2. Once the analog engine has deter-
mined its behavior for a given time, it must communicate the results to other processes in the mixed signal
simulation through events and shared variables. When an analog macro process is evaluated, the analog
engine finds a potential “solution” at a future time (the “acceptance time”), and it stores (but does not com-
municate) values^1 for all the process’s nodes up to that time. A “wake up” event is scheduled for the accep-
tance time of the process, and the process is then inactive until it is either woken up or receives an event
from another process. If it is woken up by its own “wake up” event, it calculates a new solution point, accep-
tance time (and so forth) and deactivates. If it is woken up prior to acceptance time by an event that disturbs
its current solution, it will cancel its own “wake up” event, accept at the wake-up time, recalculate its solu-
tion and schedule a new “wake up” event for the new acceptance time. The process may also wake itself up
early for reevaluation by use of a timer (which can be viewed as just another process).

If the analog process identifies future analog events such as “crossings” or timer events (see 5.10.3) then it
will schedule its wake-up event for the time of the first such event rather than the acceptance time. If the
analog process is woken by such an analog event it will communicate any related events at that time and de-
activate, rescheduling its wake-up for the next analog event or acceptance. Events to external processes gen-
erated from analog events are not communicated until the global simulation time reaches the time of the ana-
log event.

If the time to acceptance is infinite then no wake-up event needs to be scheduled^2.

Analog processes are sensitive to changes in all variables and digital signals read by the process unless that
access is only in statements ‘guarded’ by event expressions. For example the following code implements a
simple digital to analog convertor:

```
module d2a(val,vo); // 16 bit D->A
parameter Vgain = 1.0/65536;
input val;
wire [15:0] val;
```
(^1) Or derivatives w.r.t. time used to calculate the values.
(^2) The case when all derivatives are zero - the circuit is stable.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
206
```
```
electrical vo;
analog begin
V(vo) <+ Vgain * val;
end
endmodule
```
The output voltage V(vo) is reevaluated when any bit in val changes, which is not a problem if all the bits
change simultaneously and no ‘X’ values occur. A practical design would require that the digital value is
latched to avoid bad bit sequences, as in the following version:

```
module d2aC(clk,val,vo); // Clocked 16 bit D2A
parameter Vgain = 1.0/65536;
input clk;
input val;
wire [15:0] val;
electrical vo;
real v_clkd;
analog begin
@( posedge clk) v_clkd = Vgain * val;
V(vo) <+ v_clkd;
end
endmodule
```
Since _val_ is now guarded by the @(posedge clock) expression the **analog** block is not sensitive to
changes in val and only reevaluates when clk changes.

Macro processes can be evaluated separately but may be evaluated together^1 , in which case, the wake up
event for one process will cause the re-evaluation of all or some of the processes. Users should bear this in
mind when writing mixed-signal code, as it will mean that the code should be able to handle re-evaluation at
any time (not just at its own event times).

**8.4.3.3 A/D boundary timing**

In the analog kernel, time is a floating point value. In the digital kernel time is an integer value. Hence, A2D
events generally do not occur exactly at digital integer clock ticks.

For the purpose of reporting results and scheduling delayed future events, the digital kernel converts analog
event times to digital times such that the error is limited to half the precision base for the module where the
conversion occurs. For the examples below the timescale is 1ns/1ns, so the maximum scheduling error when
swapping a digital module for its analog counterpart will be 0.5ns.

Consequently an A2D event that results in a D2A event being scheduled with zero (0) delay, shall have its
effect propagated back to the analog kernel with zero ( 0 ) delay.

(^1) This is implementation-dependent.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
207
```
```
Figure 8-3: A zero delay inverter
```
If the circuit shown in Figure 8- 3 is being simulated with a digital time resolution of 1e-9 (one ( 1 ) nanosec-
ond) then all digital events shall be reported by the digital kernel as having occurred at an integer multiple of
1e-9. The A2D and D2A modules inserted are a simple level detector and a voltage ramp generator:

```
connectmodule a2d(i,o);
parameter vdd = 1.0;
ddiscrete o;
input i;
output o;
reg o;
electrical i;
always begin @( cross (V(i) - vdd/2,+1))o = 1; end
always begin @( cross (V(i) - vdd/2,-1))o = 0; end
endmodule
connectmodule d2a(i, o);
parameter vdd = 1.0;
parameter slewrate = 2.0/1e-9; // V/s
input i;
output o;
electrical o;
reg qd_val, // queued value
nw_val;
real et; // delay to event
real start_delay; // .. to ramp start
always @(driver_update i) begin
nw_val = $driver_next_state (i,0); // assume one driver
if (nw_val == qd_val) begin
// no change (assume delay constant)
end else begin
et = $driver_delay (i,0) * 1e-9; // real delay
qd_val = nw_val;
end
end
analog begin
@(qd_val) start_delay = et - (vdd/2)/slewrate;
V(o) <+ vdd * transition (qd_val,start_delay,vdd/slewrate);
end
endmodule
```
```
Connection modules
```
```
Zero delay inverter:
```
##### A B

```
always @(A) B<= !A;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
208
```
If connector A detects a positive threshold crossing, the resulting falling edge at connector B generated by the
propagation of the signal through verilog inverter model shall be reported to the analog kernel with no fur-
ther advance of analog time. The digital kernel will treat these events as if they occurred at the nearest nano-
second.

Example:

If A detects a positive crossing as a result of a transient solution at time 5.2e-9, the digital kernel shall report
a rising edge at A at time 5.0e-9 and falling edge at B at time 5.0e-9, but the analog kernel shall see the tran-
sition at B begin at time 5.2e-9, as shown in Figure 8- 4. D2As fed with zero delay events cannot be preemp-
tive, so the crossover on the return is delayed from the digital event; zero-delay inverters are not physically
realizable devices.

```
Figure 8-4: Zero delay transient solution times
```
If the inverter equation is changed to use a one unit delay (always @(A) B<= #1 !A), then the timing is as
in Figure 8- 5.

```
5 ns 6 ns
```
### A

### B

```
4 ns
```
```
analog
```
```
digital reported
```
```
analog
```
```
digital reported
```
```
signal
```
```
signal
```
```
digital real-time
```
```
digital real-time
```
```
Analog gate delay
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
209
```
```
Figure 8-5: Unit delay transient solution times
```
#### 8.4.4 The synchronization loop

Verilog-AMS uses a “conservative” simulation algorithm, the analog and digital processes that are managed
by the simulation kernel are synchronized such that neither computes results that will invalidate signal val-
ues that have already been assigned; time never goes backwards. While the implementation of the simulator
may have separate event queues for analog and digital events (see 8.4.5), it can be viewed as a single event
queue logically with a common global time. Analog processes are similar to Verilog _initial_ statements in
that they start automatically at time zero. The event sequence for the transient simulation shown in
Figure 8- 5 would be as follows:

```
Time Event Queue
4.9ns Evaluate the first analog inverter
Evaluate acceptance at 5.4ns, but schedule wake-up for
5.2 for crossing.
5.2ns Evaluate crossing event
The A2D logic sets the digital signal A, which triggers the evaluation
of the non-blocking assign to B, which schedules the actual
assignment for 6ns (rounded 1ns delay).
D2A notices queued event and schedules wake-up for 5.75 via
rampgen module.
Schedule wake-up at 5.4ns (as previously calculated).
```
```
5 ns 6 ns
```
### A

### B

```
4 ns
```
```
analog
```
```
digital reported
```
```
analog
```
```
digital reported
```
```
signal
```
```
signal
```
```
digital real-time
```
```
digital real-time
```
```
Analog gate delay
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
210
```
```
5.4ns Evaluate acceptance
Circuit evaluates stable, nothing scheduled.
5.75ns D2A/rampgen process wake-up
Start ramp in analog domain.
6.0ns Non blocking assign performed (digital event).
D2A may be sensitive, but doesn’t need to do anything.
6.25ns D2A/rampgen process wake-up
Drive 0V to complete ramp. Nothing more to schedule.
```
Any events queued ahead of the current global event time may be canceled. For instance, if the sequence
above is interrupted by a change on the primary input before digital assignment takes place as shown in
Figure 8- 6.

```
Time Event Queue
4.9ns Evaluating the first analog inverter
Evaluate acceptance at 5.4ns, but schedule wake-up
for 5.2 for crossing.
5.2ns Evaluate crossing event
The A2D logic sets the digital signal A, which triggers the
evaluation of the non-blocking assign to B, which schedules the
actual assignment for 6ns (rounded 1ns delay).
D2A notices queued event and changes value using transition filter.
Schedule wake-up at 5.4ns (as previously calculated).
5.3ns Analog event disturbs the solution
Accept at 5.3ns.
Cancel 5.4ns wake-up.
New acceptance is 5.45ns, but schedule wake-up for crossing at 5.4ns.
5.4ns Evaluate crossing event
The A2D logic sets the digital signal A, which triggers the evaluation
of the non-blocking assign to B, which schedules the actual
assignment for 6ns (rounded 1ns delay), canceling previous event.
D2A detects the driver change and qd_val toggles back to 1 before
the 0 propagates through the transition filter, so no analog change
occurs at B.
Schedule wake-up at 5.45ns (as previously calculated).
5.45ns Evaluate acceptance
Circuit evaluates stable, nothing scheduled.
6.00ns Non blocking assign performed (digital event).
Value of B doesn’t change.
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
211
```
```
Figure 8-6: Transient solution times with glitch
```
If the canceling event arrived after the ramp on B had started but before the assignment to the digital B, it is
possible to see the glitch propagate back into the analog domain without an event appearing on B.

#### 8.4.5 Synchronization and communication algorithm

Figure 8- 7 is an abstract representation of how the analog engine simulating an analog macro process com-
municates and synchronizes with the digital engine and vice-versa.

The synchronization algorithm can exploit characteristics of the analog and digital kernels described in the
next section. The arrows represent an engine moving from one synchronization point to another, which in
the case of an analog macro-process involves one or more time-steps and in the case of a digital engine,
involves once or more discrete times at which events are processed.

```
5 ns 6 ns
```
### A

### B

```
4 ns
```
```
analog
```
```
digital reported
```
```
analog
```
```
digital reported
```
```
signal
```
```
signal
```
```
digital real-time
```
```
digital real-time
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
212
```
```
Figure 8-7: Sample run
```
```
1) The analog engine begins transient analysis and sends state information (that it is good up to T2) to
the digital engine (1, 2).
2) The digital engine begins to run using its own time steps (3); however, if there is no D2A event, the
analog engine is not notified and the digital engine continues to simulate until it can not advance its
time without surpassing the time of the analog solution (4). Control of the simulation is then
returned to the analog engine (5), which accepts at T2. This process is repeated (7, 8, 9, 10, and 11).
3) If the digital engine produces a D2A event (12), control of the simulation is returned to the Analog
engine (13). The analog engine accepts at the time of the D2A event (14, which may involve recal-
culating from T3). The analog engine then calculates the next time step (15).
4) If the analog engine produces an A2D event, it returns control to the digital engine (16), which sim-
ulates up to the time of the A2D event, and then surrenders control (17 and 18).
5) This process continues until transient analysis is complete.
```
#### 8.4.6 absdelta interpolated A2D events

The **absdelta()** monitored event function allows the analog solver to generate A2D events by interpolat-
ing the times between the last time step and next time step at which the **absdelta** expression changed by
delta and schedules them in the digital event queue. The digital engine will then consume these events by
simulating up to either

```
a) the time of the next D2A event or
b) the time of the next time step in the analog engine. At this point, it will surrender control to the ana-
log engine.
```
In the case of a), unconsumed **absdelta** A2D events in the digital engine are rejected.

##### 1

##### 2

##### 34

##### 5

##### 6

##### 7

##### 8

##### 9

##### 10

##### 11

##### 12

##### 13

##### 14

##### 15

##### 16

##### 17

##### 18

```
etc.
```
##### D2A

##### A2D

##### T1 T2 T3 T4 T5 T6

##### ANALOG

##### DIGITAL


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
213
```
#### 8.4.7 Assumptions about the analog and digital algorithms

```
1) Advance of time in a digital algorithm
a) The digital engine has some minimum time granularity and all digital events occur at a time
which is some integer multiple of that granularity.
b) The digital engine can always accept events for a given simulation time provided it has not yet
executed events for a later time. Once it executes events for a given time, it can not accept
events for an earlier time.
c) The digital engine can always report the time of the most recently executed event and the time of
the next pending event.
2) Advance of time in an analog algorithm
a) The analog engine advances time by calculating a sequence of solutions. Each solution has an
associated time which, unlike the digital time, is not constrained to a particular minimum granu-
larity.
b) The analog engine can not tell for certain the time when the next solution converges. Thus, it can
tell the time of the most recently calculated solution, but not the time of the next solution.
c) In general, the analog solution is a function of one or more previous solutions. Having calcu-
lated the solution for a given time, the analog engine can either accept or reject that solution; it
cannot calculate a solution for a future time until it has accepted the solution for the current time.
3) Analog to digital events
a) Certain analog events ( above, cross , initial_step , and final_step ) cause an ana-
log solution of the time where they occur. Such events are associated with the solution that pro-
duced them until they are consumed by the digital engine. Until then, they can be rejected along
with the solution, if it is rejected.
b) absdelta analog to digital events can occur at times interpolated between two analog solution
times (that of the last analog solution and the next analog solution ). These events are associated
with the next analog solution. If the next analog solution is rejected, the absdelta events
associated with that solution, that have not be consumed by the digital engine, are rejected (see
8.4.6).
4) Digital to analog events shall cause an analog solution of the time where they occur.
```
### 8.5 Scheduling semantics for the digital engine

The scheduling semantics for Verilog-HDL simulation are outlined in Clause 11 of IEEE Std 1364 Verilog.

The digital engine of a Verilog-AMS mixed-signal simulator shall comply with that section except for the
changes outlined in this section.

For mixed-signal simulation, the major change from Clause 11 of IEEE Std 1364 Verilog is that two new
types of event must be supported by the event queue called the _explicit D2A_ (digital-to-analog) _event,_ and
the _analog macro-process event_.

Explicit D2A events are created when a digital event occurs to which an **analog** block is _explicitly sensi-
tive_. An **analog** block is explicitly sensitive to event expressions mentioned in an event control statement
in that **analog** block.

Similarly, there is also the concept of the _implicit D2A event_ that is created when a digital variable to which
an **analog** block is _implicitly sensitive_ changes value. An **analog** block is implicitly sensitive to all digi-
tal variable references that are not guarded by event control statements in that **analog** block.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
214
```
An analog macro-process event is also created when either type of D2A event occurs. The analog macro-
process event is associated with the analog macro-process that is sensitive to the D2A event. An analog
macro-process event is evaluated by calling the analog engine to solve it. Note that implicit D2A events are
not added to the stratified event queue, but as they directly cause an analog macro-process event, they effec-
tively force a digital-analog synchronization when they occur.

#### 8.5.1 The stratified event queue

The Verilog event queue is logically segmented into _seven_ different regions. Events are added to any of the
seven regions but are only removed from the _active_ region. Regions 1b and 3b have been added for mixed-
signal simulation.

1. Events that occur at the current simulation time and can be processed in any order.
    These are the _active_ events.
1b. Explicit D2A events that occur at the current simulation time shall be processed
    after all the active events are processed.
2. Events that occur at the current simulation time, but that shall be processed after all
    the active and explicit D2A events are processed. These are the _inactive_ events.
3. Events that have been evaluated during some previous simulation time, but that
    shall be assigned at this simulation time after all the active, explicit D2A and inac-
    tive events are processed. These are the non blocking assign update events.
3b. Analog macro-process events shall be processed after all active, explicit D2A
    events, inactive events and non blocking assign update events are processed.
4. Events that shall be processed after all the active, explicit D2A, inactive, non
    blocking assign update events and analog macro-process events are processed.
    These are the _monitor_ events.
5. Events that occur at some future simulation time. These are the _future_ events.
    Future events are divided into _future inactive events_ and _future non blocking_
    _assignment update events_.

The processing of all the active events is called a _simulation cycle_.

The freedom to choose any active event for immediate processing is an essential source of nondeterminism
in the IEEE Std 1364 Verilog.

An _explicit zero delay_ (#0) requires that the process be suspended and added as an inactive event for the cur-
rent time so that the process is resumed in the next simulation cycle in the current time.

A nonblocking assignment (see 9.2.2 of IEEE Std 1364 Verilog) creates a non blocking assign update event,
scheduled for current or a later simulation time.

The **$monitor** , **$strobe** and **$debug** system tasks (see 17.1 of IEEE Std 1364 Verilog) create monitor
events for their arguments. These events are continuously re-enabled in every successive time step. The
monitor events are unique in that they cannot create any other events.

The call back procedures scheduled with PLI routines such as tf_synchronize() (see Section 25.58 of
IEEE Std 1364 Verilog) or vpi_register_cb(cb_readwrite) (see 27.33 of IEEE Std 1364 Verilog)
shall be treated as inactive events.

Note that A2D events must be analog event controlled statements ( e.g., **@cross** , **@timer** ). These are
scheduled just like other event controlled statements in Verilog-HDL (e.g., **@posedge** ).


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
215
```
#### 8.5.2 The Verilog-AMS digital engine reference model

In all the examples that follow, T refers to the current simulation time of the digital engine, and all events are
held in the event queue, ordered by simulation time.

```
while (there are events){
if (no active events){
if (there are inactive events){
activate all inactive events;
else if (there are explicit D2A events) {
activate all explicit D2A events;
}else if (there are non blocking assign update events){
activate all non blocking assign update events;
}else if (there are analog macro-process events) {
activate all analog macro-process events;
}else if (there are monitor events){
activate all monitor events;
}else {
advance T to the next event time;
activate all inactive events for time T;
}
}
E =any active event;
if (E is an update event){
update the modified object;
add evaluation events for sensitive processes to event queue;
}else if (E is a D2A event) {
evaluate the D2A
modify the analog values
add A2D events to event queue, if any
}else if (E is an analog macro-process event) {
evaluate the analog macro-process
modify the analog values
add A2D events to event queue, if any
}else {/*shall be an evaluation event */
evaluate the process;
add update events to the event queue;
}
}
```
#### 8.5.3 Scheduling implication of assignments...........................................................................

Assignments are translated into processes and events as follows.

**8.5.3.1 Continuous assignment**

A continuous assignment statement (6.1 of IEEE Std 1364 Verilog) corresponds to a process, sensitive to the
source elements in the expression.When the value of the expression changes, it causes an active update event
to be added to the event queue, using current values to determine the target.

**8.5.3.2 Procedural continuous assignment**

A procedural continuous assignment (which is the **assign** or **force** statement; see 9.3 of IEEE Std 1364
Verilog) corresponds to a process that is sensitive to the source elements in the expression. When the value
of the expression changes, it causes an active update event to be added to the event queue, using current val-
ues to determine the target.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
216
```
A **deassign** or a **release** statement deactivates any corresponding **assign** or **force** statement(s).

**8.5.3.3 Blocking assignment**

A blocking assignment statement (see 9.2.1 of IEEE Std 1364 Verilog) with a delay computes the right-hand
side value using the current values, then causes the executing process to be suspended and scheduled as a
future event. If the delay is 0, the process is scheduled as an inactive event for the current time.

When the process is returned (or if it returns immediately if no delay is specified), the process performs the
assignment to the left-hand side and enables any events based upon the update of the left-hand side. The val-
ues at the time the process resumes are used to determine the target(s). Execution may then continue with the
next sequential statement or with other active events.

**8.5.3.4 Non blocking assignment**

A nonblocking assignment statement (see 9.2.2 of IEEE Std 1364 Verilog) always computes the updated
value and schedules the update as a nonblocking assign update event, either in this time step if the delay is
zero or as a future event if the delay is nonzero. The values in effect when the update is placed on the event
queue are used to compute both the right-hand value and the left-hand target.

**8.5.3.5 Switch (transistor) processing**

The event-driven simulation algorithm described in 11 of IEEE Std 1364 Verilog depends on unidirectional
signal flow and can process each event independently. The inputs are read, the result is computed, and the
update is scheduled. The IEEE Std 1364 Verilog provides switch-level modeling in addition to behavioral
and gate-level modeling. Switches provide bi-directional signal flow and require coordinated processing of
nodes connected by switches.

The IEEE Std 1364 Verilog source elements that model switches are various forms of transistors, called
**tran** , **tranif0** , **tranif1** , **rtran** , **rtranif0** , and **rtranif1**.

Switch processing shall consider all the devices in a bidirectional switch-connected net before it can deter-
mine the appropriate value for any node on the net, because the inputs and outputs interact. A simulator can
do this using a relaxation technique. The simulator can process tran at any time. It can process a subset of
tran-connected events at a particular time, intermingled with the execution of other active events. Further
refinement is required when some transistors have gate value x. A conceptually simple technique is to solve
the network repeatedly with these transistors set to all possible combinations of fully conducting and non-
conducting transistors. Any node that has a unique logic level in all cases has steady-state response equal to
this level. All other nodes have steady-state response.

**8.5.3.6 Processing explicit D2A events (region 1b)**

An explicit D2A event is processed by evaluating the **analog** block that is sensitive to this event. This is so
that the values used for the digital variables referenced inside the explicitly sensitive event control statement
in the analog block are the values of those variables after region 1 has been processed, not the values of
those variables just before region 3b is processed.

**8.5.3.7 Processing analog macro-process events (region 3b)**

An analog macro-process event is evaluated by calling the analog engine to solve the associated analog
macro-process. Note that if multiple events for a particular analog macro-process are active, then a single
evaluation of the analog macro-process shall consume all of these events from the queue.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
217
```
The reason for processing analog macro-processes after regions 1-3 have been processed is to minimize the
number of times analog macro-processes are evaluated, because such evaluations tend to be expensive.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
218
```
