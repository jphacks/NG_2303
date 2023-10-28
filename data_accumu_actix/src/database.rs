use crate::accumu::DataAccumu;

struct database;


impl DataAccumu for database {
    fn select(&self) -> anyhow::Result<Vec<crate::accumu::ObjectDetectionData>> {
        todo!()
    }

    fn insert(&self, data: crate::accumu::ObjectDetectionData) -> anyhow::Result<()> {
        todo!()
    }

    fn delete(&self, id: u64) -> anyhow::Result<()> {
        todo!()
    }
}