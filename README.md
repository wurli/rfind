# rfind

A simple command line utilty for finding an R executable on the current 
machine, if one exists. Works by:

*   First checking the `PATH` environmental variable
*   Then checking the `R_HOME` environmental variable, if it is set 
*   Then checking a number of common locations (on Mac, Linux, and Windows)

## Motivation

`R.exe` is commonly not found on the user's `PATH`, which can can be a pain
if you need to call R from another program, e.g. from Python. This utility
copies the method used by the RStudio IDE for finding an R executable.


