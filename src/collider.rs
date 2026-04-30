use core::{fmt::Debug, hash::Hash};

use alloc::vec::Vec;
use ggmath::Vec2;

use crate::Configuration;

#[non_exhaustive]
pub enum Collider<Cfg>
where
    Cfg: Configuration,
{
    Rect(RectCollider<Cfg>),
    Line(LineCollider<Cfg>),
    Polygon(PolygonCollider<Cfg>),
}

pub struct RectCollider<Cfg>
where
    Cfg: Configuration,
{
    pub extents: Vec2<Cfg::Num>,
    pub center: Vec2<Cfg::Num>,
}

pub struct LineCollider<Cfg>
where
    Cfg: Configuration,
{
    pub start: Vec2<Cfg::Num>,
    pub end: Vec2<Cfg::Num>,
    pub one_way: bool,
}

pub struct PolygonCollider<Cfg>
where
    Cfg: Configuration,
{
    pub points: Vec<Vec2<Cfg::Num>>,
    pub one_way: bool,
}

impl<Cfg> Collider<Cfg>
where
    Cfg: Configuration,
{
    pub fn rect(extents: Vec2<Cfg::Num>, center: Vec2<Cfg::Num>) -> Self {
        Self::Rect(RectCollider { extents, center })
    }

    pub fn line(start: Vec2<Cfg::Num>, end: Vec2<Cfg::Num>, one_way: bool) -> Self {
        Self::Line(LineCollider {
            start,
            end,
            one_way,
        })
    }

    pub fn polygon(points: Vec<Vec2<Cfg::Num>>, one_way: bool) -> Self {
        Self::Polygon(PolygonCollider { points, one_way })
    }

    #[track_caller]
    pub fn as_rect(&self) -> &RectCollider<Cfg> {
        match self {
            Self::Rect(value) => value,
            _ => panic!("not rect collider"),
        }
    }

    #[track_caller]
    pub fn as_rect_mut(&mut self) -> &mut RectCollider<Cfg> {
        match self {
            Self::Rect(value) => value,
            _ => panic!("not rect collider"),
        }
    }

    pub fn try_as_rect(&self) -> Option<&RectCollider<Cfg>> {
        match self {
            Self::Rect(value) => Some(value),
            _ => None,
        }
    }

    pub fn try_as_rect_mut(&mut self) -> Option<&mut RectCollider<Cfg>> {
        match self {
            Self::Rect(value) => Some(value),
            _ => None,
        }
    }

    #[track_caller]
    pub fn as_line(&self) -> &LineCollider<Cfg> {
        match self {
            Self::Line(value) => value,
            _ => panic!("not line collider"),
        }
    }

    #[track_caller]
    pub fn as_line_mut(&mut self) -> &mut LineCollider<Cfg> {
        match self {
            Self::Line(value) => value,
            _ => panic!("not line collider"),
        }
    }

    pub fn try_as_line(&self) -> Option<&LineCollider<Cfg>> {
        match self {
            Self::Line(value) => Some(value),
            _ => None,
        }
    }

    pub fn try_as_line_mut(&mut self) -> Option<&mut LineCollider<Cfg>> {
        match self {
            Self::Line(value) => Some(value),
            _ => None,
        }
    }

    #[track_caller]
    pub fn as_polygon(&self) -> &PolygonCollider<Cfg> {
        match self {
            Self::Polygon(value) => value,
            _ => panic!("not line collider"),
        }
    }

    #[track_caller]
    pub fn as_polygon_mut(&mut self) -> &mut PolygonCollider<Cfg> {
        match self {
            Self::Polygon(value) => value,
            _ => panic!("not line collider"),
        }
    }

    pub fn try_as_polygon(&self) -> Option<&PolygonCollider<Cfg>> {
        match self {
            Self::Polygon(value) => Some(value),
            _ => None,
        }
    }

    pub fn try_as_polygon_mut(&mut self) -> Option<&mut PolygonCollider<Cfg>> {
        match self {
            Self::Polygon(value) => Some(value),
            _ => None,
        }
    }
}

