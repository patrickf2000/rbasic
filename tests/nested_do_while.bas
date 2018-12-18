REM nested_do_while
REM Tests the nested do-while feature

MAIN:
	let #x = 0
	let #y = 0
	let #z = 0
	let #z2 = 0
	
	do
		println "Outer"
		#x = x +1
		
		do
			println "Inner"
			#y = y + 1
		while y < 3
		
		do
			println "Inner 2!!"
			#z = z + 1
			
			do
				println "Deep!!"
				#z2 = z2 + 1
			while z2 < 3
			
			#z2 = 0
		while z < 2
		
		#y = 0
		#z = 0
		
		println " "
	while x < 5
	
	println "Done with loops"
	exit
