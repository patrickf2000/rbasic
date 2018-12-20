REM math_practice
REM A simple math practice program written in rBasic

nl:
	println " "

MAIN:
	cls
	println "Welcome to math practice!"
	gosub nl
	
	print "Choose your level (1-3): "
	let #level = (input)
	
	if [level == 1] then #level = 10
	elif [level == 2] then #level = 100
	elif [level == 3] then #level = 1000
	else goto level_error
	
	let $op = "+"
	let #done = 1
	
	do
		gosub nl
		println "Choose your operation: +, -, *, /"
		println "+	>	Add"
		println "-	>	Subtract"
		println "*	>	Multiply"
		println "/	> 	Divide"
		
		$op = (input)
		
		if [op == "+" ] then break
		elif [op == "-" ] then break
		elif [op == "*" ] then break
		elif [op == "/" ] then break
	while done != 0
	
	gosub nl
	print "How many problems would you like? "
	let #repeat = (input)

	goto practice[level, op, repeat]
	
practice[#level, $op, #repeat]:
	cls
	let #no1 = 0
	let #no2 = 0
	let #user_answer = 0
	let #answer = 0
	let #count = 0
	let #n = 4
	
	do
		cls
		#no1 = (rand level)
		#no2 = (rand level)
		
		if [op == "+"] then #answer = no1+no2
		elif [op == "-"] then #answer = no1-no2
		elif [op == "*"] then #answer = no1*no2
		elif [op == "/"] then #answer = no1/no2
		
		do
			print "Solve: "
			print no1
			print " "
			print op
			print " "
			print no2
			println " = "
			#user_answer = (input)
			
			if [user_answer == answer] then break
			else println "Incorrect! Try again..."
		while n < 5
		
		#count = count + 1
	while count < repeat
	
level_error:
	println "Error: Unknown level."
	println "Press any key to continue..."
	input
	goto MAIN
