#!/bin/sh
# The test suite of the sandbox: passes when greet.txt says hello.
grep -q hello greet.txt
