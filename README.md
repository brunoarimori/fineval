# fineval
Evaluate operations on a file to help me track finances.

## CLI command
`$ fineval ./fin.log`

## File format
```
/* ---------------------------------
* COMMENT TEST
* ------------------------------- */

!section>

TEST ITEM ONE
-100
-100 tagexample 
+1300 tagexample
= $ #item1

TEST ITEM TWO
+300 tagexample 
-200 tagexample 
-1,000,000
= $ #item2

TEST ITEM THREE
$ [item1]
$ [item2] tagexample
+2,000,000 tagexample 
= $ #item3

!section<
```

- SECTION_START is !X> where X is an alphanumeric character up to length 8
- SECTION_END is !X< where X is an alphanumeric character up to length 8
- TITLE is an alphanumeric string preceded by an empty line up to length 32
- LINE is a string that
  - is preceded by a TITLE or a LINE
  - can be a CONST_LINE or VAR_LINE
  - has a VALUE followed by a MARK and/or a TAG (space separated)
  - MARK is an alphanumeric string up to length 16 that is surrounded by brackets
  - TAG is an alphanumeric string up to length 16
  - VALUE is a string that
  - can be a CONST_VALUE or a VAR_VALUE
  - CONST_VALUE starts with "+" or "-" followed by a comma-separated number up to 1,000,000,000
  - VAR_VALUE is a "$" necessarily followed by a MARK an optionally by a TAG
- RESULT is a string that
  - is preceded by a LINE
  - can be an CONST_RESULT or an UNPARSED_RESULT
  - starts with a "=" followed by a space and a CONST_VALUE or a RESULT_VALUE
    - a RESULT_VALUE is a "$" necessarily follower by a LABEL
    - a LABEL is an alphanumeric string up to length 16 that starts with "#"
- ITEM is a TITLE followed by up to 255 LINE(s) followed by a RESULT
	  
comments are c-style

## Observations

- a LINE is a CONST_LINE if it has a CONST_VALUE
- a LINE is a VAR_LINE if it has a VAR_VALUE
- a CONST_LINE with a MARK used to be a VAR_LINE before the last fineval process
- a RESULT with a CONST_VALUE is a CONST_RESULT
- a RESULT with a RESULT_VALUE is an UNPARSED_RESULT
- a MARK necessarily maps to a LABEL, a VAR_LINE always maps to a RESULT
- a LABEL can map to multiple MARKs
- the number of lines is the same after a fineval process
