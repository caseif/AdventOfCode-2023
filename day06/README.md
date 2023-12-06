# Day 06

I'm starting to notice a pattern here. It turns out it's actually not that hard to brute force part B - it takes around
700 ms to run on my 5800X. That's clearly not the intended solution - I imagine I was supposed to do a binary search to
find each bound of the subset of times that would beat the record (since they're necessarily contiguous), but since
brute force will do fine, I decided to stick with that.
