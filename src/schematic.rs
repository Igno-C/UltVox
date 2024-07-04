use std::{collections::HashMap, io::{Write, Read}};

use bevy::prelude::*;
use crate::{shapes, voxelization::{self, Voxelizable}};

#[derive(Resource, serde::Serialize, serde::Deserialize, Default, Debug)]
pub struct Schematic {
    points: HashMap<usize, Vec3>,
    elements: Vec<Element>
}

impl Schematic {
    pub fn save_to_file(&self, filename: &str) {
        let spath = format!("./shapes/{}", filename);
        let path = std::path::Path::new(&spath);
        let display = path.display();
    
        let mut file = match std::fs::File::create(path) {
            Err(why) => {
                println!("couldn't create {}: {}", display, why);
                return;
            },
            Ok(file) => file,
        };
        
        if let Err(why) = file.write(ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::new()).unwrap().as_bytes()) {
            println!("couldn't write {}: {}", display, why);
        }
    }

    pub fn load_from_file(&mut self, filename: &str) {
        let spath = format!("./shapes/{}", filename);
        let path = std::path::Path::new(&spath);
        let display = path.display();
    
        let mut file = match std::fs::File::open(path) {
            Err(why) => {
                println!("couldn't open {}: {}", display, why);
                return;
            },
            Ok(file) => file,
        };

        let mut s = String::new();

        match file.read_to_string(&mut s) {
            Ok(_) => {
                let s = ron::from_str(&s);
                match s {
                    Ok(sch) => *self = sch,
                    Err(why) => println!("couldn't deserialize {}: {}", display, why),
                }
            },
            Err(why) => {println!("couldn't read {}: {}", display, why);},
        }
    }

    pub fn load_from_obj_file(&mut self, filename: &str) {
        let spath = format!("./shapes/{}", filename);
        let path = std::path::Path::new(&spath);
        let display = path.display();
    
        let mut file = match std::fs::File::open(path) {
            Err(why) => {
                println!("couldn't open {}: {}", display, why);
                return;
            },
            Ok(file) => file,
        };

        let mut s = String::new();

        match file.read_to_string(&mut s) {
            Ok(_) => {
                self.points.clear();
                self.elements.clear();
                let mut curv = 1;
                for line in s.lines() {
                    let mut parts = line.split_whitespace();
                    let first = match parts.next() {
                        Some(s) => s,
                        None => {continue;},
                    };
                    if first == "v" {
                        let x: f32 = parts.next().unwrap().parse().unwrap();
                        let y: f32 = parts.next().unwrap().parse().unwrap();
                        let z: f32 = parts.next().unwrap().parse().unwrap();
                        self.points.insert(curv, Vec3::new(x, y, z));
                        curv += 1;
                    }
                    else if first == "f" {
                        let mut pts = Vec::new();
                        for p in parts {
                            let point: usize = p.split_once("/").unwrap().0.parse().unwrap();
                            pts.push(point);
                        }
                        self.elements.push(Element::Polygon(pts));
                    }
                }
            },
            Err(why) => {println!("couldn't read {}: {}", display, why);},
        }
    }

    pub fn example() -> Self {
        let mut s = Self::default();

        s.points.insert(0, Vec3::new(0., 1., -1.5));
        s.points.insert(1, Vec3::new(1., -1., 0.333));
        s.points.insert(2, Vec3::new(3., 0., 0.666));
        s.points.insert(3, Vec3::new(-2., 3., 2.));

        s.elements.push(Element::Point(0));
        s.elements.push(Element::Tri(0, 1, 2));
        s.elements.push(Element::Sphere(0, 2.1));
        s.elements.push(Element::Polygon(vec![0, 1, 2, 3]));

        s
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum Element {
    Point(usize),
    Tri(usize, usize, usize),
    Polygon(Vec<usize>),
    Sphere(usize, f32)
}

impl Default for Element {
    fn default() -> Self {
        Self::Point(0)
    }
}

// impl Element {
//     fn draw_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
//         todo!()
//     }
// }

impl voxelization::Voxelizable for Schematic {
    fn voxelize(&self) -> std::collections::HashSet<(i32, i32, i32)> {
        let mut set = std::collections::HashSet::new();
        for elem in &self.elements {
            'macz: {match elem {
                Element::Point(p) => {
                    let a = self.points[p];
                    set.insert((a.x.round() as i32, a.y.round() as i32, a.z.round() as i32));
                    
                },
                Element::Tri(p, q, l) => {
                    let pts = &self.points;
                    let (a, b, c) = (pts[p], pts[q], pts[l]);
                    let tri = shapes::Tri::from_points(a, b, c);
                    voxelization::merge(&mut set, tri.voxelize());
                },
                Element::Polygon(v) => {
                    if v.len() < 3 {break 'macz;}
                    let origin = self.points[&v[0]];
                    for i in 1..(v.len()-1) {
                        let p1 = self.points[&v[i]];
                        let p2 = self.points[&v[i+1]];

                        let tri = shapes::Tri::from_points(origin, p1, p2);
                        voxelization::merge(&mut set, tri.voxelize());
                    }
                },
                Element::Sphere(p, d) => {
                    let p = self.points[p];
                    let s = shapes::Sphere::new(p, *d);
                    voxelization::merge(&mut set, s.voxelize());
                },
            }}
        }

        set
    }
}

impl Schematic {
    pub fn voxelize_with_transform(&self, rot: Quat, scale: f32) -> std::collections::HashSet<(i32, i32, i32)> {
        let mut set = std::collections::HashSet::new();
        for elem in &self.elements {
            'macz: {match elem {
                Element::Point(p) => {
                    let a = rot*self.points[p]*scale;
                    set.insert((a.x.round() as i32, a.y.round() as i32, a.z.round() as i32));
                    
                },
                Element::Tri(p, q, l) => {
                    let pts = &self.points;
                    let (a, b, c) = (rot*pts[p]*scale, rot*pts[q]*scale, rot*pts[l]*scale);
                    let tri = shapes::Tri::from_points(a, b, c);
                    voxelization::merge(&mut set, tri.voxelize());
                },
                Element::Polygon(v) => {
                    if v.len() < 3 {break 'macz;}
                    let origin = rot*self.points[&v[0]]*scale;
                    for i in 1..(v.len()-1) {
                        let p1 = rot*self.points[&v[i]]*scale;
                        let p2 = rot*self.points[&v[i+1]]*scale;

                        let tri = shapes::Tri::from_points(origin, p1, p2);
                        voxelization::merge(&mut set, tri.voxelize());
                    }
                },
                Element::Sphere(p, d) => {
                    let p = rot*self.points[p]*scale;
                    let s = shapes::Sphere::new(p, *d*scale);
                    voxelization::merge(&mut set, s.voxelize());
                },
            }}
        }
        set
    }
}
