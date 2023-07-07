# lib_paramconfig

**For end-users, go to the [releases tab](https://github.com/CSharpM7/lib_paramconfig/releases) and select the latest version. For developers, keep reading.**


A common problem across "single slot movesets"(SSMs) is that because they often hook the same functions, they will cause crashes when multiple SSMs are active at once. This plugin serves as a middleman between the source code and mods, so that even though multiple SSMs are active, this is the only plugin that actually hooks param functions.

## Dependencies
[lib_nrohook](https://github.com/ultimate-research/nro-hook-plugin/releases) (install in your plugins folder, all end-users will need this as well)

## Caution!
This can have adverse affects in casual modes, as well as with other external mechanics that attempt to edit parameters. This assumes you, and your target audience, are willing to risk the possibility of unintended consequences regarding single slot movesets. This is also not compatible with any other gameplay modifying plugin that hooks param functions. Please make sure to disable this plugin before using such other mods/plugins.

## Usage
All end-users will need to download an updated `libparam_config.nro` from the releases page (please do not bundle with your release as this plugin is still in its infancy). Create a file called `config_param.toml` and place it in the root of your mod (feel free to copy the one provided in the source code). Fill out the toml with the fighter kind of your mod, the slots that will be affected, and the common parameters you wish to modify. I recommend testing this first before distributing and updating your mods.

## Limitations
This also can only change `int`, `float`, and `bool` values. For bools, please use a `param_int` object with a value of 0 or 1.
Currently, this does not support editing all parameters featured in `vl.prc` files (namely parameters with several arrays like MinMin's arms), though it does support most of the basic ones. Information like ledgegrab box has gone untested, as well as several other `vl.prc` parameters.

## Toml Layout

```
kind = "mario" (this should be whatever comes after FIGHTER_KIND_. Ie "MARIO" or "mario")
slots = [0,1,...] (a list of slots (alt costume numbers) to be affected)

[[param_int]]
param = "param_fireball" (the cracked hash name found in the fighter_param file)
subparam = "life" (the cracked hash of the subcategory of this param. It usually includes things like "life" or "angle")
value = 99 

[[param_int]] (a second param you wish to edit)
param = "wall_jump_type" 
subparam = "" (note for fighter attributes, you often want to leave this blank)
value = 0 (if you are working with a bool, use 0 for false, and 1 for true)

[[param_float]] (you can also get rid of this category if you are not editing floats. Same goes for ints)
param = "jump_y"
subparam = ""
value = 60.0
```