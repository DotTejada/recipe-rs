## Recipe scaling program in Rust.

I made this because I sick and tired of scaling up every recipe by hand. On top of that, only a few websites allow you to see a scaled up version, but even those are limited to 2 or 3 times. And that is if the recipe is even from a website.

So I figured this would be simple to make + personally useful.

Takes in a text file for input, outputs scaled up text file. Amounts should be before the item. Supports whole numbers, fractions, and mixed numbers. You can see some input examples in the repo.

# Command line usage is as follows:
<scale (>0)> <path to input> < -h or -t >
-h is for sending output to exe dir
-t is for sending output to input dir
