/*
 * Copyright (c) 2021-2024 Works Applications Co., Ltd.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::collections::BTreeSet;
use std::fs;
use std::io::{BufRead, BufReader};
use std::iter::FusedIterator;
use std::ops::Range;
use std::path::Path;

use thiserror::Error;

use crate::dic::category_type::CategoryType;
use crate::prelude::*;

/// Sudachi error
#[derive(Error, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    #[error("Invalid format at line {0}")]
    InvalidFormat(usize),

    #[error("Invalid type {1} at line {0}")]
    InvalidCategoryType(usize, String),

    #[error("Multiple definition for type {1} at line {0}")]
    MultipleTypeDefinition(usize, String),

    #[error("Invalid character {0:X} at line {1}")]
    InvalidChar(u32, usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CatRange {
    begin: u32,
    end: u32,
    categories: CategoryType,
}

/// CharacterCategory holds mapping from character to character category type
#[derive(Debug, Clone)]
pub struct CharacterCategory {
    /// Split the whole domain of codepoints into ranges,
    /// limited by boundaries.
    ///
    /// Ranges are half-open: `[boundaries[i], boundaries[i + 1])`
    /// meaning that the right bound is not included.
    /// 0 and u32::MAX are not stored, they are included implicitly
    /// as if they would have indices of `-1` and `boundaries.len()`.
    boundaries: Vec<u32>,

    /// Stores the category for each range.
    /// `categories[i]` is for the range `[boundaries[i - 1], boundaries[i])`.
    /// Plays well with [`std::slice::binary_search`], see [`get_category_types()`].
    /// This should be always true: `boundaries.len() + 1 == categories.len()`.
    categories: Vec<CategoryType>,
}

impl Default for CharacterCategory {
    fn default() -> Self {
        CharacterCategory {
            boundaries: Vec::new(),
            categories: vec![CategoryType::DEFAULT],
        }
    }
}

impl CharacterCategory {
    /// Creates a character category from file
    pub fn from_file(path: &Path) -> SudachiResult<CharacterCategory> {
        let reader = BufReader::new(fs::File::open(path)?);
        Self::from_reader(reader)
    }

    pub fn from_bytes(bytes: &[u8]) -> SudachiResult<CharacterCategory> {
        let reader = BufReader::new(bytes);
        Self::from_reader(reader)
    }

    pub fn from_reader<T: BufRead>(data: T) -> SudachiResult<CharacterCategory> {
        let ranges = Self::read_character_definition(data)?;
        Ok(Self::compile(&ranges))
    }

    /// Reads character type definition as a list of Ranges
    ///
    /// Definition file syntax:
    ///     Each line contains [TARGET_CHARACTER_CODE_POINT] [TYPES], where
    ///     TARGET_CHARACTER_CODE_POINT:
    ///         a code_point in hexadecimal format or two separated by ".."
    ///     TYPES:
    ///         one or more Category_types separated by white space
    ///     Loads only lines start with "0x" are loaded and ignore others
    ///
    /// Definition example:
    ///     "0x0030..0x0039 NUMERIC"
    ///     "0x3008         KANJI KANJINUMERIC"
    fn read_character_definition<T: BufRead>(reader: T) -> SudachiResult<Vec<CatRange>> {
        let mut ranges: Vec<CatRange> = Vec::new();
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || !line.starts_with("0x") {
                continue;
            }

            let cols: Vec<_> = line.split_whitespace().collect();
            if cols.len() < 2 {
                return Err(SudachiError::InvalidCharacterCategory(
                    Error::InvalidFormat(i),
                ));
            }

            let r: Vec<_> = cols[0].split("..").collect();
            let begin = u32::from_str_radix(String::from(r[0]).trim_start_matches("0x"), 16)?;
            let end = if r.len() > 1 {
                u32::from_str_radix(String::from(r[1]).trim_start_matches("0x"), 16)? + 1
            } else {
                begin + 1
            };
            if begin >= end {
                return Err(SudachiError::InvalidCharacterCategory(
                    Error::InvalidFormat(i),
                ));
            }
            if char::from_u32(begin).is_none() {
                return Err(SudachiError::InvalidCharacterCategory(Error::InvalidChar(
                    begin, i,
                )));
            }

            if char::from_u32(end).is_none() {
                return Err(SudachiError::InvalidCharacterCategory(Error::InvalidChar(
                    end, i,
                )));
            }

            let mut categories = CategoryType::empty();
            for elem in cols[1..].iter().take_while(|elem| !elem.starts_with('#')) {
                categories.insert(match elem.parse() {
                    Ok(t) => t,
                    Err(_) => {
                        return Err(SudachiError::InvalidCharacterCategory(
                            Error::InvalidCategoryType(i, elem.to_string()),
                        ))
                    }
                });
            }

            ranges.push(CatRange {
                begin,
                end,
                categories,
            });
        }

        Ok(ranges)
    }

    /// Creates a character category from given range_list
    ///
    /// Transforms given range_list to non overlapped range list
    /// to apply binary search in get_category_types
    fn compile(ranges: &Vec<CatRange>) -> CharacterCategory {
        if ranges.is_empty() {
            return CharacterCategory::default();
        }

        let boundaries = Self::collect_boundaries(ranges);
        let mut categories = vec![CategoryType::empty(); boundaries.len()];

        for range in ranges {
            let start_idx = match boundaries.binary_search(&range.begin) {
                Ok(i) => i + 1,
                Err(_) => panic!("there can not be not found boundaries"),
            };
            // apply category to all splits which are included in the current range
            for i in start_idx..boundaries.len() {
                if boundaries[i] > range.end {
                    break;
                }
                categories[i] |= range.categories;
            }
        }

        // first category is always default (it is impossible to get it assigned above)
        debug_assert_eq!(categories[0], CategoryType::empty());
        categories[0] = CategoryType::DEFAULT;
        // merge successive ranges of the same category
        let mut final_boundaries = Vec::with_capacity(boundaries.len());
        let mut final_categories = Vec::with_capacity(categories.len());

        let mut last_category = categories[0];
        let mut last_boundary = boundaries[0];
        for i in 1..categories.len() {
            if categories[i] == last_category {
                last_boundary = boundaries[i];
                continue;
            }
            final_boundaries.push(last_boundary);
            final_categories.push(last_category);
            last_category = categories[i];
            last_boundary = boundaries[i];
        }

        final_categories.push(last_category);
        final_boundaries.push(last_boundary);

        // replace empty categories with default
        for cat in final_categories.iter_mut() {
            if cat.is_empty() {
                *cat = CategoryType::DEFAULT;
            }
        }

        // and add the category after the last boundary
        final_categories.push(CategoryType::DEFAULT);

        final_boundaries.shrink_to_fit();
        final_categories.shrink_to_fit();

        CharacterCategory {
            boundaries: final_boundaries,
            categories: final_categories,
        }
    }

    /// Find sorted list of all boundaries
    fn collect_boundaries(data: &Vec<CatRange>) -> Vec<u32> {
        let mut boundaries = BTreeSet::new();
        for i in data {
            boundaries.insert(i.begin);
            boundaries.insert(i.end);
        }
        boundaries.into_iter().collect()
    }

    /// Returns a set of category types which given char has
    pub fn get_category_types(&self, c: char) -> CategoryType {
        if self.boundaries.is_empty() {
            return CategoryType::DEFAULT;
        }
        let cint = c as u32;
        match self.boundaries.binary_search(&cint) {
            //Ok means the index in boundaries, so the next category
            Ok(idx) => self.categories[idx + 1],
            //Err means the insertion index, so the current category
            Err(idx) => self.categories[idx],
        }
    }

    pub fn iter(&self) -> CharCategoryIter {
        CharCategoryIter {
            categories: self,
            current: 0,
        }
    }
}

pub struct CharCategoryIter<'a> {
    categories: &'a CharacterCategory,
    current: usize,
}

impl Iterator for CharCategoryIter<'_> {
    type Item = (Range<char>, CategoryType);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.categories.boundaries.len() + 1 {
            return None;
        }

        // char casts are safe, we are checking for correctness during the data load
        let range = if self.current == self.categories.boundaries.len() {
            let left = char::from_u32(*self.categories.boundaries.last().unwrap()).unwrap();
            (left..char::MAX, *self.categories.categories.last().unwrap())
        } else if self.current == 0 {
            let right = char::from_u32(*self.categories.boundaries.first().unwrap()).unwrap();
            let r = (0 as char)..right;
            (r, self.categories.categories[0])
        } else {
            let left = char::from_u32(self.categories.boundaries[self.current - 1]).unwrap();
            let right = char::from_u32(self.categories.boundaries[self.current]).unwrap();
            let cat = self.categories.categories[self.current];
            (left..right, cat)
        };

        self.current += 1;
        Some(range)
    }
}

impl FusedIterator for CharCategoryIter<'_> {}
