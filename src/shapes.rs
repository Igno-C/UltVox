use bevy::prelude::{Vec2, Vec3, Vec4};

// Represents a triangle to be voxelized
pub struct Tri {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    plane: Vec4 // 4th value is the d in ax+by+cz=d
}

impl Tri {
    pub fn from_points(a: Vec3, b: Vec3, c: Vec3) -> Self {
        let normal = (a-b).cross(a-c); // normal vector to the tri's plane
        Self {
            a, b, c,
            plane: normal.extend(a.dot(normal))
        }
    }

    // 'which' is a number 0..3, tells which coord to truncate (x, y, or z)
    pub fn get_flat(&self, which: i32) -> Tri2D {
        let Self {a, b, c, ..} = self;
        match which {
            0 => {
                Tri2D {
                    a: Vec2::new(a.y, a.z),
                    b: Vec2::new(b.y, b.z),
                    c: Vec2::new(c.y, c.z),
                }
            },
            1 => {
                Tri2D {
                    a: Vec2::new(a.x, a.z),
                    b: Vec2::new(b.x, b.z),
                    c: Vec2::new(c.x, c.z),
                }
            },
            2 => {
                Tri2D {
                    a: Vec2::new(a.x, a.y),
                    b: Vec2::new(b.x, b.y),
                    c: Vec2::new(c.x, c.y),
                }
            },
            _ => panic!()
        }
    }

    // 'which' is again 0..3, tells which axis the ray is on
    // returns the truncated coordinate intersection point
    pub fn plane_intersection(&self, ray: Vec2, which: i32) -> f32 {
        // ax + by + cz = d
        // The resulting expressions for xyz are:
        // x = (by + cz - d)/-a   for which = 0
        // y = (ax + cz - d)/-b   for which = 1
        // z = (ax + by - d)/-c   for which = 2
        // the abcd variables take values in the above order from self.plane
        // the expression in this function is: z = (ax + by - d)/-c
        let p = self.plane;
        let Vec2 {x, y} = ray;
        let (a, b, c, d);
        match which {
            0 => {
                (a, b, c, d) = (p.y, p.z, p.x, p.w);
            },
            1 => {
                (a, b, c, d) = (p.x, p.z, p.y, p.w);
            },
            2 => {
                (a, b, c, d) = (p.x, p.y, p.z, p.w);
            },
            _ => panic!()
        }
        if c.abs() < 0.01 { // no point in considering this intersection because the angle is too steep
            f32::INFINITY
        }
        else {
            (a*x + b*y - d)/(-c)
        }
    }
}

// Two dimensional triangle, used to check if a point is inside
#[derive(Debug)]
pub struct Tri2D {
    a: Vec2,
    b: Vec2,
    c: Vec2
}

impl Tri2D {
    pub fn points_inside(&self) -> Vec<Vec2> {
        let mut set = Vec::new();
        let minx = self.a.x.min(self.b.x).min(self.c.x).floor() as i32;
        let miny = self.a.y.min(self.b.y).min(self.c.y).floor() as i32;
        let maxx = self.a.x.max(self.b.x).max(self.c.x).ceil() as i32;
        let maxy = self.a.y.max(self.b.y).max(self.c.y).ceil() as i32;

        for x in minx..=maxx {
            for y in miny..=maxy {
                let v = Vec2::new(x as f32, y as f32);
                if self.contains(v) {
                    set.push(v);
                }
            }
        }
        
        // println!("Resulting in set: {:?}", set);
        set
    }

    // The algorithm uses barycentric coordinates
    fn contains(&self, p: Vec2) -> bool {
        let v0 = self.b - self.a; // vector a->b
        let v1 = self.c - self.a; // vector a->c
        let v2 = p - self.a; // vector a->p

        let dot00 = v0.dot(v0);
        let dot01 = v0.dot(v1);
        let dot02 = v0.dot(v2);
        let dot11 = v1.dot(v1);
        let dot12 = v1.dot(v2);

        let invdenom = 1. / (dot00*dot11 - dot01*dot01);
        let u = (dot11 * dot02 - dot01 * dot12) * invdenom;
        let v = (dot00 * dot12 - dot01 * dot02) * invdenom;

        (u >= 0.) && (v >= 0.) && (u + v <= 1.)
    }
}

pub struct Sphere {
    p: Vec3,
    d: f32
}

impl Sphere {
    pub fn new(p: Vec3, d: f32) -> Self {
        Self {p, d}
    }

    pub fn points_inside(&self) -> Vec<Vec3> {
        let mut set = Vec::new();
        let minx = (self.p.x - self.d).ceil() as i32;
        let miny = (self.p.y - self.d).ceil() as i32;
        let minz = (self.p.z - self.d).ceil() as i32;
        let maxx = (self.p.x + self.d).floor() as i32;
        let maxy = (self.p.y + self.d).floor() as i32;
        let maxz = (self.p.z + self.d).floor() as i32;

        for x in minx-1..=maxx+1 {
            for y in miny-1..=maxy+1 {
                for z in minz-1..=maxz+1 {
                    let v = Vec3::new(x as f32, y as f32, z as f32);
                    if self.contains(v) {
                        set.push(v);
                    }
                }
            }
        }

        set
    }

    pub fn contains(&self, p: Vec3) -> bool {
        (self.p.distance(p)-self.d).abs() <= (3_f32).sqrt()/2.
    }
}


#[cfg(test)]
mod tests {
    use bevy::prelude::Vec2;
    use super::Tri2D;

    #[test]
    fn tri2d_tests() {
        let p1 = Vec2::new(0., 0.);
        let p2 = Vec2::new(1., 1.);
        let p3 = Vec2::new(0.5, 3.);
        let p4 = Vec2::new(3., 0.5);
        let p5 = Vec2::new(-2., -1.);

        let tri1 = Tri2D {a: p5, b: p3, c: p4};
        assert!(tri1.contains(p1));
        assert!(tri1.contains(p2));
        assert!(!tri1.contains(Vec2::new(-2., -0.1)));

        let tri2 = Tri2D{a: p1, b: p2, c: p4};
        assert!(!tri2.contains(p3));
        assert!(!tri2.contains(p5));

        let tri3 = Tri2D{a: p5, b: p2, c: p4};
        assert!(tri3.contains(p1));
        assert!(!tri3.contains(p3))
    }

    #[test]
    fn points_inside_tests() {
        // let p1 = Vec2::new(0., 0.);
        // let p2 = Vec2::new(1., 1.);
        let p3 = Vec2::new(0.5, 3.);
        let p4 = Vec2::new(3., 0.5);
        let p5 = Vec2::new(-2., -1.);

        let tri1 = Tri2D {a: p5, b: p3, c: p4};
        let set = tri1.points_inside();
        assert!(set.contains(&Vec2::new(0., 0.)));
        assert!(set.contains(&Vec2::new(1., 1.)));
        assert!(!set.contains(&Vec2::new(3., 1.)));
        assert!(set.contains(&Vec2::new(-2., -1.)));
        assert!(!set.contains(&Vec2::new(3., 0.)));
        assert!(!set.contains(&Vec2::new(0., 3.)));
    }
}
