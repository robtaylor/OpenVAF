## 9. System tasks and functions

### 9.1 Overview......................................................................................................................................

Verilog-AMS HDL is a superset of IEEE Std 1364 Verilog and hence all the system tasks in IEEE Std 1364
Verilog are supported. Verilog-AMS adds several system tasks and system functions. These are described in
this clause. In addition, Verilog AMS HDL extends the behavior of several Verilog systems tasks and func-
tions including allowing some of them to be used in an analog context.

The system task and functions support by Verilog-AMS HDL are categorized in 9.2. A subclause is devoted
to each category from 9.4 until the end of this clause.

The behavior of a system task or function which is allowed in an analog context will be described in the con-
text of the analog simulation cycle (9.3) if required in the relevant section for that system task or function.

### 9.2 Categories of system tasks and functions

This subclause describes system tasks and functions that are considered part of the Verilog-AMS HDL. It
also states whether a particular system task or function is supported in the digital context and if it is sup-
ported in the analog context. The system tasks and functions are divided into various categories. Each cate-
gory has a table describing the support level in Verilog-AMS HDL for the system task or functions in that
category.

**Table 9-1—Display system tasks**

| Task name | Supported in digital context | Supported in analog context |
|-----------|------------------------------|----------------------------|
| $display | Yes | Yes |
| $displayb, $displayh, $displayo | Yes | No |
| $strobe | Yes | Yes |
| $strobeb, $strobeh, $strobeo | Yes | No |
| $write | Yes | Yes |
| $writeb, $writeh, $writeo | Yes | No |
| $monitor | Yes | Yes |
| $monitorb, $monitorh, $monitoro | Yes | No |
| $monitoron, $monitoroff | Yes | No |
| $debug | No | Yes |

**Table 9-2—File input-output system tasks and functions**

| Task/function name(s) | Supported in digital context | Supported in analog context |
|-----------------------|------------------------------|----------------------------|
| $fclose, $fopen | Yes | Yes |
| $fdisplay | Yes | Yes |
| $fdisplayb, $fdisplayh, $fdisplayo | Yes | No |
| $fwrite | Yes | Yes |
| $fwriteb, $fwriteh, $fwriteo | Yes | No |
| $fstrobe | Yes | Yes |
| $fstrobeb, $fstrobeh, $fstrobeo | Yes | No |
| $fmonitor | Yes | Yes |
| $fmonitorb, $fmonitorh, $fmonitoro | Yes | No |
| $fgetc, $ungetc | Yes | No |
| $fgets | Yes | Yes |
| $fscanf | Yes | Yes |
| $swrite, $sformat, $sscanf | Yes | Yes |
| $swriteb, $swriteh, $swriteo | Yes | No |
| $fread | Yes | No |
| $rewind, $fseek, $ftell | Yes | Yes |
| $fflush | Yes | Yes |
| $ferror | Yes | Yes |
| $feof | Yes | Yes |
| $readmemb, $readmemh | Yes | No |
| $sdf_annotate | Yes | No |
| $fdebug | No | Yes |

**Table 9-3—Timescale system tasks**

| Task/function name(s) | Supported in digital context | Supported in analog context |
|-----------------------|------------------------------|----------------------------|
| $printtimescale | Yes | No |
| $timeformat | Yes | No |

**Table 9-4—Simulation control system tasks**

| Task name | Supported in digital context | Supported in analog context |
|-----------|------------------------------|----------------------------|
| $finish | Yes | Yes |
| $stop | Yes | Yes |
| $fatal | No | Yes |
| $warning | No | Yes |
| $error | No | Yes |
| $info | No | Yes |

**Table 9-5—PLA modeling system tasks**

| Task name | Supported in digital context | Supported in analog context |
|-----------|------------------------------|----------------------------|
| $async$and$array | Yes | No |
| $async$nand$array | Yes | No |
| $async$or$array | Yes | No |
| $async$nor$array | Yes | No |
| $sync$and$array | Yes | No |
| $sync$nand$array | Yes | No |
| $sync$or$array | Yes | No |
| $sync$nor$array | Yes | No |
| $async$and$plane | Yes | No |
| $async$nand$plane | Yes | No |
| $async$or$plane | Yes | No |
| $async$nor$plane | Yes | No |
| $sync$and$plane | Yes | No |
| $sync$nand$plane | Yes | No |
| $sync$or$plane | Yes | No |
| $sync$nor$plane | Yes | No |

**Table 9-6—Stochastic analysis system tasks**

| Task name | Supported in digital context | Supported in analog context |
|-----------|------------------------------|----------------------------|
| $q_initialize | Yes | No |
| $q_remove | Yes | No |
| $q_exam | Yes | No |
| $q_add | Yes | No |
| $q_full | Yes | No |

**Table 9-7—Simulation time system functions**

| Function name | Supported in digital context | Supported in analog context |
|---------------|------------------------------|----------------------------|
| $realtime | Yes | No |
| $time | Yes | No |
| $stime | Yes | No |
| $abstime | Yes | Yes |

**Table 9-8—Conversion system functions**

| Function name | Supported in digital context | Supported in analog context |
|---------------|------------------------------|----------------------------|
| $bitstoreal | Yes | Yes |
| $itor | Yes | Yes |
| $signed | Yes | No |
| $realtobits | Yes | Yes |
| $rtoi | Yes | Yes |
| $unsigned | Yes | No |

**Table 9-9—Command line input system functions**

| Function name | Supported in digital context | Supported in analog context |
|---------------|------------------------------|----------------------------|
| $test$plusargs | Yes | Yes |
| $value$plusargs | Yes | Yes |

**Table 9-10—Probabilistic distribution system functions**

| Function name | Supported in digital context | Supported in analog context |
|---------------|------------------------------|----------------------------|
| $dist_chi_square | Yes | Yes |
| $dist_exponential | Yes | Yes |
| $dist_poisson | Yes | Yes |
| $dist_uniform | Yes | Yes |
| $dist_erlang | Yes | Yes |
| $dist_normal | Yes | Yes |
| $dist_t | Yes | Yes |
| $random | Yes | Yes |
| $arandom | Yes | Yes |
| $rdist_chi_square | Yes | Yes |
| $rdist_exponential | Yes | Yes |
| $rdist_poisson | Yes | Yes |
| $rdist_uniform | Yes | Yes |
| $rdist_erlang | Yes | Yes |
| $rdist_normal | Yes | Yes |
| $rdist_t | Yes | Yes |

**Table 9-11—Math system functions**

| Function name | Supported in digital context | Supported in analog context |
|---------------|------------------------------|----------------------------|
| $clog2 | Yes | Yes |
| $ln | Yes | Yes |
| $ln1p | Yes | Yes |
| $log10 | Yes | Yes |
| $exp | Yes | Yes |
| $expm1 | Yes | Yes |
| $sqrt | Yes | Yes |
| $pow | Yes | Yes |
| $floor | Yes | Yes |
| $ceil | Yes | Yes |
| $sin | Yes | Yes |
| $cos | Yes | Yes |
| $tan | Yes | Yes |
| $asin | Yes | Yes |
| $acos | Yes | Yes |
| $atan | Yes | Yes |
| $atan2 | Yes | Yes |
| $hypot | Yes | Yes |
| $sinh | Yes | Yes |
| $cosh | Yes | Yes |
| $tanh | Yes | Yes |
| $asinh | Yes | Yes |
| $acosh | Yes | Yes |
| $atanh | Yes | Yes |
| $min | Yes | Yes |
| $max | Yes | Yes |
| $abs | Yes | Yes |

**Table 9-12—Analog kernel parameter system functions**

| Function Name | Supported in digital context | Supported in analog context |
|---------------|------------------------------|----------------------------|
| $temperature | Yes | Yes |
| $vt | Yes | Yes |
| $simparam | Yes | Yes |
| $simparam$str | Yes | Yes |

**Table 9-13—Dynamic simulation probe system function**

| Function Name | Supported in digital context | Supported in analog context |
|---------------|------------------------------|----------------------------|
| $simprobe | No | Yes |

**Table 9-14—Analog kernel control system tasks and functions**

| Task/function name | Supported in digital context | Supported in analog context |
|--------------------|------------------------------|----------------------------|
| $discontinuity | No | Yes |
| $limit | No | Yes |
| $bound_step | No | Yes |

**Table 9-15—Hierarchical parameter system functions**

| Function name | Supported in digital context | Supported in analog context |
|---------------|------------------------------|----------------------------|
| $mfactor | Yes | Yes |
| $xposition | Yes | Yes |
| $yposition | Yes | Yes |
| $angle | Yes | Yes |
| $hflip | Yes | Yes |
| $vflip | Yes | Yes |

**Table 9-16—Explicit binding detection system functions**

| Function name | Supported in digital context | Supported in analog context |
|---------------|------------------------------|----------------------------|
| $param_given | No | Yes |
| $port_connected | No | Yes |

**Table 9-17—Analog node alias system function**

| Function name | Supported in digital context | Supported in analog context |
|---------------|------------------------------|----------------------------|
| $analog_node_alias | No | Yes |
| $analog_port_alias | No | Yes |

**Table 9-18—Table based interpolation and lookup system function**

| Function name | Supported in digital context | Supported in analog context |
|---------------|------------------------------|----------------------------|
| $table_model | Yes | Yes |

**Table 9-19—Connectmodule driver and receiver access system functions and operator**

| Function/operator name | Supported in digital context of connectmodule | Supported in analog context of connectmodule |
|------------------------|----------------------------------------------|---------------------------------------------|
| $driver_count | Yes | No |
| $driver_state | Yes | No |
| $driver_strength | Yes | No |
| @(driver_update) | Yes | No |
| $receiver_count | Yes | Yes |

**Table 9-20—Supplementary connectmodule driver access system functions**

| Task/function name(s) | Supported in digital context of connectmodule | Supported in analog context of connectmodule |
|-----------------------|----------------------------------------------|---------------------------------------------|
| $driver_delay | Yes | No |
| $driver_next_state | Yes | No |
| $driver_next_strength | Yes | No |
| $driver_type | Yes | No |

### 9.3 System tasks/functions executing in the context of the Analog Simulation Cycle......................

From 8.2, the analog simulation cycle has some different characteristics than the digital simulation cycle in
Verilog-AMS. These differences requires some additional description for certain system tasks or functions
that are supported in IEEE Std 1364 Verilog and have been extended to work in the analog context by Ver-
ilog-AMS.

A key difference is that the analog engine iteratively evaluates the **analog** blocks in an analog macro pro-
cess until that process is converged 8.4. The behavior of a particular system task or function during the iter-
ative evaluation process will be stated in the relevant section for that system task or function, if required.
The goal of the defined behavior of a system task or function in the analog context is that a call to a such sys-
tem task or function in an **analog** block during an iteration that is rejected should cause no side-effects on
the next iteration.

Another difference is that the analog engine supports additional analyses beyond a single transient analysis.
A single transient analysis is the only analysis that IEEE Std 1364 Verilog supports. Verilog-AMS extends
this to allows multiple analyses, including multiple transient analyses, to be run within a single simulation
process. Because of this extension, the behavior of a particular system task or function during different anal-
ysis types and between different analyses will be stated in the relevant section for that system task or func-
tion, if required.

### 9.4 Display system tasks

#### 9.4.1 Behavior of the display tasks in the analog context

Verilog-AMS extends the display tasks so that they can be used in the analog context.

The syntax for these functions are shown in Syntax 9- 1.

display_tasks_in_analog_block ::=
**$strobe (** list_of_arguments **) ;**
| **$display (** list_of_arguments **) ;**
| **$monitor (** list_of_arguments **) ;**
| **$write (** list_of_arguments **) ;**
| **$debug (** list_of_arguments **) ;**

```
Syntax 9-1—Syntax for the display_tasks_in_analog_block
```
The following rules apply to these functions.

```
— $strobe provides the ability to display simulation data when the simulator has converged on a
solution for all nodes.
— $strobe displays its arguments in the same order they appear in the argument list. Each argument
can be a quoted string, an expression which returns a value, or a null argument.
— The contents of string arguments are output literally, except when certain escape sequences are
inserted to display special characters or specify the display format for a subsequent expression.
— Escape sequences are inserted into a string in three ways:
— The special character \ indicates the character to follow is a literal or non-printable character
(see Table 9- 21 ).
— The special character % indicates the next character shall be interpreted as a format specification
which establishes the display format for a subsequent expression argument (see Table 9- 22 ).
For each % character which appears in a string, a corresponding expression argument shall be
supplied after the string.
— The special character string %% indicates the display of the percent sign character (%) (see
Table 9- 21 ).
— Any null argument produces a single space character in the display. (A null argument is character-
ized by two adjacent commas (,,) in the argument list.)
— When $strobe is invoked without arguments, it simply prints a newline character.
```
The **$display** task provides the same capabilities as **$strobe**. The **$write** task provides the same
capabilities as **$strobe** , but with no newline. The **$debug** task provides the capability to display simula-
tion data while the analog simulator is solving the equations; it displays its arguments for each iteration of
the analog solver.

