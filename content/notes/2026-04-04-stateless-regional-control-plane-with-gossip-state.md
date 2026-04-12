+++
title = "Stateless Regional Control Planes and Replicated State"
date = "2026-04-04"
draft = false

[taxonomies]
series = ["MicroVM Platform"]
+++
I like control planes that can die without drama.

If a regional API process restarts, I do not want a rescue mission. I want it to come back, rebuild what it needs, and keep serving traffic.

That pushes you toward a simple rule:

The control plane should not own more durable state than it has to.

I ran into this while thinking through the regional control plane. The more state I wanted to push into it, the less I liked the system.

## The old temptation

The easy design is familiar.

Workers report into a central database. The regional service reads and writes that database. Over time it becomes a big cache, a queue, and a graveyard of stale rows all at once.

That works until it does not.

The cracks show up in boring places:

- a node reboots and stale machine records survive
- capacity data lags behind reality
- different regional instances read slightly different views
- cleanup relies on rows that no longer mean what they used to

The database becomes the place where old assumptions go to live forever.

## A better split

The worker already has the facts about its own machines. So let it publish enough of those facts for the control plane to read.

That changes the flow:

```
before:
  worker -> central DB -> control plane

after:
  worker -> replicated cluster state
  control plane -> read local replica
```

The second model has better failure behavior.

The control plane is now a reader, scheduler, and router. It is not the sole keeper of facts it did not create.

## What the control plane should not know

There is also plenty the control plane does not need to know in detail:

- every host-side cleanup step
- every boot log line
- every process-level detail on the worker

It needs enough shared state to decide and route well. It does not need to mirror the entire private life of every node.

## Why this matters in practice

Stateless does not mean dumb.

The regional control plane still does real work:

- auth
- placement
- network lifecycle
- machine lookup
- routing decisions
- health-based placement

The difference is that it does not need its own database to do that work well.

It can boot, connect to its local replica, warm an in-memory cache if needed, and start answering questions.

If it dies, no truth is lost.

That is a much healthier role.

## What replicated state buys you

The control plane needs cluster facts, but not all facts.

It mostly needs:

- node health
- node capacity
- machine placement
- network allocation state
- enough metadata to route and schedule safely

Those are all good candidates for shared replicated state.

Corrosion became the interesting piece here. It shifts the cluster away from "everything goes through one central writer" and toward "the writer is close to the machine activity it is reporting."

That matches reality better.

Reading from a local replica helps too. It cuts the cost of always reaching back to one central place before the control plane can answer a simple question.

## The hard part is not replication

The hard part is deciding what deserves replication.

If you replicate too little, the control plane gets blind.

If you replicate too much, you turn shared state into a kitchen sink and make rollouts painful.

The trick is to replicate the facts needed for decisions, not every private detail of a node's internal life.

I think about it like this:

```
local only:
  step-by-step recovery detail
  process-local cleanup bookkeeping
  short-lived machine state

replicated:
  node capacity
  machine ownership
  placement-visible health
  network membership
  routeable identity
```

That boundary keeps the cluster readable.

## Stateless makes life easier

There is a human reason to want this too.

A control plane with its own fragile state store becomes a thing you are scared to touch. You babysit migrations. You wonder if a failover will leak meaning. You start planning around the database instead of around the product.

A stateless regional service is easier to operate, easier to scale, and easier to trust.

Start a new one. Stop an old one. Move it. Replace it.

That should be routine.

## One more benefit: the architecture gets honest

When the regional layer stops pretending to own everything, the system reads more clearly:

- workers report machine state
- the cluster replicates scheduling truth
- the regional service consumes that truth to make decisions

That is a crisp model.

Here is the simplest version of it:

```
          write state                  read state
worker -----------------> replicated state -----------------> regional control plane
   ^                                                              |
   |                                                              |
   +--------------------- commands back --------------------------+
```

The loop is tight, and each side has a job. That is what I want from a control plane. Not magic. Just clean ownership. The control plane needs enough shared state to decide. It does not need every detail to operate.
