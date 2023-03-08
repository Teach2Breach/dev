**refldump** - <br>A lot of Rust code is taken from https://github.com/postrequest/safetydump. I'd like to thank postrequest for this tool and the whole https://github.com/postrequest/link framework. Both were invaluable in helping me learn Rust and end-point exploitation.
           The additions I made on this version are based on https://github.com/hasherezade/pe-sieve/blob/master/utils/process_reflection.cpp. 
           Basically, we use process reflection to obtain a handle for a cloned lsass process and then call minidumpwritedump with the handle to the cloned process. This is very similar to my other verions that either use PSSCaptureSnapshot or CreateToolHelp32Snapshot, to avoid calling minidumpwritedump with the original process handle.<br>
           <BR>As of writing, this tool is not prevented by Windows Defender on latest Win10 build. *If you have issues with other EDR preventions, try self-signing the executable.* This version is not currently working with Win11. I think I know the problem, so I'll revisit and push a Win11 compatible version at some point.
  
  **usage**: 
  <BR>`git clone https://github.com/Teach2Breach/dev.git`<BR>
  `cd .\dev\rust\lsass\refldump\`
  <Br>compile with Rust:<BR> `cargo build --release`
  <BR>run the binary on target machine:<BR> `.\refldump.exe`
    <BR>
      a randomly named .bin file will be generated with the dump. It is base64 encoded. I recommend you move this file to a remote linux host for decoding as follows:
      <BR>`base64 -d dsaifluk.bin > dump.bin`<BR>
        `pypykatz lsa minidump dump.bin`<BR>
        
            
