+++
title = "Scheduling Dense Compute Without Lying to Yourself"
date = "2026-04-11"
draft = false

[taxonomies]
series = ["MicroVM Platform"]
+++
I have been thinking a lot about scheduling lately, especially once warm starts and reservations enter the picture.

Schedulers are easy to make impressive in a slide deck.

They are much harder to make honest.

You can pack nodes aggressively, overcommit a little, and brag about utilization. Then a few bad placement calls land on the wrong hosts and the whole thing feels brittle.

That is why I care less about flashy scheduling algorithms and more about honest inputs.

In this article, I want to focus on that part.

## The first rule

Do not schedule from fantasy.

That means your scheduler needs real inputs:

- current capacity
- reserved capacity
- ready slots
- machine shape
- node health
- whether the node should take work

If any of those are stale or vague, "smart" placement becomes random placement with extra steps.

## Why free CPU is not enough

A node can have free CPU and still be a bad target.

Maybe it is unhealthy.
Maybe it is still recovering.
Maybe it has capacity on paper but not the right prepared images.
Maybe it is already holding reservations for requests that have not committed yet.

This is why a better scheduler tracks more than free resources.

It needs to know the difference between:

- cold capacity
- reserved capacity
- hot capacity

Those are not the same thing.

Here is a very simple example:

```
node A has more free CPU, but only cold create capacity
node B has less free CPU, but already has a matching ready slot
for latency, node B is the better answer
```

That is why raw free capacity is not enough.

## The reservation model fixes a lot

One thing I like about reservation-based placement is that it forces the scheduler to act with discipline.

Instead of firing real create requests at multiple nodes, it can:

1. rank candidates
2. reserve one
3. commit to one
4. activate a ready slot when available

That cuts wasted work and keeps placement cleaner.

It also gives the system a way to talk honestly about partial ownership of resources before the machine fully exists.

## Bad scheduler inputs

The scheduler gets weird quickly if any of these are stale:

- node health
- reservations
- ready-slot counts
- image availability

At that point, even a clever ranking function starts making dumb choices.

## Dense packing needs guardrails

I am not against dense packing or measured overcommit.

I am against pretending they are free.

If you want to pack nodes tightly, you need:

- clear machine classes
- known resource envelopes
- health gating
- fast rollback when activation fails
- good observability around pressure

Otherwise density turns into support load.

## Scheduling is a product decision too

This part gets missed a lot.

The scheduler shapes the product.

If placement is sloppy:

- starts get slower
- noisy nodes get noisier
- users hit surprise failures
- ready slots get wasted

If placement is sharp:

- fast paths stay fast
- node failures are easier to contain
- prepared capacity gets used well
- the product feels responsive

That is why I treat scheduler inputs like user-facing behavior, not background math.

## What I want from a scheduler

I want the system to answer a few simple questions well:

- which nodes are healthy enough to take work?
- which nodes can do a cold create?
- which nodes can do a warm activation right now?
- which node is already close to the needed image and shape?

That is enough to get good results without pretending the platform has perfect foresight.

## The lie to avoid

The lie is telling yourself the scheduler is smart because it has a ranking function.

A ranking function over stale or incomplete state is not smart. It is decorative.

Good scheduling starts with good state.

That is why so many "scheduler" problems are really state-distribution problems, health-model problems, or lifecycle-model problems in disguise.

Fix those first.

Then the placement code gets a lot better, even if the algorithm itself stays pretty simple. A scheduler is only as good as the state it believes.
