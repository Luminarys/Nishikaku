# Nishikaku
[![Build status](https://ci.appveyor.com/api/projects/status/ro7cfyb5e77b0a49/branch/master?svg=true)](https://ci.appveyor.com/project/Luminarys/nishikaku/branch/master)
Simple danmaku game and engine. WIP.

[Info] (https://titanpad.com/l0Vjfgw4UH)

# Engine Internals
The engine is based on a single threaded entity-component system with reactive entities.
All entities interact with the engine via events passed to them, and by creating and using preprovided components the engine supplies.
The engine's main loop functions as such:
* Create a new cleared frame.
* Pass the Render event to all entities.
* Write instanced sprite data to the frame.
* Finish the frame.
* Poll the window for events(keyboard presses, mouse movement, etc.), convert them to internal events and dispatch them to all subscribed entities.
* Using time stored in an accumulator, dispatch an Update event to all entities, and update the physics aspect of the world, dispatching Collision/Proximity events as needed.
* Repeat.
