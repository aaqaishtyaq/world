+++
title = "Making Internal DNS Boring for Private Machines"
date = "2026-04-07"
draft = false

[taxonomies]
series = ["MicroVM Platform"]
+++
Once private networking starts to work, DNS becomes the next obvious problem.

If private networking is the road system, DNS is the street sign.

And like street signs, you only notice it when it is wrong.

Nothing makes a platform feel flaky faster than this:

- the machine is up
- the network path works
- the name still does not resolve

That gap is small on paper and painful in real use.

I have run into this enough times that I now care a lot about keeping internal DNS boring.

## What I want from internal DNS

I want internal DNS to be so boring people stop asking how it works.

That means:

- every machine can resolve private names
- the answer does not depend on which node happens to host the resolver
- moving a machine should not change the model
- stale node-local state should not quietly break resolution

If DNS depends too much on local accidents, the whole private network feels less trustworthy.

## Stable identity beats node-local cleverness

One tempting design is to make the guest ask a node-specific resolver.

That sounds easy because the host is right there.

The problem is that it leaks host details into the guest model. Now the guest is coupled to where it landed. If the node changes, the path changes. If node-local state drifts, resolution changes. If the resolver is healthy on one node and stale on another, you get weird half-failures.

A better model is simple:

- the guest uses one stable DNS service identity
- the host makes that identity work locally

That keeps the guest contract clean.

## Why this matters more than it sounds

Service discovery is one of those things people treat as an accessory until they build real workloads.

The moment you have:

- app talking to database
- app talking to cache
- one machine calling another by name
- a short-lived environment that should behave like a real network

you need names you can trust.

Something as simple as `db.internal` should just work. The user should not have to know which node hosts the database or which host is running the resolver.

IP addresses are fine for debugging. They are bad user experience.

## The shape I like

Here is the guest-facing model:

```
guest:
  query .internal name
      |
      v
stable DNS VIP
      |
      v
worker-local dns service
      |
      v
answer from replicated machine/network state
```

The guest always talks to the same destination.

The host does the local termination and lookup work.

That split is what makes the system feel stable.

## Keep lookup data close to the truth

The second part of the DNS problem is data freshness.

If name records are built from stale side tables or delayed repair logic, DNS becomes the first place you notice drift.

That is usually what bad DNS feels like in practice:

- the machine is up
- packets are flowing
- the name still fails

That kind of bug wastes a lot of time because it makes people question the whole network even when only one part is broken.

That is why the data feeding DNS should come from the same control-plane truth that placement and machine ownership use. Different products can wire that differently, but the principle stays the same:

Do not make DNS invent its own reality.

It should reflect the authoritative view of:

- what machine exists
- what network it belongs to
- what private address it owns

Freshness matters too. If DNS updates lag too much, users stop trusting names and start pasting IPs into config files.

## A small thing that changes the whole product

When internal DNS is good, everything above it gets easier:

- config files get simpler
- local dev feels closer to production
- private services stop needing copy-pasted IPs
- cross-machine workflows become natural

When internal DNS is bad, users build shadow systems:

- hardcoded addresses
- host file hacks
- startup scripts that query the control plane directly

That is usually a sign the platform contract is weak.

## Boring is the win

The best internal DNS story is not impressive.

It is just this:

"Every machine gets a private name, every other machine can resolve it, and nobody has to think about which worker host is involved."

That is the result I want. Not clever. Just reliable. If users keep reaching for IP addresses, the internal DNS contract is probably weak.