The **$monitor** task provides the ability to monitor and display the values of any variables or expressions
specified as arguments to the task. The arguments for this task are specified in exactly the same manner as
for the **$strobe** system task.

When a **$monitor** task is invoked with one or more arguments, the simulator sets up a mechanism
whereby for each accepted step, if the variable or an expression in the argument list changes value compared
with the last accepted step —with the exception of the **$abstime** or **$realtime** system functions—the
entire argument list is displayed at the end of the time step as if reported by the **$strobe** task. If two or
more arguments change value at the same time, only one display is produced that shows the new values.

#### 9.4.2 Escape sequences for special characters

The escape sequences shown in Table 9- 21 , when included in a string argument, print special characters.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
227
```
#### 9.4.3 Format specifications

Table 9- 22 shows the escape sequences used for format specifications. The special character % indicates that
the next character should be interpreted as a format specification that establishes the display format for a
subsequent expression argument. For each % character (except %m, %% and %l) that appears in a string, a cor-
responding expression argument shall be supplied after the string.

The formatting specification %l (or %L) is defined for displaying the library information of the specific mod-
ule. This information shall be displayed as “ _library.cell_ " corresponding to the library name from which the
current module instance was extracted and the cell name of the current module instance. See Clause 13 of
IEEE Std 1364 Verilog for information on libraries and configuring designs.

Any expression argument which has no corresponding format specification is displayed using the default
decimal format in **$strobe**.

The format specifications in Table 9- 23 are used for real numbers and have the full formatting capabilities
available in the C language. For example, the format specification %10.3g sets a minimum field width of 10
with three (3) fractional digits.

```
Table 9-21— Escape sequences for printing special characters
```
```
\n The newline character
\t The tab character
\\ The \ character
\" The " character
\ddd A character specified by 1 to 3 octal digits
%% The % character
```
```
Table 9-22— Escape sequences for format specifications
```
```
%h or %H Display in hexadecimal format
%d or %D Display in decimal format
%o or %O Display in octal format
%b or %B Display in binary format
%c or %C Display in ASCII character format
%l or %L Display library binding information
%m or %M Display hierarchical name
%s or %S Display as a string
```
```
Table 9-23— Format specifications for real numbers
```
```
%e or %E Display ‘real’ in an exponential format
%f or %F Display ‘real’ in a decimal format
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
228
```
#### 9.4.4 Hierarchical name format................................................................................................

The %m format specifier does not accept an argument. Instead, it causes the display task to print the hierar-
chical name of the module, task, function, or named block which invokes the system task containing the for-
mat specifier. This is useful when there are many instances of the module which call the system task. One
obvious application is timing check messages in a flip-flop or latch module; the %m format specifier pin-
points the module instance responsible for generating the timing check message.

#### 9.4.5 String format

The %s format specifier is used to print ASCII codes as characters. For each %s specification which appears
in a string, a corresponding argument shall follow the string in the argument list. The associated argument is
interpreted as a sequence of 8-bit hexadecimal ASCII codes, with each 8 bits representing a single character.
If the argument is a variable, its value shall be right-justified so the right-most bit of the value is the least-
significant bit of the last character in the string. No termination character or value is required at the end of a
string and leading zeros ( 0 ) are never printed.

#### 9.4.6 Behavior of the display tasks in the analog block during iterative solving

All the display tasks, except **$debug** , shall not display output unless an iteration has been accepted.

#### 9.4.7 Extensions to the display tasks in the digital context......................................................

For **$strobe** , **$display** , **$write** and **$monitor**

```
— the %r (or %R) format specifier may be used on real expressions in the digital context
```
### 9.5 File input-output system tasks and functions...............................................................................

Verilog-AMS HDL extends many of the file operation tasks so that they can be used in the analog context.
This section describes the File I/O tasks that can be used in the analog context.

The system tasks and functions for file-based operations are divided into the following categories:

```
— Functions and tasks that open and close files
— Tasks that output values into files
— Tasks that output values into variables
— Tasks and functions that read values from files and load into variables
```
#### 9.5.1 Opening and closing files................................................................................................

The syntax for **$fopen** and **$fclose** system tasks is shown in Syntax 9- 2.

file_open_function ::=
mcd **= $fopen (** filename **) ;**

```
%g or %G Display ‘real’ in exponential or decimal format, which-
ever format results in the shorter printed output
%r or %R Display ‘real’ in engineering notation, using the scale fac-
tors defined in 2.6.2
```
```
Table 9-23— Format specifications for real numbers
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
229
```
```
| fd = $fopen ( filename , type ) ;
```
file_close_task ::=
**$fclose (** multi_channel_descriptor **) ;**
| **$fclose (** fd **) ;**

```
Syntax 9-2—Syntax for $fopen and $fclose system tasks
```
The function **$fopen** opens the file specified as the _filename_ argument and returns either a 32-bit multi-
channel descriptor or a 32-bit file descriptor, determined by the absence or presence of the _type_ argument.

_filename_ is an expression that is a string literal, **string** data type, or an integral data type containing a
character string that names the file to be opened.

_type_ is a string expression containing a character string of one of the forms in Table 9- 24 that indicates how
the file should be opened. If _type_ is omitted, the file is opened for writing, and a multichannel descriptor mcd
is returned. If _type_ is supplied, the file is opened as specified by the value of _type_ , and a file descriptor _fd_ is
returned.

The multichannel descriptor _mcd_ is a 32-bit integer in which a single bit is set indicating which file is
opened. The least significant bit (bit 0) of an _mcd_ always refers to the standard output. Output is directed to
two or more files opened with multichannel descriptors by bitwise OR-ing together their multichannel
descriptors and writing to the resultant value.

The most significant bit (bit 31) of a multichannel descriptor is reserved and shall always be cleared, limit-
ing an implementation to at most 31 files opened for output via multichannel descriptors.

The file descriptor _fd_ is a 32-bit value. The most significant bit (bit 31) of a fd is reserved and shall always
be set; this allows implementations of the file input and output functions to determine how the file was
opened. The remaining bits hold a small number indicating what file is opened. Three file descriptors are
pre-opened; they are STDIN, STDOUT, and STDERR, which have the values 32'h8000_0000,
32'h8000_0001, and 32'h8000_0002, respectively. STDIN is pre-opened for reading, and STDOUT and
STDERR are pre-opened for append.

Unlike multichannel descriptors, file descriptors cannot be combined via bitwise OR in order to direct out-
put to multiple files. Instead, files are opened via file descriptor for input, output, and both input and output,
as well as for append operations, based on the value of _type_ , according to Table 9- 24.

```
Table 9-24—Types for file descriptors
```
```
Argument Description
```
```
"r" or "rb" open for reading
```
```
"w" or "wb" truncate to zero length or create for writing
```
```
"a" or "ab" append; open for writing at end of file, or create for writing
```
```
"r+", "r+b", or "rb+" open for update (reading and writing)
```
```
"w+", "w+b", or "wb+" truncate or create for update
```
```
"a+", "a+b", or "ab+" append; open or create for update at end-of-file
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
230
```
If a file cannot be opened (either the file does not exist and the _type_ specified is "r", "rb", "r+", "r+b", or
"rb+", or the permissions do not allow the file to be opened at that path), a zero is returned for the mcd or
fd. Applications can call **$ferror** to determine the cause of the most recent error (see 9.5.7).

The "b" in the above types exists to distinguish binary files from text files. Many systems (such as Unix)
make no distinction between binary and text files, and on these systems the "b" is ignored. However, some
systems (such as machines running Windows NT) perform data mappings on certain binary values written to
and read from files that are opened for text access.

The **$fclose** system task closes the file specified by _fd_ or closes the file(s) specified by the multichannel
descriptor mcd. No further output to or input from any file descriptor(s) closed by **$fclose** is allowed. The
**$fopen** function shall reuse channels that have been closed.

NOTE—The number of simultaneous input and output channels that can be open at any one time is dependent on the
operating system. Some operating systems do not support opening files for update.

**9.5.1.1 opening and closing files during multiple analyses**

Verilog AMS HDL supports multiple analyses during the same simulation process (see Clause 8 ).

If a file is opened in a write mode in the first analysis and reopened in that write mode in following analysis,
then content written from the following analyses shall be appended to the content written during the previ-
ous analyses.

**9.5.1.2 Sharing of file descriptors between the analog and digital contexts**

The file I/O system functions and tasks in both the analog and digital contexts can use file descriptors
opened in either context, if the file descriptors are opened for writing or appending.

#### 9.5.2 File output system tasks

The syntax for **$fdisplay** , **$fwrite** , **$fmonitor** , **$fstrobe** and **$fdebug** system tasks is shown
in _Syntax 9- 3_.

file_open_function ::=
file_output_task_name **(** fd [ **,** list_of_arguments] **) ;**

file_output_task_name ::=
**$fdisplay** | **$fwrite** | **$fstrobe** | **$fmonitor** | **$fdebug**

```
Syntax 9-3—Syntax for file output system tasks
```
Each of the formatted display tasks — **$display** , **$write** , **$monitor** , and **$strobe** — has a counter-
part that writes to specific files as opposed to the standard output. These counterpart tasks — **$fdisplay** ,
**$fwrite** , **$fmonitor** , **$fstrobe** , and **$fdebug** — accept the same type of arguments as the tasks
upon which they are based, with one exception: The first argument shall be either a multichannel descriptor
or a file descriptor, which indicates where to direct the file output. Multichannel descriptors are described in
detail in 9.5.1. A multichannel descriptor is either a variable or the result of an expression that takes the form
of a 32-bit unsigned integer value.

The **$fstrobe** and **$fmonitor** system tasks work just like their counterparts, **$strobe** and **$moni-
tor** , except that they write to files using the file descriptor.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
231
```
#### 9.5.3 Formatting data to a string

The syntax for the **$swrite** family of tasks and for **$sformat** system task is shown in _Syntax 9- 4_.

string_output_task ::=
**$swrite (** string_variable **,** list_of_arguments **) ;**

variable-format_string_output_task ::=
**$sformat (** string_variable **,** format_string **,** list_of_arguments **) ;**

```
Syntax 9-4—Syntax for formatting data tasks
```
The **$swrite** family of tasks is based on the **$fwrite** family of tasks and accepts the same type of argu-
ments as the tasks upon which it is based, with one exception: The first argument to **$swrite** shall be a
string variable to which the resulting string shall be written, instead of a variable specifying the file to which
to write the resulting string.

The system task **$sformat** is similar to the system task **$swrite** , with one major difference.

Unlike the display and write family of output system tasks, **$sformat** always interprets its second argu-
ment, and only its second argument, as a format string. This format argument can be a static string, such as
"data is %d" or can be a string variable whose content is interpreted as the format string. No other argu-
ments are interpreted as format strings. **$sformat** supports all the format specifiers supported by **$dis-
play** , as documented in Table 9- 22.

The remaining arguments to **$sformat** are processed using any format specifiers in the format_string,
until all such format specifiers are used up. If not enough arguments are supplied for the format specifiers or
too many are supplied, then the application shall issue a warning and continue execution. The application, if
possible, can statically determine a mismatch in format specifiers and number of arguments and issue a com-
pile time error message.

If the _format_string_ is a string variable, it might not be possible to determine its value at compile time.

#### 9.5.4 Reading data from a file

Files opened using file descriptors can be read from only if they were opened with either the r or r+ type
values. See 9.5.2 for more information about opening files.

**9.5.4.1 Reading a line at a time**

For example:

```
integer code ;
code = $fgets ( str, fd );
```
reads characters from the file specified by _fd_ into the string variable, _str_ until a newline character is read and
transferred to _str_ , or an EOF condition is encountered.

