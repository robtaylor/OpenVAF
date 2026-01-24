## 12. VPI routine definitions...........................................................................................................................

### 12.1 Overview......................................................................................................................................

```
This clause describes the Verilog Procedural Interface (VPI) routines, explaining their function, syntax, and
usage. The routines are listed in alphabetical order. The following conventions are used in the definitions of
the VPI routines.
```
```
Synopsis: A brief description of the PLI routine functionality, intended to be used as a quick reference when
searching for PLI routines to perform specific tasks.
```
```
Syntax: The exact name of the PLI routine and the order of the arguments passed to the routine.
```
```
Returns: The definition of the value returned when the PLI routine is called, along with a brief description
of what the value represents. The return definition contains the fields
```
```
Type: The data type of the C value which is returned. The data type is either a standard ANSI C type or a
special type defined within the PLI.
— Description: A brief description of what the value represents.
```
```
Arguments: The definition of the arguments passed with a call to the PLI routine. The argument definition
contains the fields
— Type: The data type of the C values which are passed as arguments. The data type is either a stan-
dard ANSI C type or a special type defined within the PLI.
— Name: The name of the argument used in the Syntax definition.
— Description: A brief description of what the value represents.
```
```
All arguments shall be considered mandatory unless specifically noted in the definition of the PLI routine.
Two tags are used to indicate arguments that might not be required:
— Conditional: Arguments tagged as conditional shall be required only if a previous argument is set to
a specific value or if a call to another PLI routine has configured the PLI to require the arguments.
The PLI routine definition explains when conditional arguments are required.
— Optional: Arguments tagged as optional can have default values within the PLI, but they can be
required if a previous argument is set to a specific value, or if a call to another PLI routine has con-
figured the PLI to require the arguments. The PLI routine definition explains the default values and
when optional arguments are required.
```
```
Related routines: A list of PLI routines which are typically used with, or provide similar functionality to,
the PLI routine being defined. This list is provided as a convenience to facilitate finding information in this
standard. It is not intended to be all-inclusive and it does not imply the related routines have to be used.
```
### 12.2 vpi_chk_error()

```
vpi_chk_error()
```
**Synopsis:** Retrieve information about VPI routine errors.

**Syntax:** vpi_chk_error(error_info_p)

```
Type Description
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
310
```
```
The VPI routine vpi_chk_error() shall return an integer constant representing an error
severity level if the previous call to a VPI routine resulted in an error. The error constants are shown in
Table 12- 1. If the previous call to a VPI routine did not result in an error, then vpi_chk_error() shall
return FALSE. The error status shall be reset by any VPI routine call except vpi_chk_error(). Calling
vpi_chk_error() shall have no effect on the error status.
```
```
If an error occurred, the s_vpi_error_info structure shall contain information about the error. If the error
information is not needed, a NULL can be passed to the routine. The s_vpi_error_info structure used by
vpi_chk_error() is defined in vpi_user.h and is listed in Figure 12- 1.
```
```
Figure 12-1: The s_vpi_error_info structure definition
```
### 12.3 vpi_compare_objects().................................................................................................................

**Returns:** int returns the error severity level if the previous VPI routine
call resulted in an error and FALSE if no error occurred

```
Type Name Description
```
**Arguments:** p_vpi_error_info error_info_p Pointer to a structure containing error information

```
Table 12-1—Return error constants for vpi_chk_error()
```
```
Error constant Severity level
```
```
vpiNotice lowest severity
vpiWarning
vpiError
vpiSystem
vpiInternal highest severity
```
```
vpi_compare_objects()
```
**Synopsis:** Compare two handles to determine if they reference the same object.

**Syntax:** vpi_compare_objects(obj1, obj2)

```
Type Description
```
```
vpi_chk_error()
```
```
typedef struct t_vpi_error_info {
int state; /* vpi[Compile,PLI,Run] */
int level; /* vpi[Notice, Warning, Error, System, Internal] */
char *message;
char *product;
char *code;
char *file;
int line;
} s_vpi_error_info, *p_vpi_error_info;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
311
```
```
The VPI routine vpi_compare_objects() shall return TRUE if the two handles refer to the same
object. Otherwise, FALSE shall be returned. Handle equivalence can not be determined with a C ‘==’ com-
parison.
```
### 12.4 vpi_free_object()..........................................................................................................................

```
The VPI routine vpi_free_object() shall free memory allocated for objects. It shall generally be used
to free memory created for iterator objects. The iterator object shall automatically be freed when vpi_s-
can() returns NULL either because it has completed an object traversal or encountered an error condition. If
neither of these conditions occur (which can happen if the code breaks out of an iteration loop before it has
scanned every object), vpi_free_object() needs to be called to free any memory allocated for the iter-
ator. This routine can also optionally be used for implementations which have to allocate memory for
objects. The routine shall return TRUE on success and FALSE on failure.
```
### 12.5 vpi_get()

**Returns:** bool true if the two handles refer to the same object. Otherwise, false

```
Type Name Description
```
**Arguments:** vpiHandle obj1 Handle to an object

```
vpiHandle obj2 Handle to an object
```
```
vpi_free_object()
```
**Synopsis:** Free memory allocated by VPI routines.

**Syntax:** vpi_free_object(obj)

```
Type Description
```
**Returns:** bool true on success and false on failure

```
Type Name Description
```
**Arguments:** vpiHandle obj Handle of an object

```
vpi_get()
```
**Synopsis:** Get the value of an integer or Boolean property of an object.

**Syntax:** vpi_get(prop, obj)

```
Type Description
```
**Returns:** int Value of an integer or Boolean property

```
Type Name Description
```
**Arguments:** int prop An integer constant representing the property of an object
for which to obtain a value

```
vpi_compare_objects()
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
312
```
```
The VPI routine vpi_get() shall return the value of object properties, for properties of type int and bool
( bool shall be defined to int ). Object properties of type bool shall return 1 for TRUE and 0 for FALSE. For
object properties of type int such as vpiSize , any integer shall be returned. For object properties of type
int which return a defined value, refer to Annex C of the IEEE Std 1364 Verilog specification for the value
that shall be returned. Note for object property vpiTimeUnit or vpiTimePrecision , if the object is
NULL, then the simulation time unit shall be returned. Should an error occur, vpi_get() shall return vpi-
Undefined.
```
### 12.6 vpi_get_cb_info().........................................................................................................................

```
The VPI routine vpi_get_cb_info() shall return information about a simulation-related callback in an
s_cb_data structure. The memory for this structure shall be allocated by the user.
```
```
The s_cb_data structure used by vpi_get_cb_info() is defined in vpi_user.h and is listed in
Figure 12- 2.
```
```
Figure 12-2: The s_cb_data structure definition
```
```
vpiHandle obj Handle to an object
```
**Related
routines:**

```
Use vpi_get_str() to get string properties
```
```
vpi_get_cb_info()
```
**Synopsis:** Retrieve information about a simulation-related callback.

**Syntax:** vpi_get_cb_info(obj, cb_data_p)

```
Type Description
```
**Returns:** void

```
Type Name Description
```
**Arguments:** vpiHandle obj Handle to a simulation-related callback

```
p_cb_data cb_data_p Pointer to a structure containing callback information
```
**Related
routines:**

```
Use vpi_get_systf_info() to retrieve information about a system task/function callback
```
```
vpi_get()
```
```
typedef struct t_cb_data {
int reason;
int (*cb_rtn)();
vpiHandle obj;
p_vpi_time time; /* structure with simulation time info */
p_vpi_value value;/* structure with simulation value info */
char *user_data; /* user data to be passed to callback function */
} s_cb_data, *p_cb_data;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
313
```
### 12.7 vpi_get_analog_delta()

```
The VPI routine vpi_get_analog_delta() shall be used determine the size of the analog time step
being attempted. It returns the elapsed time between the latest converged and accepted solution and the solu-
tion being calculated. The function shall return zero ( 0 ) during DC or the time zero transient solution.
```
### 12.8 vpi_get_analog_freq()..................................................................................................................

```
The VPI routine vpi_get_analog_freq() shall be used determine the current frequency used in the
small-signal analysis. The function shall return zero ( 0 ) during DC or transient analysis.
```
### 12.9 vpi_get_analog_time()

```
vpi_get_analog_delta()
```
**Synopsis:** Get the time elapsed since the previous solution.

**Syntax:** vpi_get_analog_delta()

```
Type true on success and false on failureDescription
```
**Returns:** double time elapsed between the solution being calculated and the last converged solution

```
Type Name Description
```
**Arguments:** NONE this function accepts no arguments

```
vpi_get_analog_freq()
```
**Synopsis:** Get the frequency for the current small-signal analysis.

**Syntax:** vpi_get_analog_freq()

```
Type true on success and false on failureDescription
```
**Returns:** double time elapsed between the solution being calculated and the last converged solution

```
Type Name Description
```
**Arguments:** NONE this function accepts no arguments

```
vpi_get_analog_time()
```
**Synopsis:** Get the time of the current solution.

**Syntax:** vpi_get_analog_time()

```
Type true on success and false on failureDescription
```
**Returns:** double time associated with the current solution

