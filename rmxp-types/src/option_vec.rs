// Copyright (C) 2023 Lily Lyons
//
// This file is part of Luminol.
//
// Luminol is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Luminol is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Luminol.  If not, see <http://www.gnu.org/licenses/>.

use std::ops::{Index, IndexMut};

use serde::ser::SerializeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A vector that can contain unused indices.
pub struct OptionVec<T> {
    vec: Vec<Option<T>>,
}

pub struct Iter<'a, T> {
    vec_iter: std::iter::Enumerate<std::slice::Iter<'a, Option<T>>>,
}

pub struct IterMut<'a, T> {
    vec_iter: std::iter::Enumerate<std::slice::IterMut<'a, Option<T>>>,
}

pub struct Visitor<T>(std::marker::PhantomData<T>);

impl<T> OptionVec<T> {
    /// Create a new OptionVec with no elements.
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.vec.get(index).map_or(None, |x| x.as_ref())
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.vec.get_mut(index).map_or(None, |x| x.as_mut())
    }

    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.vec.reserve(additional);
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.into_iter()
    }

    /// Write the element at the given index.
    /// If there is already an element at the given index, it will be overwritten.
    /// If there isn't, a new element will be added at that index.
    pub fn insert(&mut self, index: usize, element: T) {
        if index >= self.len() {
            let additional = self.len() - index + 1;
            self.reserve(additional);
            self.vec
                .extend(std::iter::repeat_with(|| None).take(additional));
        }
        self.vec[index] = Some(element);
    }

    /// Remove the element at the given index.
    /// If the OptionVec is not big enough to contain this index, this will throw an error.
    /// If there isn't an element at that index, this will throw an error.
    pub fn try_remove(&mut self, index: usize) -> Result<(), String> {
        if index >= self.len() {
            Err(String::from("index out of bounds"))
        } else if self.vec[index].is_none() {
            Err(String::from("index not found"))
        } else {
            self.vec[index] = None;
            Ok(())
        }
    }

    /// Remove the element at the given index.
    /// If the OptionVec is not big enough to contain this index, this will panic.
    /// If there isn't an element at that index, this will panic.
    pub fn remove(&mut self, index: usize) {
        self.try_remove(index).unwrap()
    }
}

impl<T> Default for OptionVec<T> {
    fn default() -> Self {
        OptionVec::new()
    }
}

impl<T> FromIterator<(usize, T)> for OptionVec<T> {
    fn from_iter<I: IntoIterator<Item = (usize, T)>>(iterable: I) -> Self {
        let mut vec = Vec::new();
        for (i, v) in iterable.into_iter() {
            if i >= vec.len() {
                let additional = i - vec.len() + 1;
                vec.reserve(additional);
                vec.extend(std::iter::repeat_with(|| None).take(additional));
            }
            vec[i] = Some(v);
        }
        Self { vec }
    }
}

impl<T> Index<usize> for OptionVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("index not found")
    }
}

impl<T> IndexMut<usize> for OptionVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("index not found")
    }
}

impl<'a, T> IntoIterator for &'a OptionVec<T> {
    type Item = (usize, &'a T);
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            vec_iter: self.vec.iter().enumerate(),
        }
    }
}

impl<'a, T> IntoIterator for &'a mut OptionVec<T> {
    type Item = (usize, &'a mut T);
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            vec_iter: self.vec.iter_mut().enumerate(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (usize, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        for (index, element) in &mut self.vec_iter {
            if let Some(element) = element {
                return Some((index, element));
            }
        }
        None
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (usize, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        for (index, element) in &mut self.vec_iter {
            if let Some(element) = element {
                return Some((index, element));
            }
        }
        None
    }
}

impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
where
    T: serde::Deserialize<'de>,
{
    type Value = OptionVec<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a key-value mapping")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        std::iter::from_fn(|| match map.next_entry() {
            Ok(Some(x)) => Some(Ok(x)),
            Err(e) => Some(Err(e)),
            Ok(None) => None,
        })
        .collect()
    }
}

impl<'de, T> serde::Deserialize<'de> for OptionVec<T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(Visitor(std::marker::PhantomData))
    }
}

impl<T> serde::Serialize for OptionVec<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut ser = serializer.serialize_map(Some(self.len()))?;
        for (index, element) in self {
            ser.serialize_key(&index)?;
            ser.serialize_value(element)?;
        }
        ser.end()
    }
}
