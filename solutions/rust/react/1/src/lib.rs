use std::{
    cell::{Cell as StdCell, RefCell},
    collections::HashMap,
    fmt::Debug,
    hash::{Hash, Hasher},
};

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(usize);

type Id = usize;

pub struct InputCell<T> {
    id: Id,
    value: StdCell<T>,
}

impl<T: Copy + Hash> Hash for InputCell<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.value.get().hash(state);
    }
}

/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(Id);

pub struct ComputeCell<'a, T> {
    id: Id,
    value: std::cell::Cell<T>,
    new_value: std::cell::Cell<Option<T>>,
    deps: Vec<CellId>,

    #[allow(clippy::type_complexity)]
    compute: Box<dyn Fn(&[T]) -> T + 'a>,

    #[allow(clippy::type_complexity)]
    callbacks: HashMap<CallbackId, RefCell<Box<dyn FnMut(T) + 'a>>>,
}

impl<'a, T: Copy> ComputeCell<'a, T> {
    pub fn value(&self) -> T {
        self.new_value.get().unwrap_or(self.value.get())
    }
}

impl<'a, T: Copy + Hash> Hash for ComputeCell<'a, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.value.get().hash(state);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(Id);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

impl CellId {
    fn id(&self) -> usize {
        match self {
            CellId::Input(cell) => cell.0,
            CellId::Compute(cell) => cell.0,
        }
    }
}

#[derive(Hash)]
pub enum Cell<'a, T: Copy + Hash> {
    Input(InputCell<T>),
    Compute(ComputeCell<'a, T>),
}

impl<'a, T: Copy + Hash> Cell<'a, T> {
    pub fn id(&self) -> Id {
        match self {
            Cell::Input(cell) => cell.id,
            Cell::Compute(cell) => cell.id,
        }
    }

    pub fn cell_id(&self) -> CellId {
        match self {
            Cell::Input(cell) => CellId::Input(InputCellId(cell.id)),
            Cell::Compute(cell) => CellId::Compute(ComputeCellId(cell.id)),
        }
    }

    fn value(&self) -> T {
        match self {
            Cell::Input(cell) => cell.value.get(),
            Cell::Compute(cell) => cell.value(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UpdateError {
    NonexistentCell,
}

#[derive(Default)]
pub struct Reactor<'a, T: Copy + Hash> {
    cells: HashMap<usize, Cell<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T> Reactor<'a, T>
where
    T: Copy + PartialEq + Debug + Hash,
{
    pub fn new() -> Self {
        Self {
            cells: Default::default(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, value: T) -> InputCellId {
        let id = self.new_cell_id();
        self.cells.insert(
            id,
            Cell::Input(InputCell {
                id,
                value: StdCell::new(value),
            }),
        );
        InputCellId(id)
    }

    fn new_cell_id(&self) -> Id {
        (0..)
            .find(|id| !self.cells.contains_key(id))
            .expect("There is no ids avaialable.")
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        if let Some(cell) = dependencies
            .iter()
            .find(|&cell| !self.cells.contains_key(&cell.id()))
        {
            return Err(*cell);
        }

        let value = self
            .compute_value(dependencies, &compute_func)
            .expect("There must be some value.");

        let id = self.new_cell_id();

        let cell = ComputeCell {
            id,
            value: StdCell::new(value),
            new_value: StdCell::new(None),
            deps: dependencies.to_vec(),
            compute: Box::new(compute_func),
            callbacks: Default::default(),
        };

        self.cells.insert(id, Cell::Compute(cell));

        Ok(ComputeCellId(id))
    }

    fn compute_value<F: Fn(&[T]) -> T>(&self, deps: &[CellId], compute: &F) -> Option<T> {
        let args = deps.iter().try_fold(vec![], |mut acc, &dep| {
            acc.push(self.value(dep)?);
            Some(acc)
        })?;

        Some(compute(&args))
    }

    fn update_dep_cells(&self, cell: &Cell<T>) -> Result<(), UpdateError> {
        let cell_id = cell.cell_id();

        if let Cell::Compute(compute_cell) = cell {
            let new_value = self
                .compute_value(&compute_cell.deps, &compute_cell.compute)
                .expect("There must be some value.");

            let old_value = compute_cell.value();

            if new_value != old_value {
                compute_cell.new_value.set(Some(new_value));

                println!(
                    "COMPUTED CELL IS UPDATED: id = {:?}, [{:?}] -> [{:?}]",
                    cell.id(),
                    old_value,
                    compute_cell.value.get()
                );
            } else {
                println!(
                    "COMPUTED CELL IS NOT UPDATED: id = {:?}, value = {:?}",
                    cell.id(),
                    compute_cell.value.get()
                );
            }
        }

        for dep_cell in self.cells.values() {
            if let Cell::Compute(compute_cell) = dep_cell {
                if compute_cell.deps.contains(&cell_id) {
                    self.update_dep_cells(dep_cell)?;
                }
            }
        }
        Ok(())
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, cell_id: CellId) -> Option<T> {
        Some(self.cells.get(&cell_id.id())?.value())
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&self, _id: InputCellId, _new_value: T) -> bool {
        if let Some(cell) = self.cells.get(&_id.0) {
            if let Cell::Input(input_cell) = cell {
                let old_value = input_cell.value.replace(_new_value);

                println!(
                    "INPUT CELL UPDATED: id = {:?}, [{:?}] -> [{:?}]",
                    cell.id(),
                    old_value,
                    _new_value
                );

                if self.update_dep_cells(cell).is_err() {
                    return false;
                }

                for cell in self.cells.values() {
                    if let Cell::Compute(updated_cell) = cell {
                        if let Some(new_value) = updated_cell.new_value.replace(None) {
                            let old_value = updated_cell.value.replace(new_value);
                            if old_value != new_value {
                                updated_cell
                                    .callbacks
                                    .iter()
                                    .for_each(|(_, cb)| cb.borrow_mut()(updated_cell.value.get()))
                            }
                        }
                    }
                }
            }
            return true;
        }
        false
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        cell_id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        if let Cell::Compute(cell) = self.cells.get_mut(&cell_id.0)? {
            let id = (0..)
                .map(CallbackId)
                .find(|id| !cell.callbacks.contains_key(id))?;

            cell.callbacks.insert(id, RefCell::new(Box::new(callback)));
            return Some(id);
        }
        None
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell_id: ComputeCellId,
        callback_id: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if let Cell::Compute(cell) = self
            .cells
            .get_mut(&cell_id.0)
            .ok_or(RemoveCallbackError::NonexistentCell)?
        {
            if cell.callbacks.remove(&callback_id).is_none() {
                return Err(RemoveCallbackError::NonexistentCallback);
            }
        }
        Ok(())
    }
}