```
Type Name Description
```
**Arguments:** NONE this function accepts no arguments


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
314
```
```
The VPI routine vpi_get_analog_time() shall be used determine the time of the solution attempted
or of the latest converged and accepted solution otherwise. The function shall return zero ( 0 ) during DC or
the time zero transient solution.
```
### 12.10 vpi_get_analog_value()................................................................................................................

```
The VPI routine vpi_get_analog_value() shall retrieve the simulation value of VPI analog vpi-
Flow or vpiPotential (node or branch) quantity objects. The value shall be placed in an
s_vpi_analog_value structure, which has been allocated by the user. The format of the value shall be set
by the format field of the structure.
```
```
The buffer this routine uses for string values shall be different from the buffer which vpi_get_str()
shall use. The string buffer used by vpi_get_analog_value() is overwritten with each call. If the
value is needed, it needs to be saved by the application.
```
```
The s_vpi_analog_value structure used by vpi_get_analog_value() is defined in vpi_user.h
and listed in Figure 12- 3.
```
```
Figure 12-3: The s_vpi_analog_value structure definition
```
```
vpi_get_analog_value()
```
**Synopsis:** Retrieve the simulation value of an analog quantity object.

**Syntax:** vpi_get_analog_value(obj, value_p)

```
Type Description
```
**Returns:** void

```
Type Name Description
```
**Arguments:** vpiHandle obj Handle to an analog quantity object

```
p_vpi_value value_p Pointer to a structure containing value information
```
**Related
routines:**

```
Use vpi_get_value() to get simulation values of digital objects.
Use vpi_put_value() to set the value of an object
```
```
typedef struct t_vpi_analog_value {
int format; /* vpiRealVal,vpiExpStrVal,vpiDecStrVal,vpiStringVal
*/
union {
char *str;
double real;
char *misc;
} real;
union {
char *str;
double real;
char *misc;
} imaginary;
} s_vpi_analog_value, *p_vpi_analog_value;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
315
```
```
The memory for the union members str and misc of the value for real and imaginary unions in the
s_vpi_analog_value structure shall be provided by the routine vpi_get_analog_value(). This
memory shall only be valid until the next call to vpi_get_analog_value().
```
```
NOTE—The user shall provide the memory for these members when calling vpi_put_value().
```
### 12.11 vpi_get_delays()...........................................................................................................................

```
The VPI routine vpi_get_delays() shall retrieve the delays or pulse limits of an object and place them
in an s_vpi_delay structure which has been allocated by the user. The format of the delay information
shall be controlled by the time_type flag in the s_vpi_delay structure. This routine shall ignore the value
of the type flag in the s_vpi_time structure.
```
```
The s_vpi_delay and s_vpi_time structures used by both vpi_get_delays() and vpi_put_de-
lays() are defined in vpi_user.h and are listed in Figure 12- 4 and Figure 12- 5.
```
```
Table 12-2—Return value field of the s_vpi_analog_value structure union
```
```
Format Union members Return description
```
```
vpiDecStrVal str Real and imaginary values of object are returned as
strings of decimal char(s) [0–9]
vpExpStrVal str Real and imaginary values of object are returned as
strings formatted like printf %e.
vpiRealVal real Real and imaginary values of the object are returned as
doubles.
vpiStringVal str Real and imaginary parts are returned as strings for-
matted like printf %g. The call shall reset the format
field to vpiExpStrVal or vpiDecStrVal to the selected
format.
```
```
vpi_get_delays()
```
**Synopsis:** Retrieve the delays or pulse limits of an object.

**Syntax:** vpi_get_delays(obj, delay_p)

```
Type Description
```
**Returns:** void

```
Type Name Description
```
**Arguments:** vpiHandle obj Handle to an object

```
p_vpi_delay delay_p Pointer to a structure containing delay information
```
**Related
routines:**

```
Use vpi_put_delays() to set the delays or timing limits of an object
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
316
```
```
Figure 12-4: The s_vpi_delay structure definition
```
```
Figure 12-5: The s_vpi_time structure definition
```
The _da_ field of the s_vpi_delay structure shall be a user-allocated array of s_vpi_time structures. This
array shall store delay values returned by **vpi_get_delays()**. The number of elements in this array
shall be determined by

```
— The number of delays to be retrieved
— The mtm_flag setting
— The pulsere_flag setting
```
The number of delays to be retrieved shall be set in the _no_of_delays_ field of the s_vpi_delay structure.
Legal values for the number of delays shall be determined by the type of object.

```
— For primitive objects, the no_of_delays value shall be 2 or 3.
— For path delay objects, the no_of_delays value shall be 1 , 2 , 3 , 6 , or 12.
— For timing check objects, the no_of_delays value shall match the number of limits existing in the
timing check.
— For inter-module path objects, the no_of_delays value shall be 2 or 3.
```
The user-allocated s_vpi_delay array shall contain delays in the same order in which they occur in the
Verilog-AMS HDL description. The number of elements for each delay shall be determined by the flags
**mtm_flag** and **pulsere_flag** , as shown in Table 12- 3.

```
typedef struct t_vpi_delay {
struct t_vpi_time *da; /* ptr to user allocated array of delay
values */
int no_of_delays; /* number of delays */
int time_type; /* [vpiScaledRealTime, vpiSimTime] */
bool mtm_flag; /* true for mtm */
bool append_flag; /* true for append, false for replace */
bool pulsere_flag; /* true for pulsere values */
} s_vpi_delay, *p_vpi_delay;
```
```
typedef struct t_vpi_time
{
int type; /* [vpiScaledRealTime, vpiSimTime] */
unsigned int high, low; /* for vpiSimTime */
double real; /* for vpiScaledRealTime */
} s_vpi_time, *p_vpi_time;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
317
```
```
The delay structure has to be allocated before passing a pointer to vpi_get_delays().
```
```
In the following example, a static structure, prim_da , is allocated for use by each call to the
vpi_get_delays() function.
```
```
display_prim_delays(prim)
vpiHandle prim;t2
```
```
{
static s_vpi_time prim_da [3];
static s_vpi_delay delay_s = {NULL, 3, vpiScaledRealTime};
static p_vpi_delay delay_p = &delay_s;
```
```
delay_s.da = & prim_da ;
vpi_get_delays (prim, delay_p);
vpi_printf("Delays for primitive %s: %6.2f %6.2f %6.2f\n",
vpi_get_str(vpiFullName, prim)
delay_p->da[0].real, delay_p->da[1].real, delay_p->da[2].real);
}
```
```
Table 12-3—Size of the s_vpi_delay->da array
```
```
Flag values Number of required for s_vpi_time s_vpi_delay->da array elements Order in which delay elementsshall be filled
```
**mtm_flag** = false
**pulsere_flag** = false _no_of_delays_

1st delay: da[0] -> 1st delay
2nd delay: da[1] -> 2nd delay
...
**mtm_flag** = true
**pulsere_flag** = false 3 * _no_of_delays_

```
1st delay: da[0] -> min delay
da[1] -> typ delay
da[2] -> max delay
2nd delay: ...
mtm_flag = false
pulsere_flag = true 3 * no_of_delays
```
```
1st delay: da[0] -> delay
da[1] -> reject limit
da[2] -> error limit
2nd delay element: ...
mtm_flag = true
pulsere_flag = true 9 * no_of_delays
```
```
1st delay: da[0] -> min delay
da[1] -> typ delay
da[2] -> max delay
da[3] -> min reject
da[4] -> typ reject
da[5] -> max reject
da[6] -> min error
da[7] -> typ error
da[8] -> max error
2nd delay: ...
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
318
```
### 12.12 vpi_get_str()

```
The VPI routine vpi_get_str() shall return string property values. The string shall be placed in a tem-
porary buffer which shall be used by every call to this routine. If the string is to be used after a subsequent
call, the string needs to be copied to another location. A different string buffer shall be used for string values
returned through the s_vpi_value structure.
```
```
The following example illustrates the usage of vpi_get_str().
```
```
char *str;
vpiHandle mod = vpi_handle_by_name("top.mod1",NULL);
vpi_printf ("Module top.mod1 is an instance of %s\n",
vpi_get_str (vpiDefName, mod));
```
### 12.13 vpi_get_analog_systf_info()

```
vpi_get_str()
```
**Synopsis:** Get the value of a string property of an object.

**Syntax:** vpi_get_str(prop, obj)

```
Type Description
```
**Returns:** char * Pointer to a character string containing the property value

```
Type Name Description
```
**Arguments:** int prop An integer constant representing the property of an object
for which to obtain a value

```
vpiHandle obj Handle to an object
```
**Related
routines:**

```
Use vpi_get() to get integer and Boolean properties
```
```
vpi_get_analog_systf_info()
```
**Synopsis:** Retrieve information about a user-defined analog system task/function-related callback.

**Syntax:** vpi_get_analog_systf_info(obj, systf_data_p)

```
Type Description
```
**Returns:** void

```
Type Name Description
```
**Arguments:** vpiHandle obj Handle to a system task/function-related callback

```
p_vpi_analog_sy
stf_data
```
```
systf_data_p Pointer to a structure containing callback information
```
**Related
routines:**

```
Use vpi_get_cb_info() to retrieve information about a simulation-related callback
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
319
```
```
The VPI routine vpi_get_analog_systf_info() shall return information about a user-defined ana-
log system task or function callback in an s_vpi_analog_systf_data structure. The memory for this
structure shall be allocated by the user.
```
```
The s_vpi_systf_data structure used by vpi_get_analog_systf_info() is defined in
vpi_user.h and is listed in Figure 12- 6.
```
```
Figure 12-6: The s_vpi_systf_data structure definition
```
```
12.14 vpi_get_systf_info()
```
```
The VPI routine vpi_get_systf_info() shall return information about a user-defined system task or
function callback in an s_vpi_systf_data structure. The memory for this structure shall be allocated by
the user.
```
```
The s_vpi_systf_data structure used by vpi_get_systf_info() is defined in vpi_user.h and is
listed in Figure 12- 7.
```
```
vpi_get_systf_info()
```
**Synopsis:** Retrieve information about a user-defined system task/function-related callback.

**Syntax:** vpi_get_systf_info(obj, systf_data_p)

```
Type Description
```
**Returns:** void

```
Type Name Description
```
**Arguments:** vpiHandle obj Handle to a system task/function-related callback

```
p_vpi_systf_data systf_data_p Pointer to a structure containing callback information
```
**Related
routines:**

```
Use vpi_get_cb_info() to retrieve information about a simulation-related callback
```
```
typedef struct t_vpi_analog_systf_data {
int type; /* vpiSys[Task,Function] */
int sysfunctype; /* vpi[IntFunc,RealFunc,TimeFunc,SizedFunc] */
char *tfname; /* first character shall be “$” */
int (*calltf)();
int (*compiletf)();
int (*sizetf)(); /* for vpiSizedFunc system functions only */
p_vpi_stf_partials (*derivtf)(); /* for partial derivatives */
char *user_data;
} s_vpi_analog_systf_data, *p_vpi_analog_systf_data;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
320
```
```
Figure 12-7: The s_vpi_systf_data structure definition
```
### 12.15 vpi_get_time()..............................................................................................................................

```
The VPI routine vpi_get_time() shall retrieve the current simulation time, using the time scale of the
object. If obj is NULL, the simulation time is retrieved using the simulation time unit. The time_p->type field
shall be set to indicate if scaled real, analog, or simulation time is desired. The memory for the time_p struc-
ture shall be allocated by the user.
```
```
The s_vpi_time structure used by vpi_get_time() is defined in vpi_user.h and is listed in
Figure 12- 8 (this is the same time structure as used by vpi_put_value() ).
```
```
Figure 12-8: The s_vpi_time structure definition
```
```
vpi_get_time()
```
**Synopsis:** Retrieve the current simulation.

**Syntax:** vpi_get_time(obj, time_p)

```
Type Description
```
**Returns:** void

```
Type Name Description
```
**Arguments:** vpiHandle obj Handle to an object

```
p_vpi_time time_p Pointer to a structure containing time information
```
**Related
routines:**

```
typedef struct t_vpi_systf_data {
int type; /* vpiSys[Task,Function] */
int sysfunctype; /* vpi[IntFunc,RealFunc,TimeFunc,SizedFunc] */
char *tfname; /* first character shall be “$” */
int (*calltf)();
int (*compiletf)();
int (*sizetf)(); /* for vpiSizedFunc system functions only */
char *user_data;
} s_vpi_systf_data, *p_vpi_systf_data;
```
```
typedef struct t_vpi_time {
int type; /* for vpiScaledRealTime, vpiSimTime,
vpiAnalogTime */
unsigned int high, low; /* for vpiSimTime */
double real; /* for vpiScaledRealTime */
} s_vpi_time, *p_vpi_time;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
321
```
### 12.16 vpi_get_value()

```
The VPI routine vpi_get_value() shall retrieve the simulation value of VPI objects (use
vpi_get_analog_value() for the simulation value of VPI analog quantity objects). The value shall
be placed in an s_vpi_value structure, which has been allocated by the user. The format of the value shall
be set by the format field of the structure.
```
```
When the format field is vpiObjTypeVal , the routine shall fill in the value and change the format field
based on the object type, as follows:
— For an integer, vpiIntVal
— For a real, vpiRealVal
— For a scalar, either vpiScalar or vpiStrength
— For a time variable, vpiTimeVal with vpiSimTime
— For a vector, vpiVectorVal
```
```
The buffer this routine uses for string values shall be different from the buffer which vpi_get_str()
shall use. The string buffer used by vpi_get_value() is overwritten with each call. If the value is needed,
it needs to be saved by the application.
```
```
The s_vpi_value, s_vpi_vecval and s_vpi_strengthval structures used by vpi_get_value()
are defined in vpi_user.h and are listed in Figure 12- 9 , Figure 12- 10 , and Figure 12- 11.
```
```
vpi_get_value()
```
**Synopsis:** Retrieve the simulation value of an object.

**Syntax:** vpi_get_value(obj, value_p)

```
Type Description
```
**Returns:** void

```
Type Name Description
```
**Arguments:** vpiHandle obj Handle to an expression

```
p_vpi_value value_p Pointer to a structure containing value information
```
**Related
routines:**

```
Use vpi_get_analog_value() for simulation value of quantity objects.
Use vpi_put_value() to set the value of an object
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
322
```
```
Figure 12-9: The s_vpi_value structure definition
```
```
Figure 12-10: The s_vpi_vecval structure definition
```
```
Figure 12-11: The s_vpi_strengthval structure definition
```
For vectors, the _p_vpi_vecval_ field shall point to an array of s_vpi_vecval structures. The size of this
array shall be determined by the size of the vector, where _array_size = ((vector_size-1)/32 + 1)_. The lsb of
the vector shall be represented by the lsb of the 0 -indexed element of s_vpi_vecval array. The 33rd bit of
the vector shall be represented by the lsb of the 1 -indexed element of the array, and so on. The memory for
the union members _str_ , _time_ , _vector_ , _strength_ , and _misc_ of the value union in the s_vpi_value structure
shall be provided by the routine **vpi_get_value()**. This memory shall only be valid until the next call
to **vpi_get_value()**. (The user shall provide the memory for these members when calling
**vpi_put_value()** ). When a value change callback occurs for a value type of **vpiVectorVal** , the sys-
tem shall create the associated memory (an array of s_vpi_vecval structures) and free the memory upon
the return of the callback.

```
typedef struct t_vpi_value {
int format; /* vpi[[Bin,Oct,Dec,Hex]Str,Scalar,Int,Real,String,
Time,Vector,Strength,ObjType]Val*/
union {
char *str;
int scalar; /* vpi[0,1,X,Z] */
int integer;
double real;
struct t_vpi_time *time;
struct t_vpi_vecval *vector;
struct t_vpi_strengthval *strength;
char *misc;
} value;
} s_vpi_value, *p_vpi_value;
```
```
typedef struct t_vpi_vecval {
int aval, bval; /* bit encoding: ab: 00=0, 10=1, 11=X, 01=Z */
} s_vpi_vecval, *p_vpi_vecval;
```
```
typedef struct t_vpi_strengthval {
int logic; /* vpi[0,1,X,Z] */
int s0, s1; /* refer to strength coding in the LRM */
} s_vpi_strengthval, *p_vpi_strengthval;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
323
```
To get the ASCII values of UDP table entries (as explained in 8.1.6 _,_ Table 8-1 of IEEE Std 1364 Verilog),
the _p_vpi_vecval_ field shall point to an array of s_vpi_vecval structures. The size of this array shall be
determined by the size of the table entry (no. of symbols per table entry), where
_array_size = ((table_entry_size-1)/4 + 1)_. Each symbol shall require a byte; the ordering of the symbols
within s_vpi_vecval shall be the most significant byte of _abit_ first, then the least significant byte of _abit_ ,
then the most significant byte of _bbit_ , and then the least significant byte of _bbit_. Each symbol can be either
one or two characters; when it is a single character, the second half of the byte shall be an ASCII “\0”.

The _misc_ field in the s_vpi_value structure shall provide for alternative value types, which can be imple-
mentation specific. If this field is utilized, one or more corresponding format types shall also be provided.

In the following example, the binary value of each net which is contained in a particular module and whose
name begins with a particular string is displayed. (This function makes use of the **strcmp()** facility nor-
mally declared in a string.h C library.)

```
void display_certain_net_values(mod, target)
vpiHandle mod;
char *target;
{
```
```
Table 12-4—Return value field of the s_vpi_value structure union
```
```
Format Union member Return description
```
```
vpiBinStrVal str String of binary char(s) [ 1, 0, x, z ]
vpiOctStrVal str String of octal char(s) [ 0–7, x, X, z, Z ]
x When all the bits are x
X When some of the bits are x
z When all the bits are z
Z When some of the bits are z
vpiDecStrVal str String of decimal char(s) [0–9]
vpiHexStrVal str String of hex char(s) [ 0–f, x, X, z, Z ]
x When all the bits are x
X When some of the bits are x
z When all the bits are z
Z When some of the bits are z
vpiScalarVal scalar vpi1, vpi0, vpiX, vpiZ, vpiH, vpiL
vpiIntVal integer Integer value of the handle. Any bits x or z in the value
of the object are mapped to a 0
vpiRealVal real Value of the handle as a double
vpiStringVal str A string where each 8-bit group of the value of the
object is assumed to represent an ASCII character
vpiTimeVal time Integer value of the handle using two integers
vpiVectorVal vector aval/bval representation of the value of the object
vpiStrengthVal strength Value plus strength information of a scalar object only
vpiObjectVal — Return a value in the closest format of the object
NOTE—If the object has a real value, it shall be converted to an integer using the rounding defined by the Verilog-
AMS HDL before being returned in a format other than vpiRealVal.
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
324
```
```
static s_vpi_value value_s = {vpiBinStrVal};
static p_vpi_value value_p = &value_s;
vpiHandle net, itr;
```
```
itr = vpi_iterate(vpiNet, mod);
while (net = vpi_scan(itr))
{
char *net_name = vpi_get_str(vpiName, net);
if (strcmp(target, net_name) == 0)
{
vpi_get_value (net, value_p);
vpi_printf("Value of net %s: %s\n",
vpi_get_str(vpiFullName, net),value_p->value.str);
}
}
}
```
The following example illustrates the use of **vpi_get_value()** to access UDP table entries. Two sample
outputs from this example are provided after the example.

```
/*
* hUDP shall be a handle to a UDP definition
*/
static void dumpUDPTableEntries(vpiHandle hUDP)
```
```
{
vpiHandle hEntry, hEntryIter;
s_vpi_value value;
int numb;
int udpType;
int item;
int entryVal;
int *abItem;
int cnt, cnt2;
numb = vpi_get(vpiSize, hUDP);
udpType = vpi_get(vpiPrimType, hUDP);
if (udpType == vpiSeqPrim)
numb++; /* There is one more table entry for state */
numb++; /* There is a table entry for the output */
hEntryIter = vpi_iterate(vpiTableEntry, hUDP);
if (!hEntryIter)
return;
value.format = vpiVectorVal;
while(hEntry = vpi_scan(hEntryIter))
{
vpi_printf("\n");
/* Show the entry as a string */
value.format = vpiStringVal;
vpi_get_value(hEntry, &value);
vpi_printf("%s\n", value.value.str);
/* Decode the vector value format */
value.format = vpiVectorVal;
vpi_get_value(hEntry, &value);
abItem = (int *)value.value.vector;
for(cnt=((numb-1)/2+1);cnt>0;cnt--)
{
entryVal = *abItem;
abItem++;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
325
```
```
/* Rip out 4 characters */
for (cnt2=0;cnt2<4;cnt2++)
{
item = entryVal&0xff;
if (item)
vpi_printf("%c", item);
else
vpi_printf("_");
entryVal = entryVal>>8;
}
}
}
vpi_printf("\n");
}
```
For a UDP table of

```
1 0 :?:1;
0 (01) :?:-;
(10) 0 :0:1;
```
The output from the preceding example is

```
10:1
_0_1___1
01:0
_1_0___0
00:1
_0_0___1
```
For a UDP table entry of

```
1 0 :?:1;
0 (01) :?:-;
(10) 0 :0:1;
```
The output from the preceding example is

```
10:?:1
_0_1_1_?
0(01):?:-
10_0_-_?
(10)0:0:1
001_1_0
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
326
```
### 12.17 vpi_get_vlog_info()

```
The VPI routine vpi_get_vlog_info() shall obtain the following information about Verilog-AMS
product execution:
— The number of invocation options ( argc )
— Invocation option values ( argv )
— Product and version strings
```
```
The information shall be contained in an s_vpi_vlog_info structure. The routine shall return TRUE on
success and FALSE on failure.
```
```
The s_vpi_vlog_info structure used by vpi_get_vlog_info() is defined in vpi_user.h and is
listed in Figure 12- 12.
```
```
Figure 12-12: The s_vpi_vlog_info structure definition
```
```
vpi_get_vlog_info()
```
**Synopsis:** Retrieve information about Verilog-AMS simulation execution.

**Syntax:** vpi_get_vlog_info(vlog_info_p)

```
Type Description
```
**Returns:** bool true on success and false on failure

```
Type Name Description
```
**Arguments:** p_vpi_vlog_info vlog_info_p Pointer to a structure containing simulation information

```
typedef struct t_vpi_vlog_info {
int argc;
char **argv;
char *product;
char *version;
} s_vpi_vlog_info, *p_vpi_vlog_info;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
327
```
### 12.18 vpi_get_real()

```
The VPI routine vpi_get_real() shall return the value of object properties, for properties of type real.
Note for object properties shown below, if the object is NULL, then the corresponding value shall be
returned.
— vpiStartTime for beginning of transient analysis time
— vpiEndTime for end of transient analysis time
— vpiTransientMaxStep for maximum analog time step
— vpiStartFrequency for the start frequency of AC analysis
— vpiEndFrequency for the end frequency of AC analysis
```
```
This function is available to analog tasks and functions only. Should an error occur, vpi_get_real()
shall return vpiUndefined.
```
### 12.19 vpi_handle()

```
vpi_get_real()
```
**Synopsis:** Fetch a real property value associated with an object.

**Syntax:** vpi_get_real(prop,obj)

```
Type Description
```
**Returns:** double value of a real property

```
Type Name Description
```
**Arguments:** int prop An integer constant representing the property of an object
for which to obtain a value

```
vpiHandle obj Handle to an object
```
```
vpi_handle()
```
**Synopsis:** Obtain a handle to an object with a one-to-one relationship.

**Syntax:** vpi_handle(type, ref)

```
Type Description
```
**Returns:** vpiHandle Handle to an object

```
Type Name Description
```
**Arguments:** int type An integer constant representing the type of object for
which to obtain a handle

```
vpiHandle ref Handle to a reference object
```
**Related
routines:**

```
Use vpi_iterate() and vpi_scan() to obtain handles to objects with a one-to-many relationship
Use vpi_handle_multi() to obtain a handle to an object with a many-to-one relationship
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
328
```
```
The VPI routine vpi_handle() shall return the object of type type associated with object ref. The one-to-
one relationships which are traversed with this routine are indicated as single arrows in the data model dia-
grams.
```
```
The following example application displays each primitive that an input net drives.
```
```
void display_driven_primitives(net)
vpiHandle net;
{
vpiHandle load, prim, itr;
vpi_printf("Net %s drives terminals of the primitives: \n",
vpi_get_str(vpiFullName, net));
itr = vpi_iterate(vpiLoad, net);
if (!itr)
return;
while (load = vpi_scan(itr))
{
switch(vpi_get(vpiType, load))
{
case vpiGate:
case vpiSwitch:
case vpiUdp:
prim = vpi_handle (vpiPrimitive, load);
vpi_printf("\t%s\n", vpi_get_str(vpiFullName, prim));
}
}
}
```
### 12.20 vpi_handle_by_index()

```
The VPI routine vpi_handle_by_index() shall return a handle to an object based on the index num-
ber of the object within a parent object. This function can be used to access all objects which can access an
expression using vpiIndex. Argument obj shall represent the parent of the indexed object. For example, to
access a net-bit, obj is the associated net, while for a memory word, obj is the associated memory.
```
```
vpi_handle_by_index()
```
**Synopsis:** Get a handle to an object using its index number within a parent object.

**Syntax:** vpi_handle_by_index(obj, index)

```
Type Description
```
**Returns:** vpiHandle Handle to an object

```
Type Name Description
```
**Arguments:** vpiHandle obj Handle to an object

```
int index Index number of the object for which to obtain a handle
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
329
```
### 12.21 vpi_handle_by_name()

```
The VPI routine vpi_handle_by_name() shall return a handle to an object with a specific name. This
function can be applied to all objects with a fullname property. The name can be hierarchical or simple. If
scope is NULL, then name shall be searched for from the top level of hierarchy. Otherwise, name shall be
searched for from scope using the scope search rules defined by the Verilog-AMS HDL.
```
### 12.22 vpi_handle_multi().......................................................................................................................

```
The VPI routine vpi_handle_multi() shall return a handle to objects of type vpiInterModPath
associated with a list of output port and input port reference objects. The ports shall be of the same size and
can be at different levels of the hierarchy. This routine performs a many-to-one operation instead of the
usual one-to-one or one-to-many.
```
#### 12.22.1 Derivatives for analog system task/functions

```
The VPI routine vpi_handle_multi() is used to access the derivative handles associated with analog
system task/functions (see also: vpi_register_analog_systf() ). The first argument is the type
vpiDerivative. The second is the handle for the task/function argument for which a partial derivative is
```
```
vpi_handle_by_name()
```
**Synopsis:** Get a handle to an object with a specific name.

**Syntax:** vpi_handle_by_name(name, scope)

```
Type Description
```
**Returns:** vpiHandle Handle to an object

```
Type Name Description
```
**Arguments:** char * name A character string or pointer to a string containing the name
of an object

```
vpiHandle scope Handle to a Verilog-AMS HDL scope
```
```
vpi_handle_multi()
```
**Synopsis:** Obtain a handle to inter-module paths with a many-to-one relationship.

**Syntax:** vpi_handle_multi(type, ref1, ref2, ...)

```
Type Description
```
**Returns:** vpiHandle Handle to an object

```
Type Name Description
```
**Arguments:** int type An integer constant representing the type of object for
which to obtain a handle

```
vpiHandle ref1, ref2, ... Handles to two or more reference objects
```
**Related
routines:**

```
Use vpi_iterate() and vpi_scan() to obtain handles to objects with a one-to-many relationship
Use vpi_handle() to obtain handles to objects with a one-to-one relationship
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
330
```
to be declared. The third argument indicates the value with respect to which the derivative being declared
shall be calculated. For example, assuming argHandle2 and argHandle3 are handles to the second and
third arguments of an analog system task, then vpi_handle_multi(vpiDerivative, argHandle2,
argHandle3) indicates the partial derivative of the returned value with respect to the third argument. For
**vpiDerivative** , the **vpi_handle_multi()** function can only be called for those derivatives allo-
cated during the _derivtf_ phase of execution.

#### 12.22.2 Examples

The following example illustrates the declaration and use of derivative handles in an analog task $resis-
tor(), which implements a conductance relationship. The task can be used as follows:

```
module resistor(p, n);
electrical p, n;
parameter real r = 1k;
real curr;
analog begin
$resistor(curr, V(p, n), r);
I(p, n) <+ curr;
end
endmodule
```
The implementation of the analog task can be performed by the **resistor_compile_tf()** and
**resistor_call_tf()** routines shown below:

```
#include "vpiutils.h"
```
```
/* compiletf() */
static int resistor_compiletf(p_cb_data cb_data) {
vpiHandle funcHandle, i_handle, v_handle, r_handle, didv_handle;
int type;
s_vpi_value value;
double g;
p_resistor_data res;
```
```
/* Retrieve handle to current function */
funcHandle = vpi_handle(vpiSysTfCall, NULL);
```
```
/* Get the handle on the first function argument*/
i_handle = vpi_handle_by_index(funcHandle, 1);
```
```
/* Check that argument exists */
if (!i_handle) {
vpi_error("Not enough arguments for $resistor function.");
}
```
```
/* Check that argument #1 is a real variable */
type = vpi_get(vpiType, v_handle);
if (type != vpiRealVar) {
vpi_error("Arg #1 of $resistor should be a real variable");
return 1;
}
```
```
/* Get the handle on the second function argument*/
v_handle = vpi_handle_by_index(funcHandle, 2);
```
```
/* Check that argument exists */
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
331
```
```
if (!v_handle) {
vpi_error("Not enough arguments for $resistor function.");
return 1;
}
```
```
/* Check that argument #1 is a real valued */
type = vpi_get(vpiType, v_handle);
if (type != vpiRealVar && type != vpiRealVal) {
vpi_error("Arg #2 of $resistor should be a real variable");
return 1;
}
/* Get the handle on the third function argument*/
r_handle = vpi_handle_by_index(funcHandle, 3);
```
```
/* Check that argument exists */
if (!v_handle) {
vpi_error("Not enough arguments for $resistor function.");
return 1;
}
```
```
/* Check that argument #3 is real valued */
type = vpi_get(vpiType, r_handle);
if (type != vpiRealVar && type != vpiRealVal) {
vpi_error("Arg #3 of $resistor should be a real variable");
return 1;
}
```
return 0;
}

/* derivtf() */
static p_vpi_stf_partials resistor_derivtf(p_cb_data cb_data) {
static t_vpi_stf_partials derivs;
static int deriv_of[] = { 1 };
static int deriv_to[] = { 2 };

```
derivs.count = 1;
derivs.derivative_of = deriv_of;
derivs.derivative_to = deriv_to;
```
return &derivs;
}

/* load() */
static int resistor_calltf(int data, int reason) {
vpiHandle funcHandle, i_handle, v_handle, didv_handle;
double g;
s_vpi_value value;

```
/* Retrieve handle to current function */
funcHandle = vpi_handle(vpiSysTfCall, NULL);
i_handle = vpi_handle_by_index(funcHandle, 1);
v_handle = vpi_handle_by_index(funcHandle, 2);
didv_handle = vpi_handle_multi(vpiDerivative, i_handle, v_handle);
```
```
/* Get resistance value, compute conductance and store it as */
/* derivative */
value.format = vpiRealVal;
vpi_get_value(r_handle, &value);
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
332
```
```
g = 1.0 / value.value.real;
```
```
value.value.real = g;
vpi_put_value(didv_handle, &value, NULL, vpiNoDelay);
```
```
/* Get voltage value, compute current and store it into "I"*/
vpi_get_value(v_handle, &value);
value.value.real *= g;
vpi_put_value(i_handle, &value, NULL, vpiNoDelay);
return 0;
}
```
```
/*
* Public structure declaring the task
*/
static s_vpi_analog_systf_data resistor_systf = {
vpiSysAnalogTask, /* type: function / task */
0, /* returned type */
"$resistor", /* name */
resistor_calltf, /* calltf callback */
resistor_compiletf, /* compiletf callback */
0, /* unused: sizetf callback */
resistor_derivtf, /* derivtf callback */
0 /* user_data: nothing */
};
```
### 12.23 vpi_iterate()..................................................................................................................................

```
The VPI routine vpi_iterate() shall be used to traverse one-to-many relationships, which are indicated
as double arrows in the data model diagrams. The vpi_iterate() routine shall return a handle to an iter-
ator, whose type shall be vpiIterator , which can used by vpi_scan() to traverse all objects of type
type associated with object ref. To get the reference object from the iterator object use vpi_handle(vpi-
Use, iterator_handle). If there are no objects of type type associated with the reference handle ref ,
then the vpi_iterate() routine shall return NULL.
```
```
vpi_iterate()
```
**Synopsis:** Obtain an iterator handle to objects with a one-to-many relationship.

**Syntax:** vpi_iterate(type, ref)

```
Type Description
```
**Returns:** vpiHandle Handle to an iterator for an object

```
Type Name Description
```
**Arguments:** int type An integer constant representing the type of object for
which to obtain iterator handles

```
vpiHandle ref Handle to a reference object
```
**Related
routines:**

```
Use vpi_scan() to traverse the HDL hierarchy using the iterator handle returned from vpi_iterate()
Use vpi_handle() to obtain handles to object with a one-to-one relationship
Use vpi_handle_multi() to obtain a handle to an object with a many-to-one relationship
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
333
```
```
The following example application uses vpi_iterate() and vpi_scan() to display each net (includ-
ing the size for vectors) declared in the module. The example assumes it shall be passed a valid module han-
dle.
```
```
void display_nets(mod)
vpiHandle mod;
{
vpiHandle net;
vpiHandle itr;
```
```
vpi_printf("Nets declared in module %s\n",
vpi_get_str(vpiFullName, mod));
```
```
itr = vpi_iterate (vpiNet, mod);
while (net = vpi_scan (itr))
{
vpi_printf("\t%s", vpi_get_str(vpiName, net));
if (vpi_get(vpiVector, net))
{
vpi_printf(" of size %d\n", vpi_get(vpiSize, net));
}
else vpi_printf("\n");
}
}
```
### 12.24 vpi_mcd_close()...........................................................................................................................

```
The VPI routine vpi_mcd_close() shall close the file(s) specified by a multichannel descriptor, mcd.
Several channels can be closed simultaneously, since channels are represented by discrete bits in the integer
mcd. On success this routine returns a zero ( 0 ); on error it returns the mcd value of the unclosed channels.
```
```
The following descriptors are predefined and can not be closed using vpi_mcd_close() :
— descriptor 1 is stdout
— descriptor 2 is stderr
— descriptor 3 is the current log file
```
```
vpi_mcd_close()
```
**Synopsis:** Close one or more files opened by vpi_mcd_open().

**Syntax:** vpi_mcd_close(mcd)

```
Type Description
```
**Returns:** unsigned int 0 if successful, the mcd of unclosed channels if unsuccessful

```
Type Name Description
```
**Arguments:** unsigned int mcd A multichannel descriptor representing the files to close

**Related
routines:**

```
Use vpi_mcd_open() to open a file
Use vpi_mcd_printf() to write to an opened file
Use vpi_mcd_name() to get the name of a file represented by a channel descriptor
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
334
```
### 12.25 vpi_mcd_name()

```
The VPI routine vpi_mcd_name() shall return the name of a file represented by a single-channel descrip-
tor, cd. On error, the routine shall return NULL. This routine shall overwrite the returned value on subsequent
calls. If the application needs to retain the string, it shall copy it.
```
### 12.26 vpi_mcd_open()

```
The VPI routine vpi_mcd_open() shall open a file for writing and return a corresponding multichannel
descriptor number ( mcd ). The following channel descriptors are predefined and shall be automatically
opened by the system:
— Descriptor 1 is stdout
— Descriptor 2 is stderr
— Descriptor 3 is the current log file
```
```
The vpi_mcd_open() routine shall return a zero ( 0 ) on error. If the file is already opened, vpi_mc-
d_open() shall return the descriptor number.
```
```
vpi_mcd_name()
```
**Synopsis:** Get the name of a file represented by a channel descriptor.

**Syntax:** vpi_mcd_name(cd)

```
Type Description
```
**Returns:** char * Pointer to a character string containing the name of a file

```
Type Name Description
```
**Arguments:** unsigned int cd A single-channel descriptor representing a file

**Related
routines:**

```
Use vpi_mcd_open() to open a file
Use vpi_mcd_close() to close files
Use vpi_mcd_printf() to write to an opened file
```
```
vpi_mcd_open()
```
**Synopsis:** Open a file for writing.

**Syntax:** vpi_mcd_open(file)

```
Type Description
```
**Returns:** unsigned int A multichannel descriptor representing the file which was opened

```
Type Name Description
```
**Arguments:** char * file A character string or pointer to a string containing the file
name to be opened

**Related
routines:**

```
Use vpi_mcd_close() to close a file
Use vpi_mcd_printf() to write to an opened file
Use vpi_mcd_name() to get the name of a file represented by a channel descriptor
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
335
```
### 12.27 vpi_mcd_printf()

```
The VPI routine vpi_mcd_printf() shall write to one or more channels (up to 32) determined by the
mcd. An mcd of 1 (bit 0 set) corresponds to Channel 1 , a mcd of 2 (bit 1 set) corresponds to Channel 2, a
mcd of 4 (bit 2 set) corresponds to Channel 3 , and so on. Channel 1 is stdout , channel 2 is stderr , and chan-
nel 3 is the current log file. Several channels can be written to simultaneously, since channels are repre-
sented by discrete bits in the integer mcd. The format strings shall use the same format as the C
fprintf() routine.The routine shall return the number of characters printed or EOF if an error occurred.
```
### 12.28 vpi_printf()

```
The VPI routine vpi_printf() shall write to both stdout and the current product log file. The format
string shall use the same format as the C printf() routine. The routine shall return the number of charac-
ters printed or EOF if an error occurred.
```
```
vpi_mcd_printf()
```
**Synopsis:** Write to one or more files opened with vpi_mcd_open().

**Syntax:** vpi_mcd_printf(mcd, format, ...)

```
Type Description
```
**Returns:** int The number of characters written

```
Type Name Description
```
**Arguments:** unsigned int mcd A multichannel descriptor representing the files to which to
write

```
char * format A format string using the C fprintf() format
```
**Related
routines:**

```
Use vpi_mcd_open() to open a file
Use vpi_mcd_close() to close a file
Use vpi_mcd_name() to get the name of a file represented by a channel descriptor
```
```
vpi_printf()
```
**Synopsis:** Write to stdout and the current product log file.

**Syntax:** vpi_printf(format, ...)

```
Type Description
```
**Returns:** int The number of characters written

```
Type Name Description
```
**Arguments:** char * format A format string using the C printf() format

**Related
routines:**

```
Use vpi_mcd_printf() to write to an opened file
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
336
```
### 12.29 vpi_put_delays()

```
The VPI routine vpi_put_delays() shall set the delays or timing limits of an object as indicated in the
delay_p structure. The same ordering of delays shall be used as described in the vpi_get_delays()
function. If only the delay changes, and not the pulse limits, the pulse limits shall retain the values they had
before the delays where altered.
```
```
The s_vpi_delay and s_vpi_time structures used by both vpi_get_delays() and vpi_put_de-
lays() are defined in vpi_user.h and are listed in Figure 12- 13 and Figure 12- 14.
```
```
Figure 12-13: The s_vpi_delay structure definition
```
```
Figure 12-14: The s_vpi_time structure definition
```
```
The da field of the s_vpi_delay structure shall be a user-allocated array of s_vpi_time structures. This
array shall store the delay values to be written by vpi_put_delays(). The number of elements in this
array shall be determined by:
```
```
vpi_put_delays()
```
**Synopsis:** Set the delays or timing limits of an object.

**Syntax:** vpi_put_delays(obj, delay_p)

```
Type Description
```
**Returns:** void

```
Type Name Description
```
**Arguments:** vpiHandle obj Handle to an object

```
p_vpi_delay delay_p Pointer to a structure containing delay information
```
**Related
routines:**

```
Use vpi_get_delays() to retrieve delays or timing limits of an object
```
```
typedef struct t_vpi_delay {
struct t_vpi_time *da; /* ptr to user allocated array of delay
values */
int no_of_delays; /* number of delays */
int time_type; /* [vpiScaledRealTime, vpiSimTime] */
bool mtm_flag; /* true for mtm */
bool append_flag; /* true for append, false for replace */
bool pulsere_flag; /* true for pulsere values */
} s_vpi_delay, *p_vpi_delay;
```
```
typedef struct t_vpi_time
{
int type; /* [vpiScaledRealTime, vpiSimTime] */
unsigned int high, low; /* for vpiSimTime */
double real; /* for vpiScaledRealTime */
} s_vpi_time, *p_vpi_time;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
337
```
```
— The number of delays to be retrieved
— The mtm_flag setting
— The pulsere_flag setting
```
```
The number of delays to be retrieved shall be set in the no_of_delays field of the s_vpi_delay structure.
Legal values for the number of delays shall be determined by the type of object.
— For primitive objects, the no_of_delays value shall be 2 or 3.
— For path delay objects, the no_of_delays value shall be 1 , 2 , 3 , 6 , or 12.
— For timing check objects, the no_of_delays value shall match the number of limits existing in the
timing check.
— For inter-module path objects, the no_of_delays value shall be 2 or 3.
```
```
The user-allocated s_vpi_delay array shall contain delays in the same order in which they occur in the
Verilog-AMS HDL description. The number of elements for each delay shall be determined by the flags
mtm_flag and pulsere_flag , as shown in Table 12- 5.
```
```
The following example application accepts a module path handle, rise and fall delays, and replaces the
delays of the indicated path.
```
```
void set_path_rise_fall_delays(path, rise, fall)
vpiHandle path;
double rise, fall;
{
static s_vpi_time path_da[2];
static s_vpi_delay delay_s = {NULL, 2, vpiScaledRealTime};
static p_vpi_delay delay_p = &delay_s;
```
```
Table 12-5—Size of the s_vpi_delay->da array
```
```
Flag values Number of required for s_vpi_time s_vpi_delay->da array elements Order in which delay elementsshall be filled
```
**mtm_flag** = false
**pulsere_flag** = false _no_of_delays_

1st delay: da[0] -> 1st delay
2nd delay: da[1] -> 2nd delay
...
**mtm_flag** = true
**pulsere_flag** = false 3 * _no_of_delays_

```
1st delay: da[0] -> min delay
da[1] -> typ delay
da[2] -> max delay
2nd delay: ...
mtm_flag = false
pulsere_flag = true 3 * no_of_delays
```
```
1st delay: da[0] -> delay
da[1] -> reject limit
da[2] -> error limit
2nd delay element: ...
mtm_flag = true
pulsere_flag = true 9 * no_of_delays
```
```
1st delay: da[0] -> min delay
da[1] -> typ delay
da[2] -> max delay
da[3] -> min reject
da[4] -> typ reject
da[5] -> max reject
da[6] -> min error
da[7] -> typ error
da[8] -> max error
2nd delay: ...
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
338
```
```
delay_s.da = &path_da;
path_da[0].real = rise;
path_da[1].real = fall;
```
```
vpi_put_delays (path, delay_p);
}
```
### 12.30 vpi_put_value()

```
The VPI routine vpi_put_value() shall set simulation logic values on an object. The value to be set
shall be stored in an s_vpi_value structure which has been allocated. The delay time before the value is
set shall be stored in an s_vpi_time structure which has been allocated. The routine can be applied to nets,
regs, variables, memory words, system function calls, sequential UDPs, and schedule events. The flags argu-
ment shall be used to direct the routine to use one of the following delay modes:
vpiInertialDelay All scheduled events on the object shall be removed before this event is
scheduled.
vpiTransportDelay All events on the object scheduled for times later than this event shall be
removed (modified transport delay).
vpiPureTransportDelay No events on the object shall be removed (transport delay).
vpiNoDelay The object shall be set to the passed value with no delay. Argument
time_p shall be ignored and can be set to NULL.
vpiForceFlag The object shall be forced to the passed value with no delay (same as the
Verilog-AMS HDL procedural force ). Argument time_p shall be
ignored and can be set to NULL.
vpiReleaseFlag The object shall be released from a forced value (same as the Verilog-
AMS HDL procedural release ). Argument time_p shall be ignored
and can be set to NULL. The value_p shall contain the current value of the
object.
vpiCancelEvent A previously scheduled event shall be canceled. The object passed to
vpi_put_value() shall be a handle to an object of type
vpiSchedEvent.
```
```
vpi_put_value()
```
**Synopsis:** Set a value on an object.

**Syntax:** vpi_put_value(obj, value_p, time_p, flags)

```
Type Description
```
**Returns:** vpiHandle Handle to the scheduled event caused by vpi_put_value()

```
Type Name Description
```
**Arguments:** vpiHandle obj Handle to an object

```
p_vpi_value value_p Pointer to a structure with value information
```
```
p_vpi_time time_p Pointer to a structure with delay information
```
```
int flags Integer constants which set the delay mode
```
**Related
routines:**

```
Use vpi_get_value() to retrieve the value of an expression
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
339
```
If the _flags_ argument also has the bit mask **vpiReturnEvent** , **vpi_put_value()** shall return a handle
of type **vpiSchedEvent** to the newly scheduled event, provided there is some form of a delay and an
event is scheduled. If the bit mask is not used, or if no delay is used, or if an event is not scheduled, the
return value shall be NULL.

The handle to the event can be canceled by calling **vpi_put_value()** with the flag set to **vpiCan-
celEvent**. It shall not be an error to cancel an event which has already occurred. The scheduled event can
be tested by calling **vpi_get()** with the flag **vpiScheduled**. If an event is canceled, it shall simply be
removed from the event queue. Any effects which were caused by scheduling the event shall remain in
effect (e.g., events which were canceled due to inertial delay).

Calling **vpi_free_object()** on the handle shall free the handle but shall not effect the event.

Sequential UDPs shall be set to the indicated value with no delay regardless of any delay on the primitive
instance.

NOTE— **vpi_put_value()** shall only return a function value in a **calltf** application, when the call to the
function is active. The action of **vpi_put_value()** to a function shall be ignored when the function is not active.

The s_vpi_value and s_vpi_time structures used by **vpi_put_value()** are defined in vpi_user.h
and are listed in Figure 12- 15 and Figure 12- 16.

```
Figure 12-15: The s_vpi_value structure definition
```
```
Figure 12-16: The s_vpi_time structure definition
```
For **vpiScaledRealTime** , the indicated time shall be in the timescale associated with the object.

```
typedef struct t_vpi_value {
int format; /* vpi[[Bin,Oct,Dec,Hex]Str,Scalar,Int,Real,String,
Time,Vector,Strength,ObjType]Val*/
union {
char *str;
int scalar; /* vpi[0,1,X,Z] */
int integer;
double real;
struct t_vpi_time *time;
struct t_vpi_vecval *vector;
struct t_vpi_strengthval *strength;
char *misc;
} value;
} s_vpi_value, *p_vpi_value;
```
```
typedef struct t_vpi_time {
int type; /* for vpiScaledRealTime, vpiSimTime */
unsigned int high, low; /* for vpiSimTime */
double real; /* for vpiScaledRealTime */
} s_vpi_time, *p_vpi_time;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
340
```
### 12.31 vpi_register_cb()

```
The VPI routine vpi_register_cb() is used for registration of simulation-related callbacks to a user-
provided application for a variety of reasons during a simulation. The reasons for which a callback can occur
are divided into three categories:
— Simulation event
— Simulation time
— Simulation action or feature
```
```
How callbacks are registered for each of these categories is explained in the following paragraphs.
```
```
The cb_data_p argument shall point to a s_cb_data structure, which is defined in vpi_user.h and given
in Figure 12- 17.
```
```
Figure 12-17: The s_cb_data structure definition
```
```
For all callbacks, the reason field of the s_cb_data structure shall be set to a predefined constant, such as
cbValueChange , cbAtStartOfSimTime , cbEndOfCompile , etc. The reason constant shall deter-
mine when the user application shall be called back. Refer to the vpi_user.h file listing in Annex G of the
IEEE Std 1364 Verilog specification for a list of all callback reason constants.
```
```
The cb_rtn field of the s_cb_data structure shall be set to the application routine name, which shall be
invoked when the simulator executes the callback. The use of the remaining fields are detailed in the follow-
ing sub clauses.
```
```
vpi_register_cb()
```
**Synopsis:** Register simulation-related callbacks.

**Syntax:** vpi_register_cb(cb_data_p)

```
Type Description
```
**Returns:** vpiHandle Handle to the callback object

```
Type Name Description
```
**Arguments:** p_cb_data cb_data_p Pointer to a structure with data about when callbacks
should occur and the data to be passed

**Related
routines:**

```
Use vpi_register_systf() to register callbacks for user-defined system tasks and functions
Use vpi_remove_cb() to remove callbacks registered with vpi_register_cb()
```
```
typedef struct t_cb_data {
int reason;
int (*cb_rtn)();
vpiHandle obj;
p_vpi_time time; /* structure defined in vpi_user.h */
p_vpi_value value; /* structure defined in vpi_user.h */
int index; /* index of memory word or var select which changed */
char *user_data; /* user data to be passed to callback function */
} s_cb_data, *p_cb_data;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
341
```
#### 12.31.1 Simulation-event-related callbacks

The **vpi_register_cb()** callback mechanism can be registered for callbacks to occur for simulation
events, such as value changes on an expression or terminal, or the execution of a behavioral statement.
When the _cb_data_p->reason_ field is set to one of the following, the callback shall occur as described
below:

```
cbValueChange After value change on an expression or terminal
cbStmt Before execution of a behavioral statement
cbForce/cbRelease After a force or release has occurred
cbAssign/cbDeassign After a procedural assign or deassign statement has been executed
cbDisable After a named block or task containing a system task or function has
been disabled
```
The following fields shall need to be initialized before passing the s_cb_data structure to **vpi_regis-
ter_cb()** :

```
cb_data_p->obj This field shall be assigned a handle to an expression, terminal, or state-
ment for which the callback shall occur. For force and release callbacks,
if this is set to NULL, every force and release shall generate a callback.
cb_data_p->time->type This field shall be set to either vpiScaledRealTime or vpiSim-
Time , depending on what time information the user application requires
during the callback. If simulation time information is not needed during
the callback, this field can be set to vpiSuppressTime.
cb_data_p->value->format This field shall be set to one of the value formats indicated in Table 12- 6.
If value information is not needed during the callback, this field can be
set to vpiSuppressVal. For cbStmt callbacks, value information is
not passed to the callback routine, so this field shall be ignored.
```
When a simulation event callback occurs, the user application shall be passed a single argument, which is a
pointer to an s_cb_data structure (this is not a pointer to the same structure which was passed to

```
Table 12-6—Value format field of cb_data_p->value->format
```
```
Format Registers a callback to return
```
```
vpiBinStrVal String of binary char(s) [ 1, 0, x, z ]
vpiOctStrVal String of octal char(s) [ 0–7, x, X, z, Z ]
vpiDecStrVal String of decimal char(s) [0–9]
vpiHexStrVal String of hex char(s) [ 0–f, x, X, z, Z ]
vpiScalarVal vpi1, vpi0, vpiX, vpiZ, vpiH, vpiL
vpiIntVal Integer value of the handle
vpiRealVal Value of the handle as a double
vpiStringVal An ASCII string
vpiTimeVal Integer value of the handle using two integers
vpiVectorVal aval/bval representation of the value of the object
vpiStrengthVal Value plus strength information of a scalar object only
vpiObjectVal Return a value in the closest format of the object
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
342
```
**vpi_register_cb()** ). The _time_ and _value_ information shall be set as directed by the time _type_ and
_value_ format fields in the call to **vpi_register_cb()**. The _user_data_ field shall be equivalent to the
_user_data_ field passed to **vpi_register_cb()**. The user application can use the information in the
passed structure and information retrieved from other VPI interface routines to perform the desired callback
processing.

For a **cbValueChange** callback, if the _obj_ is a memory word or a variable array, the _value_ in the s_cb_-
data structure shall be the value of the memory word or variable select which changed value. The _index_
field shall contain the index of the memory word or variable select which changed value.

For **cbForce** , **cbRelease** , **cbAssign** , and **cbDeassign** callbacks, the object returned in the _obj_ field
shall be a handle to the force, release, assign or deassign statement. The _value_ field shall contain the resul-
tant value of the LHS expression. In the case of a release, the _value_ field shall contain the value after the
release has occurred.

The following example shows an implementation of a simple monitor functionality for scalar nets, using a
simulation-event-related callback.

```
setup_monitor(net)
vpiHandle net;
{
static s_vpi_time time_s = {vpiScaledRealTime};
static s_vpi_value value_s = {vpiBinStrVal};
static s_cb_data cb_data_s =
{cbValueChange, my_monitor, NULL, &time_s, &value_s};
char *net_name = vpi_get_str(vpiFullName, net);
cb_data_s.obj = net;
cb_data_s.user_data = malloc(strlen(net_name)+1);
strcpy(cb_data_s.user_data, net_name);
vpi_register_cb (&cb_data_s);
}
```
```
my_monitor(cb_data_p)
p_cb_data cb_data_p; {
vpi_printf("%d %d: %s = %s\n",
cb_data_p->time->high, cb_data_p->time->low,
cb_data_p->user_data,
cb_data_p->value->value.str);
}
```
#### 12.31.2 Simulation-time-related callbacks...................................................................................

The **vpi_register_cb()** can register callbacks to occur for simulation time reasons, include callbacks
at the beginning or end of the execution of a particular time queue. The following time-related callback rea-
sons are defined:

```
cbAtStartOfSimTime Callback shall occur before execution of events in a specified time
queue. A callback can be set for any time, even if no event is present.
cbReadWriteSynch Callback shall occur after execution of events for a specified time.
cbReadOnlySynch Same as cbReadWriteSynch , except writing values or scheduling
events before the next scheduled event is not allowed.
cbNextSimTime Callback shall occur before execution of events in the next event queue.
cbAfterDelay Callback shall occur after a specified amount of time, before execution of
events in a specified time queue. A callback can be set for anytime, even
if no event is present.
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
343
```
The following fields shall need to be set before passing the s_cb_data structure to **vpi_regis-
ter_cb()** :

```
cb_data_p->time->type This field shall be set to either vpiScaledRealTime or vpiSim-
Time , depending on what time information the user application requires
during the callback.
cb_data_p->[time->low,time->high,time->real]
These fields shall contain the requested time of the callback or the delay
before the callback.
```
The _value_ fields are ignored for all reasons with simulation-time-related callbacks.

When the _cb_data_p->time->type_ is set to **vpiScaledRealTime** , the _cb_data_p->obj_ field shall be used
as the object for determining the time scaling.

For reason **cbNextSimTime** , the time structure is ignored.

When a simulation-time-related callback occurs, the user callback application shall be passed a single argu-
ment, which is a pointer to an s_cb_data structure (this is not a pointer to the same structure which was
passed to **vpi_register_cb()** ). The _time_ structure shall contain the current simulation time. The _user_-
data_ field shall be equivalent to the _user_data_ field passed to **vpi_register_cb()**.

The callback application can use the information in the passed structure and information retrieved from
other interface routines to perform the desired callback processing.

#### 12.31.3 Simulator analog and related callbacks...........................................................................

The **vpi_register_cb()** callback mechanism can be registered for callbacks to occur for analog simu-
lation events, such as acceptance of the initial or final analog solution. When the _cb_data_p->reason_ field is
set to one of the following, the callback shall occur as described below:

```
acbInitialStep Upon acceptance of the first analog solution
acbFinalStep Upon acceptance of the last analog solution
acbAbsTime Upon acceptance of the analog solution for the given time (this callback
shall force a solution at that time)
acbElapsedTime Upon acceptance of the solution advanced from the current solution by
the given interval (this callback shall force a solution at that time)
acbConvergenceTest Prior acceptance of the analog solution for the given time (this callback
allows rejection of the analog solution at that time and backup to an ear-
lier time)
acbAcceptedPoint Upon acceptance of the solution at the given time
```
#### 12.31.4 Simulator action and feature related callbacks

The **vpi_register_cb()** can register callbacks to occur for simulator action reasons or simulator fea-
ture reasons. _Simulator action reasons_ are callbacks such as the end of compilation or end of simulation.
_Simulator feature reasons_ are software-product-specific features, such as restarting from a saved simulation
state or entering an interactive mode. Actions are differentiated from features in that actions shall occur in
all VPI-compliant products, whereas features might not exist in all VPI-compliant products.

The following action-related callbacks shall be defined:

```
cbEndOfCompile End of simulation data structure compilation or build
cbStartOfSimulation Start of simulation (beginning of time 0 simulation cycle)
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
344
```
```
cbEndOfSimulation End of simulation (e.g., $finish system task executed)
cbError Simulation run-time error occurred
cbPLIError Simulation run-time error occurred in a PLI function call
cbTchkViolation Timing check error occurred
```
Examples of possible feature related callbacks are

```
cbStartOfSave Simulation save state command invoked
cbEndOfSave Simulation save state command completed
cbStartOfRestart Simulation restart from saved state command invoked
cbEndOfRestart Simulation restart command completed
cbEnterInteractive Simulation entering interactive debug mode (e.g., $stop system task
executed)
cbExitInteractive Simulation exiting interactive mode
cbInteractiveScopeChange Simulation command to change interactive scope executed
cbUnresolvedSystf Unknown user-defined system task or function encountered
```
The only fields in the s_cb_data structure which need to be setup for simulation action/feature callbacks
are the _reason_ , _cb_rtn_ , and _user_data_ (if desired) fields.

When a simulation action/feature callback occurs, the user routine shall be passed a pointer to an s_cb_-
data structure. The _reason_ field shall contain the reason for the callback. For **cbTchkViolation** call-
backs, the _obj_ field shall be a handle to the timing check. For **cbInteractiveScopeChange** , _obj_ shall
be a handle to the new scope. For **cbUnresolvedSystf** , _user_data_ shall point to the name of the unre-
solved task or function. On a **cbError** callback, the routine **vpi_chk_error()** can be called to retrieve
error information.

The following example shows a callback application which reports cpu usage at the end of a simulation. If
the user routine setup_report_cpu() is placed in the vlog_startup_routines list, it shall be called
just after the simulator is invoked.

```
static int initial_cputime_g;
```
```
void report_cpu()
{
int total = get_current_cputime() - initial_cputime_g;
vpi_printf("Simulation complete. CPU time used: %d\n", total);
}
```
```
void setup_report_cpu()
{
static s_cb_data cb_data_s = {cbEndOfSimulation, report_cpu};
initial_cputime_g = get_current_cputime();
vpi_register_cb (&cb_data_s);
}
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
345
```
### 12.32 vpi_register_analog_systf()..........................................................................................................

```
The VPI routine vpi_register_analog_systf() shall register callbacks for user-defined analog
system tasks or functions. Callbacks can be registered to occur when a user-defined system task or function
is encountered during compilation or execution of analog Verilog-AMS HDL source code. Tasks or func-
tions can be registered with either the analog or digital domain. The registration function ( vpi_regis-
ter_analog_systf() or vpi_register_systf() ) with which the task or function is registered
shall determine the context or contexts from which the task or function can be invoked and how and when
the call backs associated with the function shall be called. The task or function name shall be unique in the
domain in which it is registered. That is, the same name can be shared by two sets of callbacks, provided that
one set is registered in the digital domain and the other is registered in the analog.
```
```
The systf_analog_data_p argument shall point to a s_vpi_systf_analog_data structure, which is
defined in vpi_user.h and listed in Figure 12- 18.
```
```
Figure 12-18: The s_vpi_analog_systf_data structure definition
```
#### 12.32.1 System task and function callbacks

```
User-defined Verilog-AMS system tasks and functions which use VPI routines can be registered with
vpi_register_systf() or vpi_register_analog_systf(). The calltf , compiletf , and sizetf
system task/function-related callbacks are defined in vpi_register_systf().
```
```
vpi_register_analog_systf()
```
**Synopsis:** Register user-defined system task/function-related callbacks.

**Syntax:** vpi_register_analog_systf(systf_data_p)

```
Type Description
```
**Returns:** vpiHandle Handle to the callback object

```
Type Name Description
```
**Arguments:** p_vpi_analog_sy
stf_data

```
systf_analog_data_p Pointer to a structure with data about when callbacks
should occur and the data to be passed
```
**Related
routines:**

```
Use vpi_register_systf() to register digital domain system tasks/functions.
Use vpi_register_cb() to register callbacks for simulation-related events
```
```
typedef struct t_vpi_systf_analog_data {
int type; /* vpiAnalogSysTask,vpiAnalogSysFunc */
int sysfunctype; /* vpi[IntFunc,RealFunc] */
char *tfname; /* first character shall be “$” */
int (*calltf)();
int (*compiletf)();
int (*sizetf)(); /* for vpiSizedFunc system functions only */
p_vpi_stf_partials (*derivtf)(); /* for partial derivatives */
char *user_data;
} s_vpi_analog_systf_data, *p_vpi_analog_systf_data;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
346
```
The _type_ field of the s_vpi_systf_data structure shall register the user application to be a system task or
a system function. The _type_ field value shall be an integer constant of **vpiAnalogSysTask** or **vpiAna-
logSysFunction**.

The _sysfunctype_ field of the s_vpi_analog_systf_data structure shall define the type of value which a
system function shall return. The _sysfunctype_ field shall be an integer constant of **vpiIntFunc** of **vpiRe-
alFunc**. This field shall only be used when the _type_ field is set to **vpiAnalogSysFunction**.

The _compiletf_ , _calltf_ , _sizetf_ , and _derivtf_ fields of the s_vpi_analog_systf_data structure shall be point-
ers to the user-provided applications which are to be invoked by the system task/function callback mecha-
nism. One or more of the _compiletf_ , _calltf_ , _sizetf_ , and _derivtf_ fields can be set to NULL if they are not needed.
Callbacks to the applications pointed to by the _compiletf_ and _sizetf_ fields shall occur when the simulation
data structure is compiled or built (or for the first invocation if the system task or function is invoked from
an interactive mode). Callbacks to the applications pointed to by the _derivtf_ fields shall occur when register-
ing partial derivatives for the analog system task/function arguments or return value. Callbacks to the appli-
cation pointed to by the _calltf_ routine shall occur each time the system task or function is invoked during
simulation execution.

The _user_data_ field of the s_vpi_analog_systf_data structure shall specify a user-defined value, which
shall be passed back to the _compiletf_ , _sizetf_ , _derivtf_ , and _calltf_ applications when a callback occurs.

The usage of the _compiletf_ , _sizetf_ , and _calltf_ routines for the analog system task/function are identical to
those of digital system task/functions registered with **vpi_register_systf()**. Refer to the description
of **vpi_register_systf()** for more information.

#### 12.32.2 Declaring derivatives for analog system task/functions

Analog system tasks and functions require partial derivatives of the outputs (arguments for system tasks and
the return value for system functions). Thus it is possible (though not necessary) to have a partial derivative
of the returned value with respect to any or all of the arguments and a partial derivative of any particular
argument with respect to any or all of the other arguments.

The _derivtf_ field of the t_vpi_analog_systf_data structure can be called during the build process (sim-
ilar to _sizetf_ ) and returns a pointer to a t_vpi_stf_partials data structure containing the required infor-
mation. The purpose of this function is declarative only, it does not assign any value to the derivative being
declared. Having declared a partial derivative using this function in the _derivtf_ callback, values can then be
contributed to the derivative using the vpi_put_value function in the _calltf_ call back.

The t_vpi_stf_partials data structure is defined:

```
typedef struct t_vpi_stf_partials {
int count;
int *derivative_of; /* 0 = returned value, 1 = 1st arg, etc. */
int *derivative_wrt; /* 1 = 1st arg, 2 = 2nd arg, etc. */
} s_vpi_stf_partials, *p_vpi_stf_partials;
```
This data structure declares the derivative objects for the associated analog task/function. During the _call_tf_
phase, their handles can be retrieved via calls to **vpi_handl_multi()**.

#### 12.32.3 Examples

The following example illustrates the declaration and use of callbacks in an analog function $sampler()
which implements a sample and hold. The task is used as follows:


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
347
```
```
module sampnhold(out, in);
electrical out, in;
parameter real period = 1e-3;
analog begin
V(out) <+ $sampler (V(in), period);
end
endmodule
```
The VPI implementation of the sampler is as follows:

