#[test]
#[allow(non_snake_case)]
fn gap_row_gap_align_items_end__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node5 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_wrap: taffy::style::FlexWrap::Wrap,
                align_items: Some(taffy::style::AlignItems::FlexEnd),
                gap: taffy::geometry::Size { width: length(10f32), height: length(20f32) },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(100f32),
                    height: taffy::style::Dimension::from_length(200f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4, node5],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node, 100f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node0, 20f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node0, 0f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 90f32, "y of node {:?}. Expected {}. Actual {}", node0, 90f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node1, 20f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node1, 0f32, size.height);
    assert_eq!(location.x, 30f32, "x of node {:?}. Expected {}. Actual {}", node1, 30f32, location.x);
    assert_eq!(location.y, 90f32, "y of node {:?}. Expected {}. Actual {}", node1, 90f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node2, 20f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node2, 0f32, size.height);
    assert_eq!(location.x, 60f32, "x of node {:?}. Expected {}. Actual {}", node2, 60f32, location.x);
    assert_eq!(location.y, 90f32, "y of node {:?}. Expected {}. Actual {}", node2, 90f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node3, 20f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node3, 0f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node3, 0f32, location.x);
    assert_eq!(location.y, 200f32, "y of node {:?}. Expected {}. Actual {}", node3, 200f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node4, 20f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node4, 0f32, size.height);
    assert_eq!(location.x, 30f32, "x of node {:?}. Expected {}. Actual {}", node4, 30f32, location.x);
    assert_eq!(location.y, 200f32, "y of node {:?}. Expected {}. Actual {}", node4, 200f32, location.y);
    let layout = taffy.layout(node5).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node5, 20f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node5, 0f32, size.height);
    assert_eq!(location.x, 60f32, "x of node {:?}. Expected {}. Actual {}", node5, 60f32, location.x);
    assert_eq!(location.y, 200f32, "y of node {:?}. Expected {}. Actual {}", node5, 200f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn gap_row_gap_align_items_end__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node5 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(20f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_wrap: taffy::style::FlexWrap::Wrap,
                align_items: Some(taffy::style::AlignItems::FlexEnd),
                gap: taffy::geometry::Size { width: length(10f32), height: length(20f32) },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(100f32),
                    height: taffy::style::Dimension::from_length(200f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4, node5],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node, 100f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node0, 20f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node0, 0f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 90f32, "y of node {:?}. Expected {}. Actual {}", node0, 90f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node1, 20f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node1, 0f32, size.height);
    assert_eq!(location.x, 30f32, "x of node {:?}. Expected {}. Actual {}", node1, 30f32, location.x);
    assert_eq!(location.y, 90f32, "y of node {:?}. Expected {}. Actual {}", node1, 90f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node2, 20f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node2, 0f32, size.height);
    assert_eq!(location.x, 60f32, "x of node {:?}. Expected {}. Actual {}", node2, 60f32, location.x);
    assert_eq!(location.y, 90f32, "y of node {:?}. Expected {}. Actual {}", node2, 90f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node3, 20f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node3, 0f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node3, 0f32, location.x);
    assert_eq!(location.y, 200f32, "y of node {:?}. Expected {}. Actual {}", node3, 200f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node4, 20f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node4, 0f32, size.height);
    assert_eq!(location.x, 30f32, "x of node {:?}. Expected {}. Actual {}", node4, 30f32, location.x);
    assert_eq!(location.y, 200f32, "y of node {:?}. Expected {}. Actual {}", node4, 200f32, location.y);
    let layout = taffy.layout(node5).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node5, 20f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node5, 0f32, size.height);
    assert_eq!(location.x, 60f32, "x of node {:?}. Expected {}. Actual {}", node5, 60f32, location.x);
    assert_eq!(location.y, 200f32, "y of node {:?}. Expected {}. Actual {}", node5, 200f32, location.y);
}
