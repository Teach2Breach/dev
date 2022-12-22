This is WIP.

**Updates 12/22/22 -** got all the windows error codes 0-1022 added. Moved the error codes to a header file to be used with other tools. The main.c program now is merely to serve as an example of how to use the header file in your own tools.

The idea is to define all the windows sytem error codes in a struct and have some logic to handle converting any error codes retrieved by GetLastError, in your other tools, into human readable format with appropriate error messages for rapid debugging sessions. I'll eventually make it into a header file that can be included in any other tool. I'm also doing a version in Rust for Rust tool dev.

Currently causing an error by attempting to open a file that doesn't exist, and checking that error against our internally defined struct and printing to us the corresponding message. And I've got more error codes to add still.

![image](https://user-images.githubusercontent.com/105792760/209018867-333654c5-a922-420d-bc32-899806876a14.png)

And in the below example, I added some logic to run closehandle on an arbitrary int and cause an error 6:
![image](https://user-images.githubusercontent.com/105792760/209019565-b8cee28c-f43a-49e2-bff3-64902be99820.png)

