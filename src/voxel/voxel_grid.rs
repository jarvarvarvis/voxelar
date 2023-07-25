use super::voxel_storage::*;
use super::*;

pub struct VoxelGrid<V: Voxel> {
    size: (usize, usize, usize),
    voxels: Vec<V>,
}

impl<V: Voxel> VoxelGrid<V> {
    pub fn new(width: usize, height: usize, depth: usize) -> crate::Result<Self> {
        crate::verify!(width > 0, "Width of the voxel grid must be greater than 0");
        crate::verify!(
            height > 0,
            "Height of the voxel grid must be greater than 0"
        );
        crate::verify!(depth > 0, "Depth of the voxel grid must be greater than 0");

        let size = width * height * depth;
        let mut voxels = Vec::with_capacity(size);
        for _ in 0..size {
            voxels.push(V::empty());
        }

        Ok(Self {
            voxels,
            size: (width, height, depth),
        })
    }

    pub fn width(&self) -> usize {
        self.size.0
    }

    pub fn height(&self) -> usize {
        self.size.1
    }

    pub fn depth(&self) -> usize {
        self.size.2
    }

    pub fn is_in_bounds(&self, coordinate: VoxelCoordinate) -> bool {
        coordinate.x < self.width() && coordinate.y < self.height() && coordinate.z < self.depth()
    }

    fn convert_coordinate_to_index(&self, coordinate: VoxelCoordinate) -> usize {
        coordinate.x + coordinate.y * self.width() + coordinate.z * self.width() * self.height()
    }
}

impl<V: Voxel> VoxelStorage<V> for VoxelGrid<V> {
    fn size(&self) -> VoxelStorageBBExtent {
        VoxelStorageBBExtent {
            width: self.width(),
            height: self.height(),
            depth: self.depth(),
        }
    }

    fn get_voxel(&self, coordinate: VoxelCoordinate) -> crate::Result<&V> {
        let size = self.size;
        crate::verify!(
            self.is_in_bounds(coordinate),
            "Coordinate {coordinate} must be in bounds ({size:?}) of the voxel grid"
        );

        let index = self.convert_coordinate_to_index(coordinate);
        Ok(&self.voxels[index])
    }

    fn get_voxel_mut(&mut self, coordinate: VoxelCoordinate) -> crate::Result<&mut V> {
        let size = self.size;
        crate::verify!(
            self.is_in_bounds(coordinate),
            "Coordinate {coordinate} must be in bounds ({size:?}) of the voxel grid"
        );

        let index = self.convert_coordinate_to_index(coordinate);
        Ok(&mut self.voxels[index])
    }

