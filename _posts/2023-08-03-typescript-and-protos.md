---
layout: post
title: Typescript and Protos: WTF
---

There's a document inside Google titled "WTF protos" that I wrote a long time ago in an effort to understand the sheer complexity of the internal proto ecosystem: when you have multiple `*.proto` file versions and multiple versions of codegen for each language, the resulting state space gets really frustrating to navigate.

Outside of Google, the Typescript ecosystem for protos is possibly just as bad. It baffles me how bad it is, honestly.

## What are protos?

Protocol buffers are meant to be an inter-language interchange format that are wire-efficient and compute-efficient. They're far from the only solution in the space, but Google built the first version before anyone else was dealing with the problem, and then open sourced their solution early enough that a lot of the world follows them.

Unfortunately, everyone that wants to build something for browser users that can speak protos has to use either Javascript or Typescript. Anyone mature using JS is, most people would assert, kind of insane: JS is one of the few weakly typed languages that exists today.

## List of options

* ts-protoc-gen
* protoc-gen-ts
* ts-proto

