use anyhow::{bail, Result};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Trees {
    pub grid: Vec<Vec<u32>>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct VisibilityMap {
    pub grid: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
}

impl VisibilityMap {
    pub fn from_string(trees: &str) -> Self {
        let grid: Vec<Vec<bool>> = trees
            .split_terminator("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| -> Vec<bool> { str_to_bool_vec(*x) })
            .collect();
        let width = grid.len().clone();
        let height = grid[0].len().clone();
        Self {
            grid,
            width,
            height,
        }
    }

    pub fn sum_visible_trees(&self) -> u32 {
        let num_trees: u32 = self
            .grid
            .clone()
            .into_iter()
            .map(|x| -> u32 {
                let a: Vec<u32> = x.into_iter().map(|y| -> u32 { y.into() }).collect();
                a.into_iter().sum()
            })
            .collect::<Vec<u32>>()
            .into_iter()
            .sum();
        num_trees
    }
}

fn str_to_num_vec(x: &str) -> Vec<u32> {
    let y = x
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    y
}

fn str_to_bool_vec(x: &str) -> Vec<bool> {
    let y = x
        .chars()
        .map(|c| -> bool {
            if c.to_digit(10).unwrap() == 0 {
                false
            } else {
                true
            }
        })
        .collect::<Vec<bool>>();
    y
}

fn get_visibility(val: &u32, min_height: &u32, idx: &usize) -> bool {
    val > min_height || *idx == 0
}

fn check_visibility(row: &Vec<u32>) -> Vec<bool> {
    // Get visibility of a row
    let mut min_height: u32 = 0;
    let mut visibility: Vec<bool> = Vec::new();

    // Check left to right
    for (idx, col) in row.iter().enumerate() {
        let is_visible = get_visibility(col, &min_height, &idx);
        if is_visible {
            min_height = *col;
        }
        visibility.push(is_visible);
    }

    // Check right to left
    min_height = 0;
    let mut rev_row = row.clone();
    rev_row.reverse();
    for (idx, col) in rev_row.iter().enumerate() {
        let is_visible = get_visibility(col, &min_height, &idx);
        if is_visible {
            min_height = *col;
        }
        let rev_idx = visibility.len() - idx - 1;
        visibility[rev_idx] = visibility[rev_idx] | is_visible;
    }

    visibility
}

fn compute_scenic_score(tree_idx: &usize, row: &Vec<u32>) -> Result<u32> {
    // Get Compute the score per direction
    if *tree_idx == (row.len() - 1) {
        Ok(0)
    } else {
        let mut stop: u32 = (row.len() - 1).try_into()?;
        for idx in (*tree_idx + 1)..(row.len() - 1) {
            if row[idx] >= row[*tree_idx] {
                stop = idx.try_into()?;
                break;
            }
        }
        let idx: u32 = (*tree_idx).try_into()?;
        Ok(stop - idx)
    }
}

pub fn find_max_scenic_score(all_scores: &Vec<Vec<u32>>) -> Result<u32> {
    let mut max_score: u32 = 0;

    for (_row_idx, row) in all_scores.iter().enumerate() {
        for (_col_idx, val) in row.iter().enumerate() {
            if *val > max_score {
                max_score = *val;
            }
        }
    }

    Ok(max_score)
}

impl Trees {
    pub fn from_string(trees: &str) -> Self {
        let grid: Vec<Vec<u32>> = trees
            .split_terminator("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| -> Vec<u32> { str_to_num_vec(*x) })
            .collect();
        let width = grid.len().clone();
        let height = grid[0].len().clone();
        Trees {
            grid,
            width,
            height,
        }
    }

    pub fn get_row(&self, row: usize) -> Vec<u32> {
        // Get the row
        self.grid[row].clone()
    }

    pub fn get_col(&self, idx: usize) -> Result<Vec<u32>> {
        // Get the col, top to bottom
        let mut col: Vec<u32> = Vec::new();

        for row in &self.grid {
            let val = match row.get(idx) {
                Some(v) => *v,
                None => bail!("Invalid column idx"),
            };
            col.push(val);
        }

        Ok(col)
    }

    pub fn get_visibility(&self) -> Result<VisibilityMap> {
        // Get the entire grid visibility
        let mut visibility: Vec<Vec<bool>> = Vec::new();

        // Check the rows
        for row in &self.grid {
            visibility.push(check_visibility(row));
        }

        // The last row is visible
        for idx in 0..self.width {
            visibility[self.height - 1][idx] = true;
        }

        // Check the columns
        for idx in 0..self.width {
            let col = self.get_col(idx)?;
            let col_visibility = check_visibility(&col);

            // Update the columns
            for row in 0..(visibility.len() - 1) {
                let existing_visibility = visibility[row][idx];
                let update_visibility = col_visibility[row] || existing_visibility;
                visibility[row][idx] = update_visibility;
            }
        }

        Ok(VisibilityMap {
            grid: visibility,
            width: self.width,
            height: self.height,
        })
    }

    pub fn compute_scenic_score(&self, row: &usize, col: &usize) -> Result<u32> {
        // Compute the scenic score
        // Left-right score
        let row_heights = &self.grid[*row];
        let distance_to_right: u32 = compute_scenic_score(col, row_heights)?.into();
        let mut flip_row = self.grid[*row].clone();
        flip_row.reverse();

        let idx: usize = flip_row.len() - 1 - col;
        let distance_to_left = compute_scenic_score(&idx, &flip_row)?;

        // Up-down score
        let mut col_heights = self.get_col(*col)?;
        let distance_to_bottom = compute_scenic_score(row, &col_heights)?;

        let idx: usize = col_heights.len() - 1 - row;
        col_heights.reverse();

        let distance_to_top = compute_scenic_score(&idx, &col_heights)?;
        let score = distance_to_left * distance_to_right * distance_to_top * distance_to_bottom;

        Ok(score)
    }

    pub fn compute_all_scenic_score(&self) -> Result<Self> {
        let mut scenic_score: Vec<Vec<u32>> = Vec::new();
        // Create a blank template
        for _row_idx in 0..self.height {
            let mut row_vec: Vec<u32> = Vec::new();
            for _col_idx in 0..self.width {
                row_vec.push(0);
            }
            scenic_score.push(row_vec);
        }

        // Execute on all rows
        for row_idx in 0..self.height {
            for col_idx in 0..self.width {
                let score = self.compute_scenic_score(&row_idx, &col_idx)?;
                scenic_score[row_idx][col_idx] = score;
            }
        }

        Ok(Trees {
            grid: scenic_score,
            width: self.width.clone(),
            height: self.height.clone(),
        })
    }
}

impl fmt::Display for Trees {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for col in row {
                write!(f, "{}", col)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_row_visible() {
        let row: Vec<u32> = vec![3, 5, 3, 9, 0];
        let get_visibility = check_visibility(&row);

        assert_eq!(get_visibility, vec![true, true, false, true, true]);
    }

    #[test]
    fn test_col_visibility() {
        let row: Vec<u32> = vec![3, 2, 6, 3, 3];
        let get_visibility = check_visibility(&row);

        assert_eq!(get_visibility, vec![true, false, true, false, true]);
    }

    #[test]
    fn test_tree_from_str() {
        let mut file = File::open("test_input.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let trees = Trees::from_string(&buffer);
        assert_eq!(trees.grid[0], vec![3, 0, 3, 7, 3]);
    }

    #[test]
    fn test_tree_get_col() {
        let mut file = File::open("test_input.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let trees = Trees::from_string(&buffer);
        let col = trees.get_col(0).unwrap();
        assert_eq!(col, vec![3, 2, 6, 3, 3]);
    }

    #[test]
    fn test_tree_visibility() {
        // Get input
        let mut file = File::open("test_input.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        // Get result
        let mut file = File::open("test_result.txt").unwrap();
        let mut output: String = String::new();
        file.read_to_string(&mut output).unwrap();

        let expected = VisibilityMap::from_string(&output);

        let trees = Trees::from_string(&buffer);
        let output = trees.get_visibility().unwrap();
        assert_eq!(expected, output);

        // Count the expected sums
        assert_eq!(output.sum_visible_trees(), 21);
    }

    #[test]
    fn test_1d_scenic_score() {
        let mut row: Vec<u32> = vec![2, 5, 5, 1, 2];
        let idx: usize = 2;

        let score = compute_scenic_score(&idx, &row).unwrap();
        assert_eq!(score, 2);

        // Reverse to get the score in the other direction
        // [2, 1, 5, 5, 2]
        row.reverse();
        let idx: usize = row.len() - 1 - idx;
        let score = compute_scenic_score(&idx, &row).unwrap();
        assert_eq!(score, 1);
    }

    #[test]
    fn test_2d_scenic_score() {
        let mut file = File::open("test_input.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let trees = Trees::from_string(&buffer);

        assert_eq!(trees.compute_scenic_score(&1, &2).unwrap(), 4);
    }

    #[test]
    fn test_all_scenic_score() {
        let mut file = File::open("test_input.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let trees = Trees::from_string(&buffer);
        let scenic_scores = trees.compute_all_scenic_score().unwrap();
        let max_score = find_max_scenic_score(&scenic_scores.grid).unwrap();
        println!("Scenic scores");
        println!("{}", scenic_scores);
        assert_eq!(max_score, 8);
    }
}