If an error occurs reading from the file, then code is set to zero. Otherwise, the number of characters read is
returned in _code_. Applications can call **$ferror** to determine the cause of the most recent error (see 9.5.7).


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
232
```
**9.5.4.2 Reading formatted data**

For example:

```
integer code ;
code = $fscanf ( fd, format, args );
code = $sscanf ( str, format, args );
```
**$fscanf** reads from the files specified by the file descriptor _fd_.

**$sscanf** reads from the string, _str_. The string _str_ , shall be a string variable, string parameter or a string lit-
eral.

Both functions read characters, interpret them according to a format, and store the results. Both expect as
arguments a control string, format, and a set of arguments specifying where to place the results. If there are
insufficient arguments for the format, the behavior is undefined. If the format is exhausted while arguments
remain, the excess arguments are ignored.

If an argument is too small to hold the converted input, then, in general, the least significant bits are trans-
ferred. Arguments of any length that is supported by Verilog AMS HDL in the analog context can be used.
However, if the destination is a **real** , then the value **inf** (or **-inf** ) is transferred. The format is a string
expression. The string contains conversion specifications, which direct the conversion of input into the argu-
ments. The control string can contain the following:

```
a) White space characters (spaces, tabs, newlines, or formfeeds) that, except in one case described
below, cause input to be read up to the next nonwhite space character. For $sscanf , null charac-
ters shall also be considered white space.
b) An ordinary character (not %) that must match the next character of the input stream.
c) Conversion specifications consisting of the character %, an optional assignment suppression charac-
ter *, a decimal digit string that specifies an optional numerical maximum field width, and a conver-
sion code.
```
A conversion specification directs the conversion of the next input field; the result is placed in the variable
specified in the corresponding argument unless assignment suppression was indicated by the character *. In
this case, no argument shall be supplied.

The suppression of assignment provides a way of describing an input field that is to be skipped. An _input
field_ is defined as a string of nonspace characters; it extends to the next inappropriate character or until the
maximum field width, if one is specified, is exhausted. For all descriptors except the character c, white space
leading an input field is ignored.

```
% A single % is expected in the input at this point; no
assignment is done.
d Matches an optionally signed decimal number, consisting
of the optional sign from the set + or -, followed by a
sequence of characters from the set 0,1,2,3,4,5,6,7,8,9,
and _.
f, e, or g Matches a floating point number. The format of a floating
point number is an optional sign (either + or -), followed by
a string of digits from the set 0,1,2,3,4,5,6,7,8,9 optionally
containing a decimal point character (.), followed by an
optional exponent part including e or E, followed by an
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
233
```
```
optional sign, followed by a string of digits from the set
0,1,2,3,4,5,6,7,8,9.
r Matches a ‘real’ number in engineering notation, using the
scale factors defined in 2.6.2
s Matches a string, which is a sequence of nonwhite space
characters.
m Returns the current hierarchical path as a string. Does not
read data from the input file or str argument.
```
If an invalid conversion character follows the %, the results of the operation are implementation dependent.

If EOF is encountered during input, conversion is terminated. If EOF occurs before any characters matching
the current directive have been read (other than leading white space, where permitted), execution of the cur-
rent directive terminates with an input failure. Otherwise, unless execution of the current directive is termi-
nated with a matching failure, execution of the following directive (if any) is terminated with an input
failure.

If conversion terminates on a conflicting input character, the offending input character is left unread in the
input stream. Trailing white space (including newline characters) is left unread unless matched by a direc-
tive. The success of literal matches and suppressed assignments is not directly determinable.

The number of successfully matched and assigned input items is returned in code; this number can be 0 in
the event of an early matching failure between an input character and the control string. If the input ends
before the first matching failure or conversion, EOF is returned. Applications can call **$ferror** to deter-
mine the cause of the most recent error (see 9.5.7).

#### 9.5.5 File positioning

Example 1

```
integer pos ;
pos = $ftell ( fd );
```
returns in _pos_ the offset from the beginning of the file of the current byte of the file _fd_ , which shall be read or
written by a subsequent operation on that file descriptor.

This value can be used by subsequent **$fseek** calls to reposition the file to this point. Any repositioning
shall cancel any **$ungetc** operations. If an error occurs, EOF is returned. Applications can call **$ferror**
to determine the cause of the most recent error (see 17.2.7 of IEEE Std 1364 Verilog).

Example 2

```
code = $fseek ( fd, offset, operation );
code = $rewind ( fd );
```
sets the position of the next input or output operation on the file specified by fd. The new position is at the
signed distance offset bytes from the beginning, from the current position, or from the end of the file,
according to an operation value of 0, 1, and 2 as follows:

```
— 0 sets position equal to offset bytes
— 1 sets position to current location plus offset
— 2 sets position to EOF plus offset
```
**$rewind** is equivalent to $fseek (fd,0,0);


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
234
```
Repositioning the current file position with **$fseek** or **$rewind** shall cancel any **$ungetc** operations.

**$fseek()** allows the file position indicator to be set beyond the end of the existing data in the file. If data
are later written at this point, subsequent reads of data in the gap shall return zero until data are actually writ-
ten into the gap. **$fseek** , by itself, does not extend the size of the file.

When a file is opened for append (that is, when type is "a" or "a+"), it is impossible to overwrite informa-
tion already in the file. $fseek can be used to reposition the file pointer to any position in the file, but when
output is written to the file, the current file pointer is disregarded. All output is written at the end of the file
and causes the file pointer to be repositioned at the end of the output.

If an error occurs repositioning the file, then _code_ is set to -1. Otherwise, code is set to 0.

Applications can call **$ferror** to determine the cause of the most recent error (see 9.5.7).

#### 9.5.6 Flushing output

For example:

```
$fflush ( mcd );
$fflush ( fd );
$fflush ( );
```
writes any buffered output to the file(s) specified by mcd, to the file specified by fd, or if **$fflush** is
invoked with no arguments, to all open files.

#### 9.5.7 I/O error status

Should any error be detected by one of the file I/O routines, an error code is returned. Often this is sufficient
for normal operation (i.e., if the opening of an optional configuration file fails, the application typically
would simply continue using default values). However, sometimes it is useful to obtain more information
about the error for correct application operation. In this case, the **$ferror** function can be used:

```
integer errno ;
errno = $ferror ( fd, str );
```
A string description of type of error encountered by the most recent file I/O operation is written into str,
which should be at least 640 bits wide. The integral value of the error code is returned in errno. If the most
recent operation did not result in an error, then the value returned shall be zero, and the string variable str
shall be empty.

**9.5.8 Detecting EOF**

For example:

```
integer code;
code = $feof ( fd );
```
returns a nonzero value when EOF has previously been detected reading the input file fd. It returns zero oth-
erwise.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
235
```
#### 9.5.9 Behavior of the file I/O tasks in the analog block during iterative solving

If a file is being read from during an iterative solve and if that iteration is rejected, then the file pointer is
reset to the file position that it pointed to before the iterative solve started.

If a file is being written to during an iterative solve, then the file write operations shall not be performed
unless the iteration is accepted. The exception to this is the $fdebug. If $fdebug is evaluated during an itera-
tion, the write operation shall occur even if the evaluation occurred during an iteration that was rejected.

The features of the underlying implementation of file I/O on the host system may prevent the file position
being reset after an iteration is rejected. In this case, a fatal error will be reported.

### 9.6 Timescale system tasks

Verilog AMS HDL does not extend the timescale tasks defined in IEEE Std 1364 Verilog.

### 9.7 Simulation control system tasks

Verilog AMS HDL extends the two simulation control tasks, **$finish** and **$stop** so that they can be run
in the analog context.

This section describes their behavior if used in the analog context.

Verilog AMS HDL also supports three new simulation control tasks in the analog context only; **$fatal** ,
**$error** , **$warning**.

#### 9.7.1 $finish..............................................................................................................................

The syntax for this task is shown in Syntax 9- 5.

finish_task ::=
**$finish** [ **(** n **)** ] **;**

```
Syntax 9-5—Syntax for the finish_task
```
If **$finish** is called during an accepted iteration, then the simulator shall exit after the current solution is
complete. **$finish** called during a rejected iteration shall have no effect. As a result of the simulation ter-
minating due to a **$finish** task, it is expected that all appropriate **final_step** blocks are also triggered.
If **$finish** is called from an **analog initial** block, the simulator shall exit without performing the
simulation.

If an expression is supplied to this task, its value determines which diagnostic messages are printed after the
**$finish** call is executed, as shown in Table 9- 25. One (1) is the default if no argument is supplied.

```
Table 9-25—Diagnostic messages
```
```
Parameter Message
```
```
0 Prints nothing
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
236
```
If **$finish** is called from within an **analog initial** block, the simulator shall report that the call was
made during initialization in place of the simulation time. If **$finish** is called from the analog context
during a dc sweep (but outside of an **analog initial** block), the simulator shall report the current value
of the swept variable in place of the simulation time.

#### 9.7.2 $stop

The syntax for this task is shown in Syntax 9- 6.

stop_task ::=
**$stop** [ **(** n **)** ] **;**

```
Syntax 9-6—Syntax for the stop_task
```
A call to **$stop** during an accepted iteration causes simulation to be suspended at a converged time point.
This task takes an optional expression argument (0, 1, or 2), which determines what type of diagnostic mes-
sage is printed. The amount of diagnostic messages output increases with the value of _n_ , as shown in
Table 9- 25. The **$stop** task shall not be used within an **analog initial** block.

The mechanism for resuming simulation is left to the implementation.

#### 9.7.3 $fatal, $error, $warning, and $info

The syntax form for the severity system task is as follows:

assert_severity_task ::=
fatal_message_task
| nonfatal_message_task

fatal_message_task ::= **$fatal** [ **(** finish_number [ **,** message_argument { **,** message_argument } ] **)** ] **;**

nonfatal_message_task ::= severity_task [ **(** [ message_argument { **,** message_argument] } ] **)** ] **;**

severity_task ::= **$error** | **$warning** | **$info**

finish_number ::= **0** | **1** | **2**

```
Syntax 9-7—Assertion severity tasks
```
The behavior of assert severity tasks is as follows:

```
— $fatal shall generate a run-time fatal assertion error, which terminates the simulation with an
errorcode. The first argument passed to $fatal shall be consistent with the corresponding argument
to the Verilog $finish system task, which sets the level of diagnostic information reported by the
tool. Calling $fatal results in an implicit call to $finish.
— $error shall be a run-time error.
```
```
1 Prints simulation time and location
2 Prints simulation time, location, and statistics about the memory and CPU time used in simulation
```
```
Table 9-25—Diagnostic messages (continued)
```
```
Parameter Message
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
237
```
```
— $warning shall be a run-time warning, which can be suppressed in a tool-specific manner.
— $info shall indicate that the assertion failure carries no specific severity.
```
Non-fatal system severity tasks ( **$error** , **$warning** , **$info** ) called during a rejected iteration shall have
no effect. **$fatal** terminates the simulation without checking whether the iteration would be rejected.

If **$fatal** is executed within an **analog initial** block, then after outputting the message, the initial-
ization may be aborted, and in no case shall simulation proceed past initialization. Some of the system sever-
ity task calls may not be executed either. The _finish_number_ may be used in an implementation-specific
manner.

If **$error** is executed within an **analog initial** block, then the message is issued and the initializa-
tion continues. However, the simulation shall not proceed past initialization.

The other two tasks, **$warning** and **$info** , only output their text message but do not affect the rest of the
initialization and the simulation.

For simulation tools, these tasks shall also report the simulation run time at which the severity system task is
called. If any of these tasks is called from an analog context during a dc sweep, the simulator shall report the
current value of the swept variable in place of the simulation run time. If the task is called from an **analog
initial** block, the simulator shall report that the call was made during initialization.

Each of these system tasks can also include additional user-specified information using the same format as
the Verilog **$display**.

### 9.8 PLA modeling system tasks.........................................................................................................

Verilog AMS HDL does not extend the PLA modeling tasks defined in IEEE Std 1364 Verilog.

### 9.9 Stochastic analysis system tasks

Verilog AMS HDL does not extend the stochastic analysis tasks defined in IEEE Std 1364 Verilog.

### 9.10 Simulator time system functions..................................................................................................

Verilog AMS HDL extends the simulator time functions defined in IEEE Std 1364 Verilog as follows;

```
— A new function is added called $abstime that can be used from the analog and digital contexts.
$abstime returns the absolute time, that is a real value number representing time in seconds.
```
NOTE—In previous versions of the Verilog-AMS LRM, **$realtime** was supported in the analog context and it had an
additional argument. This version of the LRM deprecates using **$realtime** in the analog context.

### 9.11 Conversion system functions

Verilog AMS HDL extends the conversion functions defined in IEEE Std 1364 Verilog so that **$bitsto-
real** and **$realtobits,$rtoi** and **$itor** can be used in the analog context.

### 9.12 Command line input.....................................................................................................................

Verilog AMS HDL extends the command line input functions defined in IEEE Std 1364 Verilog so that they
can be used in the analog context.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
238
```
### 9.13 Probabilistic distribution system functions

