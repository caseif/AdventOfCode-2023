# Day 11

Today's was far easier. For part A, you just need to compute the Manhattan distance between each pair and then add the
number of empty columns between the y values and empty rows between the x values (since each of those counts as two).
Part B tripped me up slightly first by overflowing the `u32` I was storing the sum in, then with a sneaky off-by-one
error (you need to multiply by 1 less than a million because the row is already counted once in the Manhattan distance).

For some reason I was feeling especially functional today, and I'm pretty happy with how my implementation turned out.
Such elegant, very pure.

░░░░░█▐▓▓░████▄▄▄█▀▄▓▓▓▌█  
░░░░░▄█▌▀▄▓▓▄▄▄▄▀▀▀▄▓▓▓▓▓▌█  
░░░▄█▀▀▄▓█▓▓▓▓▓▓▓▓▓▓▓▓▀░▓▌█  
░░█▀▄▓▓▓███▓▓▓███▓▓▓▄░░▄▓▐█▌  
░█▌▓▓▓▀▀▓▓▓▓███▓▓▓▓▓▓▓▄▀▓▓▐█  
▐█▐██▐░▄▓▓▓▓▓▀▄░▀▓▓▓▓▓▓▓▓▓▌█▌  
█▌███▓▓▓▓▓▓▓▓▐░░▄▓▓███▓▓▓▄▀▐█  
█▐█▓▀░░▀▓▓▓▓▓▓▓▓▓██████▓▓▓▓▐█  
▌▓▄▌▀░▀░▐▀█▄▓▓██████████▓▓▓▌█▌  
▌▓▓▓▄▄▀▀▓▓▓▀▓▓▓▓▓▓▓▓█▓█▓█▓▓▌█▌  
█▐▓▓▓▓▓▓▄▄▄▓▓▓▓▓▓█▓█▓█▓█▓▓▓▐  
