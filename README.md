## Rux, a microkernel written in Rust

Rux is a hobbyist microkernel written in Rust, featuring a
capability-based system similar to [seL4](https://sel4.systems/).

- [Repository](https://source.that.world/diffusion/RUX/)
- [Documentation](https://that.world/~docs/rux/kernel/)

## Overview

Rux's goal is to become a safe general-purpose microkernel. It tries to
take advantage of Rust's memory model -- ownership and lifetime. While
the kernel will be small, unsafe code should be kept minimal. This makes
updating functionalities of the kernel hassle-free.

Rux uses a design that is similar to seL4. While there won't be formal
verification in the short term, it tries to address some design issues
of seL4, for example, capability allocation.

## Quickstart

Currently, due to packaging problem, the kernel is only tested to
compile and run on Linux with `x86_64`. Platforms with qemu and compiler
target of `x86_64` should all be able to run this kernel, but do it at
your own risk.

To run the kernel, first install `Rust`, `qemu`, and cross-compiled
GNU's `binutils`. The easiest way to do it is through the `shell.nix`
file provided in the source code. Install [Nix](http://nixos.org/nix/),
then go to the source code root and run the following command:

```bash
nix-shell
```

After that, run:

```bash
make run
```

You should see the kernel start to run with a qemu VGA buffer. The
buffer, after the kernel successfully booted, should show a simple
command-line interface controlled by `rinit` program launched by the
kernel. Several commands can be used to test things out.

```bash
echo [message]
```

Echo messages and print them back to the VGA buffer.

```bash
list
```

Print the current `CPool` slots into the kernel message buffer.

```bash
retype cpool [source slot id] [target slot id]
```

Retype an Untyped capability into a CPool capability. `[source slot
id]` should be a valid slot index of an Untyped capability. `[target
slot id]` should be an empty slot for holding the retyped CPool
capability.

## Source Code Structure

The development of Rux happen in the `master` branch in the source code
tree. The kernel resides in the `kernel` folder, with platform-specific
code in `kernel/src/arch`. For the `x86_64` platform, the kernel is
booted from `kernel/src/arch/x86_64/start.S`. The assembly code them
jumps to the `kinit` function in `kernel/src/arch/x86_64/init/mod.rs`.

After the kernel is bootstrapped, it will initialize a user-space
program called `rinit`, which resides in the `rinit` folder. The
user-space program talks with the kernel through system calls, with ABI
defined in the package `abi`, and wrapped in `system`.

## Kernel Design

### Capabilities

Capabilities are used in kernel to manage Kernel Objects. Those
Capabilities are reference-counted pointers that provide management for
object lifecycles.

Capabilities in user-space can be accessed using so-called `CAddress`,
refered through the root capability of the user-space task. This helps
to handle all permission managements for the kernel, and thus no
priviliged program or account is needed.

Current implemented capabilities are:

- Untyped memory capability (UntypedCap)
- Capability pool capability (CPoolCap)
- Paging capability
  - PML4Cap, PDPTCap, PDCap, PTCap
  - RawPageCap, TaskBufferPageCap
  - VGA buffer
- CPU time sharing capability (TaskCap)
- Inter-process communication capability (ChannelCap)

#### Example: Initialize a New Task

This example shows how to initialize a new task using the capability
system.

- Create an empty TaskCap.
- Create an empty CPoolCap.
- Initialize paging capabilities (One PML4Cap, Several PDPTCap, PDCap,
  PTCap and RawPageCap)
- Assign the stack pointer in TaskCap.
- Load the program into those RawPageCap.
- Assign the PML4Cap to TaskCap.
- Assign the CPoolCap to TaskCap.
- Switch to the task!

#### Implementation

Implementing reference-counted object is a little bit tricky in kernel,
as objects need to be immediately freed, and all weak pointers need to
be cleared after the last strong pointer goes out. Rux's implementation
uses something called `WeakPool` to implement this. The original
reference counted object (called `Inner`), form a double-linked list
into the nodes in multiple WeakPools.

### Capability Pools

Capability Pools (or `CPool`) are used to hold multiple capability
together. This is useful for programs to pass around permissions, and is
essential for `CPool` addressing. In implementation, capability pools
are implemented as a `WeakPool`.

### Tasks

A task capability has a pointer to a capability pool (the root for
`CPool` addressing), a task buffer (for kernel calls), and a top-level
page table. When switching to a task, the kernel switches to the page
table specified.

The `switch_to` function implemented uses several tricks to make it
"safe" as in Rust's sense. When an interrupt happens in userspace, the
kernel makes it as if the `switch_to` function has returned.

In kernel-space, interrupts are disabled.

### Channels

Tasks communicate with each other through channels. A channel has a
short buffer holding messages sent from a task, and will respond this to
the first task that calls `wait` on the channel.