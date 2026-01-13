## 4. Expressions

### 4.1 Overview........................................................................................................................................

This section describes the operators and operands available in the Verilog-AMS HDL, and how to use them
to form expressions.

An _expression_ is a construct which combines _operands_ with _operators_ to produce a result which is a func-
tion of the values of the operands and the semantic meaning of the operator. Any legal operand, such as an
integer or an indexed element from an array of reals, without a operator is also considered an expression.
Wherever a value is needed in a Verilog-AMS HDL statement, an expression can be used.

Some statement constructs require an expression to be a _constant expression_. The operands of a constant
expression consists of constant numbers and parameter names, but they can use any of the operators defined
in Table 4- 1 , Table 4- 14 , and Table 4- 15.

### 4.2 Operators........................................................................................................................................

The symbols for the Verilog-AMS HDL operators are similar to those in the C programming language.
Table 4- 1 lists these operators.

```
Table 4-1—Operators
```
```
{} {{}} Concatenation, replication
unary +, unary - Unary operators
+ - * / ** Arithmetic
% Modulus
> >= < <= Relational
! Logical negation
&& Logical and
|| Logical or
== Logical equality
!= Logical inequality
=== Case equality
!== Case inequality
~ Bitwise negation
& Bitwise and
| Bitwise inclusive or
^ Bitwise exclusive or
^~ or ~^ Bitwise equivalence
& Reduction and
~& Reduction nand
| Reduction or
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
52
```
#### 4.2.1 Operators with real operands

The operators shown in Table 4- 2 are legal when applied to real operands. All other operators are considered
illegal when used with real operands.

The result of using logical or relational operators on real numbers is an integer value 0 ( _false_ ) or 1 ( _true_ ).

If a real expression is used for the replication factor of a concatenation, the expression will first be converted
to an integer value using the rules described in 4.2.1.1, before it is used as the replication factor for the con-
catenation.

**4.2.1.1 Real to integer conversion**

Real numbers are converted to integers by rounding the real number to the nearest integer, rather than by
truncating it. Implicit conversion takes place when a real number is assigned to an integer. If the fractional
part of the real number is exactly 0.5, it shall be rounded away from zero.

Examples:

The real numbers 35.7 and 35.5 both become 36 when converted to an integer and 35.2 becomes 35.

Converting -1.5 to integer yields -2, converting 1.5 to integer yields 2.

```
~| Reduction nor
^ Reduction xor
~^ or ^~ Reduction xnor
<< Logical left shift
>> Logical right shift
<<< Arithmetic left shift
>>> Arithmetic right shift
?: Conditional
```
```
Table 4-2—Legal operators for use in real expressions
```
```
unary + unary - Unary operators
+ - * / ** Arithmetic
% Modulus
> >= < <= Relational
== != Logical equality
! && || Logical
?: Conditional
```
```
Table 4-1—Operators (continued)
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
53
```
**4.2.1.2 Integer to real conversion**

Implicit conversion shall take place when an expression is assigned to a real. Individual bits that are x or z in
the net or the variable shall be an error (see 7.3.2).

**4.2.1.3 Arithmetic conversion**

For operands, a common data type for each operand is determined before the operator is applied. If either
operand is real, the other operand is converted to real. Implicit conversion takes place when a integer num-
ber is used with a real number in an operand.

Examples:

```
a = 3 + 5.0;
The expression 3 + 5.0 is evaluated by “casting” the integer 3 to the real 3.0, and the result of the
expression is 8.0.
```
```
b = 1 / 2;
The above is integer division and the result is 0.
```
```
c = 8.0 + (1/2);
(1/2) is treated as integer division, but the result is cast to a real (0.0) during the addition, and the
result of the expression is 8.0.
```
```
d = 1 / 2.0;
Since the denominator is expressed as a real number (2.0) the above is treated as real division and
the result is 0.5;
```
#### 4.2.2 Operator precedence

The precedence order of _operators_ is shown in Table 4- 3.

