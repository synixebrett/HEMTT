# Making a dag out of deps

## Runtime DAG

Which mods can I split up into separate @ folders?

#. Preprocess all config.cpp files: `armake2 preprocess`
    #. extract CfgPatches name
    #. extract requiredAddons - can `armake2` help with this? Otherwise look at https://github.com/acemod/ACE3/blob/master/tools/extract_dependencies.py 
#. Build up graph of CfgPatches name <-- requiredAddons
    #. Arrow shows flow of code
#. Export as DOT
    #. Maybe show as ASCII if it's easy

## Compile-time DAG

Maybe at some point in the future make a compile-time DAG that tracks all the `#include`s so that HEMTT knows what to recompile? Check how quick preprocessing is - might be worth just doing this every single time? Disable with a flag.
