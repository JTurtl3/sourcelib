# sourcelib
A Rust library for using Source Engine data types and files.

## Status
Minimally functional at best. The project has basically just started.

## Features
- Reading .bsp files
- Parsing KeyValue files (like .vmt)

## Planned
- Reading .vtf files
- Loading models (.mdl, .smd, .dmx)
- More, probably.

## Goal
My long-term goal with this project is to use it in another: A full reimplementation of Half-Life 2.
I want to be able to copy and paste the original assets from Half-Life 2 and play the game in a different engine.

Besides that ridiculously ambitious task, this library could also be useful for other applications in Source Engine development.
A lot of almost essential tools (PakRat, Hammer, Crowbar) are very old software and questionably reliable. Maybe someone will make new versions in Rust using this library.

This library itself will not have any graphical features or do much processing but will easily integrate into other codebases or engines.
