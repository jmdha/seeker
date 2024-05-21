pub mod heuristic;
pub mod search;

use fxhash::FxBuildHasher;
use indexmap::IndexMap;

type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;
