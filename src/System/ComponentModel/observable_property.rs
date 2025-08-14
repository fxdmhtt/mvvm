#![allow(non_snake_case)]

use event_rs::Event;

/// Represents a property that can be observed for changes.
///
/// This struct allows subscribing to `PropertyChanging` and `PropertyChanged` events.
/// When the value is about to change, `PropertyChanging` is invoked. After the value
/// is updated, `PropertyChanged` is invoked.
///
/// # Type Parameters
/// * `T`: The type of the property value. Must implement `Eq` to allow equality comparison.
///
/// # Examples
/// ```
/// use std::{cell::RefCell, rc::Rc};
/// use mvvm::System::ComponentModel::ObservableProperty;
///
/// let log = Rc::new(RefCell::new(Vec::new()));
/// let mut prop = ObservableProperty::<i32>::default();
///
/// prop.PropertyChanging.add(|p| (*log.borrow_mut()).push(format!("Changing from {}", p.get())));
/// prop.PropertyChanged.add(|p| (*log.borrow_mut()).push(format!("Changed to {}", p.get())));
///
/// prop.set(42);
///
/// assert_eq!(prop.get(), 42);
/// assert_eq!(*log.borrow(), vec!["Changing from 0", "Changed to 42"]);
/// ```
#[derive(Default)]
pub struct ObservableProperty<'a, T>
where
    T: Eq + Default,
{
    /// The internal value of the property.
    value: T,

    /// Event Invoked after the value has changed.
    pub PropertyChanged: Event<'a, ObservableProperty<'a, T>>,

    /// Event Invoked before the value changes.
    pub PropertyChanging: Event<'a, ObservableProperty<'a, T>>,
}

impl<'a, T> ObservableProperty<'a, T>
where
    T: Eq + Default,
{
    /// Creates a new `ObservableProperty` with the specified initial value.
    ///
    /// The `PropertyChanging` and `PropertyChanged` events are initialized but do not
    /// have any subscribers initially.
    ///
    /// # Parameters
    /// - `value`: The initial value of the property.
    ///
    /// # Examples
    /// ```
    /// use mvvm::System::ComponentModel::ObservableProperty;
    ///
    /// let mut prop = ObservableProperty::new(10);
    /// assert_eq!(prop.get(), 10);
    ///
    /// prop.set(20);
    /// assert_eq!(prop.get(), 20);
    /// ```
    pub fn new(value: T) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }

    /// Invokes the `PropertyChanged` event.
    fn OnPropertyChanged(&self) {
        self.PropertyChanged.invoke(self)
    }

    /// Invokes the `PropertyChanging` event.
    fn OnPropertyChanging(&self) {
        self.PropertyChanging.invoke(self)
    }

    /// Compares the current and new values for a given property. If the value has changed,
    /// raises the `PropertyChanging` event, updates the property with the new value,
    /// then raises the `PropertyChanged` event.
    ///
    /// Returns `true` if the property was changed, `false` otherwise.
    ///
    /// The `PropertyChanging` and `PropertyChanged` events are not raised
    /// if the current and new value for the target property are the same.
    fn SetProperty(&mut self, value: T) -> bool {
        if self.value == value {
            return false;
        }

        self.OnPropertyChanging();

        self.value = value;

        self.OnPropertyChanged();

        true
    }

    /// Gets a reference to the current value of the property.
    pub fn GetValue(&self) -> &T {
        &self.value
    }

    /// Compares the current and new values for a given property. If the value has changed,
    /// raises the `PropertyChanging` event, updates the property with the new value,
    /// then raises the `PropertyChanged` event.
    ///
    /// Returns `true` if the property was changed, `false` otherwise.
    ///
    /// The `PropertyChanging` and `PropertyChanged` events are not raised
    /// if the current and new value for the target property are the same.
    pub fn SetValue(&mut self, value: T) -> bool {
        self.SetProperty(value)
    }

    /// Gets a clone of the current property value.
    ///
    /// Requires that `T` implements `Clone`.
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.GetValue().clone()
    }

    /// Compares the current and new values for a given property. If the value has changed,
    /// raises the `PropertyChanging` event, updates the property with the new value,
    /// then raises the `PropertyChanged` event.
    ///
    /// Returns `true` if the property was changed, `false` otherwise.
    ///
    /// The `PropertyChanging` and `PropertyChanged` events are not raised
    /// if the current and new value for the target property are the same.
    pub fn set(&mut self, value: T) -> bool {
        self.SetValue(value)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    #[test]
    fn test_observable_property() {
        let counter = Rc::new(RefCell::new(0));
        let mut prop = ObservableProperty::<i32>::default();

        prop.PropertyChanging.add(|p| {
            assert_eq!(p.get(), 0);
            *counter.borrow_mut() += 1 << p.get();
        });

        prop.PropertyChanged.add(|p| {
            assert_eq!(p.get(), 2);
            *counter.borrow_mut() += 1 << p.get();
        });

        assert_eq!(prop.get(), 0);

        prop.set(2);

        assert_eq!(prop.get(), 2);

        assert_eq!(*counter.borrow(), 0b101);
    }
}
