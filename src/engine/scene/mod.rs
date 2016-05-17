use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::collections::HashMap;
use std::ops::Deref;
use ncollide::world::{CollisionWorld2, CollisionGroups, GeometricQueryType, CollisionObject2};
use ncollide::shape::ShapeHandle2;
use nalgebra::{Vector2, Isometry2};
use nalgebra;

use engine::entity::Entity;
use engine::entity::component::PhysicsData;

pub struct World<E: Entity> {
    entities: RefCell<HashMap<usize, RefCell<E>>>,
    registry: RefCell<Registry>,
}

pub struct PhysicsWorld {
    world: RefCell<CollisionWorld2<f32, RefCell<PhysicsData>>>,
    interactive: CollisionGroups,
    semi_interactive: CollisionGroups,
    non_interactive: CollisionGroups,
}

pub enum PhysicsInteraction {
    // Interacts with other Interactive, and SemiInteractive objects
    Interactive,
    // Interacts with Interactive, and NonInteractive objects
    SemiInteractive,
    // Interacts with SemiInteractive objects
    NonInteractive,
}

impl PhysicsWorld {
    pub fn new() -> PhysicsWorld {
        let mut int_groups = CollisionGroups::new();
        int_groups.set_membership(&[1]);

        let mut semi_int_groups = CollisionGroups::new();
        semi_int_groups.set_membership(&[2]);
        semi_int_groups.set_whitelist(&[1, 3]);

        let mut non_int_groups = CollisionGroups::new();
        non_int_groups.set_membership(&[3]);
        non_int_groups.set_whitelist(&[2]);

        PhysicsWorld {
            world: RefCell::new(CollisionWorld2::new(0.02, true)),
            interactive: int_groups,
            semi_interactive: semi_int_groups,
            non_interactive: non_int_groups,
        }
    }

    pub fn add(&self,
               id: usize,
               position: Vector2<f32>,
               shape: ShapeHandle2<f32>,
               interactivity: PhysicsInteraction,
               data: RefCell<PhysicsData>) {
        let (group, query) = match interactivity {
            PhysicsInteraction::Interactive => {
                (self.interactive, GeometricQueryType::Contacts(0.0))
            }
            PhysicsInteraction::SemiInteractive => {
                (self.semi_interactive, GeometricQueryType::Proximity(0.0))
            }
            PhysicsInteraction::NonInteractive => {
                (self.non_interactive, GeometricQueryType::Proximity(0.0))
            }
        };
        self.world.borrow_mut().add(id,
                                    Isometry2::new(position, nalgebra::zero()),
                                    shape,
                                    group,
                                    query,
                                    data);
    }

    pub fn remove(&self, id: usize) {
        self.world.borrow_mut().deferred_remove(id);
    }
}

pub struct EntityAccessor<'a, E: 'a + Entity> {
    id: usize,
    map_ref: Ref<'a, HashMap<usize, RefCell<E>>>,
}

pub struct MutEntityAccessor<'a, E: 'a + Entity> {
    id: usize,
    map_ref: Ref<'a, HashMap<usize, RefCell<E>>>,
}

impl<'a, E: Entity> EntityAccessor<'a, E> {
    pub fn access(&'a self) -> Option<Ref<E>> {
        match self.map_ref.get(&self.id) {
            Some(res) => Some(res.borrow()),
            None => None,
        }
    }
}

impl<'a, E: Entity> MutEntityAccessor<'a, E> {
    pub fn access(&'a self) -> Option<RefMut<E>> {
        match self.map_ref.get(&self.id) {
            Some(res) => Some(res.borrow_mut()),
            None => None,
        }
    }
}

impl<E: Entity> World<E> {
    pub fn insert(&self, id: usize, e: E) {
        self.entities.borrow_mut().insert(id, RefCell::new(e));
    }

    pub fn remove(&self, id: &usize, e: E) {
        self.entities.borrow_mut().remove(id);
    }

    pub fn get_entity(&self, id: &usize) -> EntityAccessor<E> {
        let r = self.entities.borrow();
        EntityAccessor {
            id: id.clone(),
            map_ref: r,
        }
    }

    pub fn get_entity_mut(&mut self, id: &usize) -> MutEntityAccessor<E> {
        let r = self.entities.borrow();
        MutEntityAccessor {
            id: id.clone(),
            map_ref: r,
        }
    }
}

struct Registry {
}

pub struct Scene<E: Entity> {
    pub world: Rc<World<E>>,
    pub physics: Rc<PhysicsWorld>,
}
