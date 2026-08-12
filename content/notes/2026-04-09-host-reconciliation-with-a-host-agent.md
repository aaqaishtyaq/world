+++
title = "Host Reconciliation with a Host Agent"
description = "Using a host agent for host reconciliation, and why SSH still has a place in modern infrastructure."
date = "2026-04-09"
draft = false

[taxonomies]
series = ["MicroVM Platform"]
+++
I actually like SSH. It is useful, fast, and hard to avoid when you are building infrastructure.

Still, there is a point in every infrastructure project where SSH becomes a bad habit.

At first it feels fast.

Jump into a box. Edit a file. Restart a service. Fix the thing.

Then the fleet grows, and you realize your operating model is just "hope the last person remembers what changed."

That is not a platform. That is group memory with root access.

This is the thinking behind having a host agent in the first place.

## Why hosts need their own owner

A host running a worker daemon, WireGuard, Corrosion, DNS, and proxy services has real state outside the guest workload.

That state needs ownership.

You need one place that can answer:

- what should be running here?
- what revision is applied?
- are configs rendered correctly?
- is the node healthy enough for placement?
- what should happen if the mesh drifts or a service crashes?

That is the job I want a host agent to do.

## What a host agent should own

For me, the host agent should own the boring but important things:

- config rendering
- service state
- revision tracking
- health reporting

If those responsibilities are split across too many scripts and side channels, the host starts drifting faster than the team can keep up.

## The rule I like

The host agent should be the only live owner of host mutation.

That sounds strict. It should be.

If half your changes come from bootstrap scripts, a quarter come from direct SSH, and the rest come from local service side effects, you do not have clean recovery. You have overlapping authority.

That leads to drift.

## Manifest-driven hosts are calmer hosts

The idea is simple:

- describe the node in a manifest
- let the host agent reconcile toward that state
- expose health and admin APIs from the same agent

That gives you a clear loop:

```
desired node manifest
        |
        v
   host agent
        |
        v
 services / configs / health state
```

Now the node stops being a snowflake and starts being something you can reason about.

## Why this matters for placement

Host health is not just an ops concern. It affects scheduling.

If a node has enough free CPU but its mesh is broken, its config is stale, or its core services are unhealthy, it should not take new work.

That means the host agent should publish health that the control plane can trust:

- ready or not
- schedulable or not
- drained or not
- last error
- applied revision

That turns host management into a real input to compute placement.

## Self-healing without nonsense

I like self-healing, but only when it is bounded.

"Self-healing" should not mean a daemon thrashing forever while making the node harder to debug.

It should mean a small set of safe actions:

- rerun reconcile
- restart one broken service
- re-check health
- cordon if still bad
- escalate if retries are exhausted

That is enough.

The host agent should be able to help, not hide the problem.

Here is the kind of drift case I worry about:

```
service binary updated
mesh config not updated
node looks half healthy
node should report degraded and stop taking work
```

That is exactly the sort of thing I want a host agent to catch early.

## Why direct SSH should become the exception

SSH is still useful. You need it for deep debugging.

But it should not be the main operating path.

The more routine work you can move into a host agent, the better:

- service restarts
- drain and uncordon
- health checks
- config rendering
- support bundle collection

That is how you make the fleet operable by design instead of by folklore.

## This matters even more for small teams

Big teams can sometimes absorb messy host practices with process and staffing.

Small teams cannot.

If a small team wants to run real infrastructure, it needs systems that reduce memory load. You do not want every fix to require remembering which host has which hand-edited file and which bootstrap flag was last used there.

You want one path.

That is why I think host reconciliation is a first-class platform feature, not just an operations convenience.

## The simple bar

A healthy host management story should let you answer these questions quickly:

- what should be on this node?
- what is actually on this node?
- is it healthy?
- can it take more work?
- if not, what is the next safe action?

If the answer still starts with "SSH in and check around," I do not think the system is done. If hosts mutate from too many places, recovery turns into guesswork.
