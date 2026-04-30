use core::fmt::Debug;

use alloc::vec::Vec;
use ggmath::Vec2;

use crate::Num;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Collider<T>
where
    T: Num,
{
    Rect(RectCollider<T>),
    Line(LineCollider<T>),
    Polygon(PolygonCollider<T>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RectCollider<T>
where
    T: Num,
{
    pub extents: Vec2<T>,
    pub center: Vec2<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineCollider<T>
where
    T: Num,
{
    pub start: Vec2<T>,
    pub end: Vec2<T>,
    pub one_way: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PolygonCollider<T>
where
    T: Num,
{
    pub points: Vec<Vec2<T>>,
    pub one_way: bool,
}

impl<T> Collider<T>
where
    T: Num,
{
    pub fn rect(extents: Vec2<T>, center: Vec2<T>) -> Self {
        Self::Rect(RectCollider { extents, center })
    }

    pub fn line(start: Vec2<T>, end: Vec2<T>, one_way: bool) -> Self {
        Self::Line(LineCollider {
            start,
            end,
            one_way,
        })
    }

    pub fn polygon(points: Vec<Vec2<T>>, one_way: bool) -> Self {
        Self::Polygon(PolygonCollider { points, one_way })
    }

    #[track_caller]
    pub fn as_rect(&self) -> &RectCollider<T> {
        match self {
            Self::Rect(value) => value,
            _ => panic!("not rect collider"),
        }
    }

    #[track_caller]
    pub fn as_rect_mut(&mut self) -> &mut RectCollider<T> {
        match self {
            Self::Rect(value) => value,
            _ => panic!("not rect collider"),
        }
    }

    pub fn try_as_rect(&self) -> Option<&RectCollider<T>> {
        match self {
            Self::Rect(value) => Some(value),
            _ => None,
        }
    }

    pub fn try_as_rect_mut(&mut self) -> Option<&mut RectCollider<T>> {
        match self {
            Self::Rect(value) => Some(value),
            _ => None,
        }
    }

    #[track_caller]
    pub fn as_line(&self) -> &LineCollider<T> {
        match self {
            Self::Line(value) => value,
            _ => panic!("not line collider"),
        }
    }

    #[track_caller]
    pub fn as_line_mut(&mut self) -> &mut LineCollider<T> {
        match self {
            Self::Line(value) => value,
            _ => panic!("not line collider"),
        }
    }

    pub fn try_as_line(&self) -> Option<&LineCollider<T>> {
        match self {
            Self::Line(value) => Some(value),
            _ => None,
        }
    }

    pub fn try_as_line_mut(&mut self) -> Option<&mut LineCollider<T>> {
        match self {
            Self::Line(value) => Some(value),
            _ => None,
        }
    }

    #[track_caller]
    pub fn as_polygon(&self) -> &PolygonCollider<T> {
        match self {
            Self::Polygon(value) => value,
            _ => panic!("not line collider"),
        }
    }

    #[track_caller]
    pub fn as_polygon_mut(&mut self) -> &mut PolygonCollider<T> {
        match self {
            Self::Polygon(value) => value,
            _ => panic!("not line collider"),
        }
    }

    pub fn try_as_polygon(&self) -> Option<&PolygonCollider<T>> {
        match self {
            Self::Polygon(value) => Some(value),
            _ => None,
        }
    }

    pub fn try_as_polygon_mut(&mut self) -> Option<&mut PolygonCollider<T>> {
        match self {
            Self::Polygon(value) => Some(value),
            _ => None,
        }
    }
}
