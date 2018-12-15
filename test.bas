REM test.bas
REM This is our example program for our BASIC program language

REM All BASIC programs must have a main function
MAIN:
	PRINTLN "Welcome to BASIC!"
	PRINTLN "We are in the main function"
	GOSUB SayHello
	PRINTLN "Back in Main!"
	GOTO Func2
	
SayHello:
	PRINTLN "Hello!"
	RETURN
	
Func2:
	PRINTLN "We are in function two!!"
	PRINTLN "We shall exit."
	EXIT
