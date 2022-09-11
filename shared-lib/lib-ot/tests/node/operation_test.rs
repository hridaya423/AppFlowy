use lib_ot::core::{
    NodeAttributeBuilder, NodeBodyChangeset, NodeBuilder, NodeData, NodeOperation, Path, TextDeltaBuilder,
};

#[test]
fn operation_insert_node_serde_test() {
    let insert = NodeOperation::Insert {
        path: Path(vec![0, 1]),
        nodes: vec![NodeData::new("text".to_owned())],
    };
    let result = serde_json::to_string(&insert).unwrap();
    assert_eq!(result, r#"{"op":"insert","path":[0,1],"nodes":[{"type":"text"}]}"#);
}

#[test]
fn operation_insert_node_with_children_serde_test() {
    let node = NodeBuilder::new("text")
        .add_node(NodeData::new("sub_text".to_owned()))
        .build();

    let insert = NodeOperation::Insert {
        path: Path(vec![0, 1]),
        nodes: vec![node],
    };
    assert_eq!(
        serde_json::to_string(&insert).unwrap(),
        r#"{"op":"insert","path":[0,1],"nodes":[{"type":"text","children":[{"type":"sub_text"}]}]}"#
    );
}
#[test]
fn operation_update_node_attributes_serde_test() {
    let operation = NodeOperation::UpdateAttributes {
        path: Path(vec![0, 1]),
        attributes: NodeAttributeBuilder::new().insert("bold", true).build(),
        old_attributes: NodeAttributeBuilder::new().insert("bold", false).build(),
    };

    let result = serde_json::to_string(&operation).unwrap();

    assert_eq!(
        result,
        r#"{"op":"update","path":[0,1],"attributes":{"bold":"true"},"oldAttributes":{"bold":"false"}}"#
    );
}

#[test]
fn operation_update_node_body_serde_test() {
    let delta = TextDeltaBuilder::new().insert("AppFlowy...").build();
    let inverted = delta.invert_str("");
    let changeset = NodeBodyChangeset::Delta { delta, inverted };
    let insert = NodeOperation::UpdateBody {
        path: Path(vec![0, 1]),
        changeset,
    };
    let result = serde_json::to_string(&insert).unwrap();
    assert_eq!(
        result,
        r#"{"op":"edit-body","path":[0,1],"delta":[{"insert":"AppFlowy..."}],"inverted":[{"delete":11}]}"#
    );
    //
}
