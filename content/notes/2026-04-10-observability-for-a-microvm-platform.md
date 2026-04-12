+++
title = "Observability for a MicroVM Platform"
date = "2026-04-10"
draft = false

[taxonomies]
series = ["MicroVM Platform"]
+++
I love logs, but logs alone have not been enough for the kind of machine lifecycle work I have been doing.

You do not really know a platform until something slow or broken happens.

That is when the nice diagrams stop helping.

A request takes five seconds instead of two hundred milliseconds. A machine gets stuck during startup. A route exists, but traffic still fails. A node looks healthy until you ask one slightly better question.

That is when observability earns its keep.

In this post, I want to talk about the kind of visibility I actually want from a microVM platform.

## Logs alone are not enough

Logs matter. I want them. I want structured logs, and I want machine IDs in them.

But logs alone are a rough way to understand a distributed machine lifecycle.

You still end up asking:

- which service started the action?
- which node handled it?
- which transition was slow?
- how long did rootfs prep take?
- did networking finish before the guest timed out?

That is why traces and metrics matter too.

Different signals answer different questions.

## What I want to see

For a microVM platform, I want visibility at a few levels:

- request level: what API call came in?
- machine level: what happened to this machine over time?
- node level: is the host healthy and under pressure?
- service level: which component is slow or failing?

If those layers do not connect, debugging turns into archaeology.

## The fields I never want to miss

There are a few fields I always want available when I look at a request or a machine:

- machine id
- node id
- request id
- transition name

Without those, even good logs and traces start to blur together.

## The machine is the unit that matters

The cleanest organizing idea is to treat the machine as the thread through the system.

Every log, metric, and span that can reasonably attach to a machine should do it.

That way you can follow one path:

```
API request
   -> placement
   -> worker FSM
   -> rootfs prep
   -> network setup
   -> guest boot
   -> proxy exposure
```

Without that thread, you get disconnected facts. With it, you get a story.

## Why spans help so much

A machine FSM is a perfect place for spans.

Each transition already has a name and timing boundary. That means the system can show you not just that startup was slow, but where it was slow.

For example:

```
reserve_node: 8ms
prepare_rootfs: 420ms
setup_network: 95ms
wait_for_guest_agent: 1.8s
```

Now you have something real to act on.

Without that, "create was slow" is just a complaint.

That also gives you a real debugging loop:

```
create request looks slow
trace shows rootfs prep is normal
guest readiness wait is huge
look inside guest startup path next
```

That is a much better place to be than guessing between five different services.

## Local collection matters

I prefer a node-local collector for a simple reason: it keeps the system robust.

Each host can emit:

- OTLP logs
- metrics
- traces
- machine boot logs

Then a local collector ships them onward.

That makes instrumentation simpler and keeps the service code cleaner.

## Good observability changes behavior

Once the signals are strong, the engineering work changes.

You stop guessing.

You stop saying things like:

- "it might be networking"
- "maybe the guest was slow"
- "could be the proxy"

You can actually see:

- retry storms
- stuck transitions
- noisy health probes
- node-specific hot spots
- slow image paths

That makes the platform better because the fixes get sharper.

## The trap to avoid

The trap is collecting everything and understanding nothing.

A microVM platform already has enough moving parts. If the telemetry model is sloppy, the data turns into clutter.

I would rather have:

- good machine identifiers
- a few honest spans
- useful node metrics
- clean request logs

than a giant bucket of unlabeled noise.

## The boring outcome I want

When a user says, "machine create was slow," I want the answer within minutes.

Not because someone remembered a host by instinct.

Because the system can show:

- where the request went
- how placement behaved
- which machine transition stalled
- what the host looked like at the time

That is what observability is for. Not dashboards for their own sake. Explanations. That is the bar. Good observability does not just collect data. It shortens the path to the right fix.
