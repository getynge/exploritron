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
    Inventory
}

#[repr(C)]
pub struct Actor {
    kind: String,
    name: String,
    pos_x: i64,
    pos_y: i64,
    pos_z: i64,
}

#[repr(C)]
pub struct RenderObject {
    kind: String,
    pos_x: i64,
    pos_y: i64,
    pos_z: i64,
}

#[repr(C)]
pub struct WorldState {

}

#[repr(C)]
pub struct State {
    render_context: RenderContext,
    menu_context: MenuContext
}