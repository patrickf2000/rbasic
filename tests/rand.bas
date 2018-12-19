REM rand.bas
REM Tests the random number generator

MAIN:
	print "What should the max be? "
	let #max = (input)
	
	print "How many numbers should we generate? "
	let #to_gen = (input)
	
	let #index = 0
	
	do
		println (rand max)
		#index = index + 1
	while index < to_gen
	
	println "Goodbye!"
	exit
