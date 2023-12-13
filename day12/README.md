# Day 12

This one was kind of tricky too. The key to solving part B in a reasonable amount of time is to strip away the start of
the conditions string as well as the group size list as you go so that you can utilize memoization. Unfortunately, for
whatever reason this just wasn't intuitive to me and I wasted a lot of time trying to optimize my pruning algorithm in
an effort to speed up execution to less than a day.
