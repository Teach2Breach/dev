In this version of RtlCreateProcessReflection, we use LoadLibraryW and GetProcAddress to dynamically resolve the API calls we want to make for cloning a process, as opposed to using Rust crates and making typical API calls (like in /API/ in this repo). 
<BR><BR>
Please read the code and understand what it does. Note that I have purposely not added all the various RE-resistance, obfuscation and other OPSEC-friendly things. You can add all the bells and whistles for code you intend to deploy. This is meant to be an example for helping with maldev tasks.
<BR><BR>
  Usage:
  <BR><BR>
    ![image](https://user-images.githubusercontent.com/105792760/223751870-0772e309-b3ed-4b18-bc17-d2059004262f.png)

<BR>
This version is still set up to clone the current process. It can be modified to target a remote process.
