use std::iter::FromIterator;

use tabled::{builder::Builder, Table};

use crate::util::{create_vector, test_table};

mod util;

test_table!(
    builder_index,
    Table::builder(create_vector::<3, 2>()).index().build(),
    "+---+---+----------+----------+"
    "|   | N | column 0 | column 1 |"
    "+---+---+----------+----------+"
    "| 0 | 0 | 0-0      | 0-1      |"
    "+---+---+----------+----------+"
    "| 1 | 1 | 1-0      | 1-1      |"
    "+---+---+----------+----------+"
    "| 2 | 2 | 2-0      | 2-1      |"
    "+---+---+----------+----------+"
);

test_table!(
    builder_index_transpose,
    Table::builder(create_vector::<4, 2>()).index().transpose().clone().build(),
    "+----------+-----+-----+-----+-----+"
    "|          | 0   | 1   | 2   | 3   |"
    "+----------+-----+-----+-----+-----+"
    "| N        | 0   | 1   | 2   | 3   |"
    "+----------+-----+-----+-----+-----+"
    "| column 0 | 0-0 | 1-0 | 2-0 | 3-0 |"
    "+----------+-----+-----+-----+-----+"
    "| column 1 | 0-1 | 1-1 | 2-1 | 3-1 |"
    "+----------+-----+-----+-----+-----+"
);

test_table!(
    builder_index_0,
    Table::builder(create_vector::<4, 2>()).index().set_index(0).clone().build(),
    "+---+----------+----------+"
    "|   | column 0 | column 1 |"
    "+---+----------+----------+"
    "| N |          |          |"
    "+---+----------+----------+"
    "| 0 | 0-0      | 0-1      |"
    "+---+----------+----------+"
    "| 1 | 1-0      | 1-1      |"
    "+---+----------+----------+"
    "| 2 | 2-0      | 2-1      |"
    "+---+----------+----------+"
    "| 3 | 3-0      | 3-1      |"
    "+---+----------+----------+"
);

test_table!(
    builder_index_0_no_name,
    Table::builder(create_vector::<4, 2>()).index().set_index(0).set_name(None).clone().build(),
    "+---+----------+----------+"
    "|   | column 0 | column 1 |"
    "+---+----------+----------+"
    "| 0 | 0-0      | 0-1      |"
    "+---+----------+----------+"
    "| 1 | 1-0      | 1-1      |"
    "+---+----------+----------+"
    "| 2 | 2-0      | 2-1      |"
    "+---+----------+----------+"
    "| 3 | 3-0      | 3-1      |"
    "+---+----------+----------+"
);

test_table!(
    builder_index_0_name,
    Table::builder(create_vector::<4, 2>()).index().set_index(0).set_name(Some("Hello World".to_owned())).clone().build(),
    "+-------------+----------+----------+"
    "|             | column 0 | column 1 |"
    "+-------------+----------+----------+"
    "| Hello World |          |          |"
    "+-------------+----------+----------+"
    "| 0           | 0-0      | 0-1      |"
    "+-------------+----------+----------+"
    "| 1           | 1-0      | 1-1      |"
    "+-------------+----------+----------+"
    "| 2           | 2-0      | 2-1      |"
    "+-------------+----------+----------+"
    "| 3           | 3-0      | 3-1      |"
    "+-------------+----------+----------+"
);

test_table!(
    builder_index_0_name_transpose,
    Table::builder(create_vector::<4, 2>()).index().set_index(0).set_name(Some("Hello World".to_owned())).transpose().clone().build(),
    "+-------------+-----+-----+-----+-----+"
    "| Hello World | 0   | 1   | 2   | 3   |"
    "+-------------+-----+-----+-----+-----+"
    "| column 0    | 0-0 | 1-0 | 2-0 | 3-0 |"
    "+-------------+-----+-----+-----+-----+"
    "| column 1    | 0-1 | 1-1 | 2-1 | 3-1 |"
    "+-------------+-----+-----+-----+-----+"
);

test_table!(
    builder_index_with_no_columns,
    Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["d", "e", "f"]]).index().build(),
    "+---+---+---+---+"
    "|   | 0 | 1 | 2 |"
    "+---+---+---+---+"
    "| 0 | 1 | 2 | 3 |"
    "+---+---+---+---+"
    "| 1 | a | b | c |"
    "+---+---+---+---+"
    "| 2 | d | e | f |"
    "+---+---+---+---+"
);

test_table!(
    builder_index_with_no_columns_and_name,
    Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["d", "e", "f"]])
        .index()
        .set_name(Some("Hello World".to_owned()))
        .clone()
        .build(),
    "+-------------+---+---+---+"
    "|             | 0 | 1 | 2 |"
    "+-------------+---+---+---+"
    "| Hello World |   |   |   |"
    "+-------------+---+---+---+"
    "| 0           | 1 | 2 | 3 |"
    "+-------------+---+---+---+"
    "| 1           | a | b | c |"
    "+-------------+---+---+---+"
    "| 2           | d | e | f |"
    "+-------------+---+---+---+"
);

test_table!(
    builder_index_with_no_columns_transpose,
    Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["d", "e", "f"]])
        .index()
        .transpose()
        .clone()
        .build(),
    "+---+---+---+---+"
    "|   | 0 | 1 | 2 |"
    "+---+---+---+---+"
    "| 0 | 1 | a | d |"
    "+---+---+---+---+"
    "| 1 | 2 | b | e |"
    "+---+---+---+---+"
    "| 2 | 3 | c | f |"
    "+---+---+---+---+"
);

test_table!(builder_index_empty, Builder::default().index().build(), "");

test_table!(
    builder_index_transpose_empty,
    Builder::default().index().transpose().clone().build(),
    ""
);

test_table!(
    builder_index_invalid_dosnt_panic,
    Builder::default().index().set_index(100).clone().build(),
    ""
);

test_table!(
    builder_index_name_doesnt_shown_when_empty,
    Builder::default()
        .index()
        .set_name(Some("Hello World".to_owned()))
        .clone()
        .build(),
    ""
);

#[test]
fn builder_index_transpose_transpose() {
    let data = create_vector::<4, 2>();
    let mut builder = Table::builder(&data).index();

    let orig_table = builder.clone().build().to_string();
    let two_times_transposed_table = builder.transpose().transpose().clone().build().to_string();

    assert_eq!(orig_table, two_times_transposed_table,);
}

#[test]
fn builder_index_no_name_transpose_transpose() {
    let data = create_vector::<4, 2>();
    let mut builder = Table::builder(&data).index().set_name(None).clone();

    let orig_table = builder.clone().build().to_string();
    let two_times_transposed_table = builder.transpose().transpose().clone().build().to_string();

    assert_eq!(orig_table, two_times_transposed_table,);
}
