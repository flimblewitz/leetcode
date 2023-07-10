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
}

// it's worth noting that this is based on an existing solution
// I was on the right track with previous attempts, but I think the way that I stored snapshots was still too wasteful despite the fact that I tried to reduce it to only actual changes
// I refactored it a bit to fit my preferences (no parallel arrays, for instance), and added a slight optimization so that it doesn't save snapshots unless something has actually changed

#[derive(Clone, Debug, Default)]
struct Snapshot {
    snap_id: usize,
    val: i32,
}
#[derive(Debug)]
struct SnapshotArray {
    snapshots: Vec<Vec<Snapshot>>,
    current_snap_id: usize,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        let length = length as usize;
        Self {
            snapshots: vec![vec![Snapshot::default()]; length],
            current_snap_id: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        let index = index as usize;

        let latest_snapshot_for_index = self.snapshots[index].last_mut().unwrap();

        // this is my optimization: we don't actually need to bother doing anything if the most recent snapshot has the same value
        if latest_snapshot_for_index.val == val {
            return;
        }

        if latest_snapshot_for_index.snap_id == self.current_snap_id {
            latest_snapshot_for_index.val = val;
        } else {
            self.snapshots[index].push(Snapshot {
                snap_id: self.current_snap_id,
                val,
            });
        }
    }

    fn snap(&mut self) -> i32 {
        self.current_snap_id += 1;
        self.current_snap_id as i32 - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let index = index as usize;
        let snap_id = snap_id as usize;

        // there's always going to be at least one snapshot per index, so it's safe to fall back to the preceding snapshot if we can't find the snap_id we want
        let snapshot_index = self.snapshots[index]
            .binary_search_by_key(&snap_id, |sv| sv.snap_id)
            .unwrap_or_else(|index_where_snapshot_would_exist| {
                index_where_snapshot_would_exist - 1
            });

        self.snapshots[index][snapshot_index].val
    }
}
