## Project Architecture
### ECS
The core of the backend system revolves around [HECS](https://docs.rs/hecs/latest/hecs/) a "a high-performance, minimalist entity-component-system (ECS) world." 

What is an ECS? https://github.com/Ralith/hecs#why-ecs

- Each major component of the pipeline is tied to an "Entity" which contains 0 or more "Components" 
- Each Tick, a variety of "Systems" acts upon these Entities, mutating their Components
