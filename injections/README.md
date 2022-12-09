injecthandles - started out as something totally different, but whatever, its a process injection with some stealthy enum. It currently uses CreateRemoteThread for execution, but I have plans to modify that to QueueUserAPC and process snapshotting, maybe setthreadcontext too, i dunno. Modify to suit your needs.

injectwindow - uses EnumWindows to quietly enum processes for injection. Injection is currently also basic. Modify as needed.
