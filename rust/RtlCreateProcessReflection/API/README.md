This version uses published Rust crates to perform normal API calls for RtlCreateProcessReflection, without having to define everything ourselves (which we will do in /direct/). This is the easiest and most straightforward way to perform API calls in Rust, in my opinion. I think it is good to start here, with a new idea, and once it all works in this way, you can start to remove the crate dependencies and define everything yourself, or dynamically invoke, etc...
<BR>
  <BR>
    Example Usage:<BR>
    <BR>
    ![image](https://user-images.githubusercontent.com/105792760/223741063-7be59c47-6afc-41e9-b2c9-52f845c580ba.png)<BR>
I put in a couple pauses in the execution to give the user a chance to open tasklist and view the process information and verify output. Enjoy
