use super::*;

pub trait VoxelStorage<V: Voxel> {
    fn get_voxel(&self, coordinate: VoxelCoordinate) -> crate::Result<&V>;
    fn get_voxel_mut(&mut self, coordinate: VoxelCoordinate) -> crate::Result<&mut V>;

    fn set_voxel(&mut self, coordinate: VoxelCoordinate, new_voxel: V) -> crate::Result<()>;
}
