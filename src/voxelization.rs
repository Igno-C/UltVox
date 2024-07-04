use std::collections::HashSet;

use crate::shapes::*;

pub trait Voxelizable {
    fn voxelize(&self) -> HashSet<(i32, i32, i32)>;
}

impl Voxelizable for Tri {
    fn voxelize(&self) -> HashSet<(i32, i32, i32)> {
        let mut set = HashSet::new();

        let flatx = self.get_flat(0);
        let flaty = self.get_flat(1);
        let flatz = self.get_flat(2);
        // println!("Tri with flats:");
        // println!("{:?}", flatx);
        // println!("{:?}", flaty);
        // println!("{:?}", flatz);
        for ray in flatx.points_inside() {
            let x = self.plane_intersection(ray, 0).round();
            if !x.is_infinite() {
                set.insert((x as i32, ray.x as i32, ray.y as i32));
            }
        }
        for ray in flaty.points_inside() {
            let y = self.plane_intersection(ray, 1).round();
            if !y.is_infinite() {
                set.insert((ray.x as i32, y as i32, ray.y as i32));
            }
        }
        for ray in flatz.points_inside() {
            let z = self.plane_intersection(ray, 2).round();
            if !z.is_infinite() {
                set.insert((ray.x as i32, ray.y as i32, z as i32));
            }
        }
        
        set
    }
}

impl Voxelizable for Sphere {
    fn voxelize(&self) -> HashSet<(i32, i32, i32)> {
        let mut set = HashSet::new();
        for p in self.points_inside() {
            set.insert((p.x as i32, p.y as i32, p.z as i32));
        }

        set
    }
}

pub fn merge<T>(base: &mut HashSet<T>, other: HashSet<T>) where T: Eq + std::hash::Hash {
    for item in other {
        base.insert(item);
    }
}