impl<Cfg> Debug for Collider<Cfg>
where
    Cfg: Configuration,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Rect(value) => value.fmt(f),
            Self::Line(value) => value.fmt(f),
            Self::Polygon(value) => value.fmt(f),
        }
    }
}

impl<Cfg> Debug for RectCollider<Cfg>
where
    Cfg: Configuration,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RectCollider")
            .field("extents", &self.extents)
            .field("offset", &self.center)
            .finish()
    }
}

impl<Cfg> Debug for LineCollider<Cfg>
where
    Cfg: Configuration,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LineCollider")
            .field("start", &self.start)
            .field("end", &self.end)
            .field("one_way", &self.one_way)
            .finish()
    }
}

impl<Cfg> Debug for PolygonCollider<Cfg>
where
    Cfg: Configuration,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PolygonCollider")
            .field("points", &self.points)
            .field("one_way", &self.one_way)
            .finish()
    }
}

impl<Cfg> Clone for Collider<Cfg>
where
    Cfg: Configuration,
{
    fn clone(&self) -> Self {
        match self {
            Self::Rect(value) => Self::Rect(*value),
            Self::Line(value) => Self::Line(*value),
            Self::Polygon(value) => Self::Polygon(value.clone()),
        }
    }
}

impl<Cfg> Clone for RectCollider<Cfg>
where
    Cfg: Configuration,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Cfg> Clone for LineCollider<Cfg>
where
    Cfg: Configuration,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Cfg> Clone for PolygonCollider<Cfg>
where
    Cfg: Configuration,
{
    fn clone(&self) -> Self {
        Self {
            points: self.points.clone(),
            one_way: self.one_way,
        }
    }
}

impl<Cfg> Copy for RectCollider<Cfg> where Cfg: Configuration {}

impl<Cfg> Copy for LineCollider<Cfg> where Cfg: Configuration {}

impl<Cfg> PartialEq for Collider<Cfg>
where
    Cfg: Configuration,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Rect(value), Self::Rect(other)) => value == other,
            (Self::Line(value), Self::Line(other)) => value == other,
            (Self::Polygon(value), Self::Polygon(other)) => value == other,
            (Self::Rect(_) | Self::Line(_) | Self::Polygon(_), _) => false,
        }
    }
}

impl<Cfg> PartialEq for RectCollider<Cfg>
where
    Cfg: Configuration,
{
    fn eq(&self, other: &Self) -> bool {
        self.extents == other.extents && self.center == other.center
    }
}

impl<Cfg> PartialEq for LineCollider<Cfg>
where
    Cfg: Configuration,
{
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end && self.one_way == other.one_way
    }
}

impl<Cfg> PartialEq for PolygonCollider<Cfg>
where
    Cfg: Configuration,
{
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points && self.one_way == other.one_way
    }
}

impl<Cfg> Eq for Collider<Cfg> where Cfg: Configuration<Num: Eq> {}

impl<Cfg> Eq for RectCollider<Cfg> where Cfg: Configuration<Num: Eq> {}

impl<Cfg> Eq for LineCollider<Cfg> where Cfg: Configuration<Num: Eq> {}

impl<Cfg> Eq for PolygonCollider<Cfg> where Cfg: Configuration<Num: Eq> {}

impl<Cfg> Hash for Collider<Cfg>
where
    Cfg: Configuration<Num: Hash>,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Rect(value) => {
                0_u8.hash(state);
                value.hash(state);
            }
            Self::Line(value) => {
                1_u8.hash(state);
                value.hash(state);
            }
            Self::Polygon(value) => {
                2_u8.hash(state);
                value.hash(state);
            }
        }
    }
}

impl<Cfg> Hash for RectCollider<Cfg>
where
    Cfg: Configuration<Num: Hash>,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.extents.hash(state);
        self.center.hash(state);
    }
}

impl<Cfg> Hash for LineCollider<Cfg>
where
    Cfg: Configuration<Num: Hash>,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.end.hash(state);
        self.one_way.hash(state);
    }
}

impl<Cfg> Hash for PolygonCollider<Cfg>
where
    Cfg: Configuration<Num: Hash>,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.points.hash(state);
        self.one_way.hash(state);
    }
}
