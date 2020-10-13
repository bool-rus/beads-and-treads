use crate::grid::Grid;
use crate::beads::BeadsLine;
use std::hash::Hash;
use std::fmt::Debug;
use std::mem;
use crate::entities::{Side, Size};
use std::num::NonZeroUsize;

pub trait ColorTrait: Debug + Clone + Hash + Eq + PartialEq {}

impl<T> ColorTrait for T where T: Debug + Clone + Hash + Eq + PartialEq {}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Bead<T: ColorTrait> {
    pub color: T,
    pub filled: bool,
}

impl<T: ColorTrait + Default> Default for Bead<T> {
    fn default() -> Self {
        Bead {color: T::default(), filled: false}
    }
}

pub struct Model<T: ColorTrait> {
    pub grid: Grid<Bead<T>>,
    pub line: BeadsLine<Bead<T>>,
}

impl<T: ColorTrait> Model<T> {
    fn unfill_grid(&mut self) {
        self.grid = self.grid.map(|Bead { color, ..}|Bead{color: color.clone(), filled: false});
    }
    pub fn grid_color(&self) -> Grid<T> {
        self.grid.map(|bead|bead.color.clone())
    }
    pub fn line_color(&self) -> BeadsLine<T> {
        self.line.map(|bead|bead.color.clone())
    }
    pub fn set(&mut self, row: usize, column: usize, color: T) -> Result<Option<Bead<T>>, String> {
        let prev = self.grid.get_mut(row, column)?;
        if color.eq(&prev.color) {
            Ok(None)
        } else {
            let mut bead = Bead{ color, filled: false };
            mem::swap(prev, &mut bead);
            if bead.filled {
                self.unfill_grid();
            }
            self.line = self.line.knit_type.build(self.grid.as_table());
            Ok(Some(bead))
        }
    }
    pub fn toggle_filled(&mut self, index: usize) -> Result<bool, String> {
        let obj = self.line.get_mut(index).ok_or("Toggle is out of bounds")?;
        let filled = obj.filled;
        obj.filled = !filled;
        self.grid = self.line.grid();
        Ok(filled)
    }

    pub fn grow(&mut self, side: Side, value: T) {
        self.grid = self.grid.map(|Bead {color, ..}| Bead {color: color.clone(), filled: false});
        let value = Bead {color: value, filled: false};
        self.grid.grow(side, value);
        self.line = self.line.knit_type.build(self.grid.as_table());
    }
    pub fn shrink(&mut self, side: Side) -> Result<(), String>{
        self.grid.shrink(side)?;
        self.unfill_grid();
        self.line = self.line.knit_type.build(self.grid.as_table());
        Ok(())
    }
}

impl<T: ColorTrait + Default> Model<T> {
    pub fn resize(&mut self, size: Size) {
        self.grid.resize(size);
        self.unfill_grid();
        self.line = self.line.knit_type.build(self.grid.as_table());
    }
}