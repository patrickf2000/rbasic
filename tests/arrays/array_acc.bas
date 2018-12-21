REM array_acc
REM Tests functions for accessing elements in arrays

MAIN:
    array #numbers
    let #count = 0

    do
        let #no = (rand 100)
        push no to numbers
        #count = count + 1
    while count < 10

    let #len = (len numbers)
    print "The length of the array is: "
    print len
    ln

	println "The elements are:"
	#count = 0
	
	do
		println (item count in numbers)
		#count = count + 1
	while count < len

    exit
