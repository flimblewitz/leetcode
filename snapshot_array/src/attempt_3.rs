#[derive(Default)]
struct SnapshotArray {
    values: Vec<(i32, i32)>,
    snapshots: Vec<Vec<(i32, i32)>>,
}

impl SnapshotArray {
    fn new(_length: i32) -> Self {
        Self::default()
    }

    fn set(&mut self, index: i32, val: i32) {
        // println!("values: {:?}", self.values);
        // println!("setting index {index}, val {val}");
        if self.values.is_empty() {
            self.values.push((index, val));
            return;
        }

        let mut l = 0;
        let mut r = self.values.len() - 1;
        while l < r {
            let m = (l + r) / 2;
            if index <= self.values[m].0 {
                r = m;
            } else {
                l = m + 1;
            }
        }
        // so either r is the target
        // or it's bigger
        // or it's smaller but it's the biggest thing we have

        if index == self.values[r].0 {
            self.values[r].1 = val;
        } else if index < self.values[r].0 {
            self.values.insert(r, (index, val));
        } else {
            self.values.push((index, val));
        }
    }

    fn snap(&mut self) -> i32 {
        self.snapshots.push(self.values.clone());
        (self.snapshots.len() - 1) as i32
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let snapshot = &self.snapshots[snap_id as usize];
        if snapshot.is_empty() {
            return 0;
        }
        let mut l = 0;
        let mut r = snapshot.len() - 1;
        while l < r {
            let m = (l + r) / 2;
            if index <= snapshot[m].0 {
                r = m;
            } else {
                l = m + 1;
            }
        }
        if index == snapshot[r].0 {
            snapshot[r].1.clone()
        } else {
            0
        }
    }
}
