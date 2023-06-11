### FrwaR - frontend rust wasm w/ react ; or maybe Fast React Wasm Antfarm with Rust.

![image](https://github.com/sqrtM/FrawR/assets/79169638/6982f336-00c7-4158-b5df-e56cd699b024)


tricks:
- hold user SEED, LOCATION, and perhaps a DELTA value, but that seems like it would quickly get enormous.
In that case, perhaps it should be a table of entities which we hold to be important?


### journal
- 07/06: useContext is useless for us in this case. Each component "class" which grabs context immediently rerenders on every tick.
making it useless for such a large web project. Drilling the player state down will
be necessary unfortunately, though it feels pretty inefficent.

- 08/06: Firstly, when passing the player object around,  I set the type to `Player | false` because JS has a problem comparing undefineds.
Setting it as false seems to have the same effect except it is also easy to compare quickly (which is important for the memo comps, which are slow.)
I redefined the `World` struct to have a nested field for all living things. This will make it simpler to pass things around and reduce the number of calls
between JS and Rust. It generally seems to be better to pass few Large objects than several small ones. Switching languages is expensive. Ideally, we 
do it once per turn and no more.

- 09/06: When an `Entity` moves, it checks the tile it's going to move to. This doesn't seem to work with 100% accuracy and I'm not sure why.
I tried changing the Entity field of World into a BTree, but wow, that is a bad idea, I think. 
I keep putting off worldgen because it's very hard to do it the way I'm used to doing it. Maybe some bitshifts will work?
Maybe now is the time to learn the good old BSP method? Or a better crawler? We'll see.

- 11/06: world indexing fixed :) 
