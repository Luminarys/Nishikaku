use ncollide::shape::ShapeHandle2;
use ncollide_geometry::query::{self, Proximity};
use nalgebra::{Isometry2, Vector2};
use std::rc::Rc;
use engine::util::{self, HashMap};
use engine::entity::component::PhysicsData;
use engine::physics::{Collision, PhysicsEngine};
use nalgebra as na;

pub type Shape = ShapeHandle2<f32>;

pub struct DanmakuPhysics {
    scaler: f32,
    pid: usize,
    objects: HashMap<usize, Object>,
}

// Note that group 1 = player, 2 = enemy bullet, 4 = enemy, 8 = player bullet
struct Object {
    pos: Vector2<f32>,
    shape: Shape,
    data: Rc<PhysicsData>,
}

const BUCKET_SIDE_NUM: usize = 16;

impl DanmakuPhysics {
    pub fn new(scaler: f32) -> DanmakuPhysics {
        DanmakuPhysics {
            scaler: scaler,
            objects: util::hashmap(),
            pid: 50000,
        }
    }
}

impl PhysicsEngine<Shape, PhysicsData> for DanmakuPhysics {
    fn add(&mut self, id: usize, pos: Vector2<f32>, shape: Shape, data: Rc<PhysicsData>) {
        if data.group == 1 {
            self.pid = id;
        }
        self.objects.insert(id, Object {
            pos: pos,
            shape: shape,
            data: data,
        });
    }

    fn remove(&mut self, id: &usize) {
        self.objects.remove(id);
    }

    fn update(&mut self, dt: f32) -> Vec<Collision<PhysicsData>> {
        let mut collisions = Vec::new();
        if let Some(player) = self.objects.get(&self.pid) {
            let ps = player.shape.as_ref();
            let ppos = player.pos;
            let piso = Isometry2::new(ppos, na::zero());
            for (id, object) in self.objects.iter() {
                if *id != self.pid {
                    let oshape = object.shape.as_ref();
                    let oiso = Isometry2::new(object.pos, na::zero());
                    if (object.pos.x - player.pos.x).abs() < 30.0 && (object.pos.y - player.pos.y).abs() < 30.0 {
                        if query::proximity(&oiso, oshape, &piso, ps, 0.5) == Proximity::Intersecting {
                            collisions.push(Collision {
                                id1: player.data.entity_id,
                                id2: object.data.entity_id,
                                data1: player.data.clone(),
                                data2: object.data.clone(),
                            });
                        }
                    }
                }
            }
        }
        collisions
    }
    fn get_pos(&self, id: &usize) -> Option<Vector2<f32>> {
        match self.objects.get(id) {
            Some(obj) => {
                Some(obj.pos.clone())
            },
            None => None
        }
    }
    fn set_pos(&mut self, id: &usize, pos: Vector2<f32>) {
        match self.objects.get_mut(id) {
            Some(obj) => {
                obj.pos = pos;
            },
            None => { }
        }
    }
}
