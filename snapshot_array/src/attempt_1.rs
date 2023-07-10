struct SnapshotArray {
    values: Vec<i32>,
    snapshots: Vec<Vec<i32>>,
}

impl SnapshotArray {
    fn new(_length: i32) -> Self {
        Self {
            values: vec![0; _length as usize],
            snapshots: vec![],
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.values[index as usize] = val;
    }

    fn snap(&mut self) -> i32 {
        self.snapshots.push(self.values.clone());
        (self.snapshots.len() - 1) as i32
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        self.snapshots[snap_id as usize][index as usize]
    }
}