Verilog-AMS HDL extends the probabilistic distribution functions so that they are supported in the analog
context. Also, real versions of the probabilistic distribution functions are introduced. Also, real versions of
the probabilistic distribution functions are introduced as well as a special analog random system task called
**$arandom**.

#### 9.13.1 $random and $arandom...................................................................................................

This subclause describes how the **$random** and **$arandom** system functions are supported in the analog
context.

The syntax for these functions is shown in Syntax 9- 8.

random_function ::=
**$random** [ **(** random_seed **)** ]

random seed ::=
integer_variable_identifier
| reg_variable_identifier
| time_variable_identifier

analog_random_function ::=
**$arandom** [ **(** analog_random_seed [ **,** type_string] **)** ]

analog_random_seed ::=
integer_variable_identifier
| reg_variable_identifier
| time_variable_identifier
| integer_parameter_identifier
| [ sign] decimal_number

type_string ::=
**"global"**
| **"instance"**

```
Syntax 9-8—Syntax for the random_function and analog random function
```
he system functions **$random** and **$arandom** provide a mechanism for generating random numbers. The
random number returned is a 32-bit signed integer; it can be positive or negative. The two functions differ in
the arguments they take. **$arandom** is upwardly compatible with **$random** — **$arandom** can take the
same arguments as **$random** and has the same behavior.

The _random_seed_ argument may take one of several forms. It may be omitted, in which case the simulator
picks a seed. If the call to **$random** is within the analog context, the _random_seed_ may be an analog
**integer** variable. If the call to **$random** is within the digital context it may be a **reg** , **integer** , or
**time** variable. If the _random_seed_ argument is specified it is an **inout** argument; that is, a value is passed
to the function and a different value is returned. The variable should be initialized by the user prior to calling
**$random** and only updated by the system function. The function returns a new 32-bit random number each
time it is called.

The system function **$random** shall always return the same stream of values given the same initial _ran-
dom_seed_. This facilitates debugging by making the operation of the system repeatable.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
239
```
**$arandom** supports the seed argument _analog_random_seed_. The _analog_random_seed_ argument can also
be a parameter or a constant, in which case the system function does not update the parameter value. How-
ever an internal seed is created which is assigned the initial value of the parameter or constant and the inter-
nal seed gets updated every time the call to **$arandom** is made. This allows the **$arandom** system
function to be used for parameter initialization. In order to get different random values when the
_analog_random_seed_ argument is a parameter, the user can override the parameter using a method in 6.3.

The _type_string_ is an additional argument that **$arandom** supports beyond **$random**. The _type_string_ pro-
vides support for Monte-Carlo analysis and shall only by used in calls to **$arandom** from within a param-
set. If the _type_string_ is "global" (or not specified in a call within a paramset), then one value is generated
for each Monte-Carlo trial. If the _type_string_ is "instance" then one value is generated for each instance
that references this value, and a new set of values for these instances is generated for each Monte-Carlo trial.

Examples:

Where b > 0, the expression ( **$random** % b) gives a number in the following range:
[(-b+1) : (b-1)].

The following code fragment shows an example of random number generation between -59 and 59:

```
integer rand;
rand = $random % 60;
```
#### 9.13.2 Distribution functions

The section describes how the distribution functions are supported in the analog context.

The syntax for these functions are shown in Syntax 9- 9.

distribution_functions ::=
**$digital_dist_functions (** args **)**
| **$rdist_uniform (** seed **,** start_expression **,** end_expression [ **,** type_string] **)**
| **$rdist_normal (** seed **,** mean_expression **,** standard_deviation_expression [ **,** type_string] **)**
| **$rdist_exponential (** seed **,** mean_expression [ **,** type_string] **)**
| **$rdist_poisson (** seed **,** mean_expression [ **,** type_string] **)**
| **$rdist_chi_square (** seed **,** degree_of_freedom_expression [ **,** type_string] **)**
| **$rdist_t (** seed **,** degree_of_freedom_expression [ **,** type_string] **)**
| **$rdist_erlang (** seed **,** k_stage_expression **,** mean_expression [ **,** type_string] **)**

seed ::=
integer_variable_identifier
| integer_parameter_identifier
| [ sign ] decimal_number

type_string ::=
**"global"**
| **"instance"**

```
Syntax 9-9—Syntax for the probabilistic distribution functions
```
The following rules apply to these functions.

```
— All arguments to the system functions are real values, except for seed (which is defined by $ran-
dom ). For the $rdist_exponential , $rdist_poisson , $rdist_chi_square ,
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
240
```
```
$rdist_t , and $rdist_erlang functions, the arguments mean , degree_of_freedom , and
k_stage shall be greater than zero ( 0 ). Otherwise an error shall be reported.
— Each of these functions returns a pseudo-random number whose characteristics are described by the
function name, e.g., $rdist_uniform returns random numbers uniformly distributed in the
interval specified by its arguments.
— For each system function, the seed argument shall be an integer. If it is an integer variable, then it is
an inout argument; that is, a value is passed to the function and a different value is returned. The
variable is initialized by the user and only updated by the system function. This ensures the desired
distribution is achieved upon successive calls to the system function. If the seed argument is a
parameter or constant, then the system function does not update the parameter value. However an
internal seed is created which is assigned the initial value of the parameter or constant and the inter-
nal seed gets updated every time the call to the system function is made. This allows the system
function to be used for parameter initialization.
— The system functions shall always return the same value given the same seed. This facilitates debug-
ging by making the operation of the system repeatable. In order to get different random values when
the seed argument is a parameter, the user can override the parameter.
— All functions return a real value.
— In $rdist_uniform , the start and end arguments are real inputs which bound the values
returned. The start value shall be smaller than the end value.
— The mean argument used by $rdist_normal , $rdist_exponential , $rdist_poisson ,
and $rdist_erlang is an real input which causes the average value returned by the function to
approach the value specified.
— The standard_deviation argument used by $rdist_normal is a real input, which helps determine
the shape of the density function. Using larger numbers for standard_deviation spreads the returned
values over a wider range. Using a mean of zero (0) and a standard_deviation of one ( 1 ),
$rdist_normal generates Gaussian distribution.
— The degree_of_freedom argument used by $rdist_chi_square and $rdist_t is a real input,
which helps determine the shape of the density function. Using larger numbers for degree_of_free-
dom spreads the returned values over a wider range.
— The type_string provides support for Monte-Carlo analysis and shall only by used in calls to a distri-
bution function from within a paramset. If the type_string is "global" (or not specified in a call
within a paramset), then one value is generated for each Monte-Carlo trial. If the type_string is
"instance" then one value is generated for each instance that references this value, and a new set
of values for these instances is generated for each Monte-Carlo trial. See 6.4.1 for an example.
```
#### 9.13.3 Algorithm for probabilistic distribution

17.9.3 of IEEE Std 1364 Verilog contains the C-code to describe the algorithm of probabilistic system func-
tions based on the seed value passed to them.

This code also describe the algorithm of the IEEE Std 1364 Verilog probabilistic functions extensions in
Verilog-AMS HDL as indicated in Table 9- 26.

### Table 9-26—Verilog AMS to C function cross-listingss-listing

```
Verilog AMS Function C function in IEEE Std 1364 Verilog (subclause 17.9.3)
```
```
$rdist_uniform uniform
$rdist_normal normal
$rdist_exponential exponential
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
241
```
### 9.14 Math system functions

Verilog-AMS HDL extends the IEEE Std 1364-2005 Verilog HDL math functions so that they can be used
from the analog context.

All of these functions, except **$clog2** , are aliases of the analog math operators described in 4.3.1 and
Table 4- 14 shows which analog math operators are aliases of which math system functions.

The system function **$clog2** shall return the ceiling of the log base 2 of the argument (the log rounded up
to an integer value). **$clog2** is defined more completely in IEEE Std 1364-2005 Verilog HDL.

Users are encourage to use the system function version of the math operation instead of the operator for bet-
ter compatibility with IEEE Std 1364-2005 Verilog HDL.

### 9.15 Analog kernel parameter system functions..................................................................................

Verilog AMS HDL adds a set of system functions called the analog kernel parameter functions.

The syntax for these functions are shown in Syntax 9- 10.

environment_parameter_functions ::=
**$temperature**
| **$vt** [ **(** temperature_expression **)** ]
| **$simparam (** param_name [ **,** expression] **)**
| **$simparam$str (** param_name **)**

```
Syntax 9-10—Syntax for the environment parameter functions
```
These functions return information about the current environment parameters as a real value.

**$temperature** does not take any input arguments and returns the circuit’s ambient temperature in Kelvin
units.

**$vt** can optionally have temperature (in Kelvin units) as an input argument and returns the thermal voltage
( _kT/q_ ) at the given temperature. **$vt** without the optional input temperature argument returns the thermal
voltage using **$temperature**.

**$simparam()** queries the simulator for a real-valued simulation parameter named _param_name_. The
argument _param_name_ is a string value, either a string literal, string parameter, or a string variable. If
_param_name_ is known, its value is returned. If _param_name_ is not known, and the optional _expression_ is not
supplied, then an error is generated. If the optional _expression_ is supplied, its value is returned if

```
$rdist_poisson poisson
$rdist_chi_square chi_square
$rdist_t t
$rdist_erlang erlang
```
### Table 9-26—Verilog AMS to C function cross-listingss-listing

```
Verilog AMS Function C function in IEEE Std 1364 Verilog (subclause 17.9.3)
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
242
```
_param_name_ is not known and no error is generated. **$simparam()** shall always return a real value; simu-
lation parameters that have integer values shall be coerced to real. There is no fixed list of simulation param-
eters. However, simulators shall accept the strings in Table 9- 27 to access commonly-known simulation
parameters, if they support the parameter. Simulators can also accept other strings to access the same param-
eters.

The values returned by simulatorVersion and simulatorSubversion are at the vendor’s discretion, but the val-
ues shall be monotonically increasing for new versions or releases of the simulator, to facilitate checking
that the simulator supports features that were added in a certain version or sub-version.

Examples:

In this first example, the variable gmin is set to the simulator’s parameter named gmin, if it exists, other-
wise, an error is generated.

```
gmin = $simparam ("gmin");
```
In this second example, the variable sourcescale is set to the simulator’s parameter named sourceS-
caleFactor, if it exists, otherwise, the value 1.0 is returned.

```
sourcescale = $simparam ("sourceScaleFactor", 1.0);
```
```
Table 9-27—Simulation real and integer parameter names
```
```
String Units Description
```
```
gdev 1/Ohms Additional conductance to be added to nonlinear branches for conduc-
tance homotopy convergence algorithm.
gmin 1/Ohms Minimum conductance placed in parallel with nonlinear branches.
imax Amps Branch current threshold above which the constitutive relation of a non-
linear branch should be linearized.
imelt Amps Branch current threshold indicating device failure.
iteration Iteration number of the analog solver.
scale Scale factor for device instance geometry parameters.
shrink Optical linear shrink factor.
simulatorSubversion The simulator sub-version.
simulatorVersion The simulator version.
sourceScaleFactor Multiplicative factor for independent sources for source stepping homo-
topy convergence algorithm.
tnom Celsius Default value of temperature at which model parameters were extracted.
timeUnit s Time unit as specified in ‘timescale in seconds.
timePrecision s Time precision as specified in ‘timescale in seconds.
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
243
```
**$simparam$str** is similar to **$simparam**. However it is used for returning string-valued simulation
parameters. Table 9- 28 gives a list of simulation string parameter names that shall be supported by **$sim-
param$str**.

Example:
**module** testbench;
dut dut1;
**endmodule
module** dut;
**task** mytask;
$display( "%s\n%s\n%s\n", $simparam$str( "module"),
$simparam$str( "instance"),
$simparam$str( "path"));
**endtask
endmodule**

produces
dut
testbench.dut1
testbench.dut1.mytask

### 9.16 Dynamic simulation probe function.............................................................................................

Verilog-AMS HDL supports a system function that allows the probing of values within a sibling instance
during simulation.

dynamic_monitor_function ::=
**$simprobe (** inst_name **,** param_name [ **,** expression] **)**

```
Syntax 9-11—Syntax for the dynamic monitor function
```
**$simprobe()** queries the simulator for an output variable named _param_name_ in a sibling instance called
_inst_name_. The arguments _inst_name_ and _param_name_ are string values, either a string literal, string param-
eter, or a string variable. To resolve the value, the simulator will look for an instance called _inst_name_ in the
parent of the current instance i.e. a sibling of the instance containing the **$simprobe()** expression. Once
the instance is resolved, it will then query that instance for an output variable called _param_name_. If either
the _inst_name_ or _param_name_ cannot be resolved, and the optional _expression_ is not supplied, then an error
shall be generated. If the optional _expression_ is supplied, its value will be returned in lieu of raising an error.
The intended use of this function is to allow dynamic monitoring of instance quantities.

```
Table 9-28—Simulation string parameter names
```
```
String Description
```
```
analysis_name The name of the current analysis e.g. tran1, mydc
analysis_type The type of the current analysis e.g. dc, tran, ac
cwd The current working directory in which the simulator was started
module The name of the module from which $simparam$str is called.
instance The hierarchical name of the instance from which $simparam$str is called.
path The hierarchical path to the $simparam$str function.
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
244
```
Example:

```
module monitor;
parameter string inst = " default ";
parameter string quant = " default ";
parameter real threshold = 0.0;
real probe;
analog begin
probe = $simprobe (inst,quant);
if (probe > threshold) begin
$strobe ("ERROR: Time %e: %s#%s (%g) > threshold (%e)",
$abstime , inst,quant, probe, threshold);
$finish ;
end
end
endmodule
```
The module monitor will probe the quant in instance inst_._ If its value becomes larger than threshold,
then the simulation will raise an error and stop.

```
module top(d,g,s);
electrical d,g,s;
inout d,g,s;
electrical gnd; ground gnd;
SPICE_pmos #(.w(4u),.l(0.1u),.ad(4p),.as(4p),.pd(10u),.ps(10u))
mp(d,g,s,s);
SPICE_nmos #(.w(2u),.l(0.1u),.ad(2p),.as(2p),.pd(6u),.ps(6u))
mn(d,g,gnd,gnd);
monitor #(.inst("mn"),.quant("id"),.threshold(4.0e-3))
amonitor();
endmodule
```
Here the monitor instance amonitor will keep track of the dynamic quantity id in the mosfet instance mn_._
If the value of id goes above the specified threshold of 4.0e-3 amps then instance amonitor will generate
the error message and stop the simulation.

### 9.17 Analog kernel control system tasks and functions.......................................................................

Verilog AMS HDL adds a set of tasks and functions to control the analog solver’s behavior on a signals and
instances called the analog kernel control tasks.

#### 9.17.1 $discontinuity

The **$discontinuity** task is used to give hints to the simulator about the behavior of the module so the
simulator can control its simulation algorithms to get accurate results in exceptional situations. This task
does not directly specify the behavior of the module. **$discontinuity** shall be executed whenever the
analog behavior changes discontinuously.

The general form is

```
$discontinuity [ ( constant_expression ) ] ;
```
where _constant_expression_ indicates the degree of the discontinuity if the argument to **$discontinuity**
is non-negative, i.e. **$discontinuity(i)** implies a discontinuity in the _i_ ’th derivative of the constitutive
equation with respect to either a signal value or time where _i_ must be a non-negative integer. Hence, $ **dis-
continuity(0)** indicates a discontinuity in the equation, **$discontinuity(1)** indicates a discontinu-


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
245
```
ity in its slope, etc. A special form of the **$discontinuity** task, **$discontinuity(-1)** , is used with
the **$limit()** function so -1 is also a valid argument of **$discontinuity**. See 9.17.3 for an explana-
tion.

