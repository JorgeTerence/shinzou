.set AGE 0x12

-- string of characters
.org 0x000
.word 0x46
.word 0x61
.word 0x72
.word 0x65
.word 0x77
.word 0x65
.word 0x6c
.word 0x6c
.word 0x2c
.word 0x20
.word 0x43
.word 0x68
.word 0x6f
.word 0x73
.word 0x65
.word 0x6e
.word 0x20
.word 0x55
.word 0x6e
.word 0x64
.word 0x65
.word 0x61
.word 0x64
.word 0x2e

counter:
.word 0x000
inc:
.word 0x001
end:
.word 0x018

.org 0x050
loop:
-- print char
out m(counter)

-- counter++
load m(counter)
add m(inc)
stor m(counter)

-- while counter < = end
load m(end)
sub m(inc)
jump+ m(loop,0:19)

.org 0x3fe
-- end program
out m(0x02)
