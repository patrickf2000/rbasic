REM input
REM Tests the input command

MAIN:
	let #no1 = 0
	let #no2 = 0
	
	print "Enter a number: "
	input no1
	
	print "Enter another number: "
	input no2
	
	if [no1 > no2] then println "The first number is greater."
	elif [no1 < no2] then println "The second number is greater."
	else println "The numbers are equal."
	
	exit
