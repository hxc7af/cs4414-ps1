Title: Problem Set 1 Answers
Author: Harriet Cao

PROBLEM 1:

User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux i686; rv:23.0) Gecko/20100101 Firefox/23.0

This string tells us of some of the information about the user's client that is sent to the server. "Mozilla/5.0" indicates Mozilla compatibility. The information in the parentheses identifies the browser's system. "Gecko/20100101" is the browser platform, and "Firefox/23.0" should be the browser and version.

PROBLEM 2:

It is likely that many functions will try to simultaneously access this global variable. If they are all changing and referencing this variable at the same time, it is very possible that one function will try to use it as another function also uses or changes it; then the first function may end up using the wrong value of the variable, or change it in some way that will make it an incorrect variable for another function.
