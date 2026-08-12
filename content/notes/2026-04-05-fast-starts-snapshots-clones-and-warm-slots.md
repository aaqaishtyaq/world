+++
title = "Fast Starts: Snapshots, Clones, and Warm Slots for MicroVMs"
description = "How snapshots, clones, and warm slots are used to minimize microVM startup time."
date = "2026-04-05"
draft = false

[taxonomies]
series = ["MicroVM Platform"]
+++
One of the things I care about most in a compute platform is startup time.

People say they want fast machine starts.

What they usually mean is they do not want to wait while your platform does cold work right in front of them.

That distinction matters a lot.

If you create a machine from scratch on demand, you are paying for a bunch of slow steps in public:

- prepare a root filesystem
- create overlays
- set up networking
- boot or restore Firecracker
- wait for the guest agent

Even when each step is pretty fast, the total is not.

Here is a rough way to think about that budget:

```
rootfs prep: hundreds of ms to seconds
network setup: tens to hundreds of ms
boot or restore: hundreds of ms to seconds
guest readiness: hundreds of ms to seconds
```

None of those numbers are scary on their own.

Stack them together on the request path and they stop feeling cheap.

## Cold create is not the enemy

Cold create is fine. You need it. It is the fallback path and the general path.

The mistake is pretending cold create can also be your instant path.

If you care about interactive workloads, dev environments, agent sandboxes, or bursty compute, you need a different shape:

- prepare ahead of time
- reserve capacity before doing expensive work
- activate something already close to ready

That is where snapshots, clones, and warm slots come in.

In this post, I want to walk through how I think about those three paths.

## Three levels of speed

I like to think about machine startup in three buckets.

### 1. Fresh boot

This is the full path. Pull image. Build filesystem. Boot guest. Wait for readiness.

It is the most flexible and the slowest.

### 2. Clone from a prepared source

This path reuses a known-good base, often through snapshot or overlay tricks. It cuts a lot of setup time, but there is still work to do.

It is much better, but not instant.

### 3. Warm activation

This path claims something already prepared and nearly ready.

That is the path that can feel snappy.

Here is the rough picture:

```
fresh create:
  image pull -> rootfs prep -> network -> boot -> ready

clone:
  prepared base -> overlay/snapshot -> network -> restore -> ready

warm activation:
  claim ready slot -> attach identity -> expose -> run
```

The third path is where good latency lives.

## Why reservation matters

One easy mistake is to fan out real work to multiple nodes and let the fastest winner keep running.

That looks clever until you notice the waste.

Two or three nodes may do expensive rootfs and startup work for one user request. The losers then roll back after burning CPU, IO, and time.

That is not a fast path. It is a messy betting strategy.

A better model is:

1. rank nodes
2. reserve one node
3. commit cold work to that node only
4. use warm activation when an exact match already exists

This keeps the fast path honest.

## The importance of exact match

Warm pools sound great until you make them fuzzy.

If a warm slot is only "kind of close" to what the user asked for, activation turns into mutation, and mutation turns back into cold work.

So the identity of a warm slot needs to be strict:

- image digest
- machine shape
- restore capability
- any other setup that changes startup cost or correctness

That lets the scheduler ask a sharp question:

"Do I already have a prepared thing that exactly matches this request?"

If yes, activate it.

If no, reserve a node for cold work and move on.

## What breaks warm starts

Warm starts stop being warm pretty quickly when one of these shows up:

- the image digest does not match
- the machine shape does not match
- the prepared state is stale
- the guest still has expensive init work left to do

That is why I do not like fuzzy warm capacity. Either it is a real ready slot for this request, or it is not.

## Speed without lying

The biggest risk in performance work is cheating.

You call something fast because one internal benchmark looked nice, but the real path still hides slow work in the same request.

I do not want "fast" to mean:

- it was cached on one node once
- the loser nodes cleaned up eventually
- we skipped a correctness check
- the guest was not actually ready yet

I want it to mean the user asked for a machine and got a usable machine quickly, without hidden waste and without crossed fingers.

## The user-facing payoff

When this is done well, the platform feels different.

It stops feeling like "please wait while we build your environment" and starts feeling closer to "your environment is here."

That opens the door for better product shapes:

- on-demand dev machines
- ephemeral test environments
- short-lived AI sandboxes
- burst compute for jobs that should start now

Those experiences depend less on the hypervisor and more on how smart the control plane is about prepared capacity.

## The real challenge

The hard part is not building one fast restore path.

The hard part is managing the whole life around it:

- how many warm slots to keep
- which image digests deserve them
- when to recycle them
- how to avoid stale prepared state
- how to represent ready slots to the scheduler

That is why fast activation is a control-plane problem as much as a Firecracker problem.

## The simple summary

Cold create is necessary.

Clone paths are useful.

Warm activation is what changes the feel of the product.

And if you want warm activation to work well, you need more than a snapshot trick. You need clear reservation semantics, exact identity, and a scheduler that knows the difference between "this node has free CPU" and "this node has a prepared machine I can claim right now." That difference is where a lot of the speed comes from. Fast start is mostly about moving slow work off the user path.
