# lib_paramconfig (nro)

**For end-users, go to the [releases tab](https://github.com/CSharpM7/lib_paramconfig/releases) and select the latest version. Make sure to disable the plugin when using other mods that hook param functions. For developers, keep reading.**

A common problem across "single slot movesets"(SSMs) is that because they often hook the same functions, they will cause crashes when multiple SSMs are active at once. This plugin serves as a middleman between the source code and mods, so that even though multiple SSMs are active, this is the only plugin that actually hooks param functions.

## Dependencies
[lib_nrohook](https://github.com/ultimate-research/nro-hook-plugin/releases) (install in your plugins folder, all end-users will need this as well)

## Caution!
This can have adverse effects in casual modes, as well as with other external mechanics that attempt to edit parameters. This assumes you, and your target audience, are willing to risk the possibility of unintended consequences regarding SSMs, or have mitigated some of these possible unintended effects. This is also not compatible with any other gameplay modifying plugin that hooks param functions. Please make sure to disable this plugin before using such other mods/plugins.

## Usage
All end-users will need to download an updated `libparam_config.nro` from the releases page (please do not bundle with your release as this plugin is still in its infancy). Create a file called `config_param.toml` and place it in the root of your mod (feel free to copy the one provided in the source code). Fill out the toml with the fighter kind of your mod, the slots that will be affected, and the common parameters you wish to modify.

## Limitations
- This also can only change `int`, `float`, and `bool` values. For bools, please use a `param_int` object with a value of 0 or 1.
- Not all params will work (usually floats).
- Currently, this does not support editing all parameters featured in `vl.prc` files (namely parameters with several arrays like MinMin's arms), though it does support most of the basic ones. Changing information like ledgegrab boxes and hurtboxes has gone untested (ledgegrab boxes can be altered via smashline), as well as several other `vl.prc` parameters.
- Does not work with pocketed items or copy abilities

## Toml Layout

```
kind = "mario" #(~OPTIONAL~ The default kind for this toml. Any param that doesn't have a `kinds` value will default to this. This should be whatever comes after FIGHTER_KIND_. Ie "MARIO" or "mario")
slots = [0,1,...] #(~OPTIONAL~ The default list of effected slots (alt costume numbers). Any param that doesn't have a `slots` value will default to this list)

[[param_int]]
param = "param_fireball" #(the cracked hash name found when viewing in prceditor)
subparam = "life" #(the cracked hash of the subcategory of this param. It might includes things like "life" or "angle")
value = 99 

[[param_int]] #(a second param you wish to edit)
param = "wall_jump_type" 
subparam = "" #(note for fighter attributes, you often want to leave this blank)
value = 0 #(if you are working with a bool, use 0 for false, and 1 for true)

[[param_float]] #(you can also get rid of this category if you are not editing floats. Same goes for ints)
param = "0x06a0d82dad" #(you can also use the raw hash version, as long as it begins with 0x)
subparam = ""
value = 60.0

[[param_float]]
param = "param_fireball"
subparam = "gravity_accel"
value = 0.0
kinds = ["mario","mario_fireball"] #(~OPTIONAL~ For weapons, I recommend including their weapon kind in the list as well)
slots = [0,1,2] #(~OPTIONAL~)

[[param_int]]
param = "article_use_type" #(For changing the use type of an article. Usually used for allowing entry/victory articles to spawn in game)
value = 1
kinds = ["mariod_capsuleblock"]
```

# lib_paramconfig

lib_paramconfig also supports changes via your own skyline plugin.

## Setup
Your `Cargo.toml` needs to include this dependency:
`param_config = { git = "https://github.com/csharpm7/lib_paramconfig.git"}`

## Usage
```
param_config::update_int_2(kind: i32, slots: Vec<i32>,index: (u64,u64),value: i32);
param_config::update_float_2(kind: i32, slots: Vec<i32>,index: (u64,u64),value: f32);
```
Kind: Fighter/Weapon kind, as commonly used like `*FIGHTER_KIND_MARIOD`. If it's a weapon, use a negative number.

Slots: Vector Array of slots

Index: (hash40(""),hash40("")) for param/subparam hashes. For common params, the second argument should be 0.

Value: Value for the param
