use std::collections::HashMap;
use std::hash::Hash;

/// Attribute is a string of the format: "name:value"
/// The first section identifies what attribute is being read, and the value denotes the value.
/// If an attriubte has more than one argument, they will be separated by further :.
/// Examples of attributes: "status:burn", "items:5", "movement:5:5"
pub type Attribute = String;

pub type AttributeSet = HashMap<String, Attribute>;

/// Render state is a generic datastructure that renderers use to persist state between calls.
/// The engine in no way guarantees that a renderer will remain in memory indefinitely, therefore state should be stored here.
pub type RenderState = ();

#[repr(C)]
pub enum RenderContext {
    WorldTraversal,
    BuildingTraversal,
    Battle
}

#[repr(C)]
pub enum MenuContext {
    Invisible,
    Main,
    Pause,
    Inventory,
    Summary,
    Loadout
}

#[repr(C)]
pub struct Section {
    title: *mut i8,
    description: *mut i8,
    subsections: *mut [Section],
    items: *mut [Attribute], // item is of the form: "(t|s):(name|idx)"
    selected_item: isize,
    attrs: *mut AttributeSet
}

#[repr(C)]
pub struct Summary {
    target: Attribute, // target is of the form: "targetkind:identifier" e.g. "actor:primary", "terrain:(24,13)"
    kind: *mut i8,
    sections: *mut [*mut i8],
    selected_section: isize,
    attrs: *mut AttributeSet
}

#[repr(C)]
pub struct Drawable {
    kind: *mut i8,
    pos_x: i64,
    pos_y: i64,
    pos_z: i64,
    span_x: i64,
    span_y: i64,
    span_z: i64,
    attrs: *mut AttributeSet
}

#[repr(C)]
pub struct Actor {
    name: *mut i8,
    draw: Drawable,
    attr: *mut AttributeSet
}

#[repr(C)]
pub struct Terrain {
    draw: Drawable,
    attr: *mut AttributeSet
}

#[repr(C)]
pub struct WorldState {
    terrain: *mut [Terrain],
    attr: *mut AttributeSet
}

#[repr(C)]
pub struct ActorState {
    actors: *mut [Actor],
    attr: *mut AttributeSet
}

#[repr(C)]
pub struct MenuState {
    summary_state: Summary,
    attrs: *mut AttributeSet
}

#[repr(C)]
pub struct State {
    render_context: RenderContext,
    menu_context: MenuContext,
    world_state: WorldState,
    actor_state: ActorState,
    menu_state: MenuState,
    attrs: *mut AttributeSet
}