```
typedef struct {
vpiHandle returnHandle; /* Arg #0 (returned value) */
vpiHandle exprHandle; /* Arg #1 (sampled expression) */
double period; /* Arg #2 (static period expression) */
s_cb_data cb_data; /* callback structure */
s_vpi_value value;
/* value structure (holds the expression value) */
} s_sampler_data, *p_sampler_data;
```
```
/* Forward declarations */
static int sampler_callback(p_cb_data data);
static void schedule_callback(p_sampler_data sampler, double currTime);
```
```
/* compiletf() */
static int sampler_compiletf(p_cb_data task_cb_data) {
vpiHandle functionHandle, returnHandle, exprHandle, periodHandle;
s_cb_data cb_data;
int type;
p_sampler_data sampler;
s_vpi_value value;
```
```
/* Retrieve handle to current function */
functionHandle = vpi_handle(vpiSysTfCall, NULL);
```
```
/* Get the handle on the expression argument*/
exprHandle = vpi_handle_by_index(functionHandle, 1);
/* Check that expression argument exists */
if (!exprHandle) {
vpi_error("Not enough arguments for $sampler function.");
}
```
```
/* Check that expression argument is of real value */
type = vpi_get(vpiType, exprHandle);
if (type != vpiRealVal && type != vpiRealVar) {
vpi_error("Arg #1 of $sampler should be real valued.");
return 1;
}
```
```
/* Get the handle on the period argument */
periodHandle = vpi_handle_by_index(functionHandle, 2);
```
```
/* Check that period argument exists */
if (!periodHandle) {
vpi_error("Not enough arguments for $sampler function.");
}
```
```
/* Check that period argument has a real value */
type = vpi_get(vpiType, periodHandle);
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
348
```
```
if (type != vpiRealVal && type != vpiRealVar) {
vpi_error("Arg #2 of $sampler should be real valued");
return 1;
}
```
```
/* Schedule callback for time = 0 */
sampler->cb_data.reason = cbEndOfCompile;
sampler->cb_data.cb_rtn = sampler_postcompile_cb;
sampler->cb_data.time.type = 0;
sampler->cb_data.user_data = (char *) functionHandle;
sampler->cb_data.time.real = 0.0;
schedule_callback(sampler, 0.0);
```
```
vpi_register_cb(&sampler->cb_data);
```
return 0;
}