Because discontinuous behavior can cause convergence problems, discontinuity shall be avoided whenever
possible.

The filter functions ( **transition()** , **slew()** , **laplace()** , etc.) can be used to smooth discontinuous
behavior. However, in some cases it is not possible to implement the desired functionality using these filters.
In those cases, the **$discontinuity** task shall be executed when the signal behavior changes abruptly.

Discontinuity created by switch branches and filters, such as **transition()** and **slew()** , does not need
to be announced.

The following example uses the discontinuity task to model a relay.

```
module relay (c1, c2, pin, nin) ;
inout c1, c2 ;
input pin, nin ;
electrical c1, c2, pin, nin ;
parameter real r=1 ;
analog begin
@( cross (V(pin,nin))) $discontinuity ;
if (V(pin,nin) >= 0)
I(c1,c2) <+ V(c1,c2)/r;
else
I(c1,c2) <+ 0 ;
end
endmodule
```
In this example, **cross()** controls the time step so the time when the relay changes position is accurately
resolved. It also triggers the **$discontinuity** task, which causes the simulator to react properly to the
discontinuity. This would have been handled automatically if the type of the branch (c1,c2) had been
switched between voltage and current.

Another example is a source which generates a triangular wave. In this case, neither the model nor the wave-
forms generated by the model are discontinuous. Rather, the waveform generated is piecewise linear with
discontinuous slope. If the simulator is aware of the abrupt change in slope, it can adapt to eliminate prob-
lems resulting from the discontinuous slope (typically changing to a first order integration method).

```
module triangle(out);
output out;
voltage out;
parameter real period = 10.0, amplitude = 1.0;
integer slope;
real offset;
```
```
analog begin
@( timer (0, period)) begin
slope = +1;
offset = $abstime ;
$discontinuity ;
end
```
```
@( timer (period/2, period)) begin
slope = -1 ;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
246
```
```
offset = $abstime;
$discontinuity ;
end
```
```
V(out) <+ amplitude*slope*
(4*( $abstime - offset)/period - 1);
end
endmodule
```
#### 9.17.2 $bound_step task

The **$bound_step()** task puts a bound on the next time step. It does not specify exactly what the next
time step is, but it bounds how far the next time point can be from the present time point. The task takes the
maximum time step as an argument. It does not return a value.

The general form is

```
$bound_step ( expression ) ;
```
where _expression_ is a required argument and represents the maximum timestep the simulator can advance.
The _expression_ argument shall be non-negative. If the value is less than the simulator’s minimum allowable
time step, the simulator’s minimum time step shall be used instead. Refer to the simulator’s documentation
for further information regarding limits on step size for time dependent analysis.

For a given time step, the simulator shall ensure that the next time step taken is no larger than the smallest
**$bound_step()** argument currently active. The $bound_step() statement shall be ignored during a non
time-domain analysis.

The example below implements a sinusoidal voltage source and uses the **$bound_step()** task to assure
the simulator faithfully follows the output signal (it is forcing 20 points per cycle).

```
module vsine(out);
output out;
voltage out;
parameter real freq=1.0, ampl=1.0, offset=0.0;
```
```
analog begin
V(out) <+ ampl*sin(2.0*‘M_PI*freq* $abstime ) + offset;
$bound_step (0.05/freq);
end
endmodule
```
#### 9.17.3 $limit

The **$limit()** function is a special-purpose system function whose purpose, like that of the **limexp()**
function of 4.5.13, is to improve convergence of the analog solver. While **limexp()** is specifically
intended for the exponential function, **$limit()** may be used for the exponential as well as other nonlin-
ear functions and provides a method to recommend a specific approach for improving the convergence.
Syntax 9- 12 shows the methods of using the **$limit()** function.

limit_call ::=
**$limit (** access_function_reference **)**
| **$limit (** access_function_reference **,** string, arg_list **)**
| **$limit (** access_function_reference **,** analog_function_identifier **,** arg_list **)**


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
247
```
```
Syntax 9-12—Syntax for $limit()
```
As with **limexp()** , the **$limit()** function has internal state containing information about the argument
on previous iterations. It returns a real value that is derived from its first argument (the access function refer-
ence, such as a branch voltage), but it internally limits the change of the output from iteration to iteration in
order to improve convergence. On any iteration where the output value is not the same as the value of the
access function (within appropriate tolerances), the simulator is prevented from terminating the iteration. In
all cases, the simulator is responsible for determining if limiting should be applied and what the return value
is on a given iteration. In particular, the simulator may simply choose to have **$limit()** return the value of
its first argument, if it has other methods for ensuring convergence.

When more than one argument is supplied to the **$limit()** function, the second argument recommends a
function to use to compute the return value. When the second argument is a string, it refers to a built-in func-
tion of the simulator. The two most common such functions are _pnjlim_ and _fetlim_ , which are found in SPICE
and many SPICE-like simulators. Simulators may support other built-in functions and need not support
_pnjlim_ or _fetlim_. If the string refers to an unknown or unsupported function, the simulator is responsible for
determining the appropriate limiting algorithm, just as if no string had been supplied.

_pnjlim_ is intended for limiting arguments to exponentials, and the **limexp()** function of 4.5.13 may be
implemented through a function derived from _pnjlim_. Two additional arguments to the **$limit()** function
are required when the second argument to the limit function is the string “pnjlim”: the third argument to
**$limit()** indicates a step size vte and the fourth argument is a critical voltage vcrit. The step size vte
is usually the product of the thermal voltage **$vt** and the emission coefficient of the junction. The critical
voltage is generally obtained from the formula , where is the saturation
current of the junction.

_fetlim_ is intended for limiting the potential across the oxide of a MOS transistor. One additional argument to
the **$limit()** function is required when the second argument to the limit function is the string "fetlim":
the third argument to **$limit()** is generally the threshold voltage of the MOS transistor.

In the case that none of the built-in functions of the simulator is appropriate for limiting the potential (or
flow) used in a nonlinear equation, the second argument of the **$limit()** function may be an identifier
referring to a user-defined analog function. User-defined functions are described in 4.7. In this case, if the
simulator determines that limiting is needed to improve convergence, it will pass the following quantities as
arguments to the user-defined function:

```
— The first argument of the user-defined function shall be the value of the access function reference for
the current iteration.
— The second argument shall be the appropriate internal state; generally, this is the value that was
returned by the $limit() function on the previous iteration.
— If more than two arguments are given to the $limit() function, then the third and subsequent
arguments are passed as the third and subsequent arguments of the user-defined function.
```
The arguments of the user-defined function shall all be declared **input**.

In order to prevent convergence when the output of the **$limit()** function is not sufficiently close to the
value of the access function reference, the user-defined function shall call **$discontinuity** (-1) (see
9.17) when its return value is not sufficiently close to the value of its first argument.

The module below defines a diode and includes an analog function that mimics the behavior of _pnjlim_ in
SPICE. Though **limexp()** could have been used for the exponential in the current, using **$limit()**
allows the same voltage to be used in the charge calculation.

```
Vcrit = vte ln vte  2  Is Is
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
248
```
```
module diode(a,c);
inout a, c;
electrical a, c;
parameter real IS = 1.0e-14;
parameter real CJO = 0.0;
analog function real spicepnjlim;
input vnew, vold, vt, vcrit;
real vnew, vold, vt, vcrit, vlimit, arg;
begin
vlimit=vnew;
if ((vnew > vcrit) && ( abs (vnew-vold) > (vt+vt))) begin
if (vold > 0) begin
arg = 1 + (vnew-vold) / vt;
if (arg > 0)
vlimit = vold + vt * ln (arg);
else
vlimit = vcrit;
end else
vlimit = vt * ln (vnew/vt);
$discontinuity (-1);
end
spicepnjlim = vlimit;
end
endfunction
real vdio, idio, qdio, vcrit;
analog begin
vcrit=0.7;
vdio = $limit (V(a,c), spicepnjlim, $vt , vcrit);
idio = IS * ( exp (vdio/ $vt ) - 1);
I(a,c) <+ idio;
if (vdio < 0.5) begin
qdio = 0.5 * CJO * (1- sqrt (1-V(a,c)));
end else begin
qdio = CJO* (2.0*(1.0- sqrt (0.5))
+ sqrt (2.0)/2.0*(vdio*vdio+vdio-3.0/4.0));
end
I(a,c) <+ ddt (qdio);
end
endmodule
```
### 9.18 Hierarchical parameter system functions.....................................................................................

Verilog AMS HDL adds system functions that can return hierarchically inherited values in a particular
instance.

The syntax for these functions are shown in Syntax 9- 13.

hierarchical_parameter_system_functions ::=
**$mfactor**
| **$xposition**
| **$yposition**
| **$angle**
| **$hflip**
| **$vflip**


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
249
```
```
Syntax 9-13—Syntax for the hierarchical parameter system functions
```
These functions return hierarchical information about the instance of a module or paramset. Subclause 6.3.6
discusses how these parameters are specified for an instance, as well as the automatic rules applied to
instances with a non-unity value of **$mfactor**. The remaining hierarchical system parameters do not have
any automatic effect on the simulation.

**$mfactor** is the shunt multiplicity factor of the instance, that is, the number of identical devices that
should be combined in parallel and modeled.

**$xposition** and **$yposition** are the offsets, in meters, of the location of the center of the instance.

**$hflip** and **$vflip** are used to indicate that the instance has been mirrored about its center, and **$angle**
indicates that the instance has been rotated some number of degrees in the counter-clockwise directions.

Hierarchical parameter system functions can also be used as targets in parameter alias declarations (see
3.4.7)

