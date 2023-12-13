# Day 10

## Pain.

I don't particularly enjoy problems that require interpreting the input as a 2D space, and this was no exception.
Nonetheless, part A was straightforward enough - it's literally just a matter of stepping through each pipe and then
figuring out which direction to go next based on the type of pipe.

Part B was a whole different story. My first idea was to do a simple flood-fill and then subtract the number of filled
cells from the total area, but to my surprise my answer was wrong. It turns out the program also needs to be able to
handle spaces which are accessible by "squeezing between" pipes. I ultimately handled this by instead flood-filling
a grid overlaid onto the input grid offset by -0.5 cells in each direction, where flood-fill checks are calculated based
on the two real cells adjacent to the fractional cell in the respective direction. Then it took me literally hours over
two days as well as over 200 lines of debug code to work out that my algorithm wasn't considering the starting point to
be opaque to the flooding, so I was ending up with really weird results until I worked that out.