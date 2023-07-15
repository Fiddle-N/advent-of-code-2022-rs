use std::collections::HashMap;

use ndarray::{Array2, ArrayBase, Dim, OwnedRepr, ViewRepr, s};

type TwoDArray = ArrayBase<OwnedRepr<u8>, Dim<[usize; 2]>>;
type OneDArraySlice<'a> = ArrayBase<ViewRepr<&'a u8>, Dim<[usize; 1]>>;

fn forest_ndarr(input: &str) -> TwoDArray {
    let mut forest = Vec::new();
    let mut ncols = None;
    let mut nrows = 0;
    for row in input.lines() {
        let row: Vec<_> = row
            .chars()
            .map(
                |tree| tree.to_digit(10).unwrap() as u8
            )
            .collect();
        if ncols == None {
            ncols = Some(row.len())
        }
        forest.extend_from_slice(&row);
        nrows += 1;
    }
    let ncols = ncols.unwrap();
    let forest = Array2::from_shape_vec(
        (nrows, ncols),
        forest
    ).unwrap();
    forest
}


#[derive(PartialEq, Eq, Hash)]
struct Coords {
    x: u32,
    y: u32,
}


struct TreeDirectionDetails {
    dist: u32,
    visible: bool,
}


struct Tree {
    up: TreeDirectionDetails,    
    right: TreeDirectionDetails,
    down: TreeDirectionDetails,
    left: TreeDirectionDetails,    
}

impl Tree {
    
    fn is_visible(&self) -> bool {
        self.up.visible
        || self.right.visible
        || self.down.visible
        || self.left.visible
    }

    fn scenic_score(&self) -> u32 {
        self.up.dist
        * self.right.dist
        * self.down.dist
        * self.left.dist
    }

}


struct Forest(HashMap<Coords, Tree>);

impl Forest {
    fn new(input: &str) -> Forest {
        let mut forest: HashMap<_, _> = HashMap::new();
        let forest_grid: TwoDArray = forest_ndarr(input);

        for row_idx in 0..forest_grid.nrows() {
            for col_idx in 0..forest_grid.ncols() {
                let this_height = forest_grid[[row_idx, col_idx]];

                let surrounding_trees: [OneDArraySlice; 4] = [
                    forest_grid.slice(s![..row_idx;-1, col_idx]),       // up - reverse slice so that tree direction is outwards
                    forest_grid.slice(s![row_idx, (col_idx+1)..]),      // right                
                    forest_grid.slice(s![(row_idx+1).., col_idx]),      // down
                    forest_grid.slice(s![row_idx, ..col_idx;-1]),       // left - reverse slice so that tree direction is outwards
                ];
    
                let mut surrounding_tree_details: Vec<_> = Vec::new();
                for directional_trees in surrounding_trees {
                    let tree_dir_details = (
                        |directional_trees: OneDArraySlice| 
                        {
                            let mut dist: u32 = 0;
                            for (idx, height) in directional_trees.iter().enumerate() {
                                dist = (idx + 1).try_into().expect("forest dimension is not bigger than u32");
                                if *height >= this_height {
                                    return TreeDirectionDetails {dist, visible: false};
                                }
                            }
                            TreeDirectionDetails {dist, visible: true}  
                        }
                    ) (directional_trees);
                    surrounding_tree_details.push(tree_dir_details);
                }
                
                let [
                    up_details, 
                    right_details, 
                    down_details, 
                    left_details
                ]: [_; 4] = surrounding_tree_details.try_into().unwrap_or_else(|_| panic!("Shouldn't happen"));
    
                forest.insert(
                    Coords { x: col_idx as u32, y: row_idx as u32 }, 
                    Tree {
                        up: up_details,
                        right: right_details,
                        down: down_details,
                        left: left_details
                    }
                );
            }
        }
        Forest(forest)
    }

    fn sum_visible_trees(&self) -> u32 {
        self.0
            .values()
            .map(|details| details.is_visible())
            .filter(|b| *b)
            .count()
            .try_into()
            .unwrap()
    }

    fn max_scenic_score(&self) -> u32 {
        self.0
            .values()
            .map(|details| details.scenic_score())
            .max()
            .unwrap()
    }

}



pub fn part_one(input: &str) -> Option<u32> {
    let forest = Forest::new(input);
    let result = forest.sum_visible_trees();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let forest = Forest::new(input);
    let result = forest.max_scenic_score();
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
