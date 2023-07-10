fn main() {
    {
        // test case 1
        let mut s = SnapshotArray::new(3);
        s.set(0, 5);
        let snap_id = s.snap();
        assert_eq!(snap_id, 0);
        s.set(0, 6);
        let v = s.get(0, 0);
        assert_eq!(v, 5);
    }
    {
        // test case 4
        let mut s = SnapshotArray::new(4);
        let snap_id = s.snap();
        assert_eq!(snap_id, 0);
        let snap_id = s.snap();
        assert_eq!(snap_id, 1);
        let v = s.get(3, 1);
        assert_eq!(v, 0);
        s.set(2, 4);
        let snap_id = s.snap();
        assert_eq!(snap_id, 2);
        s.set(1, 4);
    }
    {
        // test case 29
        let mut s = SnapshotArray::new(2);
        // snap
        assert_eq!(s.snap(), 0);
        // get 1,0
        assert_eq!(s.get(1, 0), 0);
        // get 0,0
        assert_eq!(s.get(0, 0), 0);
        // set 1,8
        s.set(1, 8);
        // get 1,0
        assert_eq!(s.get(1, 0), 0);
        // set 0,20
        s.set(0, 20);
        // get 0,0
        assert_eq!(s.get(0, 0), 0);
        // set 0,7
        s.set(0, 7);
    }
    {
        // test case 50
        let mut s = SnapshotArray::new(3);
        // set 1,18
        s.set(1, 18);
        // set 1,4
        s.set(1, 4);
        // snap
        assert_eq!(s.snap(), 0);
        // get 0,0
        assert_eq!(s.get(0, 0), 0);
        // set 0,20
        s.set(0, 20);
        // snap
        assert_eq!(s.snap(), 1);
        // set 0,2
        s.set(0, 2);
        // set 1,1
        s.set(1, 1);
        // get 1,1
        assert_eq!(s.get(1, 1), 4);
        // get 1,0
        assert_eq!(s.get(1, 0), 4);
    }
    {
        // test case 53
        let mut s = SnapshotArray::new(4);
        // set 1,5
        s.set(1, 5);
        // snap
        assert_eq!(s.snap(), 0);
        // set 0,16
        s.set(0, 16);
        // snap
        assert_eq!(s.snap(), 1);
        // set 2,15
        s.set(2, 15);
        // snap
        assert_eq!(s.snap(), 2);
        // set 2,5
        s.set(2, 5);
        // get 1,0
        assert_eq!(s.get(1, 0), 5);
        // get 0,2
        assert_eq!(s.get(0, 2), 16);
        // snap
        assert_eq!(s.snap(), 3);
    }
    {
        // test case 54
        let mut s = SnapshotArray::new(4);
        // snap
        assert_eq!(s.snap(), 0);
        println!("snapshot 0 taken: {:?}", s);
        // snap
        assert_eq!(s.snap(), 1);
        println!("snapshot 1 taken: {:?}", s);
        // set 0,4
        s.set(0, 4);
        // snap
        assert_eq!(s.snap(), 2);
        println!("snapshot 2 taken: {:?}", s);
        // get 0,1
        assert_eq!(s.get(0, 1), 0);
        // set 0,12
        s.set(0, 12);
        // get 0,1
        assert_eq!(s.get(0, 1), 0);
        // snap
        assert_eq!(s.snap(), 3);
        println!("snapshot 3 taken: {:?}", s);
        // get 0,3
        assert_eq!(s.get(0, 3), 12);
    }
    {
        // test case 70
        let mut s = SnapshotArray::new(1);
        // snap
        assert_eq!(s.snap(), 0);
        // get 0,0
        assert_eq!(s.get(0, 0), 0);
        // snap
        assert_eq!(s.snap(), 1);
        // get 0,0
        assert_eq!(s.get(0, 0), 0);
        // set 0,13
        s.set(0, 13);
        // set 0,4
        s.set(0, 4);
        // set 0,17
        s.set(0, 17);
        // get 0,0
        assert_eq!(s.get(0, 0), 0);
        // set 0,2
        s.set(0, 2);
        // get 0,1
        assert_eq!(s.get(0, 1), 0);
        // snap
        assert_eq!(s.snap(), 2);
        // get 0,2
        assert_eq!(s.get(0, 2), 2);
        // get 0,0
        assert_eq!(s.get(0, 0), 0);
        // snap
        assert_eq!(s.snap(), 3);
    }
    {
        // test case 71
        let mut s = SnapshotArray::new(1);
        // snap
        s.snap();
        // set 0,2
        s.set(0, 2);
        // get 0,0
        assert_eq!(s.get(0, 2), 0);
        // set 0,16
        s.set(0, 16);
        // snap
        s.snap();
        // snap
        s.snap();
        // snap
        s.snap();
        // get 0,3
        assert_eq!(s.get(0, 3), 16);
        // set 0,1
        s.set(0, 1);
        // get 0,2
        assert_eq!(s.get(0, 2), 16);
        // get 0,1
        assert_eq!(s.get(0, 1), 16);
        // set 0,9
        s.set(0, 9);
        // get 0,3
        assert_eq!(s.get(0, 3), 16);
        // get 0,2
        assert_eq!(s.get(0, 2), 16);
        // get 0,0
        assert_eq!(s.get(0, 0), 0);
        // get 0,0
        assert_eq!(s.get(0, 0), 0);
        // snap
        s.snap();
    }
}