The value returned for each of these functions is computed by combining values from the top of the hierar-
chy down to the instance making the function call. The rules for combining the values are given in
Table 9- 29. The top-level value is the starting value at the top of the hierarchy. If a module is instantiated
without specifying a value of one of these system parameters (using any of the methods in 6.3), then the
value of that system parameter will be unchanged from the instantiating module. If a value is specified, then
it is combined with the value from the instantiating module according to the appropriate rule from
Table 9- 29 : the subscript “specified” indicates the value specified for the instance, and the subscript “hier”
indicates the value obtained by traversing the hierarchy from the top to the instantiating module.

For example, when a module makes a call to **$mfactor** , the simulator computes the product of the multi-
plicity factor specified for the instance (or 1.0, if no override was specified) times the value for the parent
module that instantiated the module, times the parent’s parent’s value, and so on, until the top level is
reached.

Note that **$angle** is specified and returned in degrees, but the trigonometric functions of 4.3.2 operate in
radians.

```
Table 9-29— Hierarchical parameter values
```
```
System
parameter
```
```
Top-level
value Resolved value for instance Allowed values
```
```
$angle 0 degrees $anglespecified + $anglehier ,
modulo 360 degrees
```
```
0  $angle < 360
```
```
$hflip +1 $hflipspecified * $hfliphier $hflip = +1 or -1
$mfactor 1.0 $mfactorspecified * $mfactorhier $mfactor > 0
$vflip +1 $vflipspecified * $vfliphier $vflip = +1 or -1
$xposition 0.0 m $xpositionspecified +
$xpositionhier
```
```
Any
```
```
$yposition 0.0 m $ypositionspecified +
$ypositionhier
```
```
Any
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
250
```
Example 1

```
module test_module(p,n);
inout p,n;
electrical p,n;
module_a A1(p,n);
endmodule
```
```
module module_a(p,n);
inout p,n;
electrical p,n;
module_b #(.$mfactor(2)) B1(p,n); // mfactor = 3 * 2
endmodule
```
```
module module_b(p,n);
inout p,n;
electrical p,n;
module_c #(.$mfactor(7)) C1(p,n); // mfactor = 3 * 2 * 7 = 42
endmodule
```
```
// linear resistor
module module_c(p,n);
inout p,n;
electrical p,n;
parameter r=1.0;
(* desc = "effective resistance" *) real reff;
analog begin
I(p,n) <+ V(p,n)/r; // mfactor scaling of currents
// handled automatically
reff = r / $mfactor ; // the effective resistance = 1/42
end
endmodule
```
shows how the effect mfactor of an instance, test_module.A1.B1.C1 of a linear resistance is determined.

Example 2

```
module test_module(p,n);
inout p,n;
electrical p,n;
module_a A1(p,n);
endmodule
```
```
module module_a(p,n);
inout p,n;
electrical p,n;
module_b #(. $xposition (1u)) B1(p,n); // xposition=1.1u + 1u
endmodule
```
```
module module_b(p,n);
inout p,n;
electrical p,n;
module_c #(. $xposition (2u)) C1(p,n); // xposition=1.1u + 1u + 2u = 4.1u
endmodule
```
```
// linear resistor
module module_c(p,n);
inout p,n;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
251
```
```
electrical p,n;
parameter r=1.0;
analog begin
// Expected value of xposition=4.1e-6
if ( $xposition == 4.1u)
I(p,n) <+ V(p,n)/1.0;
else
I(p,n) <+ V(p,n)/2.0;
end
endmodule
```
### 9.19 Explicit binding detection system functions

Verilog AMS HDL adds functions that can be used to check whether a parameter or port binding was explic-
itly made.

The behavioral code of a module can depend on the way in which it was instantiated. The hierarchy detec-
tion functions shown in Syntax 9- 14 may be used to determine information about the instantiation.

genvar_system_function ::=
**$param_given (** module_parameter_identifier **)**
| **$port_connected (** port_scalar_expression **)**

```
Syntax 9-14—Syntax for the hierarchy detection functions
```
Note that the return values of these functions shall be constant during a simulation; the value is fixed during
elaboration. As such, these functions can be used in genvar expressions controlling conditional or looping
behavior of the analog operators of 4.5.

The **$param_given()** function can be used to determine whether a parameter value was obtained from
the default value in its declaration statement or if that value was overridden. The **$param_given()** func-
tion takes a single argument, which must be a parameter identifier. The return value shall be one (1) if the
parameter was overridden, either by a **defparam** statement or by a module instance parameter value
assignment, and zero (0) otherwise.

The following example sets the variable temp to represent the device temperature. Note that **$tempera-
ture** is not a _constant_expression_ , so it cannot be used as the default value of the parameter tdevice. It is
important to be able to distinguish the case where tdevice has its default value (say, 27) from the declara-
tion statement from the case where the value 27 was in fact specified as an override, if the simulation is per-
formed at a different temperature.

```
if ( $param_given (tdevice))
temp = tdevice + ‘P_CELSIUS0;
else
temp = $temperature ;
```
Module ports need not be connected when the module is instantiated. The **$port_connected()** function
can be used to determine whether a connection was specified for a port. The **$port_connected()** func-
tion takes one argument, which must be a port identifier. The return value shall be one (1) if the port was
connected to a net (by order or by name) when the module was instantiated, and zero (0) otherwise. Note
that the port may be connected to a net that has no other connections, but **$port_connected()** shall still
return one.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
252
```
In the following example, **$port_connected()** is used to skip the **transition** filter for uncon-
nected nodes. In module twoclk, the instances of myclk only have connections for their vout_q ports, and
thus the filter for vout_qbar is not implemented for either instance. In module top, the vout_q2 port is
not connected, so that the vout_q port of topclk1.clk2 is not ultimately used in the circuit; however, the
filter for vout_q of clk2 is implemented, because it vout_q is connected on clk2’s instantiation line.

```
module myclk(vout_q, vout_qbar);
output vout_q, vout_qbar;
electrical vout_q, vout_qbar;
parameter real tdel = 3u from [0:inf);
parameter real trise = 1u from (0:inf);
parameter real tfall = 1u from (0:inf);
parameter real period = 20u from (0:inf);
integer q;
analog begin
@ ( timer (0, period))
q = 0;
@ ( timer (period/2, period))
q = 1;
if ($ port_connected (vout_q))
V(vout_q) <+ transition ( q, tdel, trise, tfall);
else
V(vout_q) <+ 0.0;
if ($ port_connected (vout_qbar))
V(vout_qbar) <+ transition ( !q, tdel, trise, tfall);
else
V(vout_qbar) <+ 0.0;
end
endmodule
```
```
module twoclk(vout_q1, vout_q2);
output vout_q1, vout_q2;
electrical vout_q1, vout_q1b;
myclk clk1(.vout_q(vout_q1));
myclk clk2(.vout_q(vout_q2));
endmodule
```
```
module top(clk_out);
output clk_out;
electrical clk_out;
twoclk topclk1(.vout_q1(clk_out));
endmodule
```
### 9.20 Analog node alias system functions.............................................................................................

Verilog-AMS HDL adds system functions that allows a local node to be aliased to a hierarchical node via a
string reference. The node alias system functions are shown in Syntax 9- 15.

analog_node_alias_system_function ::=
**$analog_node_alias (** analog_net_reference , hierarchical_reference_string **)**
| **$analog_port_alias (** analog_net_reference , hierarchical_reference_string **)**

```
Syntax 9-15—Syntax for the analog node alias system functions
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
253
```
Both system functions take two arguments. The _analog_net_reference_ shall be either a scalar or vector con-
tinuous node declared in the module containing the system function call. If the _analog_net_reference_ is a
vector node, it shall reference the full vector node, it shall be an error for it to be a bit select or part select of
a vector node. It shall be an error for the _analog_net_reference_ to be a port or to be involved in port connec-
tions.

The _hierarchical_reference_string_ shall be a constant string value (string literal or string parameter) con-
taining a hierarchical reference to a continuous node. It shall be an error for the _hierarchical_refer-
ence_string_ to reference a node that is used as an _analog_net_reference_ in another
**$analog_node_alias()** or **$analog_port_alias()** system function call. The _hierarchical_ref-
erence_string_ shall follow the resolution rules for hierarchical references as described in 6.7.

It shall be an error for the **$analog_node_alias()** and **$analog_port_alias()** system functions
to be used outside the **analog initial** block. Along with their enclosing **analog initial** block
scopes, both system functions shall be re-evaluated each sweep point of a dc sweep as needed (see 5.2.1 and
8.2 for details).

The **$analog_node_alias()** and **$analog_port_alias()** system functions shall not be used
inside conditional ( **if** , **case** , or **?:** ) statements unless the conditional expression controlling the statement
consists of terms which can not change during the course of a simulation.

The return value for both system functions shall be one (1) if the _hierarchical_reference_string_ points to a
valid continuous node and zero (0) otherwise. If the _hierarchical_reference_string_ references a valid contin-
uous node, then the _analog_net_reference_ will be aliased to that hierarchical node and shall refer to the same
circuit matrix position.

If the return value is zero (0), then the node referenced by the _analog_net_reference_ shall be treated as a nor-
mal continuous node declared in the module containing the system function call. As such, the user is encour-
aged to check the return value from the system function call and to take necessary steps to avoid runtime
topology issues like a singular matrix.

In addition, a node that is aliased to a valid hierarchical port reference via the **$analog_port_alias()**
function shall be allowed as an input to the port access function (see 5.4.3) and shall measure the flow
through the port of the instance referred to by the hierarchical reference.

If a particular node is involved in multiple calls to either system function, then the last evaluated call shall
take precedence.

The following rules shall be applied to determine whether the node or port referenced by the hierarchical_
reference_string is considered valid. If any of these rules are violated then the system function shall return a
value of zero (0).

```
— The hierarchical_reference_string shall refer to a scalar continuous node or a scalar element of a
continuous vector node
— The discipline of the analog_net_reference and the resolved hierarchical node reference shall be
compatible (see 3.11)
— For the $analog_port_alias() system function, the resolved hierarchical node reference
shall be a port
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
254
```
An example showing the use of these two system functions is as follows:

```
module top;
electrical a, b, c, gnd;
ground gnd;
resistor #(.r(1k)) r1(.p(a), .n(gnd));
checker #(.n1_str("top.a")) ch1();
analog V(a,gnd) <+ 5;
endmodule
```
```
module checker;
electrical n1, n2;
parameter string n1_str = "(not_given)";
integer status;
real iprobe, vprobe;
analog initial begin
// node n1 will be aliased to top.gnd
status = $analog_node_alias (n1, "top.gnd");
```
```
// Invalid alias as top.a is not a port. Node n2 at this stage
// is still just a local node in ch1(). The integer status will
// be assigned a value of 0.
status = $analog_port_alias (n2, "$root.top.a");
```
```
// even though n1 was assigned a valid alias to top.gnd, this
// one to top.a will take precedence.
status = $analog_node_alias (n1, n1_str);
```
```
// Here n2 is now successfully aliased to the port p in instance r1.
status = $analog_port_alias (n2, "top.r1.p");
end
analog begin
// since n2 is aliased to the port p of instance top.r1
// we are allowed to probe the port current. In this case,
// the probe will return a value of 5mA.
iprobe = I(<n2>);
```
```
// since n1 is aliased to the node top.a, we will be
// probing the potential of that node. In this case,
// the probe will return a value of 5V.
vprobe = V(n1);
end
endmodule
```
### 9.21 Table based interpolation and lookup system function................................................................

Verilog-AMS HDL provides a multidimensional interpolation and lookup function called **$table_-
model**. The function is designed to operate specifically on multidimensional data in a form that is com-
monly generated via parametric sweeping schemes available in most analog simulators. This type of data is
generated when simulating a system while varying (sweeping) a parameter across some range. Data dimen-
sionality increases when parameter sweeps are nested. While the samples are those of a multidimensional
function, sample generation via parametric sweeping leads to a simple recursive interpolation and extrapola-
tion process defined by the **$table_model** function.

A typical example will help to explain the process. A user may wish to create a data based model of some
function _f(x,y)_ over some range of _x_ and _y_ and use that data as the basis of a behavioral model described in
Verilog-AMS.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
255
```
```
Figure 9-1: Samples on isolines
```
We can say that _f(x,y)_ is sampled on a set of _isolines_. An isoline for each value of _y_ is generated when _y_ is
held constant and _x_ is varied across a desired range. Each isoline may exist over a different range of _x_ values
and the number and spacing of samples may be different on each isoline.

When describing the sampled set, _x_ and _y_ are called independent variables and _f(x,y)_ is called the dependent
variable. The sampling scheme also introduces the concept of an innermost and outermost dimension. In this
example, _x_ is the fastest changing or innermost dimension associated with the sampled function _f(x,y)_ and _y_
is the slowest changing or outermost dimension.

