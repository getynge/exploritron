# Exploritron
Exploritron is an RPG I'm developing in rust, 
Exploritron has an unreasonable scope and will never be done, as I finalize more details I will add them here.

## Components
### Component: Core
The root crate is considered Core, this is the component responsible for 
performing actual game logic.

### Component: Renderer
The renderer is a dynamic library separate from the core, it defines a simple API for rendering a given state.

Any library that conforms to the specification laid out in render-api/src/client.rs can be used