// there are 75 test cases
// this is technically possible just using Vecs that are set to have the given length, but this is super wasteful and hits the memory limit at test case ~40
// this is also technically possible with HashMaps (using the indexes as keys), but that's also wasteful enough to hit the memory limit at test case ~72
// it's better to use Vec<(i32, i32)> where the tuple is (index, val), but this hits the memory limit at test case ~72 too

// perhaps my problem is that I'm taking snapshots by cloning the whole values Vec
// this means my memory is O(m*n), where m is the number of snapshots and n is the number of values)
// I think the key here is that snapshots are read-only. This means I can deduplicate information between snapshots instead of storing potentially the same value for each of them
// there's a more perfect way to do the following, but for now I just want to capitalize on the fact that most values are probably not changing between sequential snapshots

// this fails due to timeout on test case 73. UGH
#[derive(Debug, Default)]
struct SnapshotValue {
    index: i32,
    current_val: i32,
    histories: Vec<SnapshotValueHistory>,
}
#[derive(Debug, Default)]
struct SnapshotValueHistory {
    first_snap_id_with_val: i32,
    latest_snap_id_with_val: i32,
    val: i32,
}
#[derive(Debug, Default)]
struct SnapshotArray {
    snapshot_count: i32,
    values: Vec<SnapshotValue>,
}

impl SnapshotArray {
    fn new(_length: i32) -> Self {
        Self::default()
    }

    fn set(&mut self, index: i32, val: i32) {
        if self.values.is_empty() {
            self.values.push(SnapshotValue {
                index,
                current_val: val,
                histories: vec![],
            });
        }
        // I need to binary search for the target index
        let mut l = 0;
        let mut r = self.values.len() - 1;
        while l < r {
            let m = (l + r) / 2;
            if index <= self.values[m].index {
                r = m;
            } else {
                l = m + 1;
            }
        }

        if index == self.values[r].index {
            self.values[r].current_val = val;
        } else if index < self.values[r].index {
            self.values.insert(
                r,
                SnapshotValue {
                    index,
                    current_val: val,
                    histories: vec![],
                },
            );
        } else {
            self.values.push(SnapshotValue {
                index,
                current_val: val,
                histories: vec![],
            })
        }
    }

    fn snap(&mut self) -> i32 {
        for SnapshotValue {
            current_val: val,
            histories,
            ..
        } in self.values.iter_mut()
        {
            match histories.last_mut() {
                Some(last_history) if last_history.val == *val => {
                    last_history.latest_snap_id_with_val = self.snapshot_count;
                }
                _ => {
                    histories.push(SnapshotValueHistory {
                        first_snap_id_with_val: self.snapshot_count,
                        latest_snap_id_with_val: self.snapshot_count,
                        val: *val,
                    });
                }
            }
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
            if index <= self.values[m].index {
                r = m;
            } else {
                l = m + 1;
            }
        }
        // if no values were ever set for that index, just give up now
        if index != self.values[r].index {
            return 0;
        }
        // println!("histories: {:?}", self.values[r].histories);

        let histories = &self.values[r].histories;
        if histories.is_empty() {
            return 0;
        }
        let mut l = 0;
        let mut r = histories.len() - 1;
        while l < r {
            let m = (l + r) / 2;
            if snap_id <= histories[m].latest_snap_id_with_val {
                r = m;
            } else {
                l = m + 1;
            }
        }
        // what makes this part weird is that the way I'm tracking differences for current vs. the last snapshot is that there are up to two histories with the latest_snap_id_with_val equal to the current snapshot_count. If we need an older snapshot than the current one, we can use what we have. Otherwise we need to use the most current value and choose the latest snapshot_value
        // what I also have to remember is that we only perform get for snapshots, not the "current" unsnapped snapshot, hence the comparison of snap_id to self.snapshot_count - 1 (note the minus one)
        if snap_id == self.snapshot_count - 1
            && r < histories.len() - 1
            && snap_id == histories[r + 1].latest_snap_id_with_val
        {
            r += 1;
        }
        // we can't use what we found unless it was set before/during the desired snapshot
        if histories[r].first_snap_id_with_val <= snap_id {
            histories[r].val
        } else {
            0
        }
    }
}
