use collab::preclude::encoding::serde::from_any;
use collab::preclude::Any;

use collab_database::fields::{Field, TypeOptionData, TypeOptionDataBuilder};
use collab_database::rows::Cell;
use flowy_error::FlowyResult;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

use crate::entities::{FieldType, TextFilterPB, URLCellDataPB};
use crate::services::cell::{CellDataChangeset, CellDataDecoder};
use crate::services::field::{
  TypeOption, TypeOptionCellDataCompare, TypeOptionCellDataFilter, TypeOptionCellDataSerde,
  TypeOptionTransform, URLCellData,
};
use crate::services::sort::SortCondition;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct URLTypeOption {
  #[serde(default)]
  pub url: String,
  #[serde(default)]
  pub content: String,
}

impl TypeOption for URLTypeOption {
  type CellData = URLCellData;
  type CellChangeset = URLCellChangeset;
  type CellProtobufType = URLCellDataPB;
  type CellFilter = TextFilterPB;
}

impl From<TypeOptionData> for URLTypeOption {
  fn from(data: TypeOptionData) -> Self {
    from_any(&Any::from(data)).unwrap()
  }
}

impl From<URLTypeOption> for TypeOptionData {
  fn from(data: URLTypeOption) -> Self {
    TypeOptionDataBuilder::from([
      ("url".into(), data.url.into()),
      ("content".into(), data.content.into()),
    ])
  }
}

impl TypeOptionTransform for URLTypeOption {}

impl TypeOptionCellDataSerde for URLTypeOption {
  fn protobuf_encode(
    &self,
    cell_data: <Self as TypeOption>::CellData,
  ) -> <Self as TypeOption>::CellProtobufType {
    cell_data.into()
  }

  fn parse_cell(&self, cell: &Cell) -> FlowyResult<<Self as TypeOption>::CellData> {
    Ok(URLCellData::from(cell))
  }
}

impl CellDataDecoder for URLTypeOption {
  fn decode_cell(&self, cell: &Cell) -> FlowyResult<<Self as TypeOption>::CellData> {
    self.parse_cell(cell)
  }
  fn decode_cell_with_transform(
    &self,
    cell: &Cell,
    from_field_type: FieldType,
    _field: &Field,
  ) -> Option<<Self as TypeOption>::CellData> {
    match from_field_type {
      FieldType::RichText => Some(Self::CellData::from(cell)),
      _ => None,
    }
  }

  fn stringify_cell_data(&self, cell_data: <Self as TypeOption>::CellData) -> String {
    cell_data.data
  }

  fn numeric_cell(&self, _cell: &Cell) -> Option<f64> {
    None
  }
}

pub type URLCellChangeset = String;

impl CellDataChangeset for URLTypeOption {
  fn apply_changeset(
    &self,
    changeset: <Self as TypeOption>::CellChangeset,
    _cell: Option<Cell>,
  ) -> FlowyResult<(Cell, <Self as TypeOption>::CellData)> {
    let url_cell_data = URLCellData { data: changeset };
    Ok((url_cell_data.clone().into(), url_cell_data))
  }
}

impl TypeOptionCellDataFilter for URLTypeOption {
  fn apply_filter(
    &self,
    filter: &<Self as TypeOption>::CellFilter,
    cell_data: &<Self as TypeOption>::CellData,
  ) -> bool {
    filter.is_visible(cell_data)
  }
}

impl TypeOptionCellDataCompare for URLTypeOption {
  fn apply_cmp(
    &self,
    cell_data: &<Self as TypeOption>::CellData,
    other_cell_data: &<Self as TypeOption>::CellData,
    sort_condition: SortCondition,
  ) -> Ordering {
    let is_left_empty = cell_data.data.is_empty();
    let is_right_empty = other_cell_data.data.is_empty();
    match (is_left_empty, is_right_empty) {
      (true, true) => Ordering::Equal,
      (true, false) => Ordering::Greater,
      (false, true) => Ordering::Less,
      (false, false) => {
        let order = cell_data.data.cmp(&other_cell_data.data);
        sort_condition.evaluate_order(order)
      },
    }
  }
}
