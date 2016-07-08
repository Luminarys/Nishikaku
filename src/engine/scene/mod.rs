use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::ops::Deref;
use std::ops::DerefMut;
use ncollide::world::{CollisionWorld2, CollisionGroups, GeometricQueryType, CollisionObject2};
use ncollide::shape::ShapeHandle2;
use ncollide::narrow_phase::{ProximityHandler, ContactHandler, ContactAlgorithm2};
use ncollide::query::{Contact, Proximity};
use nalgebra::{Vector2, Isometry2, Point2};
use nalgebra;

use engine::event::{Event, CollisionData, ProximityData, Dispatcher};
use engine::entity::Entity;
use engine::entity::component::PhysicsData;
use engine::util;
use engine::util::{HashMap, HashSet};

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

pub struct PhysicsWorld {
    world: RefCell<CollisionWorld2<f32, Rc<PhysicsData>>>,
    registry: RefCell<Registry>,
    interactive: CollisionGroups,
    semi_interactive: CollisionGroups,
    non_interactive: CollisionGroups,
    pub scaler: f32,
}

pub struct ProximityDispatcher {
    pub dispatcher: Dispatcher,
}

impl ProximityHandler<Point2<f32>, Isometry2<f32>, Rc<PhysicsData>> for ProximityDispatcher {
    fn handle_proximity(&mut self,
                        co1: &CollisionObject2<f32, Rc<PhysicsData>>,
                        co2: &CollisionObject2<f32, Rc<PhysicsData>>,
                        _old_proximity: Proximity,
                        new_proximity: Proximity) {
        self.dispatcher.dispatch(co1.data.entity_id,
                                 Event::Proximity(co2.data.entity_id,
                                                  ProximityData {
                                                      proximity: new_proximity,
                                                      this_object: co1.data.clone(),
                                                      other_object: co2.data.clone(),
                                                  }));
        self.dispatcher.dispatch(co2.data.entity_id,
                                 Event::Proximity(co1.data.entity_id,
                                                  ProximityData {
                                                      proximity: new_proximity,
                                                      this_object: co2.data.clone(),
                                                      other_object: co1.data.clone(),
                                                  }));
    }
}

pub struct CollisionDispatcher {
    pub dispatcher: Dispatcher,
    pub collector: Vec<Contact<Point2<f32>>>,
}

impl ContactHandler<Point2<f32>, Isometry2<f32>, Rc<PhysicsData>> for CollisionDispatcher {
    fn handle_contact_started(&mut self,
                              co1: &CollisionObject2<f32, Rc<PhysicsData>>,
                              co2: &CollisionObject2<f32, Rc<PhysicsData>>,
                              alg: &ContactAlgorithm2<f32>) {
        alg.contacts(&mut self.collector);
            self.dispatcher.dispatch(co1.data.entity_id,
                                     Event::Collision(co2.data.entity_id,
                                                      CollisionData {
                                                          contact: self.collector[0].clone(),
                                                          this_object: co1.data.clone(),
                                                          other_object: co2.data.clone(),
                                                      }));
            self.dispatcher.dispatch(co2.data.entity_id,
                                     Event::Collision(co1.data.entity_id,
                                                      CollisionData {
                                                          contact: self.collector[0].clone(),
                                                          this_object: co2.data.clone(),
                                                          other_object: co1.data.clone(),
                                                      }));
    }

    fn handle_contact_stopped(&mut self,
                              _co1: &CollisionObject2<f32, Rc<PhysicsData>>,
                              _co2: &CollisionObject2<f32, Rc<PhysicsData>>) {
        // Nothing for now
    }
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
    pub fn new(scaler: f32) -> PhysicsWorld {
        let mut int_groups = CollisionGroups::new();
        int_groups.set_membership(&[1]);

        let mut semi_int_groups = CollisionGroups::new();
        semi_int_groups.set_membership(&[2]);
        semi_int_groups.set_whitelist(&[1, 3]);

        let mut non_int_groups = CollisionGroups::new();
        non_int_groups.set_membership(&[3]);
        non_int_groups.set_whitelist(&[2]);

        let world = CollisionWorld2::new(0.02, true);

        PhysicsWorld {
            world: RefCell::new(world),
            registry: RefCell::new(Registry::new()),
            interactive: int_groups,
            semi_interactive: semi_int_groups,
            non_interactive: non_int_groups,
            scaler: scaler,
        }
    }

    pub fn register_handlers(&self, q: Rc<RefCell<Vec<(usize, Rc<Event>)>>>) {
        let prox = ProximityDispatcher { dispatcher: Dispatcher { queue: q.clone() } };

        let contact = CollisionDispatcher {
            dispatcher: Dispatcher { queue: q.clone() },
            collector: Default::default(),
        };

        self.world.borrow_mut().register_proximity_handler("Proximity", prox);
        self.world.borrow_mut().register_contact_handler("Contact", contact);
    }

    pub fn update(&self) {
        self.world.borrow_mut().update();
        self.registry.borrow_mut().reclaim();
    }

    pub fn add(&self,
               position: Vector2<f32>,
               shape: ShapeHandle2<f32>,
               interactivity: PhysicsInteraction,
               query: GeometricQueryType<f32>,
               data: Rc<PhysicsData>)
               -> usize {
        let group = match interactivity {
            PhysicsInteraction::Interactive => self.interactive,
            PhysicsInteraction::SemiInteractive => self.semi_interactive,
            PhysicsInteraction::NonInteractive => self.non_interactive,
        };
        let id = self.registry.borrow_mut().get_id();
        self.world.borrow_mut().add(id,
                                    Isometry2::new(position, nalgebra::zero()),
                                    shape,
                                    group,
                                    query,
                                    data);
        self.world.borrow_mut().update();
        id
    }

    pub fn remove(&self, id: usize) {
        self.world.borrow_mut().deferred_remove(id);
        self.registry.borrow_mut().return_id(id);
    }

    pub fn get_pos(&self, id: usize) -> Option<Isometry2<f32>> {
        match self.world.borrow().collision_object(id) {
            Some(obj) => Some(obj.position),
            None => None,
        }
    }

    pub fn set_pos(&self, id: usize, pos: Isometry2<f32>) {
        self.world.borrow_mut().deferred_set_position(id, pos);
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
    pub fn new(scaler: f32) -> Scene<E> {
        Scene {
            world: Rc::new(Default::default()),
            physics: Rc::new(PhysicsWorld::new(scaler)),
        }
    }

    pub fn dispatch(&self, id: usize, ev: Rc<Event>) {
        let acc = self.world.deref().get_entity_mut(&id);
        match acc.access() {
            Some(mut e) => e.deref_mut().handle_event(ev),
            None => println!("Trying to access unkown entity with id: {:?}", id),
        };
    }

    pub fn update(&self) {
        self.world.deref().update();
        self.physics.deref().update();
    }
}
