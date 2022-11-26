# Start Day 10-22-22

Implementation Fruch-Gold algorithm. Downloaded the original paper.

Okay I have the psuedo code to implement.

Okay I have some nodes and edges and positions! Next I want to be able to change my positions. Mutable right I guess? So length of positions will be fixed, but I will definitely want to be changing values.

Okay I have a value that changes! I almost surely don't do this very efficiently, but it works.

Positions makes sense. What is disp??? Displacement probably? How much to move the position each iteration?

Yes that is correct.

# Start Day 11-26-22

1. Program more of the force directed algorithm
2. Review and understand matts code
3. Actually read the rest of the paper

### Notes on programming

Okay continue working through the psuedo code. Get "any" version working, then lets go back and actually figure out how the hell rust works.

Yep looks good, next section.

Okay code at least runs! Not correctly but there...

Now lets start unpacking the issues...

1. How can i get vectors / arrays with
   1. Dynamic size (slices I think?)
   2. How do I do data types
   3. Named tuples with x,y (structs)
   4. Can I do vector math instead of one at a time?

Slice versus array
