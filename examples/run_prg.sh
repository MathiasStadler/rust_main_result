#!/bin/bash

## FROM HERE
## https://www.cyberciti.biz/faq/linux-bash-exit-status-set-exit-statusin-bash/

cargo run --example main_result_3

# store exit status of grep
# if found grep will return 0 exit status
# if not found, grep will return a nonzero exit status
status=$?
 
if test $status -eq 0
then
	echo "OK => $status. "
else
	echo "Err => $status. "
fi