    fn set_voxel(&mut self, coordinate: VoxelCoordinate, new_voxel: V) -> crate::Result<()> {
        let size = self.size;
        crate::verify!(
            self.is_in_bounds(coordinate),
            "Coordinate {coordinate} must be in bounds ({size:?}) of the voxel grid"
        );

        let index = self.convert_coordinate_to_index(coordinate);
        self.voxels[index] = new_voxel;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Eq, PartialEq)]
    struct DummyVoxel {
        id: u64,
    }

    impl Voxel for DummyVoxel {
        fn empty() -> Self {
            Self { id: 0 }
        }
    }

    #[test]
    fn new_voxel_grid_has_correct_dimensions() {
        let voxel_grid =
            VoxelGrid::<DummyVoxel>::new(20, 3, 14).expect("Creating dummy voxel grid failed");

        assert_eq!(20, voxel_grid.width());
        assert_eq!(3, voxel_grid.height());
        assert_eq!(14, voxel_grid.depth());
    }

    #[test]
    fn voxel_coordinate_inside_grid_is_in_bounds() {
        let voxel_grid =
            VoxelGrid::<DummyVoxel>::new(15, 20, 14).expect("Creating dummy voxel grid failed");

        assert!(voxel_grid.is_in_bounds(VoxelCoordinate::new(4, 8, 6)));
    }

    #[test]
    fn voxel_coordinate_just_at_edge_inside_grid_is_in_bounds() {
        let voxel_grid =
            VoxelGrid::<DummyVoxel>::new(5, 19, 96).expect("Creating dummy voxel grid failed");

        assert!(voxel_grid.is_in_bounds(VoxelCoordinate::new(4, 18, 95)));
    }

    #[test]
    fn voxel_coordinate_outside_grid_on_one_axis_is_not_in_bounds() {
        let voxel_grid =
            VoxelGrid::<DummyVoxel>::new(17, 3, 8).expect("Creating dummy voxel grid failed");

        assert!(!voxel_grid.is_in_bounds(VoxelCoordinate::new(17, 0, 4)));
    }

    #[test]
    fn voxel_coordinate_far_outside_grid_is_not_in_bounds() {
        let voxel_grid =
            VoxelGrid::<DummyVoxel>::new(31, 45, 8).expect("Creating dummy voxel grid failed");

        assert!(!voxel_grid.is_in_bounds(VoxelCoordinate::new(1000, 53589, 123)));
    }

    #[test]
    fn new_voxel_grid_has_correct_values() {
        let voxel_grid =
            VoxelGrid::<DummyVoxel>::new(2, 3, 4).expect("Creating dummy voxel grid failed");

        let expected = vec![
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
        ];

        assert_eq!(expected, voxel_grid.voxels);
    }

    #[test]
    fn getting_voxel_from_flat_grid_works_correctly() {
        let mut voxel_grid =
            VoxelGrid::<DummyVoxel>::new(3, 3, 1).expect("Creating dummy voxel grid failed");

        voxel_grid.voxels = vec![
            DummyVoxel::empty(),
            DummyVoxel { id: 12 },
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel::empty(),
            DummyVoxel { id: 190 },
        ];

        assert_eq!(
            0,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(0, 0, 0))
                .expect("Getting voxel at 0,0,0 failed")
                .id
        );
        assert_eq!(
            12,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(1, 0, 0))
                .expect("Getting voxel at 1,0,0 failed")
                .id
        );
        assert_eq!(
            0,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(2, 1, 0))
                .expect("Getting voxel at 2,1,0 failed")
                .id
        );
        assert_eq!(
            190,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(2, 2, 0))
                .expect("Getting voxel at 2,2,0 failed")
                .id
        );
    }

    #[test]
    fn getting_voxel_from_grid_with_depth_works_correctly() {
        let mut voxel_grid =
            VoxelGrid::<DummyVoxel>::new(2, 2, 2).expect("Creating dummy voxel grid failed");

        voxel_grid.voxels = vec![
            DummyVoxel { id: 12 },
            DummyVoxel { id: 5 },
            DummyVoxel { id: 31 },
            DummyVoxel { id: 90 },
            DummyVoxel { id: 0 },
            DummyVoxel { id: 17 },
            DummyVoxel { id: 22 },
            DummyVoxel { id: 59 },
        ];

        assert_eq!(
            12,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(0, 0, 0))
                .expect("Getting voxel at 0,0,0 failed")
                .id
        );
        assert_eq!(
            5,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(1, 0, 0))
                .expect("Getting voxel at 1,0,0 failed")
                .id
        );
        assert_eq!(
            31,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(0, 1, 0))
                .expect("Getting voxel at 0,1,0 failed")
                .id
        );
        assert_eq!(
            90,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(1, 1, 0))
                .expect("Getting voxel at 1,1,0 failed")
                .id
        );

        assert_eq!(
            0,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(0, 0, 1))
                .expect("Getting voxel at 0,0,1 failed")
                .id
        );
        assert_eq!(
            17,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(1, 0, 1))
                .expect("Getting voxel at 1,0,1 failed")
                .id
        );
        assert_eq!(
            22,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(0, 1, 1))
                .expect("Getting voxel at 0,1,1 failed")
                .id
        );
        assert_eq!(
            59,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(1, 1, 1))
                .expect("Getting voxel at 1,1,1 failed")
                .id
        );
    }

    #[test]
    fn setting_voxel_in_grid_getting_it_and_getting_another_one_return_correct_results() {
        let mut voxel_grid =
            VoxelGrid::<DummyVoxel>::new(3, 2, 3).expect("Creating dummy voxel grid failed");

        voxel_grid.voxels = vec![
            DummyVoxel { id: 12 },
            DummyVoxel { id: 5 },
            DummyVoxel { id: 0 },
            DummyVoxel { id: 31 },
            DummyVoxel { id: 90 },
            DummyVoxel { id: 45 },
            DummyVoxel { id: 0 },
            DummyVoxel { id: 17 },
            DummyVoxel { id: 77 },
            DummyVoxel { id: 22 },
            DummyVoxel { id: 59 },
            DummyVoxel { id: 8 },
            DummyVoxel { id: 1 },
            DummyVoxel { id: 57 },
            DummyVoxel { id: 30 },
            DummyVoxel { id: 34 },
            DummyVoxel { id: 9 },
            DummyVoxel { id: 3 },
        ];

        assert_eq!(
            0,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(0, 0, 1))
                .expect("Getting voxel at 0,0,1 failed")
                .id
        );
        assert_eq!(
            17,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(1, 0, 1))
                .expect("Getting voxel at 1,0,1 failed")
                .id
        );
        assert_eq!(
            57,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(1, 0, 2))
                .expect("Getting voxel at 1,0,2 failed")
                .id
        );

        voxel_grid
            .set_voxel(VoxelCoordinate::new(1, 0, 1), DummyVoxel { id: 1859 })
            .expect("Setting voxel at 1,0,1 failed");

        assert_eq!(
            0,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(0, 0, 1))
                .expect("Getting voxel at 0,0,1 failed")
                .id
        );
        assert_eq!(
            1859,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(1, 0, 1))
                .expect("Getting voxel at 1,0,1 failed")
                .id
        );
        assert_eq!(
            57,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(1, 0, 2))
                .expect("Getting voxel at 1,0,2 failed")
                .id
        );
    }

    #[test]
    fn setting_voxel_in_grid_using_get_mut_and_getting_from_same_position_has_correct_value() {
        let mut voxel_grid =
            VoxelGrid::<DummyVoxel>::new(45, 12, 90).expect("Creating dummy voxel grid failed");

        assert_eq!(
            0,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(22, 4, 87))
                .expect("Getting voxel at 22,4,87 failed")
                .id
        );

        (*voxel_grid
            .get_voxel_mut(VoxelCoordinate::new(22, 4, 87))
            .expect("Getting voxel at 22,4,87 failed"))
        .id = 72389;

        assert_eq!(
            72389,
            voxel_grid
                .get_voxel(VoxelCoordinate::new(22, 4, 87))
                .expect("Getting voxel at 22,4,87 failed")
                .id
        );
    }
}
