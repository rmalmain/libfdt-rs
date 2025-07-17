use crate::error::Error;
use crate::{Fdt, FdtNode, FdtProperty};

pub struct FdtNodeIter<'fdt> {
    fdt: &'fdt Fdt,
    next: Option<FdtNode<'fdt>>,
}

pub struct FdtPropertyIter<'fdt> {
    fdt: &'fdt Fdt,
    next: Option<FdtProperty<'fdt>>,
}

impl<'fdt> FdtNodeIter<'fdt> {
    pub fn new(node: &FdtNode<'fdt>) -> Result<Self, Error> {
        Ok(Self {
            fdt: node.fdt,
            next: node.fdt.first_subnode(node)?,
        })
    }
}

impl<'fdt> FdtPropertyIter<'fdt> {
    pub fn new(node: &FdtNode<'fdt>) -> Result<Self, Error> {
        Ok(Self {
            fdt: node.fdt,
            next: node.fdt.first_property(node)?,
        })
    }
}

impl<'fdt> Iterator for FdtNodeIter<'fdt> {
    type Item = FdtNode<'fdt>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.next.take() {
            self.next = self.fdt.next_subnode(&current).unwrap();
            Some(current)
        } else {
            None
        }
    }
}

impl<'fdt> Iterator for FdtPropertyIter<'fdt> {
    type Item = FdtProperty<'fdt>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.next.take() {
            self.next = self.fdt.next_property(&current).unwrap();
            Some(current)
        } else {
            None
        }
    }
}