/* calltf */
static int sampler_calltf(int data, int reason) {
vpiHandle funcHandle;
p_sampler_data sampler = (p_sampler_data) data;
s_vpi_value value;

```
/* Retrieve handle to current function */
funcHandle = vpi_handle(vpiSysTfCall, NULL);
```
/* Set returned value to held value */
vpi_set_value(sampler->returnHandle, &sampler->value, NULL,
vpiNoDelay);
return 0;
}

/* initialization callback after compile */
static int sampler_postcompile_cb(p_cb_data data) {
vpiHandle functionHandle = (vpiHandle) data;
p_sampler_data sampler;
s_vpi_value value;

```
/* Allocate the instance data and initialize it */
sampler = (p_sampler_data)malloc(sizeof(s_sampler_data));
```
```
/*Get the handle to the returned value, no need to check that one */
sampler->returnHandle = vpi_handle_by_index(functionHandle, 0);
sampler->exprHandle = vpi_handle_by_index(functionHandle, 1);
sampler->periodHandle = vpi_handle_by_index(functionHandle, 2);
```
```
/* Get the period value, it is assumed to be constant */
/* (but not necessary) */
sampler->value.format = vpiRealVal;
vpi_get_value(periodHandle, &value);
sampler->period = value.value.real;
```
```
/* Schedule callback for time = period */
sampler->cb_data.reason = acbElapsedTime;
sampler->cb_data.cb_rtn = sampler_update_cb;
sampler->cb_data.time.type = vpiScaledTme;
sampler->cb_data.user_data = (char *) sampler;
sampler->cb_data.time.real = sampler->period;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
349
```
```
schedule_callback(sampler, 0.0);
```
```
vpi_register_cb(&sampler->cb_data);
```
return 0;
}

