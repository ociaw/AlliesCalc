use core::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Quant<T> {
    pub item: T,
    pub count: u32,
}

impl<T> Quant<T> {
    pub fn new(item: T, count: u32) -> Quant<T> {
        Quant { item, count }
    }

    pub fn single(item: T) -> Quant<T> {
        Quant { item, count: 1 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct QuantDist<T> {
    outcomes: Vec<Quant<T>>,
    hash: u64,
}

impl<T: Eq + Hash> QuantDist<T> {
    pub fn outcomes(&self) -> &[Quant<T>] {
        &self.outcomes
    }

    pub fn len(&self) -> usize {
        self.outcomes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.outcomes.is_empty()
    }

    pub fn count(&self, item: &T) -> u32 {
        match self.find_index(item) {
            Some(index) => self.outcomes[index].count,
            None => 0,
        }
    }

    fn find_index(&self, item: &T) -> Option<usize> {
        self.outcomes.iter().position(|q| &q.item == item)
    }
}

impl<T: Eq + Hash> From<Vec<Quant<T>>> for QuantDist<T> {
    fn from(outcomes: Vec<Quant<T>>) -> Self {
        let mut builder = QuantDistBuilder::with_capacity(outcomes.len());
        for outcome in outcomes.into_iter() {
            builder.add_quant(outcome);
        }
        builder.build()
    }
}

impl<T: Eq + Hash> Default for QuantDist<T> {
    fn default() -> Self {
        QuantDistBuilder::new().build()
    }
}

impl<T: Eq + Hash> Hash for QuantDist<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantDistBuilder<T: Eq + Hash> {
    outcomes: Vec<Quant<T>>,
}

impl<T: Eq + Hash> QuantDistBuilder<T> {
    pub fn new() -> Self {
        Self {
            outcomes: Vec::<Quant<T>>::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            outcomes: Vec::<Quant<T>>::with_capacity(capacity),
        }
    }

    pub fn build(self) -> QuantDist<T> {
        use core::hash::Hasher;
        let mut hasher = fnv::FnvHasher::default();
        self.outcomes.hash(&mut hasher);
        let hash = hasher.finish();
        QuantDist {
            outcomes: self.outcomes,
            hash,
        }
    }
}

impl<T: Eq + Hash> Default for QuantDistBuilder<T> {
    fn default() -> Self {
        Self {
            outcomes: Vec::<Quant<T>>::new(),
        }
    }
}

impl<T: Eq + Hash> QuantDistBuilder<T> {
    pub fn add(&mut self, item: T, count: u32) {
        self.add_quant(Quant::new(item, count));
    }

    pub fn add_quant(&mut self, quant: Quant<T>) {
        if quant.count == 0 {
            return;
        }
        let index = self.find_index(&quant.item);
        match index {
            Some(index) => self.outcomes[index].count += quant.count,
            None => self.outcomes.push(quant),
        };
    }

    pub fn remove(&mut self, item: &T, count: u32) -> u32 {
        let index = self.find_index(item);
        match index {
            None => 0,
            Some(index) => {
                let removable = self.outcomes[index].count;
                if removable > count {
                    let keep = removable - count;
                    self.outcomes[index].count = keep;
                    count
                } else {
                    self.outcomes.remove(index);
                    removable
                }
            }
        }
    }

    pub fn remove_all(&mut self, item: &T) -> u32 {
        let index = self.find_index(item);
        match index {
            None => 0,
            Some(index) => {
                let removable = self.outcomes[index].count;
                self.outcomes.remove(index);
                removable
            }
        }
    }

    pub fn count(&self, item: &T) -> u32 {
        match self.find_index(item) {
            Some(index) => self.outcomes[index].count,
            None => 0,
        }
    }

    fn find_index(&self, item: &T) -> Option<usize> {
        self.outcomes.iter().position(|q| &q.item == item)
    }
}

impl<T: Eq + Hash> From<QuantDist<T>> for QuantDistBuilder<T> {
    fn from(dist: QuantDist<T>) -> Self {
        QuantDistBuilder {
            outcomes: dist.outcomes,
        }
    }
}
