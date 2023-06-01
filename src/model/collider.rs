use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Collision {
    pub point: vec2<Coord>,
    /// Normal vector pointing away from the body.
    pub normal: vec2<Coord>,
    pub penetration: Coord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collider {
    pub position: vec2<Coord>,
    pub rotation: Angle<Coord>,
    pub shape: Shape,
}

impl Collider {
    pub fn new(position: vec2<Coord>, shape: Shape) -> Self {
        Self {
            position,
            rotation: Angle::ZERO,
            shape,
        }
    }

    pub fn compute_aabb(&self) -> Aabb2<Coord> {
        let (iso, shape) = self.to_parry();
        let parry2d::bounding_volume::Aabb { mins, maxs } = shape.compute_aabb(&iso);
        Aabb2 {
            min: vec2(mins.x, mins.y).as_r32(),
            max: vec2(maxs.x, maxs.y).as_r32(),
        }
    }

    fn get_iso(&self) -> parry2d::math::Isometry<f32> {
        let vec2(x, y) = self.position.as_f32();
        let angle = self.rotation.as_radians().as_f32();
        parry2d::math::Isometry::new(parry2d::na::Vector2::new(x, y), angle)
    }

    fn to_parry(&self) -> (parry2d::math::Isometry<f32>, Box<dyn parry2d::shape::Shape>) {
        (self.get_iso(), self.shape.to_parry())
    }

    /// Check whether two colliders are intersecting.
    pub fn check(&self, other: &Self) -> bool {
        let (self_iso, self_shape) = self.to_parry();
        let (other_iso, other_shape) = other.to_parry();
        parry2d::query::intersection_test(&self_iso, &*self_shape, &other_iso, &*other_shape)
            .unwrap()
    }

    /// Return the collision info if the two colliders are intersecting.
    pub fn collide(&self, other: &Self) -> Option<Collision> {
        let (self_iso, self_shape) = self.to_parry();
        let (other_iso, other_shape) = other.to_parry();
        let prediction = 0.0;
        parry2d::query::contact(
            &self_iso,
            &*self_shape,
            &other_iso,
            &*other_shape,
            prediction,
        )
        .unwrap()
        .map(|contact| {
            let normal = contact.normal1.into_inner();
            let point = contact.point1;
            Collision {
                point: vec2(point.x, point.y).map(Coord::new),
                normal: vec2(normal.x, normal.y).map(Coord::new),
                penetration: Coord::new(-contact.dist),
            }
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Shape {
    Circle { radius: Coord },
    Rectangle { width: Coord, height: Coord },
}

impl Shape {
    pub fn circle(radius: Coord) -> Self {
        Self::Circle { radius }
    }

    pub fn rectangle(width: Coord, height: Coord) -> Self {
        Self::Rectangle { width, height }
    }

    fn to_parry(&self) -> Box<dyn parry2d::shape::Shape> {
        match *self {
            Shape::Circle { radius } => Box::new(parry2d::shape::Ball::new(radius.as_f32())),
            Shape::Rectangle { width, height } => {
                let aabb = Aabb2::ZERO.extend_symmetric(vec2(width, height).as_f32() / 2.0);
                let points = aabb.corners().map(|p| {
                    let vec2(x, y) = p;
                    parry2d::math::Point::new(x, y)
                });
                // All rectangles are convex
                Box::new(parry2d::shape::ConvexPolygon::from_convex_hull(&points).unwrap())
            }
        }
    }
}
