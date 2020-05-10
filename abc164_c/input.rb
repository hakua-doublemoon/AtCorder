
CODE_ARR = ('a'..'z').to_a.push(' ')
#p CODE_ARR
#return
$count = 0

BASE_CODE = 'a'.ord
$limits = []
num_of_code = CODE_ARR.length
CODE_ARR.length.times do |i|
    $limits << ((i+1)*200000/num_of_code)
    #$limits << ((i+1)*200/num_of_code)
end
#p $limits

def put_char(formar)
    len = formar.length
    if  len > 0  then
        head_code = formar[0].ord - BASE_CODE
        if  $count >= $limits[head_code]
            #$count += 1
            #puts formar
            return
        else 
            #if  len == 10  then
            if  len == 4  then
                $count += 1
                puts formar
                return
            end
        end
    end
    CODE_ARR.each do |code|
        if  code != ' '  then
            put_char(formar+code)
        else
            $count += 1
            puts formar
        end
    end
end

put_char("")
puts $count-1 #空行が1つある

