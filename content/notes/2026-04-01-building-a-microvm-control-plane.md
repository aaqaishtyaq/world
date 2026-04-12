+++
title = "Building a MicroVM Control Plane in Public"
date = "2026-04-01"
draft = false

[taxonomies]
series = ["MicroVM Platform"]
+++
Over the past couple of years, I have been spending a lot of time with Firecracker, microVMs, and a distributed compute platform I have been building.

It started with a simple goal: run Firecracker on a machine I control and understand every moving part.

That sounds small, but it really is not.

The first few steps are easy enough to explain. Boot a VM. Get KVM working. Start Firecracker. Launch a guest. Once that works, the next question shows up right away: what if this was not one machine, but a fleet?

That is where the real work starts.

Running one microVM is a neat trick. Running thousands means you need a control plane. You need placement, state, networking, ingress, observability, cleanup, and recovery when the host does something rude like reboot in the middle of a machine start.

In this post, I want to set up the rest of the series and talk about that jump.

## The shape of the system

Over time, the repo turned into a small compute platform with a few clear pieces:

- a regional control-plane service decides where work should go
- a worker daemon runs machines on worker hosts
- an image service prepares root filesystems from OCI images
- an ingress proxy handles public traffic, TLS, and SSH access
- an internal DNS service gives private names to machines
- a host agent owns host-level changes and node health
- a guest agent runs inside the guest and brings the machine to life

At a high level it looks like this:

```
                public API
                    |
                    v
             +--------------+
             | regional API |
             | placement    |
             | auth         |
             +------+-------+
                    |
          commands / reads
                    |
     +--------------+------------------+
     |                                 |
     v                                 v
+------------+                   +------------+
| worker     |                   | worker     |
| worker A   |                   | worker B   |
+------+-----+                   +------+-----+
       |                                |
       | local services                 | local services
       v                                v
  image service / ingress proxy / DNS / host agent / Firecracker
```

That diagram is clean. The real thing was not.

## A request in motion

One of the easiest ways to understand the platform is to follow one machine request from start to finish:

```
client asks for a machine
        |
        v
control plane picks a worker
        |
        v
worker prepares and starts the machine
        |
        v
proxy and DNS get updated
        |
        v
user reaches a running machine
```

That flow looks simple on paper.

The hard part is everything hiding inside each step.

## The first lesson: every shortcut becomes a subsystem

Early on, a lot of things can live in shell scripts and one-off flows. That is normal. You need speed while you are learning.

Then the edges show up.

You need network setup that works every time, not just on a good day. You need logs that survive long enough to explain a bad boot. You need to know if a machine is still real after the host process dies. You need a way to expose HTTP and SSH without making every guest solve TLS and routing on its own.

Each shortcut turns into a real component:

- Root filesystem prep becomes a real image service.
- Traffic routing becomes a real ingress proxy.
- Guest boot logic becomes a guest agent.
- Host reconciliation becomes a host agent.
- Cluster state becomes a control-plane problem, not a local script problem.

That is the pattern I keep coming back to: small experiments turn into real features faster than you think.

## The second lesson: the hard parts are not the obvious parts

People hear "microVM platform" and think the hard part is booting Firecracker.

It is not.

Booting Firecracker is table stakes. The real pain lives in everything around it:

- deciding where a machine should run
- cleaning up half-failed starts
- preserving state across crashes
- connecting private networks across hosts
- making machine names resolve correctly
- exposing services without leaking the whole host
- understanding why one request took 150ms and the next took 5s

The control plane is not glamorous, but it is the difference between a demo and a platform.

## The third lesson: state is the whole game

Once you run more than one node, you stop asking "did the API succeed?" and start asking better questions:

- What happens if the writer crashes after the side effect but before the state update?
- What if two nodes both think they own the same action?
- What data can be rebuilt, and what data must survive?

That is why so much of this series ends up talking about FSMs, state replication, networking, and cleanup paths. Those are not side details. That is the platform.

## Why I am writing this series

There is a lot of writing about containers, and there is a lot of writing about big cloud systems. There is less writing about the middle ground: building your own compute platform when you still want to understand every moving piece.

That middle ground is where most of the interesting tradeoffs live.

You do not have infinite headcount. You do not want five databases because a diagram looked impressive. You do not want a service mesh because you got bored. You want a system that can survive failure, stay understandable, and move fast enough to keep building.

That is the lens for the posts that follow.

## What is coming next

This series will cover:

- durable machine state and crash recovery
- how a worker node actually brings up a machine
- how a stateless regional control plane can still make good decisions
- faster starts with snapshots, clones, and warm slots
- private networking for microVM fleets
- boring internal DNS
- ingress and SSH routing
- host reconciliation and self-healing
- observability that can explain real failures
- scheduling compute without cheating
- what this all means for AI workloads

That is the kind of system I am aiming for here: small enough to understand, but solid enough to run real work. A single microVM is mostly a runtime problem. A fleet of microVMs is a coordination problem.
