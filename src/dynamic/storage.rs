//! Dynamic tensor storage: `Arc<Vec<Element>>` + view metadata + CoW (RFC-012).
//!
//! # Architecture
//!
//! A dynamic tensor wraps a shared `Arc<Vec<Element>>`. Slicing and reshape
//! create new `DynamicTensor` structs that share the same underlying storage
//! via cloned `Arc`s — no element copies. Mutation (if/when added) uses CoW:
//! `Arc::make_mut` materialises a private copy only when the storage is
//! actually shared.
//!
//! ## View representation
//!
//! Two cases are handled:
//!
//! - **`ViewKind::Contiguous`** — flat slice of storage from `offset` for
//!   `len` elements; strides are implicit row-major. Covers construction,
//!   reshape (when element count matches), and axis-aligned slices.
//!
//! - **`ViewKind::Indexed(Vec<usize>)`** — explicit list of storage indices
//!   for each logical element. Covers non-contiguous slices that cannot be
//!   described by a single offset+stride. Correct but potentially larger
//!   metadata footprint; a future RFC may add a stride view optimisation.
//!
//! Materialisation converts either form into a fresh owned `Vec<Element>` in
//! row-major order, producing a `Contiguous` view.

use super::element::Element;
use std::sync::Arc;

/// View kind stored inside a [`DynamicTensor`].
#[cfg(feature = "dynamic")]
#[derive(Debug, Clone)]
#[allow(dead_code)] // Indexed is used in tests and future mutation APIs
pub(crate) enum ViewKind {
    /// Logical elements are `storage[offset .. offset + len]` in row-major order.
    Contiguous { offset: usize },
    /// Logical element `i` is at `storage[indices[i]]`.
    Indexed(Vec<usize>),
}

/// Internal representation of a dynamic (heterogeneous) tensor.
///
/// Users never see this struct; it is only accessible through the public
/// `Tensor` API methods that are `#[cfg(feature = "dynamic")]`.
#[cfg(feature = "dynamic")]
#[derive(Debug, Clone)]
pub(crate) struct DynamicTensor {
    pub(crate) storage: Arc<Vec<Element>>,
    pub(crate) shape: Vec<usize>,
    pub(crate) len: usize, // cached product of shape
    pub(crate) view: ViewKind,
}

#[cfg(feature = "dynamic")]
#[allow(dead_code)] // methods used in tests; CoW mutation APIs land later
impl DynamicTensor {
    /// Creates a contiguous dynamic tensor from owned element data.
    #[allow(dead_code)] // used in tests and public API methods
    pub(crate) fn from_vec(data: Vec<Element>, shape: Vec<usize>) -> Self {
        let len = data.len();
        DynamicTensor {
            storage: Arc::new(data),
            shape,
            len,
            view: ViewKind::Contiguous { offset: 0 },
        }
    }

    /// Returns the element at logical position `flat` in the tensor.
    pub(crate) fn get_flat(&self, flat: usize) -> Option<&Element> {
        if flat >= self.len {
            return None;
        }
        let storage_idx = match &self.view {
            ViewKind::Contiguous { offset } => offset + flat,
            ViewKind::Indexed(idxs) => idxs[flat],
        };
        self.storage.get(storage_idx)
    }

    /// Returns `true` if the underlying storage is not shared with another tensor.
    pub(crate) fn is_unique(&self) -> bool {
        Arc::strong_count(&self.storage) == 1
    }

    /// Materialises the logical elements into a fresh contiguous `Vec<Element>`,
    /// resetting this tensor to a `Contiguous` view. No-op if already contiguous
    /// and uniquely owned.
    pub(crate) fn materialize(&mut self) {
        match &self.view {
            ViewKind::Contiguous { offset: 0 } if self.is_unique() => return,
            _ => {}
        }
        let data: Vec<Element> = (0..self.len)
            .map(|i| self.get_flat(i).cloned().unwrap_or(Element::None))
            .collect();
        self.storage = Arc::new(data);
        self.view = ViewKind::Contiguous { offset: 0 };
    }

    /// Creates a slice sharing storage with this tensor. The slice covers
    /// `indices` (logical flat indices into *this* tensor's logical layout).
    pub(crate) fn slice_indices(
        &self,
        indices: Vec<usize>,
        new_shape: Vec<usize>,
    ) -> DynamicTensor {
        let new_len = indices.len();
        // Map logical indices of this tensor to storage indices.
        let storage_indices: Vec<usize> = indices
            .iter()
            .map(|&i| match &self.view {
                ViewKind::Contiguous { offset } => offset + i,
                ViewKind::Indexed(idxs) => idxs[i],
            })
            .collect();

        DynamicTensor {
            storage: Arc::clone(&self.storage),
            shape: new_shape,
            len: new_len,
            view: ViewKind::Indexed(storage_indices),
        }
    }

    /// Returns a reshape of this tensor if the element count matches.
    /// Shares storage; no element copy.
    pub(crate) fn reshape(&self, new_shape: Vec<usize>) -> Option<DynamicTensor> {
        let new_len: usize = if new_shape.is_empty() {
            1
        } else {
            new_shape.iter().product()
        };
        if new_len != self.len {
            return None;
        }
        Some(DynamicTensor {
            storage: Arc::clone(&self.storage),
            shape: new_shape,
            len: new_len,
            view: self.view.clone(),
        })
    }

    /// Converts to a flat `Vec<Element>` in logical row-major order.
    pub(crate) fn to_vec(&self) -> Vec<Element> {
        (0..self.len)
            .map(|i| self.get_flat(i).cloned().unwrap_or(Element::None))
            .collect()
    }
}
