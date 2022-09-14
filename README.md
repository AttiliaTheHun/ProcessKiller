# ProcessKiller
A simple Rust program for Windows that kills specified processes
# config
Configuration file (called simply "config") where you can specify the processes you want to kill, whether it is supposed to be a one-time purge or a start-up prevention and the interval of the checks.<br>
<br>
**Make sure the file is in the same directory as the `.exe`!**
## Add process
Simply add a line that obliges this format: `NAME=MYPROCESSNAME` where `MYPROCESSNAME` is obviously name of the process you want to kill.<br>
<br>
You can add as many of these as desired.
## Start-up prevention
If you want the program to periodically check if the target processes are running and kill them if so, make sure the file contains a line like this: `AUTOSTOP=false`.<br>
<br>
If `true`, the program will perform the purge and then finish. Otherwise it will go on infinitely killing spurious processes
### Changing the interval
Make sure the file contains a line like this: `INTERVAL=600` and by changing `600` to something else you will change the interval.<br>
<br>
You can add as many of these as desired, but only the last one will make a difference.
## Fuck! How do I stop it now...
If the program is running in the start-up prevention mode and you want to close it, you can simply run the `stop.bat` script to get it done for you.
# How to build
This is gonna hurt, unless you have [the Rust compiler](https://www.rust-lang.org/tools/install) already installed.<br>
<br>
As soon as you have it (let's say a week or two of birth-giving-like pain), navigate to the same directory as the files and run `cargo build`.<br>
<br>
Somewhere under the `target` folder you will find your new rusty `.exe`. Make sure to create a proper `config` file here and you're good to go!
