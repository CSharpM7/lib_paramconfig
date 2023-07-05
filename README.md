# lib_commonconfig

A common problem across "single slot movesets"(SSMs) is that because they often hook the same functions, they will cause crashes when multiple SSMs are active at once. This plugin serves as a middleman between the source code and mods, so that even though multiple SSMs are active, this is the only plugin that actually hooks param functions.

## Dependencies
[lib_nrohook](https://github.com/ultimate-research/nro-hook-plugin/releases) (install in your plugins folder, all end-users will need this as well)

## Usage
All end-users will need to download an updated `lib_commonconfig.nro` from the releases page (please do not bundle with your release as this plugin is still in its infancy). Create a file called `config_param.toml` and place it in the root of your mod (feel free to copy the one provided in the source code). Fill out the toml with the fighter kind of your mod, the slots that will be affected, and the common parameters you wish to modify. I recommend testing this first before distributing and updating your mods.

## Known Issues
Currently, this does not support editing parameters featured in `vl.prc` files. Most of those values can be "single slotted" by reimplementing the main status of the affected parameters, however hurtbox information and ledgegrab box data cannot.
This also can only change `int`, `float`, and `bool` values. For bools, please use a `param_int` object.

## Toml Layout

```
kind = "string" (this should be whatever comes after FIGHTER_KIND_. Ie "MARIO" or "mario")
slots = [int,int,...] (a list of slots (alt costume numbers) to be affected)

[[param_int]]
key = "string" (the cracked hash name found in the fighter_param file)
value = int (if you are working with a bool, use 0 for false, and 1 for true)

[[param_int]] (a second param you wish to edit)
key = "string" 
value = int 

[[param_float]] (you can also get rid of this category if you are not editing floats. Same goes for ints)
key = "string"
value = float
```