Understanding that the underlying multidimensional function is sampled on a set of isolines, we can now
describe a simple recursive process to interpolate, extrapolate, or perform lookup on this sampled function.

```
Figure 9-2: Interpolation on isolines
```
##### 4

##### 3

##### 2

##### 1

(^246)
_y=0.0
y=1.0
y=0.5
f(x,y)
x_

##### 4

##### 3

##### 2

##### 1

##### 2 4 6

```
y=0.0
```
```
y=1.0
y=0.5
```
```
f(x,y)
```
```
x
```
```
x 1 =3.5
```
```
yh yl
```
```
f(x 1 ,y)
```
```
0.0 y 1.0
```
```
y 1 =0.25
```
```
y
```
```
2.75
```
```
1.75
```
```
f(x 1 ,y 1 )=2.0
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
256
```
Using the above example let us assume the user requests a value for the lookup pair _(x 1 ,y 1 )_. We first look
through the set of isolines in _y_ and find the pair that bracket _y 1_. Now for each isoline in _y_ we find the two
points that bracket _x 1_ and interpolate each isoline to find _f(x 1 ,yh)_ and _f(x 1 ,yl)_. Having thus generated an iso-
line in _y_ for the point _x 1_ in _x_ , we may interpolate this isoline to find the value _f(x1,y1)_. If the lookup point
falls off the end of any given isoline then we extrapolate its value on that isoline.

As a consequence of this algorithm, the interpolation and extrapolation schemes always operate in a single
dimension analogous to how the data was originally generated, so the interpolation and extrapolation
schemes used may be specified on a per dimension basis.

The minimum data requirement is to have the product of at least two points per dimension (2N for N dimen-
sions). In addition, the result of the bracketing to produce intermediate points (as per Figure 9- 2 and descrip-
tion above) must also produce at least two points per subsequent lower dimension. Within the data set, each
point shall be distinct in terms of its independent variable values. If there are two or more data points with
the same independent and dependent values, then the duplicates shall be ignored and the tool may generate a
warning. If there are two or more data points with the same independent values but different dependent val-
ues then an error is generated.

The **$table_model** function defines a format to represent the isolines of multidimensional data and a set
of interpolation schemes that we need only define for single dimensional data. The data may be stored in a
file or as a sequence of one-dimension arrays or a single two-dimensional array.

The interpolation schemes are closest point (discrete) lookup, linear, quadratic splines, and cubic splines.
Extrapolation may be specified as being constant, linear, or error (meaning if extrapolation occurs the sys-
tem should error out).

The lookup variables, _(x 1 , y 1 )_ in the example above (table_inputs in Syntax 9- 16 ) may be any legal
expression that can be assigned to an analog signal.

The syntax for the **$table_model** function is shown in Syntax 9- 16.

table_model_function ::=
**$table_model (** table_inputs **,** table_data_source [ **,** table_control_string] **)**

table_inputs ::=
expression [ **,** 2nd_dim_expression [ **,** nth_dim_expression]]

table_data_source ::=
file_name | table_model_array

file_name ::=
string_literal | string_parameter

table_model_array ::=
1st_dim_array_identifier [ **,** 2nd_dim_array_identifier [ **,** nth_dim_array_identifier]] **,**
output_array_identifier

table_control_string::=
**"** [interp_control[ **;** dependent_selector]] **"**

interp_control::=
1st_dim_table_ctrl_substr_or_null [ **,** 2nd_dim_table_ctrl_substr_or_null [ **,** nth_dim_table_ctrl_-
substr_or_null]]

dependent_selector::=
integer

table_ctrl_substr ::=
[table_interp_char][table_extrap_char [higher_table_extrap_char]]


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
257
```
table_interp_char ::=
**I** | **D** | **1** | **2** | **3**

table_extrap_char ::=
**C** | **L** | **E**

```
Syntax 9-16—Syntax for table model function
```
#### 9.21.1 Table data source

_table_data_source_ specifies the data source, samples of a multidimensional function arranged on isolines.
The data is specified as columns in a text file or as a set of arrays (hence the name _table_ model). In either
case the layout is conceptually the same.

A table of M dependent variables of dimension N are laid out in N+M columns in the file, with the indepen-
dent variables appearing in the first N columns followed by the dependent variables in the remaining M col-
umns. The independent variables are ordered from the outermost (slowest changing) variable to the
innermost (fastest changing) variable. Though an isoline ordinate does not change for a given isoline, in this
scheme the ordinate is repeated for each point of that isoline (thus keeping the input data as a set of data
rows all with the same number of points). The result is a sequential listing of each isoline with the total num-
ber of points in the listing being equal to the total number of samples on all isolines.

Again, the above example described via samples will help illustrate the layout. The function being described
is
_f(x,y)=0.5x + y_

_f(x,y)_ is the only dependent variable we consider in this case, and there are three isolines for values of _y_ 0.0,
0.5 and 1.0; _x_ is sampled at various points on each of the three isolines.

```
# 2-D table model sample example
#
# y x f(x,y)
#y=0 isoline
0.0 1.0 0.5
0.0 2.0 1.0
0.0 3.0 1.5
0.0 4.0 2.0
0.0 5.0 2.5
0.0 6.0 3.0
#y=0.5 isoline
0.5 1.0 1.0
0.5 3.0 2.0
0.5 5.0 3.0
#y=1.0 isoline
1.0 1.0 1.5
1.0 2.0 2.0
1.0 4.0 3.0
```
As can be seen here, the slowly changing outer independent variable appears to the left while the rapidly
changing inner independent variable appears to the right; isoline ordinates are repeated for each sample on a
given isoline.

Each sample point is separated by a newline and each column is separated by one or more spaces or tabs.
Comments begin with ‘#’ and continue to the end of that line. They may appear anywhere in the file. Blank
lines are ignored. The numbers shall be real or integer.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
258
```
When the data source is a sequence of 1-D arrays the isolines are laid out in conceptually the same way with
each array being just as a column in the file format described above. Arrays may be specified directly via the
concatenation operator or via array variable names.

The state of the data source is captured on the first call to the table model function. Any change after this
point is ignored.

While it is suggested that the user arrange the sampled isolines in sorted order (one isoline following another
in all dimensions); if the user provides the data in random order the system will sort the data into isolines in
each dimension. Whether the data is sorted or not, the system determines the isoline ordinate by reading its
exact value from the file or array. Any noise on the isoline ordinate may cause the system to incorrectly gen-
erate multiple isolines where the user intended a single isoline.

The input example above illustrated the isoline format for a single two-dimensional function (or dependent
variable). The file may contain multiple dependent variables, all sharing the same set of isoline samples. A
column in the data source may also be marked as _ignore_. These and all interpolation control settings are pro-
vided via the interpolation control string.

#### 9.21.2 Control string

The control string is used to specify how the **$table_model** function should interpolate or lookup the
data in each dimension and how it should extrapolate at the boundaries of each dimension. It also provides
for some control on how to treat columns of the input data source. The string consists of a set of comma sep-
arated sub-strings followed by a semicolon and the dependent selector. The first group of sub-strings pro-
vide control over each independent variable with the first sub-string applying to the outermost dimension
and so on. The dependent variable selector is a column number allowing us to specify which dependent vari-
able in the data source we wish to interpolate. This number runs 1 though M with M being the total number
of dependent variables specified in the data source.

Each sub-string associated with interpolation control has at most 3 characters. The first character controls
interpolation and obeys Table 9- 30.

```
Table 9-30—Interpolation control character
```
```
Control
character Description
```
```
I Ignore this input column
D Closest point (discrete) lookup
1 Linear interpolation (default)
2 Quadratic spline interpolation
3 Cubic spline interpolation
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
259
```
The remaining character(s) in the sub-string specify the extrapolation behavior.

The constant extrapolation method returns the table endpoint value. Linear extrapolation extends linearly to
the requested point from the endpoint using a slope consistent with the selected interpolation method. The
user may also disable extrapolation by choosing the error extrapolation method. With this method, an
extrapolation error is reported if the **$table_model** function is requested to evaluate a point beyond the
interpolation region.

For each dimension, users may use up to 2 extrapolation method characters to specify the extrapolation
method used for each end. When no extrapolation method character is given, the linear extrapolation method
will be used for both ends as default. Error extrapolation results in a fatal error being raised. When one
extrapolation method character is given, the specified extrapolation method will be used for both ends.
When two extrapolation method characters are given, the first character specifies the extrapolation method
used for the end with the lower coordinate value, and the second character is used for the end with the higher
coordinate value.

```
Table 9-31—Extrapolation control character
```
```
Control
character Description
```
```
C Constant extrapolation
L Linear extrapolation (default)
E Error on an extrapolation request
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
260
```
#### 9.21.3 Example control strings

#### 9.21.4 Interpolation algorithms

The closest point lookup algorithm returns the closest point in the specified dimension. When the lookup
ordinate is equidistant from two bracketing samples the function snaps away from zero.

The linear interpolation algorithm provides a simple linear interpolation between the closest sample points
on a given isoline. Cubic spline interpolation^1 generates a spline for each isoline being interpolated. The
extrapolation option specified is taken into account when generating the spline coefficients so as to avoid
end point discontinuities in the first order derivative of the interpolated function.

When formulating the cubic spline equations the desired derivative of the interpolation function at both end
points must be specified in order to provide the complete set of constraints for the cubic spline equations. It
is convenient then that the table model function specifies end point extrapolation behavior. If the user selects
linear extrapolation this leads to a _natural_ spline. If constant extrapolation is specified the end point deriva-
tive is set to zero thus avoiding a discontinuity in the first order derivative at that end point.

Quadratic splines are similar to cubic splines, offering more efficient evaluation with generally less favor-
able interpolation results. Again one should attempt to avoid end point discontinuities, though it is not
always possible in this case.

As a general rule cubic splines are best applied to smoothly varying samples (such as the DC I-V character-
istic of a diode) while linear interpolation is a better option for data with abrupt transitions (such as a tran-
sient pulsed waveform).

```
Table 9-32—Example control strings
```
```
Control string Description
```
```
"" or control
string omitted
```
```
Null string, default linear interpolation and extrapolation. Dimensional-
ity of the data is assumed to be N. Column N+1 is taken as the depen-
dent.
"1L,1L" Data is 2-D, linear interpolation and extrapolation in both dimensions.
"1LL,1LL" Same as above, extrapolation method specified for both ends in each
dimension.
"1LL,1LL;1" Same as above, dependent variable 1 is specified. This is the default
behavior when there are multiple dependent variables in the file and
there is no dependent variable selector specified in the control string
"D,1,3" Closest point lookup in the outer dimension, linear interpolation on
dimension two and cubic spline interpolation on the inner dimension.
"I,1CC,1CC;3" Ignore column 1, linear interpolation and constant extrapolation in all
dimensions, interpolation applies to dependent variable 3. There are at
least 6 column in the data file.
"3,D,I,1;3" Cubic spline interpolation in dimension 3, (column 1), closest lookup in
dimension 2 (column 2), ignore column 3, and use linear interpolation on
the innermost dimension (dimension 1, column 4). Interpolate dependent
variable 3 (column 7). This file has at least 7 columns.
“C,,3” Data is 3D, equivalent to “1CC, 1LL, 3LL”.
```
(^1) Numerical Methods in Scientific Computing, Germund Dahlquist and Åke Björck.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
261
```
#### 9.21.5 Example

A simple call to the **$table_model** function is illustrated using the two dimensional sample set given in
the above plots and tables. Let’s first assume this data is stored in a file called _sample.dat_. The data is two
dimensional with a single dependent variable. The independent variables were named _x_ and _y_ above. _y_ is the
outermost independent variable in the sample set while _x_ is the innermost independent variable.

```
module example_tb(a, b);
electrical a, b;
inout a, b;
analog begin
I(a, b) <+ $table_model (0.0, V(a,b),"sample.dat");
end
endmodule
```
This instance specifies zero for _y_ and uses a module potential to interpolate _x_. No control string is specified
and so the function defaults to performing linear interpolation and linear extrapolation in both dimensions.

It is possible to specify how to perform the interpolation via the control string:

```
I(a, b) <+ $table_model (0.0, V(a,b),"sample.dat", "1LL,3LL");
```
Linear interpolation and extrapolation are specified in _y_ and cubic interpolation with linear extrapolation in
_x_.

The data source may also be specified as an array.

```
module example_tb(a, b);
electrical a, b;
inout a, b;
real y[0:11], x[0:11],f_xy[0:11];
analog begin
@( initial_step ) begin
// y=0.0 isoline
y[0] =0.0; x[0] =1.0; f_xy[0] =0.5;
y[1] =0.0; x[1] =2.0; f_xy[1] =1.0;
y[2] =0.0; x[2] =3.0; f_xy[2] =1.5;
y[3] =0.0; x[3] =4.0; f_xy[3] =2.0;
y[4] =0.0; x[4] =5.0; f_xy[4] =2.5;
y[5] =0.0; x[5] =6.0; f_xy[5] =3.0;
// y=0.5 isoline
y[6] =0.5; x[6] =1.0; f_xy[6] =1.0;
y[7] =0.5; x[7] =3.0; f_xy[7] =2.0;
y[8] =0.5; x[8] =5.0; f_xy[8] =3.0;
// y=1.0 isoline
y[9] =1.0; x[9] =1.0; f_xy[9] =1.5;
y[10]=1.0; x[10]=2.0; f_xy[10]=2.0;
y[11]=1.0; x[11]=4.0; f_xy[11]=3.0;
end
I(a, b) <+ $table_model (0, V(a,b), y, x, f_xy);
end
endmodule
```
Here the array is specified via array variables. The variables are initialized inside an **initial_step** block
ensuring that they do not change after the first call to **$table_model**. Arrays may also be specified
directly via an array assignment pattern.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
262
```
### 9.22 Connectmodule driver access system functions and operator

