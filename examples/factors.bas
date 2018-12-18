REM factors.bas
REM Lists all factors of an integer

MAIN:
	let #no = 0
	let #index = 1
	let #stop = 0
	let #mod = 0
	
	print "Enter a number: "
	input no
	#stop = no + 1
	
	println " "
	println "The factors of your number are:"
	
	do
		#mod = no % index
		if [mod == 0] then println index
		#index = index + 1
	while index < stop
	
	exit
