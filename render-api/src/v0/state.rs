use crate::c_array;

/// Attribute is a string of the format: "name:value"
/// The first section identifies what attribute is being read, and the value denotes the value.
/// If an attriubte has more than one argument, they will be separated by further :.
/// Examples of attributes: "status:burn", "items:5", "movement:5:5"
///
/// # Notes
/// * Extensions can add new attributes, therefore the list of attributes for any given type
///   may not be complete.
/// * Any value that can have items added by an extension will usually be an attribute
///   (noteable exception: MenuState kind)
pub type Attribute = *mut i8;


/// RenderContext communicates to the server what kind of scene needs to be rendered
///
/// # Variants
/// * WorldTraversal - The scene takes place on a map, outside any buildings
/// * BuildingTraversal - The scene takes place inside a building with no map context
/// * Battle - The scene takes place inside a battle map
#[repr(C)]
pub enum RenderContext {
    WorldTraversal,
    BuildingTraversal,
    Battle
}

/// MenuContext communicates to the server what kind of menu needs to be rendered
///
/// Aside from subsection length, none of these contexts have a fixed impact on menu information
/// communicated by the client. The purpose of these contexts is to give the server the option
/// to use different graphical styles for different menu kinds.
///
/// # Variants
/// * Invisible - no menu is visible
/// * Main - main menu is open; no map is loaded
/// * Pause - pause screen is open; map is loaded, pauses game
/// * Inventory - user inventory is open; map is loaded, does not pause game but should capture input
/// * Summary - actor/world summary screen is open; map is loaded, pauses game
/// * Loadout - user loadout is open; map is loaded, does not pause game but should capture input
#[repr(C)]
pub enum MenuContext {
    Invisible,
    Main,
    Pause,
    Inventory,
    Summary,
    Loadout
}

/// Section represents an individual section of a menu
///
/// In summary, loadout, and inventory menus an intuitive way to represent this would be tabs, since those menus don't have subsections.
///
/// In main and pause menus an intuitive way to represent this would be a menu tree, as those menus do have (potentially nested) subsections
/// # Fields
/// * title is the title of the section, must not be null
/// * description is text describing the section, may be null
/// * subsections is a list of sections reachable from this section and under it in the hierarchy
/// * items is a list of items in this section,
///   item may be of the form "t:<string>" in which case it is a plaintext string,
///   or it may be of the form "s:<index>" in which case it is a subsection at a given index
/// * selected item indicates which item is selected
/// * attrs is an attribute set
#[repr(C)]
pub struct Section {
    title: *mut i8,
    description: *mut i8,
    subsections: c_array<Section>,
    subsections_length: isize,
    items: c_array<Attribute>,
    items_length: isize,
    selected_item: isize,
    attrs: c_array<Attribute>,
    attrs_length: isize
}

/// Drawable represents a tile to be drawn on the world
///
/// # Fields
/// * kind is the name of the kind of thing being drawn, details depend on what contains this drawable
/// * pos_* is the position of this tile
/// * span_* is the size of this tile
/// * attrs is an attribute set
#[repr(C)]
pub struct Drawable {
    kind: *mut i8,
    pos_x: i64,
    pos_y: i64,
    pos_z: i64,
    span_x: i64,
    span_y: i64,
    span_z: i64,
    attrs: c_array<Attribute>,
    attrs_length: isize
}

/// Actor represents an in-world character, either a player or an AI
///
/// # Fields
/// * name is the actors name, how this is displayed is up to the server
/// * description is the actors description text, how this is displayed is up to the server
/// * draw is the tile information for this actor (valid kind details in the Valid Kinds section)
/// * attrs is an attribute set
///
/// # Valid Kinds
/// * player - indicates this actor is controllable by the player
/// * computer - indicates this actor is not controllable by the player
///
/// # Standard Client Attributes
/// * control:\<current|standby|distant\> - indicates whether this actor is currently being controlled,
///   they are able to be selected for control, or they are controllable but not currently selectable
///   (only given for kind: player)
/// * status:\<text\> - indicates actor has status effect (may occur more than once)
/// * affinity:\<text\> - indicates one of the actors strongest attack types (may occur more than once)
/// * affinity_interaction:\<name\>:\<value\> - indicates what affinities this actor is strong or weak against
/// * stat:\<name\>:\<value\>\[:delta\] - indicates the name of each stat to display (if needed) and it's value.
///   If delta is provided, that indicates what changes are occurring to this stat,
///   positive delta means stat increase and negative delta means stat decrease
#[repr(C)]
pub struct Actor {
    name: *mut i8,
    description: *mut i8,
    draw: Drawable,
    attrs: c_array<Attribute>,
    attrs_length: isize
}

/// Terrain represents a world tile that may or may not be traversable
///
/// # Fields
/// * draw is the tile information for this terrain (valid kinds listed in Valid Kinds section)
/// * attrs is an attribute set
///
/// # Valid Kinds
/// * terminal - indicates this terrain can be stood on
/// * entrance - indicates this terrain is an entrance to another world (building, tunnel, etc)
/// * no_entrance - indicates this terrain is an entrance that cannot currently be used
/// * passable - indicates this terrain can be passed through but connect be a stopping location
/// * impassable - indicates this terrain cannot be passed through
///
/// # Standard Client Attributes
/// * note:\<text\> - A plaintext note with details about this attribute (may occur more than once)
/// * kind_note:\<text\> - A note indicating why this terrain is not terminal or entrance
/// * type:\<text\> - indicates what type of terrain this is (e.g. plains, fields, mountains, etc)
/// * status:\<text\> - An effect on this terrain (may occur more than once)
/// * resource:\<name\>:\<value\> - The quantity of a given resource available on this terrain (may occur more than once)
#[repr(C)]
pub struct Terrain {
    draw: Drawable,
    attrs: c_array<Attribute>,
    attrs_length: isize
}

/// WorldState is the collective game state of all terrain in the current world
///
/// # Fields
/// * terrain is a flattened array of length terrain_len_x * terrain_len_y * terrain_len_z
/// * terrain_len_* is the length of the terrain along a given axis
/// * attrs is an attribute set
#[repr(C)]
pub struct WorldState {
    terrain: c_array<Terrain>, // NOTE: this is an array of length terrain_length
    terrain_len_x: i64,
    terrain_len_y: i64,
    terrain_len_z: i64,
    attrs: c_array<Attribute>,
    attrs_length: isize
}

/// ActorState is the collective game state of all actors in the current world
///
/// # Fields
/// * actors is an array of all actors in the current world
/// * attrs is an attribute set
#[repr(C)]
pub struct ActorState {
    actors: c_array<Actor>, // NOTE: this is an array of length actors_length
    actors_length: isize,
    attrs: c_array<Attribute>,
    attrs_length: isize
}

/// MenuState is the current state of the game menu
///
/// # Fields
/// * kind is the kind of menu this is (see Valid Kinds section, NOTE: extensions can add new kinds)
/// * sections is a list of top level sections in this menu
/// * selected_section is the currently active section
/// * attrs is an attribute set
#[repr(C)]
pub struct MenuState {
    kind: *mut i8,
    sections: c_array<*mut i8>,
    sections_length: isize,
    selected_section: isize,
    attrs: c_array<Attribute>,
    attrs_length: isize
}

/// State is the aggregated state of the current world
///
/// See the documentation for each type for a description as to their purpose
#[repr(C)]
pub struct State {
    render_context: RenderContext,
    world_state: WorldState,
    actor_state: ActorState,
    menu_state: MenuState,
    attrs: c_array<Attribute>,
    attrs_length: isize
}