pub struct MemoryManager {
    pub memory_map: Vec<Option<String>>,
}

impl MemoryManager {
    fn find_available_index(&self, size: usize) -> Option<usize> {
        // search for an index with available memory
        // potential off-by-one here?
        let mut i = 0;
        while i < self.memory_map.len() - size {
            let is_region_empty = (i..i + size).all(|n| self.memory_map[n].is_none());

            if is_region_empty {
                return Some(i);
            }

            // TODO: maybe optimize by skipping to the end of the region?
            i += 1;
        }

        None
    }

    pub fn allocate(&mut self, key: String, size: usize) -> Result<usize, String> {
        if size > self.memory_map.len() {
            let error_message = format!(
                "{} takes too much memory: {} vs {} available",
                key,
                size,
                self.memory_map.len()
            );
            return Err(error_message);
        }

        let available_memory_index = self.find_available_index(size);

        let available_memory_index = match available_memory_index {
            None => return Err(String::from("Out of memory")),
            Some(idx) => idx,
        };

        for i in available_memory_index..available_memory_index + size {
            // mark all of the memory as allocated
            self.memory_map[i] = Some(key.clone())
        }

        Ok(available_memory_index)
    }

    pub fn deallocate(&mut self, key: &str) {
        for i in 0..self.memory_map.len() {
            match &self.memory_map[i] {
                None => {
                    continue;
                }
                Some(value) => {
                    if value != key {
                        continue;
                    }
                }
            }

            self.memory_map[i] = None;
        }
    }

    pub fn get_memory_location(&self, key: String) -> Option<usize> {
        for i in 0..self.memory_map.len() {
            if self.memory_map[i] == Some(key.clone()) {
                return Some(i);
            }
        }

        None
    }
}
