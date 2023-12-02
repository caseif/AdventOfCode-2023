# Day 01

For this year's AoC, I've decided to attempt to solve all problems in Rust. I have _some_ limited experience working
with the language (my hobby game engine has
[a module](https://github.com/caseif/argus/tree/master/engine/libs/shadertools) written in it), but I could definitely
stand to gain a little more familiarity, especially with the standard library.

[Last year](https://github.com/caseif/AdventOfCode-2023) I attempted and completed the polyglot challenge where each
day's solutions were written in a different language, and while I feel I gained a lot from the exposure to languages I
would ordinarily never have any reason to touch, it also required _far_ too much effort in a month that's already quite
busy on its own, so it's a little difficult to justify it again.

Day 1 was straightforward as expected, but part B was a lot more involved than I anticipated. For comparison, last
year's day 1 problem involved finding the set of numbers with the largest sum, and then for part B, finding the top
three sets with the largest sums instead. Today's part B was a fundamentally different problem, at least given my
approach for part A. It also included a weird edge case involving overlapping digit words, and usually those aren't seen
until at least a few days in.

I initially pulled in the `regex` crate, but thought it might be easier to just use `str::matches` and `str::rmatches`.
And of course, part B turned out to basically require it. Oh well.


