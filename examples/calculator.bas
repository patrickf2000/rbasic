REM calculator.bas
REM A simple calculator program

MAIN:
	println "Welcome!"
	let #choice = 0
	
	do
		println "1) Add"
		println "2) Subtract"
		println "3) Multiply"
		println "4) Divide"
		println "5) Exit"
		input choice
		
		if [choice == 1] then gosub add
		elif [choice == 2] then gosub sub
		elif [choice == 3] then gosub mp
		elif [choice == 4] then gosub div
		elif [choice == 5] then println " "
		else println "Unknown option."
	while choice != 5
	
	println "Goodbye!"
	exit

add:
	println "Add"
	let .no1 = 0
	let .no2 = 0
	
	print "Enter no1: "
	input no1
	
	print "Enter no2: "
	input no2
	
	let .answer = no1 + no2
	print "The answer is: "
	print answer
	println " "
	
	println " "
	input

sub:
	println "Subtract"
	let .no1 = 0
	let .no2 = 0
	
	print "Enter no1: "
	input no1
	
	print "Enter no2: "
	input no2
	
	let .answer = no1 - no2
	print "The answer is: "
	print answer
	println " "
	
	println " "
	input

mp:
	println "Multiply"
	let .no1 = 0
	let .no2 = 0
	
	print "Enter no1: "
	input no1
	
	print "Enter no2: "
	input no2
	
	let .answer = no1 * no2
	print "The answer is: "
	print answer
	println " "
	
	println " "
	input

div:
	println "Divide"
	let .no1 = 0
	let .no2 = 0
	
	print "Enter no1: "
	input no1
	
	print "Enter no2: "
	input no2
	
	let .answer = no1 / no2
	print "The answer is: "
	print answer
	println " "
	
	println " "
	input
