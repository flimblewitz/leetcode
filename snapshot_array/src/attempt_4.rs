#[derive(Debug, Default)]
struct SnapshotArray {
    snapshot_count: i32,
    // Vec<(index, Vec<(first_snapshot_with_this_value, greatest_snapshot_id_of_consecutive_snapshots_with_unchanging_val, val))
    values: Vec<(i32, Vec<(i32, i32, i32)>)>,
}

impl SnapshotArray {
    fn new(_length: i32) -> Self {
        Self::default()
    }

    fn set(&mut self, index: i32, val: i32) {
        if self.values.is_empty() {
            self.values
                .push((index, vec![(self.snapshot_count, self.snapshot_count, val)]));
        }
        // I need to binary search for the target index
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
        // if it does exist, I have to carefully update the value_snapshot
        // otherwise, I insert it
        if index == self.values[r].0 {
            let value_snapshots = &mut self.values[r].1;
            let last_value_snapshot = value_snapshots.last_mut().unwrap();
            // if we're still on the first snapshot, then there's no information to preserve
            if self.snapshot_count == 0 {
                last_value_snapshot.2 = val;
            } else if val != last_value_snapshot.2 {
                value_snapshots.push((self.snapshot_count, self.snapshot_count, val));
            }
        } else if index < self.values[r].0 {
            self.values.insert(
                r,
                (index, vec![(self.snapshot_count, self.snapshot_count, val)]),
            );
        } else {
            self.values
                .push((index, vec![(self.snapshot_count, self.snapshot_count, val)]));
        }
    }

    fn snap(&mut self) -> i32 {
        for (_, value_snapshots) in self.values.iter_mut() {
            // actually I just need to increment the snapshot_id of the last item
            value_snapshots.last_mut().unwrap().0 += 1;
        }
        self.snapshot_count += 1;
        self.snapshot_count - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        // println!("asking for index {index}, snap_id {snap_id}");
        // println!("current self: {self:?}");
        if self.values.is_empty() {
            return 0;
        }
        // first, we need to binary search for that index
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
        // if no values were ever set for that index, just give up now
        if index != self.values[r].0 {
            return 0;
        }

        let value_snapshots = &self.values[r].1;
        let mut l = 0;
        let mut r = value_snapshots.len() - 1;
        while l < r {
            let m = (l + r) / 2;
            if snap_id <= value_snapshots[m].0 {
                r = m;
            } else {
                l = m + 1;
            }
        }
        // what makes this part weird is that the way I'm tracking differences for current vs. the last snapshot is that there are up to two value_snapshots with the .0 equal to the current snapshot_count. If we need an older snapshot than the current one, we can use what we have. Otherwise we need to use the most current value and choose the latest snapshot_value
        if snap_id == self.snapshot_count
            && r < value_snapshots.len() - 1
            && snap_id == value_snapshots[r + 1].0
        {
            r += 1;
        }
        if value_snapshots[r].1 <= snap_id {
            value_snapshots[r].2
        } else {
            0
        }
    }
}
