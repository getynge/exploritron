# Exploritron
Exploritron is an RPG I'm developing in rust, 
Exploritron has an unreasonable scope and will never be done, as I finalize more details I will add them here.

## Components
### Component: Core
The root crate is considered Core, this is the component responsible for 
performing actual game logic.

### Component: Render Server
The server is a dynamic library separate from the core, it is responsible for rendering frames

Any library that conforms to the specification laid out in render-api/src/client.rs can be used