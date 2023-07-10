use std::collections::HashMap;
#[derive(Default)]
struct SnapshotArray {
    values: HashMap<i32, i32>,
    snapshots: Vec<HashMap<i32, i32>>,
}

impl SnapshotArray {
    fn new(_length: i32) -> Self {
        Self::default()
    }

    fn set(&mut self, index: i32, val: i32) {
        self.values.insert(index, val);
    }

    fn snap(&mut self) -> i32 {
        self.snapshots.push(self.values.clone());
        (self.snapshots.len() - 1) as i32
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        self.snapshots[snap_id as usize]
            .get(&index)
            .unwrap_or(&0)
            .clone()
    }
}
