use crate::prelude::*;

/// Display suitable for frontend.
/// Changes [`Debug`] and [`Display`]
pub enum PlaceItemError<T>
where
  T: TableDescriptor,
{
  CouldntInsertItem(surrealdb::Error),
  MultipleItemsReturned(Vec<<T as TableDescriptor>::Id>),
  NoItemsReturned,
}

impl<T> std::error::Error for PlaceItemError<T>
where
  T: TableDescriptor,
{
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::CouldntInsertItem(err) => Some(err as &dyn std::error::Error),
      _ => None,
    }
  }
}

impl<T> From<surrealdb::Error> for PlaceItemError<T>
where
  T: TableDescriptor,
{
  fn from(err: surrealdb::Error) -> Self {
    PlaceItemError::CouldntInsertItem(err)
  }
}

impl<T> PlaceItemError<T>
where
  T: TableDescriptor,
{
  pub fn handle_vec(items: Vec<T>) -> Result<T, Self> {
    if items.len() > 1 {
      Err(PlaceItemError::MultipleItemsReturned(
        items.into_iter().map(|item| item.id()).collect(),
      ))
    } else {
      items
        .into_iter()
        .next()
        .ok_or(PlaceItemError::NoItemsReturned)
    }
  }
}

impl<T> std::fmt::Debug for PlaceItemError<T>
where
  T: TableDescriptor,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let name = format!("Place{}Error", T::debug_name());
    match self {
      PlaceItemError::CouldntInsertItem(err) => {
        write!(f, "{name}::CouldntInsert{}({err:?})", T::debug_name())
      }
      PlaceItemError::MultipleItemsReturned(ids) => {
        write!(
          f,
          "{name}::Multiple{}Returned({ids:?})",
          T::debug_name_plural()
        )
      }
      PlaceItemError::NoItemsReturned => write!(f, "{name}::No{}Returned", T::debug_name()),
    }
  }
}

impl<T> std::fmt::Display for PlaceItemError<T>
where
  T: TableDescriptor,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      PlaceItemError::CouldntInsertItem(_err) => write!(f, "Couldn't place order"),
      PlaceItemError::MultipleItemsReturned(_ids) => {
        write!(f, "Error placing order (More than one order returned)")
      }
      PlaceItemError::NoItemsReturned => write!(f, "Error placing order (No order returned)"),
    }
  }
}
