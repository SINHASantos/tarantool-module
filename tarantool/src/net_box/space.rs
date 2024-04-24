use std::rc::Rc;

use crate::error::Error;
use crate::index::IteratorType;
use crate::tuple::{Encode, ToTuple, Tuple};

use super::index::{RemoteIndex, RemoteIndexIterator};
use super::inner::ConnInner;
use super::options::Options;
use super::protocol;

/// Remote space
pub struct RemoteSpace {
    conn_inner: Rc<ConnInner>,
    space_id: u32,
}

impl RemoteSpace {
    #[inline(always)]
    pub(crate) fn new(conn_inner: Rc<ConnInner>, space_id: u32) -> Self {
        RemoteSpace {
            conn_inner,
            space_id,
        }
    }

    /// Find index by name (on remote space)
    pub fn index(&self, name: &str) -> Result<Option<RemoteIndex>, Error> {
        Ok(self
            .conn_inner
            .lookup_index(name, self.space_id)?
            .map(|index_id| RemoteIndex::new(self.conn_inner.clone(), self.space_id, index_id)))
    }

    /// Returns index with id = 0
    #[inline(always)]
    pub fn primary_key(&self) -> RemoteIndex {
        RemoteIndex::new(self.conn_inner.clone(), self.space_id, 0)
    }

    /// The remote-call equivalent of the local call `Space::get(...)`
    /// (see [details](../space/struct.Space.html#method.get)).
    #[inline(always)]
    pub fn get<K>(&self, key: &K, options: &Options) -> Result<Option<Tuple>, Error>
    where
        K: ToTuple + ?Sized,
    {
        self.primary_key().get(key, options)
    }

    /// The remote-call equivalent of the local call `Space::select(...)`
    /// (see [details](../space/struct.Space.html#method.select)).
    #[inline(always)]
    pub fn select<K>(
        &self,
        iterator_type: IteratorType,
        key: &K,
        options: &Options,
    ) -> Result<RemoteIndexIterator, Error>
    where
        K: ToTuple + ?Sized,
    {
        self.primary_key().select(iterator_type, key, options)
    }

    /// The remote-call equivalent of the local call `Space::insert(...)`
    /// (see [details](../space/struct.Space.html#method.insert)).
    #[inline(always)]
    pub fn insert<T>(&self, value: &T, options: &Options) -> Result<Option<Tuple>, Error>
    where
        T: ToTuple + ?Sized,
    {
        self.conn_inner.request(
            &protocol::Insert {
                space_id: self.space_id,
                value,
            },
            options,
        )
    }

    /// The remote-call equivalent of the local call `Space::replace(...)`
    /// (see [details](../space/struct.Space.html#method.replace)).
    #[inline(always)]
    pub fn replace<T>(&self, value: &T, options: &Options) -> Result<Option<Tuple>, Error>
    where
        T: ToTuple + ?Sized,
    {
        self.conn_inner.request(
            &protocol::Replace {
                space_id: self.space_id,
                value,
            },
            options,
        )
    }

    /// The remote-call equivalent of the local call `Space::update(...)`
    /// (see [details](../space/struct.Space.html#method.update)).
    #[inline(always)]
    pub fn update<K, Op>(
        &self,
        key: &K,
        ops: &[Op],
        options: &Options,
    ) -> Result<Option<Tuple>, Error>
    where
        K: ToTuple + ?Sized,
        Op: Encode,
    {
        self.primary_key().update(key, ops, options)
    }

    /// The remote-call equivalent of the local call `Space::upsert(...)`
    /// (see [details](../space/struct.Space.html#method.upsert)).
    #[inline(always)]
    pub fn upsert<T, Op>(
        &self,
        value: &T,
        ops: &[Op],
        options: &Options,
    ) -> Result<Option<Tuple>, Error>
    where
        T: ToTuple + ?Sized,
        Op: Encode,
    {
        self.primary_key().upsert(value, ops, options)
    }

    /// The remote-call equivalent of the local call `Space::delete(...)`
    /// (see [details](../space/struct.Space.html#method.delete)).
    #[inline(always)]
    pub fn delete<K>(&self, key: &K, options: &Options) -> Result<Option<Tuple>, Error>
    where
        K: ToTuple + ?Sized,
    {
        self.primary_key().delete(key, options)
    }
}
