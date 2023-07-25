use super::*;

/// The size of the voxel storage's bounding box on all three axes
pub struct VoxelStorageBBExtent {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}

pub trait VoxelStorage<V: Voxel> {
    fn size(&self) -> VoxelStorageBBExtent;

    fn get_voxel(&self, coordinate: VoxelCoordinate) -> crate::Result<&V>;
    fn get_voxel_mut(&mut self, coordinate: VoxelCoordinate) -> crate::Result<&mut V>;

    fn set_voxel(&mut self, coordinate: VoxelCoordinate, new_voxel: V) -> crate::Result<()>;
}
