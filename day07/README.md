# Day 07

So I read this problem and thought "Okay, ranking poker hands, not exactly trivial but pretty straightfoward" and
proceeded to implement a function that compared poker hands. The trouble is that for the purposes of this problem, 2AAAA
is a worse hand than A2222 because for equivalent hand types the cards are meant to be compared from left-to-right. It
took me poring over my sorted list and checking by hand for correctness before I eventually went to the AoC subreddit
and realized my mistake. I had implemented this neat way of serializing the hand strength into a single number that
could be prepared, but that turned out to be a waste of time.

Part B was pretty straightforward, but I tripped myself up by attempting to implement the wildcard logic purely
imperatively by handling each possible case of the largest group and number of wild cards. I eventually got the right
answer, but I went back after and replaced the imperative logic with pattern matching which is a lot more elegant. It
also turns out that you can just add the number of wildcards to the largest group size and then reuse the same pattern
matching logic, so I was able to consolidate most of the code for the two parts.
