# x86-litmus

Model-checks x86-TSO litmus test, in [TLC style](https://lamport.azurewebsites.net/tla/tla.html).

For x86-TSO, see [x86-TSO: A Rigorous and Usable Programmer’s Model for x86 Multiprocessors](https://www.cl.cam.ac.uk/~pes20/weakmemory/cacm.pdf) for details.

## Example Litmus

See [sb.rs](tests/sb.rs), corresponding to `sb` example in above paper, which states two concurrent CPUs performing these two snippets,

```asm
mov [x], $1
mov eax, [y]
---
mov [y], $1
mov ebx, [x]
```

should allow eax and ebx on each processor both equal to 0 due to the store buffer on x86 architecture.
