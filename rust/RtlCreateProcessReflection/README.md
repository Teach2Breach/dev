Rust implementations of RtlCreateProcessReflection, which allows us to clone a process and then work with that cloned process, without making any other calls to the original (target) process. This is implemented in one of the lsass dumpers in this same repo. There are other potential use cases as well, such as process injection. The cloned process is spawned in a suspended state, but you have full control over it... Anyhow, I'll be doing 3 versions:
<BR><BR>
**API** - uses published Rust crates to simply perform the API calls. Typical API call usage.<BR>
**direct** - load the needed library and locate the function. define structs and types ourselves<BR>
**dinvoked** - use https://github.com/Kudaes/DInvoke_rs<BR>
<BR>
It's for my learning and yours. This program isn't meant to be deployed as is. It's meant to teach you how to perform this specific task as a step in a larger piece of malware. Be responsible.
