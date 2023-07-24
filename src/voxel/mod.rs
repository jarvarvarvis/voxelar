pub mod voxel_storage;
pub mod voxel_grid;

pub type VoxelCoordinate = crate::nalgebra::Vector3<usize>;

pub trait Voxel {
    fn empty() -> Self;
}
