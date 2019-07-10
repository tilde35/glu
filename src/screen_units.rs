use glium::glutin::dpi::{LogicalPosition, LogicalSize};
use noisy_float::prelude::*;

/// Represents a 2D screen position or size. There are two ways of accessing the
/// data -- either logical (size independent of DPI) or physical (actual pixels).
///
/// Note: All math operations are performed in logical pixel units.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Screen2d {
    logical: [R32; 2],
    hidpi_factor: R32,
}
impl Screen2d {
    pub fn from_logical(logical: [f32; 2], hidpi_factor: f32) -> Self {
        Self {
            logical: [r32(logical[0]), r32(logical[1])],
            hidpi_factor: r32(hidpi_factor),
        }
    }
    pub fn from_physical(physical: [i32; 2], hidpi_factor: f32) -> Self {
        Self::from_physical_f32([physical[0] as f32, physical[1] as f32], hidpi_factor)
    }
    pub fn from_physical_u32(physical: [u32; 2], hidpi_factor: f32) -> Self {
        Self::from_physical_f32([physical[0] as f32, physical[1] as f32], hidpi_factor)
    }
    pub fn from_physical_f32(physical: [f32; 2], hidpi_factor: f32) -> Self {
        let logical = [physical[0] / hidpi_factor, physical[1] / hidpi_factor];
        Self::from_logical(logical, hidpi_factor)
    }
    pub(crate) fn from_logical_r32(logical: [R32;2], hidpi_factor: R32) -> Self {
        Self {
            logical, hidpi_factor
        }
    }
    pub(crate) fn from_logical_position(pos: LogicalPosition, hidpi_factor: R32) -> Self {
        Self {
            logical: [r32(pos.x as f32), r32(pos.y as f32)],
            hidpi_factor,
        }
    }
    pub(crate) fn from_logical_size(pos: LogicalSize, hidpi_factor: R32) -> Self {
        Self {
            logical: [r32(pos.width as f32), r32(pos.height as f32)],
            hidpi_factor,
        }
    }
    pub(crate) fn from_line_delta(
        delta_x: R32,
        delta_y: R32,
        line_factor: R32,
        hidpi_factor: R32,
    ) -> Self {
        Self {
            logical: [delta_x * line_factor, delta_y * line_factor],
            hidpi_factor,
        }
    }
    pub fn logical(&self) -> [f32; 2] {
        [self.logical[0].raw(), self.logical[1].raw()]
    }
    pub fn physical(&self) -> [i32; 2] {
        [
            self.to_physical(self.logical[0]),
            self.to_physical(self.logical[1]),
        ]
    }
    pub fn physical_u32(&self) -> [u32; 2] {
        let phys = self.physical();
        if phys[0] < 0 || phys[1] < 0 {
            panic!(
                "Cannot call physical_u32 method if screen values are negative: {:?}",
                phys
            );
        }
        [phys[0] as u32, phys[1] as u32]
    }
    fn to_physical(&self, v: R32) -> i32 {
        (v * self.hidpi_factor).raw().round() as i32
    }
}
impl std::fmt::Display for Screen2d {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let phys = self.physical();
        write!(
            f,
            "(logical={},{} phys={},{})",
            self.logical[0], self.logical[1], phys[0], phys[1]
        )
    }
}
impl std::fmt::Debug for Screen2d {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl std::ops::Sub for Screen2d {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let a = self.logical;
        let b = other.logical;
        Self {
            logical: [a[0] - b[0], a[1] - b[1]],
            hidpi_factor: self.hidpi_factor,
        }
    }
}
impl std::ops::SubAssign for Screen2d {
    fn sub_assign(&mut self, other: Self) {
        self.logical[0] -= other.logical[0];
        self.logical[1] -= other.logical[1];
    }
}
impl std::ops::Add for Screen2d {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let a = self.logical;
        let b = other.logical;
        Self {
            logical: [a[0] + b[0], a[1] + b[1]],
            hidpi_factor: self.hidpi_factor,
        }
    }
}
impl std::ops::AddAssign for Screen2d {
    fn add_assign(&mut self, other: Self) {
        self.logical[0] += other.logical[0];
        self.logical[1] += other.logical[1];
    }
}
