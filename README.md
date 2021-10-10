# cu.rs
A Bancho implementation made in Rust for the *cursed* stack. **THIS PROJECT IS REALLY UNFINISHED AND IN ITS EARLY STAGES**

A drag and drop replacement for the [pep.py](https://github.com/osuripple/pep.py) Bancho implementation, focusing on performance and scalability.

**EVERYTHING BELOW APPLIES TO THE INTENDED FINISHED PRODUCT.**

## What does it do?
The objective of cu.rs is to handle all of the realtime features of an osu! server over the osu! bancho protocol. This includes the in-game features of:
- In-game authentication
- User listing
- Private and public chat
- User spectation
- Multiplayer
- In-game bot
- Command interactions

## Why should I use it?
As mentioned before, cu.rs uses the infrastructure provided by the existing Ripple stack (MySQL database schema, redis API). This means that for existing servers, 
it is a trivial task to upgrade your server to cu.rs.

Alongside this, everyone is able to gain the following benefits over existing alternatives:
- Efficient usage of resources
- Ridiculously high performance
- Ability to handle high throughput scenarios
