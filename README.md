# dev
maldev obviously

please note - most of the tools in this repo are not completed in a way that they are meant to be deployed on operations. For example, pthandles currently enumerates EVERY process handle and thread handle on a system. You probably would want to target a specific process, etc... on an op. So please review the code, use the program and understand what it does, then snip out the code you need, or modify the tool to fit your use-case. These "tools" should be basically known good code snippets for accomplishing certain malware-dev related tasks. 

Think of it this way. Let's say you want to work on a process hollowing or new process injection targeting Windows x64. Well are you going to write all the boilerplate and rewrite OpenProcess for the nth time, or try to remember what's a sneaky way to get the handle you want and how to setup the pointer to the struct to pass to the.... NOOOOO! You come here and look in the enum folder and choose which method you want to use for enumerating handles, and just snip the code you want. Thank Christ.
