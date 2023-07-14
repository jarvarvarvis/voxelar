pub struct PerFrame<T> {
    values: Vec<T>,
    current_frame_index: usize,
}

impl<T> PerFrame<T> {
    pub fn empty() -> Self {
        Self {
            values: vec![],
            current_frame_index: 0
        }
    }

    pub fn init<C: Fn() -> T>(constructor: C, count: usize) -> Self {
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push((constructor)());
        }

        Self {
            values,
            current_frame_index: 0,
        }
    }

    pub fn try_init<C: Fn() -> crate::Result<T>>(
        constructor: C,
        count: usize,
    ) -> crate::Result<Self> {
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push((constructor)()?);
        }

        Ok(Self {
            values,
            current_frame_index: 0,
        })
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn select(&mut self, current_frame_index: usize) {
        self.current_frame_index = current_frame_index;
    }

    pub fn get_select(&mut self, current_frame_index: usize) -> &T {
        self.current_frame_index = current_frame_index;
        &self.values[self.current_frame_index]
    }

    pub fn get_select_mut(&mut self, current_frame_index: usize) -> &mut T {
        self.current_frame_index = current_frame_index;
        &mut self.values[self.current_frame_index]
    }

    pub fn get(&self, current_frame_index: usize) -> &T {
        &self.values[current_frame_index]
    }

    pub fn get_mut(&mut self, current_frame_index: usize) -> &mut T {
        &mut self.values[current_frame_index]
    }

    pub fn current(&self) -> &T {
        &self.values[self.current_frame_index]
    }

    pub fn current_mut(&mut self) -> &mut T {
        &mut self.values[self.current_frame_index]
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.values.iter_mut()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<T> {
        self.values.into_iter()
    }
}

impl<T> IntoIterator for PerFrame<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}