/* timer callback */
static int sampler_update_cb(p_cb_data data) {
p_sampler_data sampler = (p_sampler_data)data->user_data;
s_vpi_value value;

```
/* Hold expression value */
vpi_get_value(sampler->exprHandle, &value);
```
/* Schedule next callback */
sampler->cb_data.reason = acbAbsTime;
sampler->cb_data.cb_rtn = sampler_update_cb;
sampler->cb_data.time.type = vpiScaledTime;
sampler->cb_data.user_data = (char *) sampler;
sampler->cb_data.time.real =
vpi_get_analog_time() + sampler->period;
register_callback(&sampler->cb_data);
return 0;
}

/*
* Public structure declaring the function
*/
static s_vpi_systf_data sampler_systf = {
vpiSysFunc, /* type: function / function */
vpiRealFunc, /* returned type */
"$sampler", /* name */
sampler_calltf, /* calltf callback */
sampler_compiletf, /* compiletf callback */
0, /* unused: sizetf callback */
0, /* unused: derivtf callback */
0 /* user_data: nothing */
};


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
350
```
### 12.33 vpi_register_systf()

```
The VPI routine vpi_register_systf() shall register callbacks for user-defined system tasks or func-
tions. Callbacks can be registered to occur when a user-defined system task or function is encountered
during compilation or execution of Verilog-AMS HDL source code.
```
```
The systf_data_p argument shall point to a s_vpi_systf_data structure, which is defined in vpi_user.h
and listed in Figure 12- 19.
```
```
Figure 12-19: The s_vpi_systf_data structure definition
```
#### 12.33.1 System task and function callbacks

```
User-defined Verilog-AMS system tasks and functions which use VPI routines can be registered with
vpi_register_systf(). The following system task/function-related callbacks are defined.
```
```
The type field of the s_vpi_systf_data structure shall register the user application to be a system task or
a system function. The type field value shall be an integer constant of vpiSysTask or vpiSysFunc-
tion. vpiSysTask shall register a task with the digital domain. vpiSysFunction shall register a
function with the digital domain.
```
```
The sysfunctype field of the s_vpi_systf_data structure shall define the type of value the system function
returns. The sysfunctype field shall be an integer constant of vpiIntFunc , vpiRealFunc , vpiTime-
Func , or vpiSizedFunc. This field shall only be used when the type field is set to vpiSysFunction.
```
```
vpi_register_systf()
```
**Synopsis:** Register user-defined system task/function-related callbacks.

**Syntax:** vpi_register_systf(systf_data_p)

```
Type Description
```
**Returns:** vpiHandle Handle to the callback object

```
Type Name Description
```
**Arguments:** p_vpi_systf_data systf_data_p Pointer to a structure with data about when callbacks
should occur and the data to be passed

**Related
routines:**

```
Use vpi_register_analog_systf() to register analog system task/functions.
Use vpi_register_cb() to register callbacks for simulation-related events
```
```
typedef struct t_vpi_systf_data {
int type; /* vpiSys[Task,TaskA,Function,FunctionA] */
int sysfunctype; /* vpi[IntFunc,RealFunc,TimeFunc,SizedFunc] */
char *tfname; /* first character shall be “$” */
int (*calltf)();
int (*compiletf)();
int (*sizetf)(); /* for vpiSizedFunc system functions only */
char *user_data;
} s_vpi_systf_data, *p_vpi_systf_data;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
351
```
The _compiletf_ , _calltf_ , and _sizetf_ fields of the s_vpi_systf_data structure shall be pointers to the user-pro-
vided applications which are to be invoked by the system task/function callback mechanism. One or more of
the _compiletf_ , _calltf_ , and _sizetf_ fields can be set to NULL if they are not needed. Callbacks to the applications
pointed to by the _compiletf_ and _sizetf_ fields shall occur when the simulation data structure is compiled or
built (or for the first invocation if the system task or function is invoked from an interactive mode). Call-
backs to the application pointed to by the _calltf_ routine shall occur each time the system task or function is
invoked during simulation execution.

The _sizetf_ application shall only called if the PLI application type is **vpiSysFunction** and the _sysfunc-
type_ is **vpiSizedFunc**. If no _sizetf_ is provided, a user-defined system function of **vpiSizedFunc** shall
return 32-bits.

The _user_data_ field of the s_vpi_systf_data structure shall specify a user-defined value, which shall be
passed back to the _compiletf_ , _sizetf_ , and _calltf_ applications when a callback occurs.

The following example application demonstrates dynamic linking of a VPI system task. The example uses
an imaginary routine, dlink(), which accepts a file name and a function name and then links that function
dynamically. This routine derives the target file and function names from the target _systf_ name.

```
link_systf(target)
char *target;
{
char task_name[strSize];
char file_name[strSize];
char compiletf_name[strSize];
char calltf_name[strSize];
static s_vpi_systf_data task_data_s = {vpiSysTask};
static p_vpi_systf_data task_data_p = &task_data_s;
```
```
sprintf(task_name, "$%s", target);
sprintf(file_name, "%s.o", target);
sprintf(compiletf_name, "%s_compiletf", target);
sprintf(calltf_name, "%s_calltf", target);
```
```
task_data_p->tfname = task_name;
task_data_p->compiletf = (int (*)()) dlink(file_name, compiletf_name);
task_data_p->calltf = (int (*)()) dlink(file_name, calltf_name);
vpi_register_systf (task_data_p);
}
```
#### 12.33.2 Initializing VPI system task/function callbacks

A means of initializing system task/function callbacks and performing any other desired task just after the
simulator is invoked shall be provided by placing routines in a NULL-terminated static array,
**vlog_startup_routines**. A C function using the array definition shall be provided as follows:

```
void ( *vlog_startup_routines []) ();
```
This C function shall be provided with a VPI-compliant product. Entries in the array shall be added by the
user. The location of **vlog_startup_routines** and the procedure for linking **vlog_start-
up_routines** with a software product shall be defined by the product vendor. (Callbacks can also be reg-
istered or removed at any time during an application routine, not just at startup time).

This array of C functions shall be for registering system tasks and functions. User tasks and functions which
appear in a compiled description shall generally be registered by a routine in this array.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
352
```
```
The following example uses vlog_startup_routines to register system tasks and functions and to
run a user initialization routine.
```
```
/*In a vendor product file which contains vlog_startup_routines ...*/
extern void register_my_systfs();
extern void my_init();
void ( *vlog_startup_routines [])() =
{
setup_report_cpu,/* user routine example in 23.24.3 */
register_my_systfs, /* user routine listed below */
0 /* shall be last entry in list */
}
```
```
/* In a user provided file... */
void register_my_systfs()
{
static s_vpi_systf_data systf_data_list[] = {
{vpiSysTask, 0 "$my_task", my_task_calltf, my_task_compiletf},
{vpiSysFunc, vpiIntFunc,"$my_func", my_func_calltf,
my_func_compiletf},
{vpiSysFunc, vpiRealFunc, "$my_real_func", my_rfunc_calltf,
my_rfunc_compiletf},
{0}
};
```
```
p_vpi_systf_data systf_data_p = &amp;(systf_data_list[0]);
while (systf_data_p-&gt;type)
vpi_register_systf(systf_data_p++);
}
```
### 12.34 vpi_remove_cb()

