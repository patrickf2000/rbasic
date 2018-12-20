REM loop.bas
REM Tests the loop command

MAIN:
    let #x = 0
    let #count = 0

    loop
        println "Hello!"

        if [x == 5] then break
        else #x = x + 1

        do
            println count
            #count = count + 1
        while count < x

        #count = 0
    end

    println "Goodbye!"
    exit