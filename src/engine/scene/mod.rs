use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::ops::Deref;
use std::ops::DerefMut;
use ncollide_geometry::shape::ShapeHandle2;
use nalgebra::{Vector2};

use engine::event::{Event, CollisionData};
use engine::entity::Entity;
use engine::entity::component::PhysicsData;
use engine::util;
use engine::util::{HashMap, HashSet};
use engine::physics::{Collision, PhysicsEngine};

pub struct World<E: Entity> {
    pub entities: RefCell<HashMap<usize, RefCell<E>>>,
    pub registry: RefCell<Registry>,
}

impl<E: Entity> World<E> {
    pub fn update(&self) {
        self.registry.borrow_mut().reclaim();
    }
}

impl<E: Entity> Default for World<E> {
    fn default() -> World<E> {
        World {
            entities: RefCell::new(util::hashmap()),
            registry: RefCell::new(Registry::new()),
        }
    }
}

pub type PhysicsHandler = Box<PhysicsEngine<ShapeHandle2<f32>, PhysicsData>>;

pub struct PhysicsWorld {
    registry: RefCell<Registry>,
    engine: RefCell<Box<PhysicsEngine<ShapeHandle2<f32>, PhysicsData>>>,
    pub scaler: f32,
}

impl PhysicsWorld {
    pub fn new(scaler: f32, engine: Box<PhysicsEngine<ShapeHandle2<f32>, PhysicsData>>) -> PhysicsWorld {
        PhysicsWorld {
            engine: RefCell::new(engine),
            registry: RefCell::new(Registry::new()),
            scaler: scaler,
        }
    }

    pub fn update(&self, dt: f32) -> Vec<Collision<PhysicsData>> {
        self.registry.borrow_mut().reclaim();
        self.engine.borrow_mut().update(dt)
    }

    pub fn remove(&self, id: &usize) {
        self.engine.borrow_mut().remove(id);
        self.registry.borrow_mut().return_id(*id);
    }

    pub fn add(&self, pos: Vector2<f32>, shape: ShapeHandle2<f32>, data: Rc<PhysicsData>) -> usize {
        let id = self.registry.borrow_mut().get_id();
        self.engine.borrow_mut().add(id, pos, shape, data);
        id
    }

    pub fn get_pos(&self, id: &usize) -> Option<Vector2<f32>> {
        self.engine.borrow_mut().get_pos(id)
    }

    pub fn set_pos(&self, id: &usize, pos: Vector2<f32>) {
        self.engine.borrow_mut().set_pos(id, pos)
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

    pub fn remove(&self, id: &usize) {
        self.entities.borrow_mut().remove(id);
    }

    pub fn get_entity(&self, id: &usize) -> EntityAccessor<E> {
        let r = self.entities.borrow();
        EntityAccessor {
            id: id.clone(),
            map_ref: r,
        }
    }

    pub fn get_entity_mut(&self, id: &usize) -> MutEntityAccessor<E> {
        let r = self.entities.borrow();
        MutEntityAccessor {
            id: id.clone(),
            map_ref: r,
        }
    }
}

#[derive(Default)]
pub struct Registry {
    counter: usize,
    free: Vec<usize>,
    reclaimed: Vec<usize>,
    reclaimable: bool,
    names: HashMap<String, usize>,
    rev_names: HashMap<usize, String>,
    tags: HashMap<String, HashSet<usize>>,
}

impl Registry {
    pub fn new() -> Registry {
        Registry {
            counter: 1,
            free: vec![],
            reclaimed: vec![],
            reclaimable: true,
            names: util::hashmap(),
            rev_names: util::hashmap(),
            tags: util::hashmap(),
        }
    }

    pub fn no_reclaim(&mut self) {
        self.reclaim();
        self.reclaimable = false;
    }

    pub fn tag_id(&mut self, id: usize, tag: String) {
        if self.tags.contains_key(&tag) {
            self.tags.get_mut(&tag).unwrap().insert(id);
        } else {
            let mut set = util::hashset();
            set.insert(id);
            self.tags.insert(tag, set);
        }
    }

    pub fn untag_id(&mut self, id: &usize, tag: &String) {
        if self.tags.contains_key(tag) {
            self.tags.get_mut(tag).unwrap().remove(id);
        }
    }

    pub fn get_tagged(&mut self, tag: &String)  -> Option<&mut HashSet<usize>> {
        self.tags.get_mut(tag)
    }

    pub fn remove_tagged(&mut self, tag: &String)  -> Option<HashSet<usize>> {
        self.tags.remove(tag)
    }

    pub fn create_alias(&mut self, name: String, id: usize) {
        self.names.insert(name.clone(), id.clone());
        self.rev_names.insert(id, name);
    }

    pub fn get_aliased_id(&mut self, name: &String) -> Option<&usize> {
        self.names.get(name)
    }

    pub fn get_aliased_string(&mut self, id: &usize) -> Option<&String> {
        self.rev_names.get(id)
    }

    pub fn remove_alias(&mut self, name: &String) {
        if let Some(id) = self.names.remove(name) {
            self.rev_names.remove(&id);
        }
    }

    pub fn get_id(&mut self) -> usize {
        match self.free.pop() {
            Some(num) => num,
            None => {
                self.counter += 1;
                self.counter - 1
            }
        }
    }

    pub fn return_id(&mut self, id: usize) {
        if self.reclaimable {
            self.reclaimed.push(id);
        } else {
            self.free.push(id);
        }
    }

    pub fn reclaim(&mut self) {
        self.free.append(&mut self.reclaimed);
    }
}

pub struct Scene<E: Entity> {
    pub world: Rc<World<E>>,
    pub physics: Rc<PhysicsWorld>,
}

impl<E: Entity> Scene<E> {
    pub fn new(scaler: f32, physics: Box<PhysicsEngine<ShapeHandle2<f32>, PhysicsData>>) -> Scene<E> {
        Scene {
            world: Rc::new(Default::default()),
            physics: Rc::new(PhysicsWorld::new(scaler, physics)),
        }
    }

    pub fn dispatch(&self, id: usize, ev: Rc<Event>) {
        let acc = self.world.deref().get_entity_mut(&id);
        match acc.access() {
            Some(mut e) => e.deref_mut().handle_event(ev),
            None => println!("Trying to access unkown entity with id: {:?} for event {:?}", id, ev),
        };
    }

    pub fn update(&self, dt: f32) {
        self.world.update();
        // TODO: Dispatch
        let collisions = self.physics.update(dt);
        for collision in collisions {
            self.dispatch(collision.id2, Rc::new(Event::Collision(collision.id1, CollisionData {
                this_object: collision.data2.clone(),
                other_object: collision.data1.clone(),
            })));
            self.dispatch(collision.id1, Rc::new(Event::Collision(collision.id2, CollisionData {
                this_object: collision.data1.clone(),
                other_object: collision.data2.clone(),
            })));
        }
    }
}
