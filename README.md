# Lumen

**Real-Time Path Tracer & Physics Engine**

A from-scratch rendering and simulation engine combining physically based rendering, low-level GPU compute and custom physics simulation. Built to explore the fundamentals of computer graphics, computational geometry and real-time simulation without relying on pre-existing game engines.

## Key Features

- **Real-Time Path Tracing** - Custom Monte Carlo path tracer with physically based light transport and importance sampling.
- **BVH Acceleration** - Bounding Volume Hierarchy implementation for accelerated ray-scene intersection.
- **Raw Vulkan Compute** - Direct GPU compute integration with custom kernels, descriptor sets and memory management.
- **Custom ECS** - Bespoke Entity Component System for modular scene and object management.
- **Rigid Body Physics** - Deterministic fixed-timestep simulation with custom collision detection and response.
- **GJK / EPA Collision** - Convex collision detection and penetration depth calculation.
- **Verlet Cloth Simulation** - Constraint-based cloth physics using Verlet integration.
- **Performance Focused** - Designed around efficient memory management, parallel computation and low-level optimisation.

## Technology Stack

- **Language:** C++
- **Graphics API:** Vulkan
- **GPU Programming:** GLSL / Compute Shaders
- **Build System:** CMake
- **Rendering:** Monte Carlo Path Tracing, BVH
- **Physics:** GJK, EPA, Verlet Integration
- **Architecture:** Custom ECS

## Core Architecture

Lumen is developed from the ground up, with rendering, physics and simulation systems implemented independently rather than delegated to existing engines or frameworks.

Each subsystem is designed to provide direct control over its underlying algorithms, data structures and execution pipeline.

## Showcase

Coming soon.
