## rbasic

### What is it?
rbasic is a BASIC dialect/interpreter. While rbasic is very similar to many other dialects of basic, it does have its own differences. The interpreter is written in Rust (hence the name).

### Supported Commands
REM					-> comments
PRINTLN				-> Print to the console (with a new line)
PRINT				-> Print to the console (no newline)
INPUT				-> Get console input and save to a variable (eg, INPUT x)
LET					-> Create a variable and assign a value to it
GOSUB				-> Go to a function and return to the calling function
GOTO				-> Go to a function but do not return from it
RETURN				-> Return a value from a function
EXIT				-> Exit the program
DO...WHILE			-> A do-while loop
IF...ELIF...ELSE	-> Conditional statements
MEMSET				-> Set a variable from memory

Note that commands are NOT case-sensitive. PRINT, print, and Print are all valid.

### Variables
Three datatypes are current supported: int (integer), dec (decimal, floating-point numbers), and str (strings). The datatype is determined by a symbol, which you must use when creating/re-assigning variables. For example:   
Assigning: let $x = "Hello!"   
Re-assigning: $x = "How are you?"   

However, you do not need the datatype specifier when you are using variables as arguments. For example:   
println x	-> Correct   
println $x	-> Incorrect   

Datatypes:
* #	-> Integers   
* .	-> Decimal numbers   
* $	-> Strings   

### Functions
To define a function, do something simply give it a name, specify any arguments, and end with a colon:   
myFunction:   
myFunctionWithArgs[$arg1, #arg2, .arg3]:   

To call:   
goto myFunction
gosub myFunctionWithArgs["Hello", 5, 5.6]

Returning values from functions is currently a little tricky. Below is an example:   
MAIN:   
	gosub saySomething   
	let $x = ""   
	memset x   
	println x   
	
saySomething:   
	let $y = "Hello!"   
	return y   
	
*** Important ***   
In rbasic, everything has to be in a function. Therefore, you have to have a main function, which is simply titled "MAIN". Note that this function name IS case-sensitive; it must be all-caps.

### Do-while Loops
Do-while loops are currently the only supported loops. Below is a working example:   
println "Before Loop:"   
let #x = 0   
let #y = 3   
   	
do   
	println "In Loop!!"   
	#x = x + 1   
while x != y   
	   
println "After loop"   

Only integers can be used in the loop condition. Note that this will probably change in the near future. Also, you can nest do-whle loops.

### Conditionals
Conditionals look like this:   
let #x = 10   
let #y = 1   
let $result = "Don't know"   
   
if [x > y] then $result = "Greater"   
elif [x < y] then $result = "Less"   
else $result = "Maybe they are equal!"   
   
println result    

### Operators
The following mathematical operators are supported:   
* +		-> Addition   
* -		-> Subtraction   
* *		-> Multiplication   
* /		-> Division   
* %		-> Modulus   

The following logical operators are supported:   
* >		-> Greater than   
* <		-> Less than   
* ==	-> Equal to   
* !=	-> Not equal to   
