// A lightweight Entity Component System (ECS) implementation.
// This file defines the core structures needed to build an ECS without
// relying on any external game engine crates.
//
// The ECS pattern helps organise game logic by separating data (components)
// from behaviour (systems). An entity is simply an identifier that can
// have multiple components attached to it. Systems operate on collections
// of entities that contain specific component sets.
//
// We keep the design minimal for clarity and to make it easy for
// beginners to understand. Rust's type system ensures components are
// strongly typed, while dynamic downcasting allows us to store different
// component types in a single map.

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Represents an entity in the world.
///
/// Each entity is identified by a unique integer. Components are stored in
/// maps using this ID as a key.
pub type Entity = u32;

/// The `World` manages entities and their components.
///
/// Components are stored in a nested `HashMap`. The outer map keys on the
/// component's `TypeId`, while the inner map keys on the `Entity` ID.
/// Values are boxed so that any component type can be stored.
#[derive(Default)]
pub struct World {
    next_id: Entity,
    components: HashMap<TypeId, HashMap<Entity, Box<dyn Any>>>,
}

impl World {
    /// Creates an empty world with no entities or components.
    pub fn new() -> Self {
        Self { next_id: 0, components: HashMap::new() }
    }

    /// Spawns a new entity and returns its ID.
    pub fn spawn(&mut self) -> Entity {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Adds a component to the given entity.
    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        self.components
            .entry(type_id)
            .or_insert_with(HashMap::new)
            .insert(entity, Box::new(component));
    }

    /// Attempts to fetch an immutable reference to a component of type `T`
    /// from the given entity.
    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|map| map.get(&entity))
            .and_then(|boxed| boxed.downcast_ref())
    }

    /// Attempts to fetch a mutable reference to a component of type `T`
    /// from the given entity.
    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .and_then(|map| map.get_mut(&entity))
            .and_then(|boxed| boxed.downcast_mut())
    }

    /// Iterates over all entities that have a component of type `T`,
    /// applying the given closure to each `(Entity, &mut T)` pair.
    ///
    /// This is a simple way to implement systems that operate on one
    /// component type at a time.
    pub fn for_each<T: 'static, F: FnMut(Entity, &mut T)>(&mut self, mut f: F) {
        if let Some(map) = self.components.get_mut(&TypeId::of::<T>()) {
            for (entity, component) in map.iter_mut() {
                // `downcast_mut` lets us convert the boxed `Any` back to `&mut T`.
                if let Some(comp) = component.downcast_mut::<T>() {
                    f(*entity, comp);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Health(u32);

    #[test]
    fn basic_usage() {
        let mut world = World::new();
        let entity = world.spawn();
        world.add_component(entity, Health(100));

        // Retrieve the component immutably
        let health = world.get_component::<Health>(entity).unwrap();
        assert_eq!(*health, Health(100));

        // Modify the component using a system-like closure
        world.for_each::<Health, _>(|_, h| h.0 -= 50);
        let health = world.get_component::<Health>(entity).unwrap();
        assert_eq!(*health, Health(50));
    }
}

