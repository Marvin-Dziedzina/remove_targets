# Remove Targets

This program will remove all target directories recoursively safely with ```sh cargo clean```.

Your disk is always full?

You don't want to keep all of the target directories of your "I'll do it later" rust projects?

This program is right for you!

It cleans all child directories!

Just tell it where to start and it will do it's job well.

# Usage

This will clean your full `Documents` directory from target dirs:
```sh
remove_targets ~/Documents/
```

This one is for Windows:
```sh
remove_targets.exe "%USERPROFILE%\Documents\"
```
