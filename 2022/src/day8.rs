use advent_of_code_common::read_data_from_file;

fn main() -> Result<(), anyhow::Error> {
    let data = process_data(read_data_from_file("data/day8.txt")?)?;
    let result = trees_visible_from_outside_grid(data.clone())?;
    println!("{} trees are visible from outside the patch.", result);

    let result = highest_scenic_score(data);

    println!("The best scenery score is {}", result);

    Ok(())
}

fn process_data(data: Vec<String>) -> Result<TreePatch, anyhow::Error> {
    let height = data.len();
    let width = data[0].len();
    let mut treepatch = TreePatch {
        width,
        height,
        ..Default::default()
    };

    for line in data.iter() {
        let mut row = Vec::new();
        for cha in line.chars() {
            row.push(Tree {
                height: cha
                    .to_digit(10)
                    .ok_or_else(|| anyhow::anyhow!("Unable to convert digit?s"))?
                    .try_into()?,
            });
        }
        treepatch.trees.push(row);
    }
    Ok(treepatch)
}

fn highest_scenic_score(treepatch: TreePatch) -> usize {
    let mut high_score = 0;

    let mut left_score = 0;
    let mut right_score = 0;
    let mut up_score = 0;
    let mut down_score = 0;

    for (y, row) in treepatch.trees.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            // Count visible trees left
            for i in (0..x).rev() {
                let h = treepatch.trees[y][i].height;
                left_score += 1;

                if h >= tree.height {
                    break;
                }
            }
            // Count visible trees right
            for i in (x + 1)..treepatch.width {
                let h = treepatch.trees[y][i].height;
                right_score += 1;

                if h >= tree.height {
                    break;
                }
            }
            // Count visible trees up
            for i in (0..y).rev() {
                let h = treepatch.trees[i][x].height;
                up_score += 1;

                if h >= tree.height {
                    break;
                }
            }
            // Count visible trees down
            for i in (y + 1)..treepatch.height {
                let h = treepatch.trees[i][x].height;
                down_score += 1;

                if h >= tree.height {
                    break;
                }
            }

            // Math
            let score = left_score * right_score * up_score * down_score;
            if score > high_score {
                high_score = score;
            }
            left_score = 0;
            right_score = 0;
            up_score = 0;
            down_score = 0;
        }
    }

    high_score
}

fn trees_visible_from_outside_grid(treepatch: TreePatch) -> Result<usize, anyhow::Error> {
    let mut visible_trees = 0_usize;

    // Handle all the trees on the border
    visible_trees += ((treepatch.width * 2) + (treepatch.height * 2)) - 4;

    'rows: for (y, row) in treepatch.trees.iter().enumerate() {
        // Top and bottom borders handled already
        if y == 0 || y == treepatch.height - 1 {
            continue 'rows;
        }
        'columns: for (x, col) in row.iter().enumerate() {
            // Left and right borders handled already
            if x == 0 || x == treepatch.width - 1 {
                continue 'columns;
            }

            let tree = col;

            // For all four checks, if tree is visible, cancel this iteration
            // Check to the left
            let mut tree_visible = true;
            for i in 0..x {
                if treepatch.trees[y][i].height >= tree.height {
                    tree_visible = false;
                }
            }
            if tree_visible {
                visible_trees += 1;
                continue 'columns;
            }

            // Check to the right
            tree_visible = true;
            for i in (x + 1)..treepatch.width {
                if treepatch.trees[y][i].height >= tree.height {
                    tree_visible = false;
                }
            }
            if tree_visible {
                visible_trees += 1;
                continue 'columns;
            }

            // Check upward
            tree_visible = true;
            for i in 0..y {
                if treepatch.trees[i][x].height >= tree.height {
                    tree_visible = false;
                }
            }
            if tree_visible {
                visible_trees += 1;
                continue 'columns;
            }

            // Check downward
            tree_visible = true;
            for i in (y + 1)..treepatch.height {
                if treepatch.trees[i][x].height >= tree.height {
                    tree_visible = false;
                }
            }
            if tree_visible {
                visible_trees += 1;
                continue 'columns;
            }
        }
    }

    Ok(visible_trees)
}

#[derive(Debug, Default, Clone)]
struct TreePatch {
    pub width: usize,
    pub height: usize,
    pub trees: Vec<Vec<Tree>>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Tree {
    pub height: usize,
}

#[cfg(test)]
mod tests {
    use crate::{highest_scenic_score, process_data, trees_visible_from_outside_grid};

    fn test_data() -> Vec<String> {
        r##"30373
25512
65332
33549
35390
"##
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
    }

    #[test]
    fn process_data_works() {
        let data = process_data(test_data()).unwrap();
        assert!(!data.trees.is_empty());
    }

    #[test]
    fn trees_visible_from_outside_grid_works() {
        let data = process_data(test_data()).unwrap();
        let result = trees_visible_from_outside_grid(data).unwrap();
        assert_eq!(result, 21);
    }

    #[test]
    fn highest_scenic_score_works() {
        let data = process_data(test_data()).unwrap();
        let result = highest_scenic_score(data);
        assert_eq!(result, 8);
    }
}