```
The VPI routine vpi_remove_cb() shall remove callbacks which were registered with vpi_regis-
ter_cb(). The argument to this routine shall be a handle to the callback object. The routine shall return a
1 (TRUE) if successful, and a 0 (FALSE) on a failure. After vpi_remove_cb() is called with a handle to
the callback, the handle is no longer valid.
```
```
vpi_remove_cb()
```
**Synopsis:** Remove a simulation callback registered with vpi_register_cb().

**Syntax:** vpi_remove_cb(cb_obj)

```
Type Description
```
**Returns:** bool **1** (true) if successful; **0** (false) on a failure

```
Type Name Description
```
**Arguments:** vpiHandle cb_obj Handle to the callback object

**Related
routines:**

```
Use vpi_register_cb() to register callbacks for simulation-related events
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
353
```
### 12.35 vpi_scan().....................................................................................................................................

```
The VPI routine vpi_scan() shall traverse the instantiated Verilog-AMS HDL hierarchy and return han-
dles to objects as directed by the iterator itr. The iterator handle shall be obtained by calling vpi_iter-
ate() for a specific object type. Once vpi_scan() returns NULL, the iterator handle is no longer valid
and can not be used again.
```
```
The following example application uses vpi_iterate() and vpi_scan() to display each net (includ-
ing the size for vectors) declared in the module. The example assumes it shall be passed a valid module han-
dle.
```
```
void display_nets(mod)
vpiHandle mod;
{
vpiHandle net;
vpiHandle itr;
```
```
vpi_printf("Nets declared in module %s\n",
vpi_get_str(vpiFullName, mod));
```
```
itr = vpi_iterate (vpiNet, mod);
while (net = vpi_scan (itr))
{
vpi_printf("\t%s", vpi_get_str(vpiName, net));
if (vpi_get(vpiVector, net))
{
vpi_printf(" of size %d\n", vpi_get(vpiSize, net));
}
else vpi_printf("\n");
}
}
```
```
vpi_scan()
```
**Synopsis:** Scan the Verilog-AMS HDL hierarchy for objects with a one-to-many relationship.

