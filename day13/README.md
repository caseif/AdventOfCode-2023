# Day 13

Easier problem today, although as usual I forgot to read and didn't notice that part 2 requires excluding the original
reflection lines.

Anyway, I feel pretty clever about my solution for the second part. The core algorithm is to iterate the lines and for
each line iterate out in both directions (up/down) so that first the line is checked against its sibling, then the
respective siblings of those two lines are checked, and so on. For part 2, the first time a mismatch is found, the two
lines are interpreted as base-2 integers and XORed. If they only differ by one bit (character), then the result will be
a power of two (since only one bit will be set) and we allow it.
