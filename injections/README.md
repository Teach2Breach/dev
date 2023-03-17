injecthandles - started out as something totally different, but whatever, its a process injection with some stealthy enum. It currently uses CreateRemoteThread for execution, but I have plans to modify that to QueueUserAPC and process snapshotting, maybe setthreadcontext too, i dunno. Modify to suit your needs.<BR><BR>

injectwindow - uses EnumWindows to quietly enum processes for injection. Injection is currently also basic. Modify as needed.<BR>

snapinject - Clones a target process (clone is created by PSSCaptureSnapshot in a suspended state), then we inject the clone process with our shellcode. The idea was to perform a remote process injection without using CreateProcess or something ususal to spawn a sacrificial process. 
