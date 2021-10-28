// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use cid::Cid;
use ipld_hamt::Hash;
use std::marker::PhantomData;

use crate::errors::Error;
use crate::traits::{BlockStore, HashAlgorithm, HashedBits, Node};

/// This is a simplified implementation of HAMT based on:
/// http://lampwww.epfl.ch/papers/idealhashtrees.pdf
///
/// This implementation has only implemented the read related functions
/// as we only care about generating the path to the node
#[derive(Debug)]
pub struct Hamt<'a, BS, K: Eq, V, H: HashedBits, N: Node<K, V, H>, HashAlgo> {
    root: N,
    store: &'a BS,
    bit_width: u8,
    hash: PhantomData<HashAlgo>,
    _k: PhantomData<K>,
    _v: PhantomData<V>,
    _h: PhantomData<H>,
}

impl<'a, BS, K, V, H, N, HashAlgo> Hamt<'a, BS, K, V, H, N, HashAlgo>
where
    K: Eq + Hash,
    H: HashedBits,
    HashAlgo: HashAlgorithm<Output = H>,
    N: Node<K, V, H>,
    BS: BlockStore<K, V, H, N>,
{
    /// Lazily instantiate a hamt from this root Cid with a specified bit width.
    pub fn new(root_cid: &Cid, store: &'a BS, bit_width: u8) -> Result<Self, Error> {
        match store.get(root_cid)? {
            Some(root) => Ok(Self {
                root,
                store,
                bit_width,
                hash: Default::default(),
                _k: Default::default(),
                _v: Default::default(),
                _h: Default::default(),
            }),
            None => Err(Error::CidNotFound(root_cid.to_string())),
        }
    }

    pub fn generate_proof(&self, k: &K) -> Result<Option<Vec<Vec<u8>>>, Error> {
        let mut path = Vec::new();
        if self.root.path_to_key(
            &mut HashAlgo::hash(k),
            k,
            &mut path,
            self.bit_width,
            self.store,
        )? {
            Ok(Some(path))
        } else {
            Ok(None)
        }
    }
}