Verilog-AMS HDL extends IEEE Std 1364-2005 Verilog HDL with a set of driver access functions. Ver-
ilog-AMS HDL also adds a new operator, **driver_update** that is used in combination with the driver
access functions which is described here for that reason.

Access to individual drivers and net resolution is necessary for accurate implementation of connect modules
(see 7.5). A _driver_ of a signal is a process which assigns a value to the signal, or a connection of the signal to
an output port of a module instance or simulation primitive.

The driver access functions described here only access drivers found in ordinary modules and not to those
found in connect modules. Driver access functions can only be called from connect modules.

A signal can have any number of drivers; for each driver the current status, value, and strength can be
accessed.

#### 9.22.1 $driver_count

**$driver_count** returns an integer representing the number of drivers associated with the signal in ques-
tion. The syntax is shown in Syntax 9- 17.

driver_count_function ::=
**$driver_count (** signal_name **)**

```
Syntax 9-17—Syntax for $driver_count
```
The drivers are arbitrarily numbered from 0 to N-1, where N is the total number of ordinary drivers contrib-
uting to the signal value. For example, if this function returns a value 5 then the signal has five drivers num-
bered from 0 to 4.

#### 9.22.2 $receiver_count

**$receiver_count** returns an integer representing the number of receivers associated with the signal in
question. The syntax is shown in Syntax 9- 18.

receiver_count_function ::=
**$receiver_count (** signal_name **)**

```
Syntax 9-18—Syntax for $receiver_count
```
The receivers are arbitrarily numbered from 0 to N-1, where N is the total number of ordinary receivers
being contributed by the signal value. For example, if this function returns a value 5 then the signal has five
receivers numbered from 0 to 4.

#### 9.22.3 $driver_state

**driver_state** returns the current value contribution of a specific driver to the state of the signal. The
syntax is shown in Syntax 9- 19.

driver_state_function ::=


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
263
```
```
$driver_state ( signal_name , driver_index )
```
```
Syntax 9-19—Syntax for $driver_state
```
```
driver_index is an integer value between 0 and N-1, where N is the total number of drivers contributing to
the signal value. The state value is returned as 0 , 1 , x, or z.
```
#### 9.22.4 $driver_strength

```
driver_strength returns the current strength contribution of a specific driver to the strength of the sig-
nal. The syntax is shown in Syntax 9- 20.
```
```
driver_strength_function ::=
$driver_strength ( signal_name , driver_index )
```
```
Syntax 9-20—Syntax for $driver_strength
```
```
driver_index is an integer value between 0 and N-1, where N is the total number of drivers contributing to
the signal value. The strength value is returned as two strengths, Bits 5-3 for strength0 and Bits 2-0
for strength1 (see IEEE Std 1364-2005 Verilog HDL, subclauses 7.10 and 7.11).
```
```
If the value returned is 0 or 1 , strength0 returns the high-end of the strength range and strength1
returns the low-end of the strength range. Otherwise, the strengths of both strength0 and strength1 is
defined as shown in Figure 9- 3 below.
```
```
Figure 9-3: Strength value mapping
```
#### 9.22.5 driver_update

```
The status of drivers for a given signal can be monitored with the event detection keyword driver_up-
date. It can be used in conjunction with the event detection operator @ to detect updates to any of the driv-
ers of the signal.
```
```
Example:
```
```
always @ ( driver_update clock)
statement;
```
```
strength0 strength1
```
Bits 7
Su0

```
6
St0
```
```
5
Pu0
```
```
4
La0
```
```
3
We
0
```
```
2
Me0
```
```
1
Sm0
```
```
0
HiZ0
```
```
0
HiZ1
```
```
1
Sm1
```
```
2
Me1
```
```
3
We
1
```
```
4
La1
```
```
5
Pu1
```
```
6
St1
```
```
7
Su1
```
```
Bits
```
```
B511110 0 0 0 0 0 0 01111B2
B411001 1 0 0 0 0 1 10011B1
B310101 0 1 0 0 1 0 10101B0
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
264
```
causes the statement to execute any time a driver of the signal clock is updated. Here, an update is
defined as the addition of a new pending value to the driver. This is true whether or not there is a change in
the resolved value of the signal.

#### 9.22.6 Receiver net resolution....................................................................................................

As a result of driver receiver segregation, the drivers and receivers are separated so that any analog con-
nected to a mixed net has the opportunity to influence the value driving the digital receivers. Since a single
digital port is used in the connect module, the user must specify the value that the receivers will see. By not
specifying the receiver value directly in the connect module driver, receiver segregation will be ignored,
which is the default case. This assignment of the receiver value is done via the **assign** statement in which
the digital port will be used to read the driver values as well as to set the receiver value.

```
1) The default is equivalent of assign d_receivers = d_drivers ;
Where the value passed to the receivers through driver receiver segregation is the value being driven
without delay or any impact from analog connections to the net. This is essentially bypassing driver
receiver segregation.
2) Anything else is done explicitly, such as:
reg out; // value of out determined in CM, see example in 9.22.7
assign d = out;
In this case, the digital port of the connect module will drive the receivers with a value determined in
the connect module. This value may potentially be different from the value of the drivers of the con-
nect module digital port.
```
#### 9.22.7 Connect module example using driver access functions

Using the example shown in Figure 9- 4 , a connect module can be created using driver access functions to
accurately model the effects of multiple drivers on an interface.

```
Figure 9-4: Driver-receiver segregation connect module example
```
The connect module below takes advantage of much of the mixed-signal language including driver access
functions. This module effectively adds another parallel resistor from output to ground whenever a digital
output connected to the net goes high, and another parallel resistor from output to rail ( _supply_ ) whenever a
digital output connected to the net goes low. If this is used as the connect module in Figure 9- 4 , not only is

```
d3
```
```
d1
```
```
d2
```
```
c2e
```
```
c1
```
```
n1
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
265
```
the delay from digital outputs to the digital input a function of the value of the capacitor, for a given capaci-
tance it takes approximately half the time (since two gates are driving the signal rather than one).

```
connectmodule c2e(d,a);
input d;
output a;
```
```
ddiscrete d;
```
### electrical a, rail, gnd;

```
reg out;
ground gnd;
branch (rail,a)pull_up;
branch (a,gnd)pull_down;
branch (rail,gnd)power;
parameter real impedance0 = 120.0;
parameter real impedance1 = 100.0;
parameter real impedanceOff = 1e6;
parameter real vt_hi = 3.5;
parameter real vt_lo = 1.5;
parameter real supply = 5.0;
integer i, num_ones, num_zeros;
```
```
assign d=out;
```
### initial begin

```
num_ones = 0;
num_zeros = 0;
end
```
```
always @(driver_update (d)) begin
num_ones = 0;
num_zeros = 0;
for ( i = 0; i < $driver_count (d); i=i+1)
if ( $driver_state (d,i) == 1 )
num_ones = num_ones + 1;
else
num_zeros = num_zeros + 1;
end
```
```
always @(cross (V(a) - vt_hi, -1) or cross (V(a) - vt_lo, +1))
out = 1'bx;
always @(cross (V(a) - vt_hi, +1))
out = 1'b1;
always @(cross (V(a) - vt_lo, -1))
out = 1'b0;
```
```
analog begin
// Approximately one impedance1 resistor to rail per high output
// connected to the digital net
V(pull_up) <+ 1/((1/impedance1)*num_ones+(1/impedanceOff)) *
I(pull_up);
// Approximately one impedance0 resistor to ground per low output
// connected to the digital net
V(pull_down) <+ 1/((1/impedance0)*num_zeros+(1/impedanceOff)) *
I(pull_down);
V(power) <+ supply;
end
endmodule
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
266
```
### 9.23 Supplementary connectmodule driver access system functions

Verilog-AMS HDL extends IEEE Std 1364-2005 Verilog HDL so that a set of supplementary driver access
functions are supported in the digital context of connectmodules.

These driver access functions are provided for access to digital events which have been scheduled onto a
driver but might not have matured by the current simulation time.

These functions can be used to create analog waveforms which cross a specified threshold at the same time
the digital event matures, thus providing accurate registration of analog and digital representations of a sig-
nal. This assumes there is at least as long a delay in the maturation of the digital signal as the required rise/
fall times of the analog waveform.

NOTE—Because the scheduled digital events can be scheduled with an insufficient delay or canceled before they
mature, be careful when using these functions.

#### 9.23.1 $driver_delay

**$driver_delay** returns the delay, from current simulation time, after which the pending state or strength
becomes active. If there is no pending value on a signal, it returns the value minus one (-1.0). The syntax is
shown in Syntax 9- 21.

driver_delay_function ::=
**$driver_delay (** signal_name **,** driver_index **)**

```
Syntax 9-21—Syntax for $driver_delay
```
_driver_index_ is an integer value between 0 and N-1, where N is the total number of drivers contributing to
the signal value. The returned delay value is a real number, which is defined by the **`timescale** for that
module where the call has been made. The fractional part arises from the possibility of a driver being
updated by an A2D event off the digital timeticks.

#### 9.23.2 $driver_next_state

**$driver_next_state** returns the pending state of the driver, if there is one. If there is no pending state,
it returns the current state. The syntax is shown in Syntax 9- 22.

driver_next_state_function ::=
**$driver_next_state (** signal_name **,** driver_index **)**

```
Syntax 9-22—Syntax for $driver_next_state
```
_driver_index_ is an integer value between 0 and N-1, where N is the total number of drivers contributing to
the signal value. The pending state value is returned as 1'b0, 1'b1, 1'bx, or 1'bz.

#### 9.23.3 $driver_next_strength

**$driver_next_strength** returns the strength associated with the pending state of the driver, if there is
one. If there is no pending state, it returns the current strength. The syntax is shown in Syntax 9- 23.

driver_next_strength_function ::=


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
267
```
```
$driver_next_strength ( signal_name , driver_index )
```
```
Syntax 9-23—Syntax for$ driver_next_strength
```
_driver_index_ is an integer value between 0 and N-1, where N is the total number of drivers contributing to
the signal value. The pending strength value is returned as an integer between 0 and 7.

#### 9.23.4 $driver_type

**$driver_type** returns an integer value with its bits set according to the system header file “driver_ac-
cess.vams” (refer to Annex D for the header file) for the driver specified by the signal_name and the driv-
er_index. Connect modules for digital to analog conversion can use the returned information to help
minimize the difference between the digital event time and the analog crossover when the user swaps
between coding styles and performs backannotation^1. A simulator that cannot provide proper information
for a given driver type should return 0 (‘DRIVER_UNKNOWN). All drivers on _wor_ and _wand_ nets will
have a bit set indicating such, and any extra drivers added by the kernel for pull-up or pull-down will be
marked as belonging to the kernel. The syntax is shown in Syntax 9- 24.

driver_type_function ::=
**$driver_type (** signal_name **,** driver_index **)**

```
Syntax 9-24—Syntax for$ driver_type
```
Digital primitives (like nand and nor gates) should always provide data about their scheduled output
changes; i.e., a gate with a 5ns delay should provide 5ns of look-ahead. Behavioral code with blocking
assigns cannot provide look-ahead, but non-blocking assigns with delays can. However, since the capability
is implementation- and configuration-dependent, this function is provided so that the connect module can
adapt for a particular instance.

(^1) SDF backannotation will not change which D2A is inserted.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
268
```
