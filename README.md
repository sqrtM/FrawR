### FrwaR - frontend rust wasm w/ react ; or maybe Fast React Wasm Antfarm with Rust.

![image](https://github.com/sqrtM/FrawR/assets/79169638/6982f336-00c7-4158-b5df-e56cd699b024)


tricks:
- hold user SEED, LOCATION, and perhaps a DELTA value, but that seems like it would quickly get enormous.
In that case, perhaps it should be a table of entities which we hold to be important?


### stuff ive been learnign
- useContext is useless for us in this case. Each component "class" which grabs context immediently rerenders on every tick.
making it useless for such a large web project. Drilling the player state down will
be necessary unfortunately, though it feels pretty inefficent.