# Rust Bloom Filter Demo: Sync Photo Collections By Date

## Problem

You and two friends each have a common photo collection organized by date, with each day’s photos kept in separate folders identified by the date (e.g.,“2024-04-15”).   
Each photo within a day’s folder can be uniquely identified by a hash value. It’s possible that none of you have the complete “database” for each day.   
Your task is to create a program that helps all three of you compile a complete collection for each specific day, identifying which photos are missing from your collection and which friend has them.

## Requirements Summary

1. Use a unique id to identify each photo. No actual photos or hashes needed.
3. Use hash comparisons to check if friends collections are the same for a given date, and if not, locate missing id's from your collection
4. For each day collections, output a list of missing id's, along with refs to their friend location
5. Include sufficient test coverage

> Note: See the pdf file in ./docs for full details and requirements. 

## Discussion

Let's consider the problem a bit, with some simple cases:

Given 3 friends, each called F.X, each containing some elements {a, b, c, ...}, and H being the hash func

Case 1: "Full Sync", this is the desired outcome, when all friends have the same photos.

F.1 = {a}, F.2 = {a}, F.3 = {a} -> H(F.1) = H(F.2) = H(F.3)

Case 2: "Full Unsync", when each friend has a completely different set

F.1 = {a}, F.2 = {b}, F.3 = {c} -> H(F.1) != H(F.2) != H(F.3), however, in this case, the desired outcome is F.1 = F.2 = F.3 = {a, b, c} -> H(F.X) = H(a,b,c). Which implies, that **in sync, the hash of each node's concatenated hashes must be equal to the hash of all unique elements across all nodes**

I think this could be useful to track INCREMENTAL CHANGES over time, not entirely sure. However, the instructions are concerned with locating missing identifiers within a given day, that is **not crossing day boundaries** 

## Solution Design

After some research, and consider approaches with Merkle Trees, Hash tables and hash maps, I decided to use something called a "Bloom filter", which is a data structure which tells you if an element is probably present in a set, or if it is definitely not present (See references section below).

For the record, a Merkle Tree would work for daily incremental syncronization (I think), as follows:

1. At the end of Day 1, we calculate a tree, with each photo hash, then a hash for the node, then a root hash for the network.
2. Next day we recalculate, and compare with the tree from the previous day, and now we can tell, which Nodes had changing elements, and then we can have these nodes send their updates as appropriate.

However, if we are talking about SINGLE DAY sync, we don't have a previous Merkle Tree, unless we calculated it for each photo added to each friend, as it happens. Which could be I guess.
But, I decided to go with the Bloom filter, as an "End of Day" sync, to save calculations and bandwith. We do have to consider that it may produce a small rate of error, so in a real world app we should both test the performance and the actual error observed for various configurations and decide if it's an appropriate course of action.  
But since we're checking for negatives, I think there will be no errors.

In any case it is very cool that this approach exists :)

### Bloom 

1. F.1 will be the "Master" node. At the end of the day, it knows what elements does it have, but doesn't know what elements the other nodes have that it is missing.
2. So, as requested, we'll check the hashes to check if collections are different. 
3. If so, F.1 sends its bloom filter (made from its elements) to nodes F.2 and F.3
4. Nodes F.2 and F.3 check the filter for the presence of their own elements, and if missing, send the report back to F.1
5. Thus achieving the required goals, where F.1 now contains references to all photos in all friends folders.

## Implementation

- Rust code is attached
- 3 "friends" (nodes), 1 is "Master", the one who keeps the database "source of truth".
- X "photos", identified as unique numbers [1..X]
- Since we are only concerned with "per day's folder" sync, we can obviate having multiple days, since ultimately, each day syncs independently. So the code runs various test scenarios for one day.
- There are various bloom crates in Rust, I just chose one. No need to implement the bloom filter anew.
- The test makes 5 runs, ofc this could be extended ad infinitum, along with an arbitrary number of "friends".

> .cargo/config.toml has env vars to configure

MAX_NUM_OF_ELEMS_PER_FRIEND -> Each "Friend" will have up to this number of photos, with each run actually having a random number assigned. Default is 10.  
FALSE_POSITIVE_RATE -> The bloom filter desired false positive rate  
FILTER_ITEMS -> The number of elements in the filter.

### Deps & exec

Rust 1.78  
`cargo run`

## References

https://stackoverflow.com/questions/23197145/efficient-synchronization-algorithm  
https://cronokirby.com/posts/2021/07/on_multi_set_hashing/  
https://medium.com/iovlabs-innovation-stories/incremental-multiset-hashing-62b2b9d16aa9  
https://technion-staging.elsevierpure.com/en/publications/efficient-multiset-synchronization  
https://www.geeksforgeeks.org/bloom-filters-introduction-and-python-implementation/  
https://www.codementor.io/blog/merkle-trees-5h9arzd3n8#so-what-are-merkle-trees  