**Syntax:** vpi_scan(itr)

```
Type Description
```
**Returns:** vpiHandle Handle to an object

```
Type Name Description
```
**Arguments:** vpiHandle itr Handle to an iterator object returned from vpi_iterate()

**Related
routines:**

```
Use vpi_iterate() to obtain an iterator handle
Use vpi_handle() to obtain handles to an object with a one-to-one relationship
Use vpi_handle_multi() to obtain a handle to an object with a many-to-one relationship
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
354
```
### 12.36 vpi_sim_control().........................................................................................................................

```
The VPI routine vpi_sim_control shall be used to pass information from user code to Verilog simulator. All
standard compliant simulators must support the following three operations:
```
```
vpiStop — cause $stop built-in Verilog system task to be executed upon return of user function. This
operation shall be passed one additional diagnostic message level integer argument that is the same as the
argument passed to $stop (see 9.7.2).
```
```
vpiFinish — cause $finish built-in Verilog system task to be executed upon return of user function.
This operation shall be passed one additional diagnostic message level integer argument that is the same as
the argument passed to $finish (see 9.7.1).
```
```
vpiReset — cause $reset informative built-in Verilog system task to be executed upon return of user
VPI function. This operation shall be passed three integer value arguments: stop_value, reset_value,
diagnostic_level that are the same values passed to the $reset system task (see F.7 of IEEE Std 1364
Verilog).
```
```
vpiSetInteractiveScope — cause interactive scope to be immediately changed to new scope. This
operation shall be passed one argument that is a vpiHandle object with type vpiScope.
```
```
vpiRejectTransientStep — cause the current analog simulation time point to be rejected. This opera-
tion shall pass one argument which is the current timestep (delta).
```
```
vpiTransientFailConverge — cause the current analog simulation to continue iterating for a (valid)
solution.
```
```
Because there may be a need for user VPI applications to pass simulator specific information from back
from a user application to control simulation, additional operators and operation specific arguments may be
defined.
```
```
vpi_sim_control()
```
**Synopsis:** Provide software-specific simulation control.

**Syntax:** vpi_sim_control(flag, ...)

```
Type Description
```
**Returns:** bool **1** (true) if successful; **0** (false) on a failure

```
Type Name Description
```
**Arguments:** int flag Descriptor of the simulation control request

```
var args ... Variable number and type of arguments depending on flag
```
**Related
routines:**

```
NONE
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
355
```
