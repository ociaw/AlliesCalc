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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuantDist<T> {
    pub outcomes: Vec<Quant<T>>,
}

impl<T> QuantDist<T> {
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
}

impl<T> Default for QuantDist<T> {
    fn default() -> Self {
        Self {
            outcomes: Vec::<Quant<T>>::new(),
        }
    }
}

impl<T: Eq> QuantDist<T> {
    pub fn add(&mut self, item: T, count: u32) {
        if count == 0 {
            return;
        }
        let index = self.find_index(&item);
        match index {
            Some(index) => self.outcomes[index].count += count,
            None => self.outcomes.push(Quant::new(item, count)),
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