```
Table 4-3—Precedence rules for operators
```
```
+ -! ~ & ~& | ~| ^ ~^ ^~ (unary) Highest precedence
**
* / %
+ - (binary)
<< >> <<< >>>
< <= > >=
== != === !==
& (bitwise)
^ ^~ ~^ (bitwise)
| (bitwise)
&&
|| (logical) or (event) , (event)
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
54
```
Operators shown on the same row in Table 4- 3 have the same precedence. Rows are arranged in order of
decreasing precedence for the operators. For example, ***** , **/** , and **%** all have the same precedence, which is
higher than that of the binary **+** and **-** operators.

All operators associate left to right with the exception of the conditional operator which associates right to
left. Associativity refers to the order in which the operators having the same precedence are evaluated.

In the following example B is added to A and then C is subtracted from the result of A+B.

```
A + B - C
```
When operators differ in precedence, the operators with higher precedence associate first.

In the following example, B is divided by C (division has higher precedence than addition) and then the
result is added to A.

```
A + B / C
```
Parentheses can be used to change the operator precedence.

```
(A + B) / C // not the same as A + B / C
```
#### 4.2.3 Expression evaluation order

The operators shall follow the associativity rules while evaluating an expression as described in 4.2.2. Some
operators (&&, ||, and ?:) shall use _short-circuit evaluation_ ; in other words, some of their operand expres-
sions shall not be evaluated as long as the expression contains no analog operators and their value is not
required to determine the final value of the operation. All other operators shall not use _short-circuit evalua-
tion_ - all of their operand expressions are always evaluated. When short circuiting occurs, any side effects or
runtime errors that would have occurred due to evaluation of the short-circuited operand expression shall not
occur.

Example 1 - All operand expressions being evaluated:

```
integer varA, varB, varC, result;
analog function integer myFunc;
...
endfunction
result = varA & (varB | myFunc(varC));
```
Even if varA is known to be zero, the subexpression (varB | myFunc(varC)) will be evaluated and any
side effects caused by calling myFunc(varC) will occur.

Example 2 - Short-circuiting being applied:

```
integer varA, varB, varC, result;
result = varA && (varB || varC);
```
```
?: (conditional operator)
{} {{}} Lowest precedence
```
```
Table 4-3—Precedence rules for operators (continued)
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
55
```
If varA is known to be zero ( 0 ), the result of the expression can be determined as zero ( 0 ) without evaluating
the sub-expression (varB || varC).

Note that implementations are free to optimize by omitting evaluation of subexpressions as long as the sim-
ulation behavior (including side effects) is as if the standard rules were followed.

#### 4.2.4 Arithmetic operators

Table 4- 4 shows the binary arithmetic operators.

Integer division truncates any fractional part toward zero ( 0 ).

The unary arithmetic operators take precedence over the binary operators. Table 4- 5 shows the unary opera-
tors.

The _modulus_ operator, (for example a % b), gives the remainder when the first operand is divided by the
second, and thus is zero (0) when b divides a exactly. The result of a modulus operation takes the sign of the
first operand.

It shall be an error to pass zero (0) as the second argument to the modulus operator.

For the case of the modulus operator where either argument is real, the operation performed is:

```
a % b = ((a/b) < 0)? (a - ceil (a/b)*b) : (a - floor (a/b)*b);
```
Table 4- 6 gives examples of modulus operations.

```
Table 4-4—Arithmetic operators defined
```
```
a + b a plus b
a – b a minus b
a * b a multiply by b
a / b a divide by b
a % b a modulo b
a ** b a to power of b
```
```
Table 4-5—Unary operators defined
```
```
+m Unary plus m (same as m)
```
- m Unary minus m

```
Table 4-6—Examples of modulus operations
```
```
Modulus expression Result Comments
```
```
11 % 3 2 11/3 yields a remainder of 2.
12 % 3 0 12/3 yields no remainder.
-10 % 3 -1 The result takes the sign of the first operand.
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
56
```
#### 4.2.5 Relational operators

Table 4- 7 lists and defines the relational operators.

An expression using these _relational operators_ yields the value zero ( 0 ) if the specified relation is _false_ or
the value one ( 1 ) if it is _true_.

All the relational operators have the same precedence. Relational operators have lower precedence than
arithmetic operators.

The following examples illustrate the implications of this precedence rule:

```
a < foo - 1 // this expression is the same as
a < (foo - 1) // this expression, but...
foo - (1 < a) // this one is not the same as
foo - 1 < a // this expression
```
When foo - (1 < a) is evaluated, the relational expression is evaluated first and then either zero (0) or
one (1) is subtracted from foo. When foo - 1 < a is evaluated, the value of foo operand is reduced by
one (1) and then compared with a.

#### 4.2.6 Case equality operators

The _case equality operators_ share the same level of precedence as the _logical equality operators_. These
operators have limited support in the **analog** block (see 7.3.2). Additional information on these operators
can also be found in the IEEE Std 1364 Verilog.

#### 4.2.7 Logical equality operators.................................................................................................

The _logical equality operators_ rank lower in precedence than the relational operators. Table 4- 8 lists and
defines the equality operators.

```
11 % -3 2 The result takes the sign of the first operand.
10 % 3.75 2.5 [10 - floor(10/3.75)*3.75 ] yields a remainder of 2.5.
```
```
Table 4-7—The relational operators defined
```
```
a < b a less than b
a > b a greater than b
a <= b a less than or equal to b
a >= b a greater than or equal to b
```
```
Table 4-8—The equality operators defined
```
```
a ==b a equal to b
a !=b a not equal to b
```
```
Table 4-6—Examples of modulus operations (continued)
```
```
Modulus expression Result Comments
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
57
```
Both equality operators have the same precedence. These operators compare the value of the operands. As
with the relational operators, the result shall be zero ( 0 ) if comparison fails, one ( 1 ) if it succeeds.

#### 4.2.8 Logical operators...............................................................................................................

The operators _logical and_ (&&) and _logical or_ (||) are logical connectives. The result of the evaluation of a
logical comparison can be one (1) (defined as _true_ ) or zero (0) (defined as _false_ ). The precedence of && is
greater than that of || and both are lower than relational and equality operators.

A third logical operator is the unary _logical negation_ operator (!). The negation operator converts a non-zero
or true operand into zero ( 0 ) and a zero or false operand into one ( 1 ).

The following expression performs a _logical and_ (&&) of three sub-expressions without needing any paren-
theses:

```
a < param1 && b != c && index != lastone
```
However, parentheses can be used to clearly show the precedence intended, as in the following rewrite of the
above example:

```
(a < param1) && (b != c) && (index != lastone)
```
#### 4.2.9 Bitwise operators...............................................................................................................

The _bitwise operators_ perform bitwise manipulations on the operands—that is, the operator combines a bit
in one operand with its corresponding bit in the other operand to calculate one bit for the result. The follow-
ing logic tables (Table 4- 9 — Table 4- 13 ) show the results for each possible calculation.

```
Table 4-9—Bitwise binary and operator
```
```
&01
```
```
0^00
```
```
1^01
```
```
Table 4-10—Bitwise binary or operator
```
```
|01
```
```
0^01
```
```
1^11
```
```
Table 4-11—Bitwise binary exclusive or operator
```
```
^01
```
```
0^01
```
```
1^10
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
58
```
When one or both operands are unsigned. the expression shall be interpreted as a comparison between
unsigned values. If the operands are of unequal bit lengths, the smaller operand shall be zero-extended to the
size of the larger operand.

When both operands are signed, the expression shall be interpreted as a comparison between signed values.
If the operands are of unequal bit lengths, the smaller operand shall be sign-extended to the size of the larger
operand.

#### 4.2.10 Reduction operators

The reduction operators can not be used inside the **analog** block and only have meaning when used in the
digital context. Information on these operators can also be found in the IEEE Std 1364 Verilog.

#### 4.2.11 Shift operators

There are two types of _shift operators_ : the logical shift operators, **<<** and **>>** , and the arithmetic shift opera-
tors, **<<<** and **>>>**. The arithmetic shift operators can not be used in an **analog** block. Further information
on these operators can be found in IEEE Std 1364 Verilog. The logical shift operators, **<<** and **>>** , perform
left and right shifts of their left operand by the number of bit positions given by the right operand. Both the
**<<** and **>>** shift operators fill the vacated bit positions with zeroes (0).The right operand is always treated as
an unsigned number and has no effect on the signedness of the result.

Examples:

```
integer start, result;
analog begin
start = 1;
result = (start << 2);
end
In this example, the integer result is assigned the binary value 0100 , which is 0001 shifted to the
left two positions and zero-filled.
```
```
integer start, result;
analog begin
start = 3;
result = (start >> 1);
```
```
Table 4-12—Bitwise binary exclusive nor operator
```
```
^~
~^^01
```
```
0^10
```
```
1^01
```
```
Table 4-13—Bitwise unary negation operator
```
```
~
```
```
0^1
```
```
1^0
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
59
```
```
end
In this example, the integer result is assigned the binary value 0001 , which is 0011 shifted to the
right one position and zero-filled.
```
#### 4.2.12 Conditional operator

The _conditional operator_ , also known as _ternary operator_ , is right associative and shall be constructed using
three operands separated by two operators, as shown in Syntax 4- 1.

conditional_expression ::= _// from A.8.3_
expression1**?** { attribute_instance } expression2 **:** expression3

```
Syntax 4-1—Syntax for conditional operator
```
The evaluation of a conditional operator begins with the evaluation of _expression1_. If _expression1_ evaluates
to _false_ ( 0 ), then _expression3_ is evaluated and used as the result of the conditional expression. If _expression1_
evaluates to _true_ (any value other than zero ( 0 )), then _expression2_ is evaluated and used as the result.

#### 4.2.13 Concatenations

A concatenation is the result of the joining together of bits resulting from one or more expressions into a sin-
gle value. The concatenation shall be expressed using the brace characters **{** and **}** , with commas separating
the expressions within. It should not be confused with the assignment pattern **'{ }** which is used in Ver-
ilog-AMS to specify literal lists of constants and expressions for purposes such as the assignment of array
initializers and coefficient arguments to the Laplace analog filters. Confusion can arise because **{ }** is used
to describe lists of values for array initialization in the C language whereas it means something very differ-
ent (concatenation) in the Verilog HDL and Verilog-AMS HDL languages.

Unsized constant numbers shall not be allowed in concatenations. This is because the size of each operand in
the concatenation is needed to calculate the complete size of the concatenation.

This example concatenates two expressions:

```
{1'b1, 3'b101}
```
It is equivalent to the following example:

```
{1'b1, 1'b1, 1'b0, 1'b1}
```
Its value is 4'b1101.

The next example concatenates three strings:

```
{ "hello", " ", "world" }
```
Its value is "hello world".

An operator that can be applied only to concatenations is replication, which is expressed by a concatenation
preceded by a non-negative, non-x and non-z constant expression, called a replication constant, enclosed
together within brace characters, and which indicates a joining together of that many copies of the concate-
nation. Unlike regular concatenations, expressions containing replications shall not appear on the left-hand
side of an assignment and shall not be connected to output or inout ports.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
60
```
The following example replicates w four times:

```
{4{w}} // This yields the same value as {w, w, w, w}
```
The next example illustrates a replication nested within a concatenation:

```
{b, {3{a, b}}} // This yields the same value as
// {b, a, b, a, b, a, b}
```
A replication operation may have a replication constant with a value of zero. This is useful in parameterized
code. A replication with a zero replication constant is considered to have a size of zero and is ignored. Such
a replication shall appear only within a concatenation in which at least one of the operands of the concatena-
tion has a positive size. For example:

```
parameter P = 32;
```
```
// The following is legal for all P from 1 to 32
assign b[31:0] = { {32-P{1'b1}}, a[P-1:0] };
```
```
// The following is illegal for P=32 because the zero
// replication appears alone within a concatenation
assign c[31:0] = { {{32-P{1'b1}}}, a[P-1:0] };
```
```
// The following is illegal for P=32
initial
$displayb({32-P{1'b1}}, a[P-1:0]);
```
When a replication expression is evaluated, the operands shall be evaluated exactly once, even if the replica-
tion constant is zero. For example:

```
result = {4{func(w)}} ;
```
would be computed as:

```
y = func(w) ;
result = {y, y, y, y} ;
```
#### 4.2.14 Assignment patterns

The assignment pattern **'{ }** , is the way to specify lists of expressions of particular type in Verilog-AMS
during assignments, particularly array assignments. It is a feature imported from the IEEE Std 1800 System-
Verilog language.

assignment_pattern ::= // from A.8.1
**'{** expression { **,** expression } **}**
| **'{** constant_expression **{** expression { **,** expression } **} }**

constant_assignment_pattern ::=
**'{** constant_expression { **,** constant_expression } **}**
| **'{** constant_expression **{** constant_expression { **,** constant_expression } **} }**

```
Syntax 4-2—Syntax for assignment pattern
```
In the example below, a real array is initialized using an assignment pattern


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
61
```
```
parameter real data1[0:4] = '{3.4, 5.6, 2.3, 4.5, 7.1};
```
In the example below, a real array variable is initialized using an assignment pattern. The example also uses
a replication operator to repeat 0.0 five times so that every element of data2 is assigned to 0.0.

```
parameter real data2[0:4] = '{ 5{0.0} };
```
The example below assigns the array measurements in the _analog_ block using an assignment pattern com-
posed of three variables; a,b,c.

```
real measurements[0:2];
real a,b,c;
analog begin
```
```
measurements = '{a,b,c};
```
Here are the contexts in Verilog-AMS where an array assignment pattern is allowed;

```
— Analog operator arguments which are expected to be of type array (see 4.5.1)
— The data_source argument of the $table_model system task
— Parameter array assignment in an instantiation
— The RHS of an array variable or array parameter default assignment
— The RHS of an array variable assignment
— Array arguments in calls to user-defined functions
```
IEEE Std 1800 SystemVerilog has additional uses for the assignment pattern beyond array assignments.
IEEE Std 1800 SystemVerilog disallows the usage of the assignment pattern in particular contexts e.g. argu-
ments to system tasks: $my_system_task('{4.2,5.1,6.3} ). Verilog-AMS also adopts these restric-
tions. IEEE Std 1800 SystemVerilog should be consulted for a more detailed understanding of these
restrictions.

### 4.3 Built-in mathematical functions.....................................................................................................

Verilog-AMS HDL supports both the standard and transcendental mathematical functions. Both the IEEE
Std 1364 Verilog system function syntax style and the traditional Verilog-AMS HDL style are supported.
Users are encouraged to adopt the IEEE Std 1364 Verilog system function style when using the mathemati-
cal functions but the traditional Verilog-AMS HDL style will continue to be supported for backwards com-
patibility. The following tables Table 4- 14 and Table 4- 15 show both syntax styles as well as the equivalent
C function.

#### 4.3.1 Standard mathematical functions

The standard mathematical functions supported by Verilog-AMS HDL are shown in Table 4- 14. The oper-
ands shall be numeric (integer or real). For **min()** , and **max()** , if both operands are integer, then the result


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
62
```
is an integer, else both operands are converted to real, as is the result.For **abs()** , if the operand is an inte-
ger, then the result is an integer, else the result is real. All other arguments are converted to real.

The **min()** , **max()** , and **abs()** functions have discontinuous derivatives; it is necessary to define the
behavior of the derivative of these functions at the point of the discontinuity. In this context, these functions
are defined so:

```
min(x,y) is equivalent to (x < y)? x : y
max(x,y) is equivalent to (x > y)? x : y
abs(x) is equivalent to (x > 0) x : –x
```
The **ln1p** ( _x_ ) function returns the natural logarithm of one plus x: **ln** ( _1+x_ ). For small magnitude values of
x, **ln1p** ( _x_ ) can be more accurate than **ln** ( _1+x_ ).

The **expm1** ( _x_ ) function returns the exponential raised to the power x minus one: ex-1. For small magnitude
values of x, **expm1** ( _x_ ) can be more accurate than **exp** ( _x_ ) _-1_.

#### 4.3.2 Transcendental functions

The trigonometric and hyperbolic functions supported by Verilog-AMS HDL are shown in Table 4- 15. All
operands shall be numeric (integer or real) and are converted to real if necessary. Arguments to the trigono-

```
Table 4-14—Standard functions
```
```
Verilog
function style
```
```
Traditional
Verilog-AMS
function style
```
```
Equivalent C
function Description Domain
```
```
$ln ( x ) ln ( x ) log ( x ) Natural logarithm x > 0
$ln1p ( x ) ln1p ( x ) log1p ( x ) Natural logarithm of 1 plus x x > -1
$log10 ( x ) log ( x ) log10 ( x ) Decimal logarithm x > 0
$exp ( x ) exp ( x ) exp ( x ) Exponential All x
$expm1 ( x ) expm1 ( x ) expm1 ( x ) Exponential minus 1 All x
$sqrt ( x ) sqrt ( x ) sqrt ( x ) Square root x >= 0
$min ( x,y ) min ( x, y ) fmin ( x,y ) Minimum All x , all y
$max ( x,y ) max ( x, y ) fmax ( x,y ) Maximum All x , all y
$abs ( x ) abs ( x ) fabs ( x ) Absolute All x
$pow ( x,y ) pow ( x, y ) pow ( x,y ) Power ( xy ) if x > 0, all y ;
if x = 0, y > 0;
if x < 0,all integer y
$floor ( x ) floor ( x ) floor ( x ) Floor All x
$ceil ( x ) ceil ( x ) ceil ( x ) Ceiling All x
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
63
```
metric functions ( **sin** , **cos** , **tan** ) and return values of the inverse trigonometric functions ( **asin** , **acos** ,
**atan** , **atan2** ) are in radians. Input values outside of the valid range for the operator shall report an error.

### 4.4 Signal access functions

Access functions are used to access signals on nets, ports, and branches. There are two types of access func-
tions, _branch access functions_ and _port access function_ s. The name of the access function for a signal is
taken from the discipline of the net, port, or branch where the signal or port is associated and utilizes the **()**
operator. A port access function also takes its name from the discipline of the port to which it is associated
but utilizes the port access ( **< >** ) operator.

As an alternative to using the access attribute specified in the discipline, the generic **potential** and **flow**
access functions are also supported (see 5.5.1).

If the signal or port access function is used in an expression, the access function returns the value of the sig-
nal. If the signal access function is being used on the left side of a branch assignment or contribution state-
ment, it assigns a value to the signal. A port access function can not be used on the left side of a branch
assignment or contribution statement.

Table 4- 16 shows how access functions can be applied to branches, nets, and ports. In this table, _b1_ refers to
a branch, _n1_ and _n2_ represent either nets or ports, and _p1_ represents a port. These branches, nets, and ports

```
Table 4-15—Trigonometric and hyperbolic functions
```
```
Verilog function
style
```
```
Traditional
Verilog-AMS
function style
```
```
Equivalent C
function Description Domain
```
```
$sin ( x ) sin ( x ) sin ( x ) Sine All x
$cos ( x ) cos ( x ) cos ( x ) Cosine All x
$tan ( x ) tan ( x ) tan ( x ) Tangent x != n ( / 2), n is odd
$asin ( x ) asin ( x ) asin ( x ) Arc-sine -1 <= x <=
$acos ( x ) acos ( x ) acos ( x ) Arc-cosine -1 <= x <=
$atan ( x ) atan ( x ) atan ( x ) Arc-tangent All x
$atan2 (y, x ) atan2 ( y,x ) atan2 ( y,x ) Arc-tangent of y / x All x, all y ;
atan2(0,0) = 0
$hypot ( x,y ) hypot ( x,y ) hypot ( x,y ) All x, all y
$sinh ( x ) sinh ( x ) sinh ( x ) Hyperbolic sine All x
$cosh ( x ) cosh ( x ) cosh ( x ) Hyperbolic cosine All x
$tanh ( x ) tanh ( x ) tanh ( x ) Hyperbolic tangent All x
$asinh ( x ) asinh ( x ) asinh ( x ) Arc-hyperbolic sine All x
$acosh ( x ) acosh ( x ) acosh ( x ) Arc-hyperbolic cosine x >= 1
$atanh ( x ) atanh ( x ) atanh ( x ) Arc-hyperbolic tangent -1 < x < 
```
```
x^2 + y^2
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
64
```
are assumed to belong to the electrical discipline, where _V_ is the name of the access function for the voltage
(potential) and _I_ is the name of the access function for the current (flow).

The argument expression list for signal access functions shall be a branch identifier, or a list of one or two
nets or port expressions. If two net expressions are given as arguments to a flow access function, they shall
not evaluate to the same signal. The net identifiers shall be scalar or resolve to a constant net of a composite
net type (array or bus) accessed by a genvar expression. If only one net expression is given as the argument
to a signal access function, it is implicitly assumed that the second terminal of that unnamed branch is
ground.

The operands of an expression shall be unique to define a valid branch. The access function name shall
match the discipline declaration for the nets, ports, or branch given in the argument expression list. In this
case, V and I are used as examples of access functions for electrical potential and flow.

For port access functions, the expression list is a single port of the module. The port identifier shall be scalar
or resolve to a constant net of a bus port accessed by a genvar expression. The access function name shall
match the discipline declaration for the port identifier.

### 4.5 Analog operators

Analog operators are functions which operate on more than just the current value of their arguments.
Instead, they maintain their internal state and their output is a function of both the input and the internal
state.

Analog operators are also referred to as analog filter functions. They include the time derivative, time inte-
gral, and delay operators from calculus. They also include the transition and slew filters, which are used to
remove discontinuity from piecewise constant and piecewise continuous waveforms. Finally, they include
more traditional filters, such as those described with Laplace and Z-transform descriptions.

One special analog operator is the **limexp()** function, which is a version of the **exp()** function with
built-in limits to improve convergence.

```
Table 4-16—Access functions examples
```
```
Example Comments
```
```
V(b1) Accesses the voltage across branch b1
potential(b1) Alternative access of the voltage across the branch b1
V(n1) Accesses the voltage of n1 (a net or a port) relative to ground
V(n1,n2) Accesses the voltage difference between n1 and n2 (nets or ports)
V(n1,n1) Error
I(b1) Accesses the current flowing in branch b1
I(n1) Accesses the current flowing in the unnamed branch from n1 to ground
flow(n1) Alternative access of the current flowing in the unnamed branch from n1
to ground
I(n1,n2) Accesses the current flowing in the unnamed branch between n1 and n2
I(n1,n1) Error
I(<p1>) Accesses the current flow into the module through port p1
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
65
```
The syntax for the analog operators is shown in Syntax 4- 3.

analog_filter_function_call ::= _// from A.8.2_
**ddt (** analog_expression [ **,** abstol_expression ] **)**
| **ddx (** analog_expression **,** branch_probe_function_call **)**
| **idt (** analog_expression [ **,** analog_expression [ **,** analog_expression [ **,** abstol_expression ] ] ] **)**
| **idtmod (** analog_expression [ **,** analog_expression [ **,** analog_expression [ **,** analog_expression
[ **,** abstol_expression ] ] ] ] **)**
| **absdelay (** analog_expression **,** analog_expression [ **,** constant_expression ] **)**
| **transition (** analog_expression [ **,** analog_expression [ **,** analog_expression
[ **,** analog_expression [ **,** constant_expression ] ] ] ] **)**
| **slew (** analog_expression [ **,** analog_expression [ **,** analog_expression ] ] **)**
| **last_crossing (** analog_expression [ **,** analog_expression ] **)**
| **limexp (** analog_expression **)**
| laplace_filter_name **(** analog_expression **,** [ analog_filter_function_arg ] **,**
[ analog_filter_function_arg ] [ **,** constant_expression ] **)**
| zi_filter_name **(** analog_expression **,** [ analog_filter_function_arg ] **,**
[ analog_filter_function_arg ] **,** constant_expression
[ **,** analog_expression [ **,** constant_expression ] ] **)**

analog_filter_function_arg ::=
parameter_identifier
| parameter_identifier **[** msb_constant_expression **:** lsb_constant_expression **]**
| constant_assignment_pattern_or_null

```
Syntax 4-3—Syntax for the analog operators
```
#### 4.5.1 Vector or array arguments to analog operators

Certain analog operators require arrays or vectors to be passed as arguments: Laplace filters, Z-transform fil-
ters, **noise_table()** and **noise_table_log()**. An array can either be passed as an _array_identifier_
(e.g. an array parameter or an array variable) or an array assignment pattern (see 4.2.14).

#### 4.5.2 Analog operators and equations

Generally, simulators formulate the mathematical description of the system in terms of first-order differen-
tial equations and solve them numerically. There is no direct way to solve a set of nonlinear differential
equations so iterative approaches are used. When using iterative approaches, some criteria ( _tolerances_ ) is
needed to determine when the algorithm knows when it is close enough to the solution and then stops the
iteration. Thus, each equation, at a minimum, shall have a tolerance defined and associated with it.

Occasionally, analog operators require new equations and new unknowns be introduced by the simulator to
convert a module description into a set of first-order differential equations. In this case, the simulator
attempts to determine from context which tolerance to associate with the new equation and new unknown.
Alternatively, these operators can be used to specify tolerances.

Specifying natures also directly enforces reusability and allows other signal attributes to be accessed by the
simulator.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
66
```
#### 4.5.3 Time derivative operator ...................................................................................................

The **ddt** operator computes the time derivative of its argument, as shown in Table 4- 17.

In DC analysis, **ddt()** returns zero ( 0 ). The optional parameter _abstol_ is used as an absolute tolerance if
needed. Whether an absolute tolerance is needed depends on the context where **ddt()** is used. See 4.5.2 for
more information on the application of tolerances to equations. The absolute tolerance, _abstol_ or derived
from _nature_ , applies to the output of the **ddt** operator and is the largest signal level that is considered negli-
gible.

#### 4.5.4 Time integral operator.......................................................................................................

The **idt** operator computes the time-integral of its argument, as shown in Table 4- 18.

When used in DC or IC analyses, **idt()** returns the initial condition ( _ic_ ) if specified. If not specified, the idt
operator must be contained within a negative feedback loop that forces its argument to zero. Otherwise the
output of the idt operator is undefined.

```
Table 4-17—Time derivative
```
```
Operator Comments
```
```
ddt ( expr )
Returns ,
```
```
the time-derivative of x, where x is expr.
ddt ( expr, abstol ) Same as above, except absolute tolerance is specified explicitly.
ddt ( expr, nature ) Same as above, except nature is specified explicitly.
```
```
Table 4-18—Time integral
```
```
Operator Comments
```
```
idt ( expr ) Returns ,
```
```
where x () is the value of expr at time , t 0 is the start time of the simulation, t is
the current time, and c is the initial starting point as determined by the simulator
and is generally the DC value (the value that makes expr equal to zero).
idt ( expr , ic ) Returns ,
```
```
where in this case c is the value of ic at t 0.
idt ( expr,ic,assert ) Returns ,
```
```
where c is the value of ic at t a, which is the time when assert was last nonzero or t 0
if assert was never nonzero.
idt ( expr,ic,assert,abstol ) Same as above, except the absolute tolerance used to control the error in the
numerical integration process is specified explicitly with abstol.
idt ( expr,ic,assert,nature ) Same as above, except the absolute tolerance used to control the error in the
numerical integration process is taken from the specified nature.
```
```
dt
```
```
dxt ()
```
```
tx () d 
0
```
```
t
```
##  + c

```
tx () d + c
0
```
```
t
```
## 

```
tx () d + c
a
```
```
t
```
## 


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
67
```
When specified with initial conditions but without assert, **idt()** returns the value of the initial condition on
the initial point of a transient analysis. When specified with both initial conditions and _assert_ , **idt()**
returns the initial conditions during DC and IC analyses, and whenever _assert_ is nonzero. Once assert
becomes zero, **idt()** returns the integral of the argument starting from the last instant where assert was
nonzero.

The optional parameter _abstol_ or _nature_ is used to derive an absolute tolerance if needed. Whether an abso-
lute tolerance is needed depends on the context where **idt()** is used. (See 4.5.2 for more information.) The
absolute tolerance applies to the input of the **idt** operator and is the largest signal level that is considered
negligible.

A simple example that demonstrates the first form is a simple model for an opamp.

```
module opamp(out, pin, nin);
output out;
input pin, nin;
voltage out, pin, nin;
analog
V(out) <+ idt (V(pin,nin));
endmodule
```
Here the opamp is simply modeled as an integrator. In this case the initial condition for the integrator is
found by the simulator, generally the DC operating point is used. For the DC operating point to exist for an
integrator that does not have an initial condition explicitly specified, the integrator must exist within a nega-
tive feedback loop that drives its argument to 0. Forcing the output of the integration operator to be a partic-
ular value at start of the simulation using something like

```
V(out) <+ idt (V(pin,nin), 0);
```
avoids this issue.

Using the _assert_ argument, the output of the integration operator can be reset to a given value at any time.
This feature is demonstrated in the following model, which uses the **idt()** operator to generate a periodic
ramp waveform:

```
module ramp_generator(out);
output out;
voltage out;
integer reset;
analog begin
reset = 0;
@( timer (1, 1))
reset = 1;
V(out) <+ idt (1.0, 0, reset);
end
endmodule
```
The output of this model is shown in Figure 4- 3. Notice that in this model the reset occurs instantaneously.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
68
```
```
Figure 4-3: The output from the ramp generator
```
#### 4.5.5 Circular integrator operator...............................................................................................

The **idtmod** operator, also called the _circular integrator_ , converts an expression argument into its indefi-
nitely integrated form similar to the **idt** operator, as shown in Table 4- 19.

```
Table 4-19—Circular integrator
```
```
Operator Comments
```
```
idtmod ( expr ) Returns ,
```
```
where x () is the value of expr at time , t 0 is the start time of the simu-
lation, t is the current time, and c is the initial starting point as deter-
mined by the simulator and is generally the DC value (the value that
makes expr equal to zero).
idtmod ( expr , ic ) Returns ,
```
```
where in this case c is the value of ic at t 0.
idtmod ( expr,ic,modulus ) Returns k , where 0  k < modulus and k is
, n = ... –3, –2, –1, 0, 1, 2, 3 ...,
```
```
and c is the value of ic at t 0.
idtmod ( expr,ic,modulus,offset ) Returns k , where offset  k < offset + modulus , k is
,
```
```
and c is the value of ic at t 0.
```
```
0 V
```
```
200 mV
```
```
400 mV
```
```
600 mV
```
```
800 mV
```
```
1 V
```
```
0 s 500 ms 1 s 1.5 s 2 s
```
```
0 V
```
```
200 V
```
```
1 s 1.0002 s
```
```
tx () d 
0
```
```
t
```
##  + c

```
tx () d + c
0
```
```
t
```
## 

```
tx  d
0
```
```
t
```
##  + c = n  modulus + k

```
tx  d
0
```
```
t
```
##  + c = n  modulus + k


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
69
```
The initial condition is optional. If the initial condition is not specified, it defaults to zero ( 0 ). Regardless,
the initial condition shall force the DC solution to the system.

If **idtmod()** is used in a system with feedback configuration which forces expr to zero ( 0 ), the initial con-
dition can be omitted without any unexpected behavior during simulation. For example, an operational
amplifier alone needs an initial condition, but the same amplifier with the right external feedback circuitry
does not need a forced DC solution.

The output of the **idtmod()** function shall remain in the range

```
offset <= idtmod < offset+modulus
```
The modulus shall be an expression which evaluates to a positive value. If the modulus is not specified,
then **idtmod()** shall behave like **idt()** and not limit the output of the integrator.

The default for offset shall be zero ( 0 ).

The following relationship between **idt()** and **idtmod()** shall hold at all times.

If

```
y = idt (expr, ic);
z = idtmod (expr, ic, modulus, offset);
```
then

```
y = n * modulus + z; // n is an integer
```
where

```
offset  z < modulus + offset
```
In this example, the circular integrator is useful in cases where the integral can get very large, such as a
VCO. In a VCO, only the output values in the range [0,2] are of interest, e.g.,

```
phase = idtmod (fc + gain*V(in), 0, 1, 0);
V(OUT) <+ sin (2*‘M_PI*phase);
```
Here, the circular integrator returns a value in the range [0,1].

#### 4.5.6 Derivative operator

**ddx()** provides access to symbolically-computed partial derivatives of expressions in the **analog** block.
The analog simulator computes symbolic derivatives of expressions used in contribution statements in order
to use Newton-Raphson iteration to solve the system of equations. In many cases in compact modeling, the

```
idtmod ( expr,ic,modulus,offset,abstol ) Same as above, except the absolute tolerance used to control the error in
the numerical integration process is specified explicitly with abstol.
idtmod ( expr,ic,modulus,offset, nature ) Same as above, except the absolute tolerance used to control the error in
the numerical integration process is taken from the specified nature.
```
```
Table 4-19—Circular integrator (continued)
```
```
Operator Comments
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
70
```
values of these derivatives are useful quantities for design, such as the trans conductance of a transistor (gm)
or the capacitance of a nonlinear charge-storage element such as a varactor. The syntax for this operator is
shown in Syntax 4- 3.

The general form for the **ddx()** operator is:

```
ddx ( expr , unknown_quantity )
```
where:

```
— expr is the expression for which the symbolic derivative needs to be calculated.
— unknown_quantity is the branch probe (voltage or current probe) with respect to which the deriva-
tive of the expression needs to be computed.
```
The operator returns the partial derivative of its first argument with respect to the unknown indicated by the
second argument, holding all other unknowns fixed and evaluated at the current operating point. The second
argument shall be the potential of a scalar net or port or the flow through a branch, because these are the
unknown variables in the system of equations for the analog solver. For the modified nodal analysis used in
most SPICE-like simulators, these unknowns are the node voltages and certain branch currents.

If the expression does not depend explicitly on the unknown, then **ddx()** returns zero ( 0 ). Care must be
taken when using implicit equations or indirect assignments, for which the simulator may create internal
unknowns; derivatives with respect to these internal unknowns cannot be accessed with **ddx()**.

Unlike the **ddt()** operator, no tolerance is required because the partial derivative is computed symbolically
and evaluated at the current operating point.

This first example uses **ddx()** to obtain the conductance of the diode. The variable gdio is declared as an
output variable (see 3.2.1) so that its value is available for inspection by the designer.

```
module diode(a,c);
inout a, c;
electrical a, c;
parameter real IS = 1.0e-14;
real idio;
(*desc="small-signal conductance"*)
real gdio;
analog begin
idio = IS * ( limexp (V(a,c)/ $vt ) - 1);
gdio = ddx (idio, V(a));
I(a,c) <+ idio;
end
endmodule
```
The next example adds a series resistance to the diode using an implicit equation. Note that gdio does not
represent the total conductance because the flow access I(a,c) requires introduction of another unknown in
the system of equations. The conductance of the diode is properly reported as geff, which includes the
effects of RS and the nonlinear equation.

```
module diode(a,c);
inout a, c;
electrical a, c;
parameter real IS = 1.0e-14;
parameter real RS = 0.0;
real idio, gdio;
(*desc="effective conductance"*)
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
71
```
```
real geff;
analog begin
idio = IS * ( limexp ((V(a,c)-RS*I(a,c))/ $vt ) - 1);
gdio = ddx (idio, V(a));
geff = gdio / (RS * gdio + 1.0);
I(a,c) <+ idio;
end
endmodule
```
The final example implements a voltage-controlled dependent current source and is used to illustrate the
computations of partial derivatives.

```
module vccs(pout,nout,pin,nin);
inout pout, nout, pin, nin;
electrical pout, nout, pin, nin;
parameter real k = 1.0;
real vin, one, minusone, zero;
analog begin
vin = V(pin,nin);
one = ddx (vin, V(pin));
minusone = ddx (vin, V(nin));
zero = ddx (vin, V(pout));
I(pout,nout) <+ k * vin;
end
endmodule
```
The names of the variables indicate the values of the partial derivatives: +1, -1, or 0. A SPICE-like simulator
would use these values (scaled by the parameter k) in the Newton-Raphson solution method.

#### 4.5.7 Absolute delay operator ....................................................................................................

**absdelay()** implements the absolute transport delay for continuous waveforms (use the **transi-
tion()** operator to delay discrete-valued waveforms). The general form is

```
absdelay ( input , td [ , maxdelay ] )
```
_input_ is delayed by the amount _td_. In all cases _td_ shall be a positive number. If the optional _maxdelay_ is spec-
ified, then _td_ can vary. If _td_ becomes greater than _maxdelay_ , _maxdelay_ will be used as a substitute for _td_. If
_maxdelay_ is not specified, the value of _td_ when the **absdelay()** is first evaluated shall be used and any
future changes to _td_ shall be ignored.

In DC and operating point analyses, **absdelay()** returns the value of its _input_. In AC and other small-sig-
nal analyses, the **absdelay()** operator phase-shifts the input expression to the output of the delay opera-
tor based on the following formula.

td is evaluated as a constant at a particular time for any small signal analysis. In time-domain analyses,
**absdelay()** introduces a transport delay equal to the instantaneous value of _td_ based on the following for-
mula.

_Output_ () _Input_ () _e_

- _j_ td
= 

_Output t_ = _Input max t_ –td, 0


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
72
```
The transport delay _td_ can be either constant (typical case) or vary as a function of time (when _maxdelay_ is
defined). When calculating the output at time _t_ , the **absdelay()** operator will use linear interpolation as
needed to determine the input around time:

A time-dependent transport delay is shown in Figure 4- 4 , with a ramp input to the **absdelay** operator for
both positive and negative changes in the transport delay _td_ and a maxdelay of 5.

```
Figure 4-4: Transport delay example
```
From time 0 until 2s, the output remains at input(0). With a delay of 2s, the output then starts tracking
input(t - 2). At 3s, the transport delay changes from 2s to 4s, switching the output back to input(0),
since input(max(t-td,0)) returns 0. The output remains at this level until 4s when it once again starts
tracking the input from t = 0. At 5s the transport delay goes to 1s and the output correspondingly jumps
from its current value to the value defined by input(t - 1).

#### 4.5.8 Transition filter

**transition()** smooths out piecewise constant waveforms. The transition filter is used to imitate transi-
tions and delays on digital signals (for non-piecewise constant signals, see 4.5.9). This function provides
controlled transitions between discrete signal levels by setting the rise time and fall time of signal transi-
tions, as shown in Figure 4- 5.

```
Figure 4-5: Transition filter example
```
```
max t –td, 0
```
```
Input
```
```
Output
```
### td (s)

##### 4

##### 3

##### 2

##### 1

##### 2 4 6

```
tr tf
t 0 t 0
```
```
d
```
```
input_expression(t) output_expression(t)
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
73
```
**transition()** stretches instantaneous changes in signals over a finite amount of time and can delay the
transitions, as shown in Figure 4- 6.

```
Figure 4-6: Shifting the transition filter
```
The general form of the **transition()** filter is

```
transition ( expr [ , td [ , rise_time [ , fall_time [ , time_tol ] ] ] ] )
```
The input expression is expected to evaluate over time to a piecewise constant waveform. When applied,
**transition()** forces all positive transitions of _expr_ to occur over _rise_time_ and all negative transitions
to occur in _fall_time_ (after an initial delay of _td_ ). Thus, _td_ models transport delay and _rise_time_ and _fall_time_
model inertial delay.

**transition()** returns a real number which describes a piecewise linear function over time. The transi-
tion function causes the simulator to place time points at both corners of a transition. If _time_tol_ is not spec-
ified, the transition function causes the simulator to assure each transition is adequately resolved.

_td, rise_time_ , _fall_time,_ and _time_tol_ are optional, but if specified shall be non-negative. If _td_ is not speci-
fied, it is taken to be zero (0.0). If only a positive _rise_time_ value is specified, the simulator uses it for both
rise and fall times. If neither _rise_time_ nor _fall_time_ are specified or are equal to zero (0.0), the rise and fall
time default to the value defined by **‘default_transition** .If a time_tol value of zero (0.0) is speci-
fied, the simulator shall apply a suitable value.

If **‘default_transition** is not specified the default behavior approximates the ideal behavior of a
zero-duration transition. Forcing a zero-duration transition is undesirable because it could cause conver-
gence problems. Instead, a negligible, but non-zero, transition time is used. The small non-zero transition
time allows the simulator to shrink the timestep small enough so a smooth transition occurs and any conver-
gence problems are avoided. The simulator does not force a time point at the trailing corner of a transition to
avoid causing the simulator to take very small time steps, which would result in poor performance.

In DC analysis, **transition()** passes the value of the _expr_ directly to its output. The **transition** fil-
ter is designed to smooth out piecewise constant waveforms. When applied to waveforms which vary
smoothly, the simulation results are generally unsatisfactory. In addition, applying the transition function to
a continuously varying waveform can cause the simulator to run slowly. Use **transition()** for discrete
signals and **slew()** (see 4.5.9) for continuous signals.

A transition is created when the input expression changes, and at this point it uses the value of _td_ , _rise_time_ ,
_fall_time_ and _time_tol_ to determine the new pending transition operator. If the effects are immediate, _td=0_ ,
then current transitions and scheduled ones are canceled, and the new one is created. If _td_ is before a previ-
ously scheduled transition, then the previously scheduled transition(s) are canceled and a new one is created.
If _td_ is after previously scheduled transition then it adds the new transition to the pending transition(s) allow-
ing an arbitrary number of pending transitions. Consider a digital clock that is required to be driven out onto
an analog port.

```
Input to transition filter
```
```
Response of transition filter
with transition times specified
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
74
```
```
always #5 clk = ~clk;
analog V(aclk) <+ transition (clk,0,1p);
```
If the delay on the transition is greater than ½ period, then multiple pending transitions are stored on the
transition operator.

```
always #5 clk = ~clk;
analog V(aclk) <+ transition (clk,5.1n,1p);
```
A transition is considered active during the period of time (rise or fall) that we are transitioning the output
from one value to another.

An active transition shall be interrupted if the input to the **transition()** changes value while in this
active transitioning region. An interrupted transition is not considered a new transition, but rather a readjust-
ment of the original transition. To determine the time that the readjusted transition will reach the new desti-
nation, the slope shall be calculated using either the original transition's origin or destination as the new
origin based on the following criteria:

```
— If the original transition was rising and the new destination value is below the value at the interrup-
tion, then the original transition's destination shall be used to compute the new origin. Referring to
Figure 4- 7 , consider an original transition that rises from (t1,v1) to (t2,v2) with a rise time of tr1=t2-
t1, which is interrupted at the point (ti,vi) with a new destination value (v3), where v3 < vi. Then the
original transition's destination (v2) shall be used along with the rescheduled transition's actual fall
time (tf3), when calculating the slope: (v3-v2)/tf3. This slope will be applied from the point of inter-
ruption (ti,vi), and the readjusted transition's expected end time (t3) is then calculated using this
slope, now shifted left, along with the time and value level at the point of the interruption: t3 = ti +
(v3-vi)/slope. The new origin for the transition is now (t4,v4), which will be used if the transition is
interrupted again.
```
```
Figure 4-7: Interrupted rising transition (falling)
```
```
— If the original transition was rising and the new destination value is above the value at the interrup-
tion, then the original transition's origin shall be used to compute the new origin. Referring to
Figure 4- 8 , consider an original transition that rises from (t1,v1) to (t2,v2) with a rise time of tr1=t2-
t1, which is interrupted at the point (ti,vi) with a new destination value (v3), where v3 > vi, and with
a new transition time tr3. Then the original transition's origin (v1) shall be used along with the new
rise time (tr3), to calculate the slope: (v3-v1)/tr3.
This slope will be applied from the point of interruption (ti,vi), and the readjusted transition's ex-
pected end time (t3) is then calculated using this slope, along with the time and value level at the
point of the interruption: t3 = ti + (v3-vi)/slope. The new origin for the transition is now (t4,v4),
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
75
```
```
which will be used if the transition is interrupted again. In Figure 4- 8 , the transition is shifted left
and t4 < t1 because the new rise time is longer (tr3 > tr1). In Figure 4- 9 the new rise time is shorter
(tr3 < tr1), so the transition is shifted right.
```
```
Figure 4-8: Interrupted rising transition (rising tr3>tr1)
```
```
Figure 4-9: Interrupted rising transition (rising tr3<tr1)
```
— If the original transition was falling and the new destination value is below the value at the interrup-
tion, then the original transition's origin shall be used to compute the new origin. Referring to Figure
4, consider an original transition that falls from (t1,v1) to (t2,v2) with a fall time of tf1=t2-t1, which
is interrupted at the point (ti,vi) with a new destination value (v3), where v3 < vi, and with a new
transition time tf3. Then the original transition's origin (v1) shall be used along with the new fall
time (tf3), to calculate the slope: (v3-v1)/tf3.
This slope will be applied from the point of interruption (ti,vi), and the readjusted transition's
expected end time (t3) is then calculated using this slope, along with the time and value level at the
point of the interruption: t3 = ti + (v3-vi)/slope. The new origin for the transition is now (t4,v4),
which will be used if the transition is interrupted again. In Figure 4- 10 , the transition is shifted left
and t4 < t1 because the new fall time is longer (tf3 > tf1). In Figure 4- 11 , the new fall time is shorter
(tf3 < tf1), so the transition is shifted right.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
76
```
```
Figure 4-10: Interrupted falling transition (falling tf3>tf1)
```
```
Figure 4-11: Interrupted falling transition (falling tf3<tf1)
```
— If the original transition was falling and the new destination value is above the value at the interrup-
tion, then the original transition's destination shall be used to compute the new origin. Referring to
Figure 4- 12 , consider an original transition that falls from (t1,v1) to (t2,v2) with a fall time of
tf1=t2-t1, which is interrupted at the point (ti,vi) with a new destination value (v3), where v3 > vi.
Then the original transition's destination (v2) shall be used along with the rescheduled transition's
actual rise time (tr3), when calculating the slope: (v3-v2)/tr3. This slope will be applied from the
point of interruption (ti,vi), and the readjusted transition's expected end time (t3) is then calculated
using this slope, now shifted left, along with the time and value level at the point of the interruption:
t3 = ti + (v3-vi)/slope. The new origin for the transition is now (t4,v4), which will be used if the tran-
sition is interrupted again.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
77
```
```
Figure 4-12: Interrupted falling transition (rising)
```
Because the transition function can not be linearized in general, it is not possible to accurately represent a
transition function in AC analysis. The AC transfer function is approximately modeled as having unity
transmission for all frequencies in all situations. Because the transition function is intended to handle dis-
crete-valued signals, the small signals present in AC analysis rarely reach transition functions. As a result,
the approximation used is generally sufficient.

Example 1 — QAM modulator

In this example, the transition function is used to control the rate of change of the modulation signal in a
QAM modulator.

```
module qam16(in, out);
input [0:3] in;
output out;
voltage [0:3] in;
voltage out;
```
```
parameter real freq = 1.0 from (0:inf);
parameter real ampl = 1.0;
parameter real thresh = 2.5;
parameter real tdelay = 0 from [0:inf);
localparam real ttransit = 1/freq;
```
```
real x, y, phi;
integer row, col;
```
```
analog begin
row = 2 * (V(in[3]) > thresh) + (V(in[2]) > thresh);
col = 2 * (V(in[1]) > thresh) + (V(in[0]) > thresh);
```
```
x = transition( row - 1.5, tdelay, ttransit ) ;
y = transition( col - 1.5, tdelay, ttransit ) ;
```
```
phi = `M_TWO_PI * freq * $abstime ;
V(out) <+ ampl * (x * cos( phi ) + y * sin( phi ) );
end
endmodule
```
Example 2 — A/D converter


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
78
```
In this example, an analog behavioral N-bit analog to digital converter, demonstrates the ability of the transi-
tion function to handle vectors.

```
module adc(in, clk, out);
parameter bits = 8, fullscale = 1.0, dly = 0, ttime = 10n;
input in, clk;
output [0:bits-1] out;
electrical in, clk;
electrical [0:bits-1] out;
real sample, thresh;
integer result[0:bits-1];
genvar i;
```
```
analog begin
@( cross (V(clk)-2.5, +1)) begin
sample = V(in);
thresh = fullscale/2.0;
for (i = bits - 1; i >= 0; i = i - 1) begin
if (sample > thresh) begin
result[i] = 1.0;
sample = sample - thresh;
end
else begin
result[i] = 0.0;
end
sample = 2.0*sample;
end
end
for (i = 0; i < bits; i = i + 1) begin
V(out[i]) <+ transition (result[i], dly, ttime);
end
end
endmodule
```
#### 4.5.9 Slew filter

The **slew** analog operator bounds the rate of change (slope) of the waveform. A typical use for **slew()** is
generating continuous signals from piecewise continuous signals. (For discrete-valued signals, see 4.5.8.)
The general form is

```
slew ( expr [ , max_pos_slew_rate [ , max_neg_slew_rate ] ] )
```
When applied, **slew()** forces all transitions of _expr_ faster than _max_pos_slew_rate_ to change at _max-
_pos_slew_rate_ rate for positive transitions and limits negative transitions to _max_neg_slew_rate_ rate as
shown in Figure 4- 13.

```
Figure 4-13: Slew filter transition
```
```
 y
------ t  ratepmax
 y
 t
```
```
output_expression(t)
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
79
```
The two rate values are optional. _max_pos_slew_rate_ shall be greater than zero(0)and _max_neg_slew_rate_
shall be less than zero (0). If the _max_neg_slew_rate_ is not specified, it defaults to the opposite of the _max-
_pos_slew_rate_. If no rates are specified, **slew()** passes the signal through unchanged. If the rate of
change of _expr_ is less than the specified maximum slew rates, **slew()** returns the value of _expr_.

In DC analysis, **slew()** simply passes the value of the destination to its output. In small-signal analyses,
the **slew()** function has a transfer function from the first argument to the output of 1.0 when not slewing
(e.g. for a small-signal analysis following a dc operating point) and 0.0 when slewing.

#### 4.5.10 last_crossing function

The **last_crossing()** function returns a real value representing the simulation time when a signal
expression last crossed zero ( 0 ). The general form is

```
last_crossing ( expr [ , direction ] )
```
The optional _direction_ indicator shall evaluate to an integer expression +1, -1, or 0. If it is set to 0, the
**last_crossing()** will return the most recent time the input expression had either a rise or falling edge
transition. If direction is +1 (-1), the **last_crossing()** will return the last time the input expression had
a rising (falling) edge transition.

The **last_crossing()** function does not control the timestep to get accurate results; it uses linear inter-
polation to estimate the time of the last crossing. However, it can be used with the **cross()** or **above()**
function for improved accuracy.

The following example measures the period of its input signal using the **cross()** and **last_cross-
ing()** functions.

```
module period(in);
input in;
voltage in;
integer crossings;
real latest, previous;
```
```
analog begin
@(initial_step) begin
crossings = 0;
previous = 0;
end
```
```
@( cross (V(in), +1)) begin
crossings = crossings + 1;
previous = latest;
end
latest = last_crossing (V(in), +1);
```
```
@(final_step) begin
if (crossings < 2)
$strobe ("Could not measure period.");
else
$strobe ("period = %g, crossings = %d",
latest-previous, crossings);
end
end
endmodule
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
80
```
Before the expression crosses zero ( 0 ) for the first time, the **last_crossing()** function returns a nega-
tive value.

#### 4.5.11 Laplace transform filters

The Laplace transform filters implement lumped linear continuous-time filters. Each filter takes an optional
parameter , which is a real number or a nature used for deriving an absolute tolerance (if needed). Whether
an absolute tolerance is needed depends on the context where the filter is used. The zeros argument may be
represented as a null argument. The null argument is characterized by two adjacent commas (,,) in the argu-
ment list.

For arguments that require a vector, the vector may be represented as either a literal vector or a reference to
a vector parameter.

**4.5.11.1 laplace_zp**

**laplace_zp()** implements the zero-pole form of the Laplace transform filter. The general form is:

```
laplace_zp ( expr ,  ,  [ ,  ] )
```
where  (zeta) is a vector of _M_ pairs of real numbers. Each pair represents a zero, the first number in the pair
is the real part of the zero and the second is the imaginary part. Similarly,(rho) is the vector of _N_ real pairs,
one for each pole. The poles are given in the same manner as the zeros. The transfer function is

where and are the real and imaginary parts of the zero ( 0 ), while and are the real and imagi-
nary parts of the pole. If a root (a pole or zero) is real, the imaginary part shall be specified as zero ( 0 ). If
a root is complex, its conjugate shall also be present. If a root is zero, then the term associated with it is
implemented as _s,_ rather than (where _r_ is the root).

**4.5.11.2 laplace_zd**

**laplace_zd()** implements the zero-denominator form of the Laplace transform filter. The general form
is:

```
laplace_zd ( expr ,  , d [ ,  ] )
```
where  (zeta) is a vector of _M_ pairs of real numbers. Each pair represents a zero, the first number in the pair
is the real part of the zero and the second is the imaginary part. Similarly, _d_ is the vector of _N_ real numbers
containing the coefficients of the denominator. The transfer function is

```
Hs 
```
```
1 s
 kr + j  ki
```
##### –--------------------

```
k = 0
```
```
M – 1
```
## 

```
1 s
 kr + j  ki
```
##### –---------------------

```
k = 0
```
```
N – 1
```
## 

##### =----------------------------------------------

```
 kr  ki kth  kr  ki
kth
```
```
 1 – sr 
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
81
```
where and are the real and imaginary parts of the zero, while is the coefficient of the
power of _s_ in the denominator. If a zero is real, the imaginary part shall be specified as zero ( 0 ). If a zero is
complex, its conjugate shall also be present. If a zero is zero ( 0 ), then the term associated with it is imple-
mented as _s,_ rather than.

**4.5.11.3 laplace_np**

**laplace_np()** implements the numerator-pole form of the Laplace transform filter. The general form is

```
laplace_np ( expr , n ,  [ ,  ] )
```
where _n_ is a vector of _M_ real numbers containing the coefficients of the numerator. Similarly,  (rho) is a
vector of _N_ pairs of real numbers. Each pair represents a pole, the first number in the pair is the real part of
the pole and the second is the imaginary part. The transfer function is

where is the coefficient of the power of _s_ in the numerator, while and are the real and imagi-
nary parts of the pole. If a pole is real, the imaginary part shall be specified as zero ( 0 ). If a pole is com-
plex, its conjugate shall also be present. If a pole is zero ( 0 ), then the term associated with it is implemented
as _s,_ rather than.

**4.5.11.4 laplace_nd**

**laplace_nd()** implements the numerator-denominator form of the Laplace transform filter.

The general form is:

```
laplace_nd ( expr , n , d [ ,  ] )
```
where _n_ is an vector of _M_ real numbers containing the coefficients of the numerator and _d_ is a vector of _N_
real numbers containing the coefficients of the denominator. The transfer function is:

```
Hs 
```
```
1 s
 kr + j  ki
```
##### –--------------------

```
k = 0
```
```
M – 1
```
## 

```
dksk
k = 0
```
```
N – 1
```
## 

##### =----------------------------------------------

```
 k
r
 ki kth dk kth
```
```
 1 – s 
```
```
Hs 
```
```
nks
k
k = 0
```
```
M – 1
```
## 

```
1 s
 kr + j  ki
```
##### –---------------------

```
k = 0
```
```
N – 1
```
## 

##### =----------------------------------------------

```
nk kth  kr  ki
kth
```
```
 1 – s 
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
82
```
where is the coefficient of the power of _s_ in the numerator and is the coefficient of the power
of _s_ in the denominator.

**4.5.11.5 Examples**

```
V(out) <+ laplace_zp (V(in), '{-1,0}, '{-1,-1,-1,1});
```
implements

and

```
V(out) <+ laplace_nd (V(in), '{0,1}, '{-1,0,1});
```
implements.

This example

```
V(out) <+ laplace_zp ( white_noise (k), , '{1,0,1,0,-1,0,-1,0});
```
implements a band-limited white noise source as.

#### 4.5.12 Z-transform filters

The _Z-transform_ filters implement linear discrete-time filters. Each filter supports a parameter _T_ which spec-
ifies the sampling period of the filter. A filter with unity transfer function acts like a simple sample-and-hold
which samples every _T_ seconds and exhibits no delay. The zeros argument may be represented as a null
argument. The null argument is characterized by two adjacent commas (,,) in the argument list.

All Z-transform filters share three common arguments: _T_ and _t_ . _T_ specifies the period of the filter, is
mandatory, and shall be positive.  specifies the transition time, is optional, and shall be nonnegative.

If the transition time is specified and is non-zero, the timestep is controlled to accurately resolve both the
leading and trailing corner of the transition. If it is not specified, the transition time is taken to be one ( 1 ) unit

```
Hs 
```
```
nks
k
```
```
k = 0
```
```
M – 1
```
## 

```
dksk
k = 0
```
```
N – 1
```
## 

##### =---------------------

```
nk kth dk kth
```
```
Hs ^1 + s
1 s
1 + j
+------------^1 s
1 – j
```
##### +-----------

##### =------------------------------------------------------

```
Hs  s
s^2 – 1
```
##### =-------------

```
vout^2 k
s^2 – 12
```
##### =-------------------


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
83
```
of time (as defined by the **`default_transition** compiler directive) and the timestep is not controlled
to resolve the trailing corner of the transition. If the transition time is specified as zero ( 0 ), then the output is
abruptly discontinuous. A _Z_ -filter with zero ( 0 ) transition time shall not be directly assigned to a branch.

Finally _t_  specifies the time of the first transition, and is also optional. If not given, the first transition occurs
at _t_ = 0.

For arguments that require a vector, the vector may be represented as either a literal vector or a reference to
a vector parameter.

**4.5.12.1 zi_zp**

**zi_zp()** implements the zero-pole form of the _Z_ -transform filter. The general form is:

```
zi_zp ( expr ,  ,  , T [ ,  [ , t  ] ] )
```
where  (zeta) is a vector of _M_ pairs of real numbers. Each pair represents a zero, the first number in the pair
is the real part of the zero ( 0 ) and the second is the imaginary part. Similarly, (rho) is the vector of _N_ real
pairs, one for each pole. The poles are given in the same manner as the zeros. The transfer function is

where and are the real and imaginary parts of the zero, while and are the real and imagi-
nary parts of the pole. If a root (a pole or zero) is real, the imaginary part shall be specified as zero. If a
root is complex, its conjugate shall also be present. If a root is zero ( 0 ), then the term associated with it is
implemented as _z,_ rather than (where _r_ is the root).

**4.5.12.2 zi_zd**

**zi_zd()** implements the zero-denominator form of the _Z_ -transform filter. The form is:

```
zi_zd ( expr ,  , d , T [ ,  [ , t  ] ] )
```
where  (zeta) is a vector of _M_ pairs of real numbers. Each pair represents a zero, the first number in the pair
is the real part of the zero and the second is the imaginary part. Similarly, _d_ is the vector of _N_ real numbers
containing the coefficients of the denominator. The transfer function is

```
Hz 
```
```
1 z
```
- 1
-  _kr_ + _j_  _ki
k_ = 0

```
M – 1
```
## 

```
1 z
```
- 1
-  _kr_ + _j_  _ki
k_ = 0

```
N – 1
```
## 

##### =----------------------------------------------------

```
 k
r
 ki kth  k
r
 ki
kth
```
```
 1 – zr 
```
```
Hz 
```
```
1 – z –^1  kr + j  ki
k = 0
```
```
M – 1
```
## 

```
dkz – k
k = 0
```
```
N – 1
```
## 

##### =----------------------------------------------------


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
84
```
where and are the real and imaginary parts of the zero, while is the coefficient of the
power of _s_ in the denominator. If a zero is real, the imaginary part shall be specified as zero ( 0 ). If a zero is
complex, its conjugate shall also be present. If a zero is zero ( 0 ), then the term associated with it is imple-
mented as _z,_ rather than.

**4.5.12.3 zi_np**

**zi_np()** implements the numerator-pole form of the _Z_ -transform filter. The general form is:

```
zi_np ( expr , n ,  , T [ ,  [ , t  ] ] )
```
where _n_ is a vector of _M_ real numbers containing the coefficients of the numerator. Similarly,  (rho) is a
vector of _N_ pairs of real numbers. Each pair represents a pole, the first number in the pair is the real part of
the pole and the second is the imaginary part. The transfer function is

where is the coefficient of the power of _s_ in the numerator, while and are the real and imagi-
nary parts of the pole. If a pole is real, the imaginary part shall be specified as zero ( 0 ). If a pole is com-
plex, its conjugate shall also be present. If a pole is zero ( 0 ), then the term associated with it is implemented
as _z,_ rather than.

**4.5.12.4 zi_nd**

**zi_nd()** implements the numerator-denominator form of the _Z_ -transform filter. The general form is:

```
zi_nd ( expr , n , d , T [ ,  [ , t  ] ] )
```
where _n_ is an vector of _M_ real numbers containing the coefficients of the numerator and _d_ is a vector of _N_
real numbers containing the coefficients of the denominator. The transfer function is

where is the coefficient of the power of _s_ in the numerator and is the coefficient of the power
of _s_ in the denominator.

#### 4.5.13 Limited exponential

```
 k
r
 ki kth dk kth
```
```
 1 – z 
```
```
Hz 
```
```
nkz – k
k = 0
```
```
M – 1
```
## 

```
1 – z –^1  kr + j  ki
k = 0
```
```
N – 1
```
## 

##### =----------------------------------------------------

```
nk kth  k
r
 ki
kth
```
```
 1 – z 
```
```
Hz 
```
```
nkz – k
k = 0
```
```
M – 1
```
## 

```
dkz – k
k = 0
```
```
N – 1
```
## 

##### =-----------------------

```
nk kth dk kth
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
85
```
The **limexp()** function is a special-purpose operator whose purpose is to improve convergence of the ana-
log solver (generally, a Newton-Raphson linear solver) when faced with the strongly nonlinear behavior of
the exponential function. The operator has internal state containing information about the argument on pre-
vious iterations. It returns a real value which is the exponential of its single real argument; however, it inter-
nally limits the change of its output from iteration to iteration in order to improve convergence. It is
therefore not useful for any exponential whose argument does not change from iteration to iteration. On any
iteration where the change in the output of the **limexp()** function is bounded, the simulator is prevented
from terminating the iteration. Thus, the simulator can only converge when the output of **limexp()**
equals the exponential of the input.

The general form is:

```
limexp ( expr )
```
The apparent behavior of **limexp()** is not distinguishable from **exp()** , except using **limexp()** to
model semiconductor junctions generally results in dramatically improved convergence. There are different
ways of implementing limiting algorithms for the exponential^1 2.

Other nonlinearities besides the exponential may be in behavioral models. The **$limit()** system function
described in 9.17.3 provides a method to indicate these nonlinearities to the simulator to improve conver-
gence.

#### 4.5.14 Constant versus dynamic arguments

Some of the arguments to the analog operators described in this section, the events described in Clause 5 ,
and the **$limit()** function in 9.17.3 expect dynamic expressions and others expect constant expressions.
The dynamic expressions can be functions of circuit quantities and can change during an analysis. The con-
stant expressions remain static throughout an analysis.

Table 4- 20 summarizes the arguments of the analog operators defined in this section.

(^1) Laurence W. Nagel, "SPICE2: A computer program to simulate semiconductor circuits," Memorandum No. ERL-M520, University of
California, Berkeley, California, May 1975.
(^2) W. J. McCalla, _Fundamentals of Computer-Aided Circuit Simulation_. Kluwer Academic Publishers, 1988.
**Table 4-20—Analog operator arguments
Operator Constant expression arguments Dynamic expression arguments
absdelay** maxdelay expr, td
**ddt** abstol expr
**ddx** wrt_what expr
**idt** abstol expr, ic, assert
**idtmod** abstol expr, ic,modulus, offset
**laplace_zp** zeros, poles, abstol expr
**laplace_zd** zeros, denominator, abstol expr
**laplace_np** numerator, poles, abstol expr
**laplace_nd** numerator, denominator,
abstol
expr
**last_crossing** expr, dir


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
86
```
If a dynamic expression is passed as an argument which expects a constant expression, the value of the
dynamic expression at the start of the analysis defaults to the constant value of the argument. Any further
change in value of that expression is ignored during the iterative analysis.

#### 4.5.15 Restrictions on analog operators

Analog operators are subject to several important restrictions because they maintain their internal state. It is
important to ensure that all analog operators are evaluated every iteration of a simulation to ensure that the
internal state is maintained. The analog operator **ddx()** is the only exception to this rule as it does not
require an internal state to be maintained. All analog operators are considered to have no state history prior
to time t == 0.

```
— Analog operators shall not be used inside conditional ( if , case , or ?: ) statements unless the con-
ditional expression controlling the statement consists of terms which can not change their value
during the course of a simulation.
— Analog operators shall not be used inside event triggered statements.
— Analog operators are not allowed in the repeat , while and non-genvar for looping statements.
— Analog operators can only be used inside an analog block; they can not be used inside an ini-
tial or always block, or inside a user-defined function.
— It is illegal to specify a null argument in the argument list of an analog operator, except as specified
elsewhere in this document.
```
These restrictions help prevent usage which could cause the internal state to be corrupted or become out-of-
date, which results in anomalous behavior.

### 4.6 Analysis dependent functions

This section describes the **analysis()** function, which is used to determine what type of analysis is being
performed, and the small-signal source functions. The small-signal source functions only affect the behavior
of a module during small-signal analyses. The small-signal analyses provided by SPICE include the AC and
noise analyses, but others are possible. When not active, the small-signal source functions return zero ( 0 ).

```
limexp expr
slew expr, max_pos_slew_rate,
max_neg_slew_rate
transition expr, td, rise_time,
fall_time, time_tol
zi_zp zeros, poles, T, t0 expr, t
zi_zd zeros, denominator, T, t0 expr, t
zi_np numerator, poles, T, t0 expr, t
zi_nd numerator, denominator, T,
t0
```
```
expr, t
```
```
Table 4-20—Analog operator arguments (continued)
```
```
Operator Constant expression arguments Dynamic expression arguments
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
87
```
#### 4.6.1 Analysis.............................................................................................................................

The **analysis()** function takes one or more string arguments and returns one ( 1 ) if any argument
matches the current analysis type. Otherwise it returns zero ( 0 ). The general form is:

```
analysis ( analysis_list )
```
There is no fixed set of analysis types. Each simulator can support its own set. However, simulators shall use
the names listed in Table 4- 21 to represent analyses which are similar to those provided by SPICE.

Any unsupported type names are assumed to not be a match.

Table 4- 22 describes the implementation of the analysis function. Each column shows the return value of the
function. A status of one ( 1 ) represents _True_ and zero ( 0 ) represents _False._

Using the **analysis()** function, it is possible to have a module behave differently depending on which
analysis is being run.

```
Table 4-21—Analysis types
```
```
Name Analysis description
```
```
"ac" .AC analysis
"dc" .OP or .DC analysis (single point or dc sweep analysis)
"noise" .NOISE analysis
"tran" .TRAN analysis
"ic" The initial-condition analysis which precedes a transient analysis.
"static" Any equilibrium point calculation, including a DC analysis as well as those that precede
another analysis, such as the DC analysis which precedes an AC or noise analysis, or the
IC analysis which precedes a transient analysis.
"nodeset" The phase during an equilibrium point calculation where nodesets are applied.
```
```
Table 4-22—Analysis function implementation
```
```
Analysis Argument DC Sweep
```
```
a
d1 d2 dN
```
```
aSweep refers to a dc analysis in which a parameter is swept through multiple values. d1, d2 and dN above refer
to dc points within the same sweep analysis.
```
```
TRAN
OP Tran
```
```
AC
OP AC
```
```
NOISE
OP AC
```
```
First part of "static" when
nodesets are applied
```
```
"nodeset" 1 1 0 0 1 0 1 0 1 0
```
```
Initial DC state "static" 1 1 1 1 1 0 1 0 1 0
Initial condition "ic" 0 0 0 0 1 0 0 0 0 0
DC "dc" 1 1 1 1 0 0 0 0 0 0
Transient "tran" 0 0 0 0 1 1 0 0 0 0
Small-signal "ac" 0 0 0 0 0 0 1 1 0 0
Noise "noise" 0 0 0 0 0 0 0 0 1 1
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
88
```
For example, to implement nodesets or initial conditions using the **analysis()** function and switch
branches, use the following.

```
if ( analysis ("ic"))
V(cap) <+ initial_value;
else
I(cap) <+ ddt (C*V(cap));
```
#### 4.6.2 DC analysis

Verilog-AMS supports a single-point dc analysis and also a multipoint dc sweep analysis in which multiple
dc points are computed over a sweep of parameter values. An operating point analysis is done for each dc
point in the sweep. A single-point dc analysis is the same as an operating point analysis. The
**analysis** ("dc") and **analysis** ("static") function calls shall return true for a single-point dc analysis
and also for every dc point in a sweep analysis. The **analysis** ("nodeset") function call shall return true
only during the phase of an operating point analysis in which nodeset values are applied; that phase may
occur in a single-point dc analysis or the first point of a multipoint dc sweep analysis, but does not occur for
subsequent points of a dc sweep.

During a dc sweep analysis, the values of variables at the conclusion of the operating point analysis for one
dc point shall be used as the starting values for those variables for the next dc point. However, variable val-
ues shall not be carried over between two independent dc sweep analyses (from the last dc point of one anal-
ysis to the first dc point of the next analysis). Variables shall be re-initialized to zero (or x, for integers
whose values are assigned in a digital context) at the start of each new analysis.

#### 4.6.3 AC stimulus.......................................................................................................................

A small-signal analysis computes the steady-state response of a system which has been linearized about its
operating point and is driven by a small sinusoid. The sinusoidal stimulus is provided using the **ac_s-
tim()** function. The general form is:

```
ac_stim ( [ analysis_name [ , mag [ , phase ] ] ] )
```
The AC stimulus function returns zero ( 0 ) during large-signal analyses (such as DC and transient) as well as
on all small-signal analyses using names which do not match _analysis_name._ The name of a small-signal
analysis is implementation dependent, although the expected name (of the equivalent of a SPICE AC analy-
sis) is “ac”, which is the default value of _analysis_name_. When the name of the small-signal analysis
matches _analysis_name_ , the source becomes active and models a source with magnitude _mag_ and phase
_phase_. The default magnitude is one ( 1 ) and the default phase is zero ( 0 ). _phase_ is given in radians.

#### 4.6.4 Noise

Several functions are provided to support noise modeling during small-signal analyses. To model large-sig-
nal noise during transient analyses, use the **$random()** or **$arandom()** system tasks. The noise func-
tions are often referred to as noise sources. There are four noise functions, **white_noise()** models white
noise processes, **flicker_noise()** models _1/f_ or flicker noise processes, **noise_table()** interpo-
lates a vector to model a process where the spectral density of the noise varies as a piecewise linear function
of frequency, and lastly, **noise_table_log()** interpolates a vector to model a process where the spec-
tral density of the noise varies as a piecewise linear function of the base-10 logarithm of frequency. The
noise functions are only active in small-signal noise analyses and return zero ( 0 ) otherwise.

The syntax for noise functions is shown in Syntax 4- 4.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
89
```
analog_small_signal_function_call ::= _// from A.8.2_
...
| **white_noise (** analog_expression [ **,** string ] **)**
| **flicker_noise (** analog_expression **,** analog_expression [ **,** string ] **)**
| **noise_table (** noise_table_input_arg [ **,** string ] **)**
| **noise_table_log (** noise_table_input_arg [ **,** string ] **)**

```
Syntax 4-4—Syntax for the noise functions
```
**4.6.4.1 white_noise**

White noise processes are those whose current value is completely uncorrelated with any previous or future
values. This implies their spectral density does not depend on frequency. They are modeled using:

```
white_noise ( pwr [ , name ] )
```
which generates white noise with a power of _pwr_.

For example, the thermal noise of a resistor could be modeled using:

```
I(a,b) <+ V(a,b)/R +
white_noise (4 * ‘P_K * $temperature /R, "thermal");
```
The optional _name_ argument acts as a label for the noise source used when the simulator outputs the individ-
ual contribution of each noise source to the total output noise. The contributions of noise sources with the
same _name_ from the same instance of a module are combined in the noise contribution summary.

**4.6.4.2 flicker_noise**

The **flicker_noise()** function models flicker noise. The general form is:

```
flicker_noise ( pwr , exp [ , name ] )
```
which generates pink noise with a power of _pwr_ at 1Hz which varies in proportion to _1/f exp_.

The optional _name_ argument acts as a label for the noise source used when the simulator outputs the individ-
ual contribution of each noise source to the total output noise. The contributions of noise sources with the
same _name_ from the same instance of a module are combined in the noise contribution summary.

**4.6.4.3 noise_table**

The **noise_table()** function interpolates a set of values to model a process where the spectral density of
the noise varies as a piecewise linear function of frequency. The general form is:

```
noise_table ( input [ , name ] )
```
The argument _input_ can either be a vector or a string indicating a filename.

When the _input_ is a vector it contains pairs of real numbers: the first number in each pair is the frequency in
Hertz and the second is the power. The vector can either be specified as an array parameter or an array
assignment pattern.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
90
```
When the _input_ is a file name, the indicated file will contain the frequency / power pairs. The file name argu-
ment shall be constant and will be either a string literal or a string parameter. Each frequency / power pair
shall be separated by a newline and the numbers in the pair shall be separated by one or more spaces or tabs.
To increase the readability of the data file, comments may be inserted before or after any frequency / power
pair. Comments begin with ‘#’ and end with a newline. The input file shall be in text format only and the
numbers shall be real or integer.

The following shows an example of the input file:

```
# noise_table_input.tbl
# Example of input file format for noise_table
#
# freq pwr
1.0e0 1.657580e-23
1.0e1 3.315160e-23
1.0e2 6.636320e-23
1.0e3 1.326064e-22
1.0e4 2.652128e-22
1.0e5 5.304256e-22
1.0e6 1.060851e-21
```
```
# End of example input file.
```
Although the user is encouraged to specify each noise pair in order of ascending frequency, the simulator
shall internally sort the pairs into ascending frequency if required. Each frequency value must be unique.
**noise_table()** performs piecewise linear interpolation to compute the power spectral density generated
by the function at each frequency between the lowest and highest frequency in the set of values. For frequen-
cies lower than the lowest frequency in the value set, **noise_table()** returns the power specified for the
lowest frequency, and for frequencies higher than the highest frequency, **noise_table()** returns the
power specified for the highest frequency.

The optional _name_ argument acts as a label for the noise source used when the simulator outputs the individ-
ual contribution of each noise source to the total output noise. The contributions of noise sources with the
same _name_ from the same instance of a module are combined in the noise contribution summary.

**4.6.4.4 noise_table_log**

The **noise_table_log()** function interpolates a set of values to model a process where the spectral
density of the noise varies as a piecewise linear function of the base-10 logarithm of frequency. The general
form is:

```
noise_table_log ( input [ , name ] )
```
The argument _input_ can either be a vector or a string indicating a filename; in either case, the meaning and
restrictions on the input are the same as for **noise_table()**. The difference is that
**noise_table_log()** interpolates logarithmically. For a frequency _f_ not specified in the input data, the
noise power shall be computed using the two pairs ( _f1,p1_ ) and ( _f2,p2_ ) in the input (whether an array or file),
where _f1_ is the largest frequency value in the input data less than _f_ and _f2_ is the smallest frequency larger
than _f_ (that is, _f1 < f < f2_ ); the noise power _P_ is computed as:

```
P = pow (10, log (p1) + ( log (p2)- log (p1)) * ( log (f)- log (f1)) / ( log (f2)-
log (f1)) )
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
91
```
As with **noise_table()** , for frequencies lower than the lowest frequency in the value set,
**noise_table_log()** returns the power specified for the lowest frequency, and for frequencies higher
than the highest frequency, **noise_table_log()** returns the power specified for the highest frequency.

The optional _name_ argument acts as a label for the noise source used when the simulator outputs the individ-
ual contribution of each noise source to the total output noise. The contributions of noise sources with the
same _name_ from the same instance of a module are combined in the noise contribution summary.

The difference between **noise_table()** and **noise_table_log()** is illustrated in Figure 4- 14.

```
Figure 4-14: Comparison of noise_table and noise_table_log
```
The **noise_table_log()** function produces a straight line on a log-log plot from just two points:

```
V(out) <+ noise_table_log ('{1,1, 1e6,1e-6});
```
whereas the linear interpolation of noise_table produces a series of curves between the interpolating points,
depending on the number of points specified in the function call and the number of points per decade in the
small-signal analysis. Here, one point per decade is specified:

```
V(out) <+ noise_table ('{1,1, 1e1,1e-1, 1e2,1e-2, 1e3,1e-3, 1e4,1e-4,
1e5,1e-5, 1e6,1e-6});
```
```
noise_table noise_table_log
```
```
1uV^2/Hz
```
```
10uV^2/Hz
```
```
100uV^2/Hz
```
```
1mV^2/Hz
```
```
1kHz 10kHz 100kHz 1MHz
frequency
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
92
```
**4.6.4.5 Noise model for diode**

The noise of a junction diode could be modeled as shown in the following example.

```
I(a,c) <+ is*( exp (V(a,c) / (n * $vt )) - 1)
+ white_noise (2*‘P_Q*I(<a>))
+ flicker_noise (kf* pow ( abs (I(<a>)), af), ef);
```
**4.6.4.6 Correlated noise**

Each noise function generates noise which is uncorrelated with the noise generated by other functions. _Per-
fectly correlated noise_ is generated by using the output of one noise function for more than one noise source.
_Partially correlated noise_ is generated by combining the output of shared and unshared noise functions.

Example 1 — Two noise voltages are perfectly correlated.

```
n = white_noise (pwr);
V(a,b) <+ c1*n;
V(c,d) <+ c2*n;
```
Example 2 - Partially correlated noise sources can also be modeled.

```
n1 = white_noise (1-corr);
n2 = white_noise (1-corr);
n12 = white_noise (corr);
V(a,b) <+ Kv*(n1 + n12);
I(b,c) <+ Ki*(n2 + n12);
```
**4.7 User-defined functions**

A user-defined function can be used to return a value (for an expression). All functions are defined within
modules. Each function can be an analog user-defined function or a digital function (as defined in IEEE Std
1364 Verilog).

#### 4.7.1 Defining an analog user-defined function.........................................................................

The syntax for defining an analog user-defined function is shown in Syntax 4- 5.

analog_function_declaration ::= _// from A.2.6_
**analog function** [ analog_function_type ] analog_function_identifier **;**
analog_function_item_declaration { analog_function_item_declaration }
analog_function_statement
**endfunction**

analog_function_type ::= **integer** | **real** | **string**

analog_function_item_declaration ::=
analog_block_item_declaration
| input_declaration **;**
| output_declaration **;**
| inout_declaration **;**

analog_block_item_declaration ::= _// from A.2.8_
{ attribute_instance } parameter_declaration **;**
| { attribute_instance } integer_declaration
| { attribute_instance } real_declaration


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
93
```
```
| { attribute_instance } string_declaration
```
```
Syntax 4-5—Syntax for an analog user-defined function declaration
```
An analog user-defined function declaration shall begin with the keywords **analog function** , option-
ally followed by the type of the return value from the function, then the name of the function and a semico-
lon, and ending with the keyword **endfunction**.

The _analog_function_type_ specifies the return value of the function; its use is optional. _type_ can be **real** ,
**integer** , or **string** ; if unspecified, the default is **real**.

An analog user-defined function:

```
— can use any statements available for conditional execution (see 5.2);
— shall not use access functions;
— shall not use analog filter functions;
— shall not use contribution statements or event control statements;
— shall have at least one formal argument declared;
— all formal arguments shall have an associated block item declaration specifying the data type of the
argument;
— all formal arguments shall have an associated direction specification that shall be either input ,
output , or inout ;
— shall not use named blocks;
— shall only reference locally-defined variables, variables passed as arguments, locally-defined param-
eters and module-level parameters; and
— if a locally-defined parameter with the specified name does not exist, then the module-level parame-
ter of the specified name will be used.
```
Example 1 — Determine max value:

This example defines an analog user-defined function called maxValue, which returns the potential of
whichever signal is larger.

```
analog function real maxValue;
input n1, n2;
real n1, n2;
begin
// code to compare potential of two signals
maxValue = (n1 > n2)? n1 : n2;
end
endfunction
```
Example 2 — Area and perimeter of a rectangle

This example defines an analog user-defined function called geomcalc, which returns both the area and
perimeter of a rectangle.

```
analog function real geomcalc;
input l, w;
output area, perim;
real l, w, area, perim;
begin
area = l * w;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
94
```
```
perim = 2 * ( l + w );
end
endfunction
```
Example 3 — Initialization of a vector variable

The analog user-defined function called arrayadd adds the contents of a second array to the first.

```
analog function real arrayadd;
inout [0:1]a;
input [0:1]b;
real a[0:1], b[0:1];
integer i;
begin
for (i = 0; i < 2; i = i + 1) begin
a[i] = a[i] + b[i];
end
end
endfunction
```
#### 4.7.2 Returning a value from an analog user-defined function..................................................

There are four ways to return a value from an analog user-defined function: using the implicit analog user-
defined function identifier variable, using a **return** statement, using an output argument, or using an inout
argument.

**4.7.2.1 Analog user-defined function identifier variable**

The analog user-defined function definition implicitly declares a variable, internal to the analog user-defined
function, with the same name as the _analog_function_identifier_. This variable inherits the same type as the
type specified in the analog user defined function declaration. This internal variable is initialized to zero (0)
if the inherited type is numerical, or the empty string (“”) if a string, and can be used within the body of the
analog user-defined function. The last value assigned to this variable will be the return value of the analog
user-defined function. If this internal variable is not assigned during the execution of the analog user-defined
function, then the analog user-defined function will return the variables default initial value. An analog user-
defined function shall always return a scalar value, either numeric or string based on the type specified for
the analog user-defined function.

The following line (from the first example in 4.7.1) illustrates this concept:

```
maxValue = (n1 > n2)? n1 : n2;
```
**4.7.2.2 Analog function return statement**

The **return** statement shall override any value assigned to the function name. When the **return** state-
ment is used, the function shall specify an expression with the **return** of the correct type for the function.

```
return (n1 > n2)? n1 : n2;
```
**4.7.2.3 Output arguments**

An **output** argument allows the user to return more than one value. The argument passed to an **output**
argument must be an analog variable reference. If the **output** argument is defined as an array then the
argument passed into the function must be an analog variable or an array assignment pattern of analog vari-
ables of equivalent size. All **output** arguments of an analog user-defined function are initialized, zero (0)


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
95
```
if numeric, empty string (“”) if a string, which in turn means that the argument passed to it is reset to zero (0)
or the empty string (“”) accordingly. During the execution of the function, these variables can be read and
assigned in the flow. At the end of the execution of the analog user-defined function, the last value assigned
to the **output** argument is then assigned to the corresponding analog variable reference that was passed
into the function.

The following lines (from the second example in 4.7.1) illustrate this concept:

```
area = l * w;
perim = 2 * ( l + w );
```
**4.7.2.4 Inout arguments**

**inout** arguments allow the user to pass in a value to the function and return a different value from it using
the same argument. The argument passed to an **inout** argument must be an analog variable reference. If the
**inout** argument is defined as an array then the argument passed into the function must be an analog vari-
able or an array assignment pattern of analog variables of equivalent size. The **inout** arguments of an ana-
log user-defined function do not get initialized like those defined as **output**. During the execution of the
function, these variables can be read and assigned in the flow. At the end of the execution of the analog user-
defined function, the last value assigned to the **inout** argument is then assigned to the corresponding ana-
log variable reference that was passed into the function. If a value was not assigned to the **inout** argument
during the execution of the analog user-defined function, then the corresponding analog variable reference is
left untouched.

The following lines (from the third example in 4.7.1) illustrate the use of an **inout** argument.

```
for (i = 0; i < 2; i = i + 1) begin
a[i] = a[i] + b[i];
end
```
Note: **inout** arguments are not “pass by reference”, but more closely related to “copy in” and “copy out”.
Care should be taken to avoid passing the same analog variable reference to different **inout** and **output**
arguments of the same analog user-defined function as the results are undefined.

#### 4.7.3 Calling an analog user-defined function

An analog user-defined function call is an operand within an expression. Syntax 4- 6 shows the analog user-
defined function call.

analog_function_call ::= _// from A.8.2_
analog_function_identifier { attribute_instance } **(** analog_expression { **,** analog_expression } **)**

```
Syntax 4-6—Syntax for function call
```
The order of evaluation of the arguments to an analog user-defined function call is undefined. The argument
expressions are assigned to the declared inputs, outputs, and inouts in the order of their declaration.

An analog user-defined function:

```
— shall not call itself directly or indirectly, i.e., recursive functions are not permitted; and
— shall only be called within the analog context, either from an analog block or from within another
analog user-defined function;
```

```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
96
```
The following example uses the maxValue function defined in 4.7.1.

```
V(out) <+ maxValue(val1, val2);
```
The following example uses the geomcalc function defined in 4.7.1.

```
dummy = geomcalc(l-dl, w-dw, ar, per);
```
Note that the first two arguments are expressions, and match up with the inputs l and w for the function; the
second two arguments must be real identifiers because they match up with the function outputs.

The following example incorrectly uses the geomcalc function defined in 4.7.1.

```
dummy = geomcalc(l-dl, w-dw, ar, V(a));
```
Here the last two arguments to the user-defined function geomcalc are declared as **output** arguments, but
the fourth argument is passed the potential probe _V(a)_. Only analog variable references can be passed to
**output** and **inout** arguments of an analog user-defined function so this example will result in a compila-
tion error.

The following example uses the arrayadd example defined in 4.7.1, to add values from one array to
another.

```
x[0] = 5; x[1] = 10;
y = 3; z = 6;
dummy = arrayadd(x,'{y,z});
```
Here the first and second arguments are both expecting vectors. A vector variable is passed for the first argu-
ment and an array assignment pattern of two scalar analog variables has been used for the second argument.
Since the first argument is an **inout** argument, the result of calling the arrayinit function will update the
vector variable x with values x[0] = 8 and x[1] = 16.


```
Accellera Standard for VERILOG-AMS - Analog and Mixed-signal Extensions to Verilog HDL
```
```
97
```
