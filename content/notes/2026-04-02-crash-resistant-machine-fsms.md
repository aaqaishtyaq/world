+++
title = "Why a MicroVM Platform Needs a Crash-Resistant Machine FSM"
date = "2026-04-02"
draft = false

[taxonomies]
series = ["MicroVM Platform"]
+++
While working on the worker side of the platform, one thing became obvious pretty quickly: machine lifecycle code looks easy only when you test happy paths.

Create a machine. Start it. Stop it. Delete it.

That idea falls apart the first time a host reboots halfway through startup.

Now you have harder questions:

- Was the VM process created?
- Did networking finish?
- Did the root filesystem get prepared?
- Did the API return before the host died?
- Should restart continue, roll back, or wait?

You cannot answer those questions with request handlers and hope.

That is why I keep coming back to a durable machine FSM.

In this article, I want to explain why I think this matters so much.

## The problem with one-shot handlers

A normal HTTP handler wants to do work and return. That is fine for small things. It is not fine for machine lifecycle.

Machine lifecycle is long, messy, and full of partial failure. It has steps that touch:

- local storage
- network namespaces
- cgroups
- Firecracker process state
- proxy registration
- DNS registration
- snapshots
- cleanup

If each endpoint owns its own little workflow, recovery gets scattered everywhere. One path retries here. Another path cleans up there. A third path adds a special boot-time scan to heal the leftovers.

You end up with a system where every bug fix adds one more side lane.

## What the FSM gives you

The useful part of an FSM is not the word "state machine." The useful part is discipline.

You write down:

- the current durable state
- the desired next state
- the allowed transitions
- the work for each transition that is safe to run twice
- what to do when the process comes back after a crash

That forces the system to stop improvising.

I like to think of it this way:

```
request handler:
  validate input
  record desired change
  wake machine run

machine run:
  inspect durable state
  inspect what is really on the host
  do the next safe step
  persist progress
  repeat until steady
```

That split is boring, which is exactly why it works.

## The key idea: one main machine run

The clean model is one main run per machine.

Not one FSM for create, another for stop, another for cleanup, and a startup reconciler that tries to guess what the others meant.

One machine run.

That run owns the job of moving the machine toward its desired state.

It can decide:

- should this machine be started?
- does it need cleanup?
- is the VM still alive?
- did the last transition finish?
- do we need to recover from a crash?

This is the shape:

```
desired state + durable record + observed reality
                  |
                  v
            +-----------+
            | machine   |
            |   run     |
            +-----------+
                  |
      +-----------+------------+
      |           |            |
      v           v            v
   create      start        destroy
   network     wait         cleanup
   rootfs      recover      release
```

That machine run becomes the adult in the room.

## A real failure case

Here is the kind of failure that changed how I think about this:

```
network setup succeeds
Firecracker starts
guest never becomes ready
host reboots
machine run resumes
system decides whether to retry, wait, or destroy
```

If your lifecycle model cannot explain that sequence cleanly, it is going to leak mess all over the node.

## Safe retries are not optional

The most important phrase in lifecycle code is not "fast." It is "safe to run twice."

If the host crashes after setting up a tap device but before marking the step complete, you need to be able to run that step again without making things worse.

Same for:

- creating directories
- allocating leases
- registering routes
- restoring snapshots
- deleting leftovers

The system should be able to ask, "What is true right now?" and then move one safe step forward.

If your lifecycle logic depends on perfect memory of the previous process, it will fail in production.

## Cleanup has to be first-class

A lot of systems treat cleanup like an apology at the end.

That is backwards.

Cleanup is part of the lifecycle. It needs the same level of care as create.

When a machine dies badly, the leftovers are not random junk. They are real resources:

- disk overlays
- snapshots
- proxy routes
- IP allocations
- namespaces
- DNS records
- leases

If cleanup is scattered across handlers, deferred callbacks, and boot-time repair code, it will drift.

When cleanup is owned by the same machine run that owns creation, the system gets simpler. There is one place that knows what the machine owns and one place that can release it safely.

## Why this matters for user-facing latency

This is not just a correctness story. It affects speed too.

A good FSM lets you separate cold work from fast work.

It lets you:

- prepare assets ahead of time
- resume interrupted work instead of starting over
- expose exact lifecycle state to the control plane
- measure where time really goes

Without that, every slow start looks the same from the outside. You have no idea whether the machine was waiting on rootfs prep, network setup, guest boot, or cleanup from a previous failure.

## What I watch for now

When I read lifecycle code, I ask a few blunt questions:

- Is there one owner of recovery and cleanup?
- Can it resume after a crash?
- Are transitions explicit?
- Is cleanup part of the same model?
- Can every step run twice safely?

If the answer is no, the code may still work on a laptop. It will not stay honest under load.

For me, that is the real value of the machine FSM. It gives the platform a way to explain half-finished work instead of just leaving a mess behind. If a machine lifecycle cannot resume after a crash, it is not finished.
