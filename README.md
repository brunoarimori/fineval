fineval

# command line interface
fineval FILE [SECTION] 

$ fineval ./fin.log
$ fineval ./fin.log 1222

# file format

SECTION_START is !X> where X is an alphanumeric character up to length 8
SECTION_END is !X< where X is an alphanumeric character up to length 8
TITLE is an alphanumeric string preceded by an empty line up to length 32
LINE is a string that
  is preceded by a TITLE or a LINE
  can be a CONST_LINE or VAR_LINE
  has a VALUE followed by a MARK and/or a TAG (space separated)
  MARK is an alphanumeric string up to length 16 that is surrounded by brackets
  TAG is an alphanumeric string up to length 16
  VALUE is a string that
    can be a CONST_VALUE or a VAR_VALUE
	  CONST_VALUE starts with "+" or "-" followed by a comma-separated number up to 1,000,000,000
	  VAR_VALUE is a "$" necessarily followed by a MARK an optionally by a TAG
RESULT is a string that
  is preceded by a LINE
  can be an CONST_RESULT or an UNPARSED_RESULT
  starts with a "=" followed by a space and a CONST_VALUE or a RESULT_VALUE
    a RESULT_VALUE is a "$" necessarily follower by a LABEL
	  a LABEL is an alphanumeric string up to length 16 that starts with "#"
ITEM is a TITLE followed by one or up to 255 LINE(s) followed by a RESULT
	  
comments are c-style

# observations

a LINE is a CONST_LINE if it has a CONST_VALUE
a LINE is a VAR_LINE if it has a VAR_VALUE
a CONST_LINE with a MARK used to be a VAR_LINE before the last fineval process
a RESULT with a CONST_VALUE is a CONST_RESULT
a RESULT with a RESULT_VALUE is an UNPARSED_RESULT
a MARK necessarily maps to a LABEL, a VAR_LINE always maps to a RESULT
the number of lines is the same after a fineval process

# routine

### if SECTION argument is present
validate SECTION format from argument
open the file
execute the full section routine with target
  read file line by line until a matching SECTION_START is found
  save the SECTION_START line number
  continue reading line by line and save everything in a buffer until SECTION_END is found
  save the SECTION_END line number
  pass a pointer to the saved section buffer to the section parse routine
  use the SECTION_START and SECTION_END line numbers to write the parsed section buffer to file
close the file

### if no SECTION argument
open the file
execute the full section routine without target
  start reading the file from last end of section or 0
  read file line by line until any SECTION_START is found
  save the SECTION_START line number
  continue reading line by line and save everything in a buffer until SECTION_END is found
  save the SECTION_END line number
  pass a pointer to the saved section buffer to the section parse routine
  use the SECTION_START and SECTION_END line numbers to write the parsed section buffer to file
  save the SECTION_END line number in last end of section

### Section parse routine
LINE
  value
  mark
  tag
RESULT
  value
  label
DEP
  line
  mark
ITEM
  line_start
  title
  DEP[]
  LINE[]
  RESULT

##### first pass
read buffer line by line
build array of ITEMs with DEPs

##### second pass
do depth-first-search starting from ITEM[0]

it has dependencies and nodes
it is a directed graph, because dependencies have only one side
https://algs4.cs.princeton.edu/42digraph/
https://www.tutorialspoint.com/data_structures_algorithms/depth_first_traversal.htm
https://www.cs.usfca.edu/~galles/visualization/DFS.html

# Plan
- section read routine
- item eval routine
- section parse routine
- section write routine

# Notes
see if tarjan instead of DFS is better (for small and large files)
see if you can apply concurrency to speed up parsing
use debugging tools
