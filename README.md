# Slope2D

A simple physics engine for 2D platformer games.

The engine doesn't exist yet, since making it is a hard task that first requires a ton of experimentation. This repository contains crates that solve one problem at a time so ultimately I understand how to build the full engine.

The goal here is to make a physics engine that works well for precise platformers. Examples for games that would benefit from this engine (if I succeed in making it) are Donkey Kong Country Tropical Freeze, the Ori games, and Mario platformers (I could list more, but these are the most complex examples I could think of physics-wise). These games all require precise control of movement and interactions with slopes, moving/rotating platforms and other dynamic bodies. 

There are strict limits to what this engine should support. The games this engine is meant for do not need the following: angular velocity, joints, mass, friction, other realistic physics constants. The engine does prioritize giving developers direct control over interactions between bodies. For example, instead of using mass to determine at what speed bodies push other bodies (if at all), this engine calls a user callback that tells it exactly how much the bodies should push each other.
