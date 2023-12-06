# Day 05

So last year's problem 5 involved moving different numbers of crates between piles. Part B modified the problem from
moving crates one at a time to moving many at once.

This year, it took me probably an hour and a half to even figure out how to approach part B. I snuck a peek at part A
last night and was relieved the problem seemed a bit easier today, so imagine my disappointment when the problem space
suddenly increased by, like, 8 orders of magnitude. It turns out the trick is to recursively map each level's output
ranges to a series of subranges where each subrange covers a discrete range of the next level down. For example, if
level 1 has a range that maps to 3-8 and level 2 has ranges 0-5 and 6-10 with offsets 2 and 4 respectively, we generate
two subranges: one from 3-5 with offset 2, and one from 6-8 with offset 4.

The bright side is that once I actually got the program to compile and run without errors it gave the right answer on
the first try, so that felt pretty nice. For some reason it does generate overlapping ranges with the same offset, but
it doesn't actually affect the final answer nor have a noticeable effect on performance so I didn't see any reason to
take the time to fix it.
