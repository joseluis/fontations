//! splitting of MarkToBase positioning subtables

use std::collections::HashSet;

use font_types::{FixedSize, Offset16};
use read_fonts::tables::{gpos as rgpos, layout as rlayout};

use crate::{tables::layout::CoverageTable, write::TableData};

use super::{Graph, ObjectId};

pub(crate) fn split_mark_to_base(graph: &mut Graph, lookup: ObjectId) {
    super::split_subtables(graph, lookup, split_mark_to_base_subtable)
}

// based off of <https://github.com/harfbuzz/harfbuzz/blob/f26fd69d858642d764/src/graph/markbasepos-graph.hh#L212>
fn split_mark_to_base_subtable(graph: &mut Graph, subtable: ObjectId) -> Option<Vec<ObjectId>> {
    // the base size for the table, not the size of the 'base glyphs subtable'
    const BASE_SIZE: usize = 6 * u16::RAW_BYTE_LEN // six short fields in subtable
                               + u16::RAW_BYTE_LEN // empty mark array table
                               + u16::RAW_BYTE_LEN; // empty base array table
    let data = &graph.objects[&subtable];
    let base_coverage_id = data.offsets[1].object;
    let base_coverage_size = graph.objects[&base_coverage_id].bytes.len();
    debug_assert!(data.reparse::<rgpos::MarkBasePosFormat1>().is_ok());

    let min_subtable_size = BASE_SIZE + base_coverage_size;
    let class_info = get_class_info(graph, subtable);

    let base_array_off = &data.offsets[3];
    let base_array_data = &graph.objects[&base_array_off.object];

    let mark_class_count: u16 = data.read_at(6).unwrap_or(0);
    debug_assert_eq!(class_info.len(), mark_class_count as usize);
    let base_count: u16 = base_array_data.read_at(0).unwrap_or(0);

    let mut partial_coverage_size = 4;
    let mut accumulated = min_subtable_size;
    let mut split_points = Vec::new();
    let mut visited = HashSet::new();

    for i in 0..mark_class_count {
        let info = &class_info[i as usize];
        partial_coverage_size += u16::RAW_BYTE_LEN * info.marks.len();

        let mut accumulated_delta =
            // the records for the marks in this class
            rgpos::MarkRecord::RAW_BYTE_LEN * info.marks.len()
            // plus an offset in each base record for this class
            + Offset16::RAW_BYTE_LEN * base_count as usize;
        accumulated_delta += compute_subgraph_size(&info.children, graph, &mut visited);
        accumulated += accumulated_delta;
        let total = accumulated + partial_coverage_size;

        if total > super::MAX_TABLE_SIZE {
            log::trace!("adding split at {i}");
            split_points.push(i as usize);
            accumulated = min_subtable_size + accumulated_delta;
            partial_coverage_size = 4 + u16::RAW_BYTE_LEN * info.marks.len();
            visited.clear();
        }
    }

    log::debug!(
        "nothing to split, size '{}'",
        accumulated + partial_coverage_size
    );

    if split_points.is_empty() {
        return None;
    }

    split_points.push(mark_class_count as _);
    let mut new_subtables = Vec::new();
    let mut prev_split = 0;

    for next_split in split_points {
        let new_subtable = split_off_mark_pos(graph, subtable, prev_split, next_split, &class_info);
        prev_split = next_split;
        new_subtables.push(graph.add_object(new_subtable));
    }

    Some(new_subtables)
}

// based off of <https://github.com/harfbuzz/harfbuzz/blob/f26fd69d858642/src/graph/markbasepos-graph.hh#L411>
fn split_off_mark_pos(
    graph: &mut Graph,
    subtable: ObjectId,
    start: usize,
    end: usize,
    class_info: &[Mark2BaseClassInfo],
) -> TableData {
    let mark_coverage_id = graph.objects[&subtable].offsets.first().unwrap().object;
    let mark_coverage = &graph.objects[&mark_coverage_id];
    let mark_coverage = mark_coverage.reparse::<rlayout::CoverageTable>().unwrap();
    let data = &graph.objects[&subtable];
    let base_coverage_id = data.offsets[1].object;
    let mark_array_id = data.offsets[2].object;
    let base_array_id = data.offsets[3].object;
    let old_mark_array = &graph.objects[&mark_array_id];
    let old_base_array = &graph.objects[&base_array_id];
    let mark_class_count = (end - start) as u16;
    let mut new_subtable = TableData::new(data.type_);

    let mark_glyphs_by_cov_id: HashSet<_> = (start..end)
        .flat_map(|class_idx| class_info[class_idx].marks.iter())
        .copied()
        .collect();
    let new_mark_coverage: CoverageTable = mark_coverage
        .iter()
        .enumerate()
        .filter_map(|(i, gid)| mark_glyphs_by_cov_id.contains(&i).then_some(gid))
        .collect();
    let new_mark_coverage = super::make_table_data(&new_mark_coverage);
    let new_mark_array = split_off_mark_array(old_mark_array, start as u16, &mark_glyphs_by_cov_id);
    let new_base_array = split_off_base_array(old_base_array, start, end, class_info.len());
    let new_mark_coverage_id = graph.add_object(new_mark_coverage);
    let new_mark_array_id = graph.add_object(new_mark_array);
    let new_base_array_id = graph.add_object(new_base_array);

    new_subtable.write(1u16); // format
    new_subtable.add_offset(new_mark_coverage_id, 2, 0);
    new_subtable.add_offset(base_coverage_id, 2, 0);
    new_subtable.write(mark_class_count);
    new_subtable.add_offset(new_mark_array_id, 2, 0);
    new_subtable.add_offset(new_base_array_id, 2, 0);

    new_subtable
}

// <https://github.com/harfbuzz/harfbuzz/blob/f26fd69d858642d76413b8f/src/graph/markbasepos-graph.hh#L170>
fn split_off_mark_array(
    old_mark_array: &TableData,
    first_class: u16,
    mark_glyph_coverage_ids: &HashSet<usize>,
) -> TableData {
    let mark_array = old_mark_array.reparse::<rgpos::MarkArray>().unwrap();
    let mark_count = mark_glyph_coverage_ids.len() as u16;

    let mut new_mark_array = TableData::new(old_mark_array.type_);
    new_mark_array.write(mark_count);

    for (i, mark_record) in mark_array.mark_records().iter().enumerate() {
        if !mark_glyph_coverage_ids.contains(&i) {
            continue;
        }
        let new_class = mark_record.mark_class() - first_class;
        let anchor_offset = old_mark_array.offsets[i].object;

        new_mark_array.write(new_class);
        new_mark_array.add_offset(anchor_offset, 2, 0);
    }

    new_mark_array
}

fn split_off_base_array(
    old_base_array: &TableData,
    start: usize,
    end: usize,
    old_mark_class_count: usize,
) -> TableData {
    let mut new_base_array = TableData::new(old_base_array.type_);
    let base_count: u16 = old_base_array.read_at(0).unwrap_or(0);
    new_base_array.write(base_count);

    // the base array contains a (base_count x mark_count) matrix of offsets.
    // for each base, we want to prune the marks to only include those
    // in the range (start..end).

    let base_array: rgpos::BaseArray = old_base_array
        .reparse_with_args(&(old_mark_class_count as u16))
        .unwrap();

    let mut next_offset_idx = 0;
    // because offsets may be null, and there is no pattern, we visit each one
    for base_record in base_array.base_records().iter() {
        let base_record = base_record.unwrap();
        for (mark_class, offset) in base_record.base_anchor_offsets().iter().enumerate() {
            let has_offset = !offset.get().is_null();
            let in_range = (start..end).contains(&mark_class);

            if in_range {
                if has_offset {
                    let id = old_base_array.offsets[next_offset_idx].object;
                    new_base_array.add_offset(id, 2, 0);
                } else {
                    // manually write null value
                    new_base_array.write(0u16);
                }
            }

            if has_offset {
                next_offset_idx += 1;
            }
        }
    }

    new_base_array
}

/// Information about a single mark class in a Mark2Base subtable
#[derive(Clone, Debug, Default)]
struct Mark2BaseClassInfo {
    // value is the order in the coverage table
    marks: HashSet<usize>,
    children: Vec<ObjectId>,
}

// this is not general purpose! tailored to mark2pos
fn compute_subgraph_size(
    objects: &[ObjectId],
    graph: &Graph,
    visited: &mut HashSet<ObjectId>,
) -> usize {
    objects
        .iter()
        .map(|id| {
            if !visited.insert(*id) {
                return 0;
            }
            // the size of the anchor table
            let base_size = graph.objects[id].bytes.len();
            // the size of any devices or variation indices.
            let children_size = graph.objects[id]
                .offsets
                .iter()
                .map(|id| {
                    // the mark2pos subgraph is only ever two layers deep
                    debug_assert!(graph.objects[&id.object].offsets.is_empty());
                    visited
                        .insert(id.object)
                        .then(|| graph.objects[&id.object].bytes.len())
                        .unwrap_or(0)
                })
                .sum::<usize>();
            base_size + children_size
        })
        .sum()
}

// get info about the mark classes in a MarkToBase subtable.
//
// based on <https://github.com/harfbuzz/harfbuzz/blob/f26fd69d858642d76/src/graph/markbasepos-graph.hh#L316>
fn get_class_info(graph: &Graph, subtable: ObjectId) -> Vec<Mark2BaseClassInfo> {
    let data = &graph.objects[&subtable];
    let mark_class_count: u16 = data.read_at(6).unwrap_or(0);
    let mut class_to_info = vec![Mark2BaseClassInfo::default(); mark_class_count as usize];
    let mark_array_off = &data.offsets[2];
    assert_eq!(mark_array_off.pos, 8);
    let mark_array_data = &graph.objects[&mark_array_off.object];
    let mark_array = mark_array_data.reparse::<rgpos::MarkArray>().unwrap();
    // okay so:
    // - there is one mark record for each mark glyph
    // - there may be multiple mark glyphs with the same class
    for (i, mark_record) in mark_array.mark_records().iter().enumerate() {
        let mark_class = mark_record.mark_class();
        // this shouldn't happen unless data is malformed? but harfbuzz includes
        // this check, and it doesn't hurt.
        if mark_class >= mark_class_count {
            continue;
        }
        let anchor_table_id = mark_array_data.offsets[i].object;
        class_to_info[mark_class as usize].marks.insert(i);
        class_to_info[mark_class as usize]
            .children
            .push(anchor_table_id);
    }

    // - base array declares one record for each base glyph (in cov table order)
    // - each record has an anchor for each mark glyph
    let base_array_off = &data.offsets[3];
    assert_eq!(base_array_off.pos, 10);
    let base_array_data = &graph.objects[&base_array_off.object];

    let base_count: u16 = base_array_data.read_at(0).unwrap_or(0);
    assert_eq!(
        base_array_data.offsets.len(),
        (base_count as usize * mark_class_count as usize)
    );

    for offsets in base_array_data.offsets.chunks_exact(mark_class_count as _) {
        for (i, off) in offsets.iter().enumerate() {
            class_to_info[i].children.push(off.object)
        }
    }

    class_to_info
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use crate::{
        tables::gpos::{AnchorTable, BaseArray, BaseRecord, MarkArray, MarkRecord},
        TableWriter,
    };

    use super::*;

    // too fancy, but:
    //
    // we want to create anchor tables for each glyph, and later check that
    // we have the correct anchor tables after splitting.
    //
    // the idea here is to map u16s to i16s in a way that is easy to understand
    // at a glance, so we do 0, 1 .. i16::MAX, -1, -2 .. i16::MIN
    fn u16_to_i16(val: u16) -> i16 {
        match i16::try_from(val) {
            Ok(val) => val,
            Err(_) => i16::MAX.saturating_sub_unsigned(val),
        }
    }

    fn make_mark_array(class_count: u16, glyphs_per_class: u16) -> MarkArray {
        let mark_glyph_count = class_count * glyphs_per_class;
        let records = (0..mark_glyph_count)
            .map(|cov_idx| {
                let mark_class = cov_idx / glyphs_per_class;
                let anchor_val = u16_to_i16(cov_idx);
                MarkRecord::new(mark_class, AnchorTable::format_1(anchor_val, anchor_val))
            })
            .collect();
        MarkArray::new(records)
    }

    fn make_base_array(base_count: u16, mark_class_count: u16) -> BaseArray {
        let base_records = (0..base_count)
            .map(|i| {
                let mark_anchors = (0..mark_class_count)
                    .map(|j| {
                        // even anchors exist, odd anchors are null
                        (j % 2 == 0).then(|| {
                            let val = u16_to_i16(i * mark_class_count + j);
                            AnchorTable::format_1(val, val)
                        })
                    })
                    .collect();
                BaseRecord::new(mark_anchors)
            })
            .collect();
        BaseArray::new(base_records)
    }

    #[test]
    fn test_my_test_helper() {
        assert_eq!(u16_to_i16(1), 1);
        assert_eq!(u16_to_i16(32767), 32767);
        assert_eq!(u16_to_i16(32768), -1);
        assert_eq!(u16_to_i16(u16::MAX), i16::MIN);
    }

    #[test]
    fn split_mark_array() {
        const N_GLYPHS: u16 = 900;
        const N_CLASSES: u16 = 75;
        const GLYPHS_PER_CLASS: u16 = N_GLYPHS / N_CLASSES;
        const SPLIT_CLASS_RANGE: Range<u16> = 20..25;

        let mark_array = make_mark_array(N_CLASSES, GLYPHS_PER_CLASS);

        let graph = TableWriter::make_graph(&mark_array);
        let data = &graph.objects[&graph.root];
        // now let's imagine we're splitting off classes 20..25
        let mark_glyph_coverage_ids = mark_array
            .mark_records
            .iter()
            .enumerate()
            .filter_map(|(i, rec)| SPLIT_CLASS_RANGE.contains(&rec.mark_class).then_some(i))
            .collect::<HashSet<_>>();

        assert_eq!(
            mark_glyph_coverage_ids.len(),
            SPLIT_CLASS_RANGE.len() * GLYPHS_PER_CLASS as usize
        );

        let result = split_off_mark_array(data, SPLIT_CLASS_RANGE.start, &mark_glyph_coverage_ids);
        assert_eq!(
            result.offsets.len(),
            GLYPHS_PER_CLASS as usize * SPLIT_CLASS_RANGE.len()
        );

        let reparsed = result.reparse::<rgpos::MarkArray>().unwrap();
        // ensure classes are correct
        for (i, rec) in reparsed.mark_records().iter().enumerate() {
            let exp_class = i as u16 / GLYPHS_PER_CLASS;
            assert_eq!(rec.mark_class(), exp_class);
        }

        // ensure anchors are right
        for (i, offset) in result.offsets.iter().enumerate() {
            let gid_delta = SPLIT_CLASS_RANGE.start * GLYPHS_PER_CLASS;
            let gid = i as u16 + gid_delta;
            let anchor_val = u16_to_i16(gid);
            let anchor_table = &graph.objects[&offset.object];
            let anchor_table = anchor_table.reparse::<rgpos::AnchorFormat1>().unwrap();
            assert_eq!(anchor_table.x_coordinate(), anchor_val);
        }
    }

    #[test]
    fn split_base_array() {
        const N_CLASSES: u16 = 20;
        const N_BASES: u16 = 10;
        const SPLIT_CLASS_RANGE: Range<u16> = 15..20;

        let base_array = make_base_array(N_BASES, N_CLASSES);
        let graph = TableWriter::make_graph(&base_array);
        let data = &graph.objects[&graph.root];

        let result = split_off_base_array(
            data,
            SPLIT_CLASS_RANGE.start as _,
            SPLIT_CLASS_RANGE.end as _,
            N_CLASSES as _,
        );

        assert_eq!(result.read_at::<u16>(0).unwrap(), N_BASES);
        let mut idx = 0;
        for base in 0..N_BASES {
            for mark_class in 0..SPLIT_CLASS_RANGE.len() as u16 {
                let original_mark_class = mark_class + SPLIT_CLASS_RANGE.start;
                if original_mark_class % 2 == 0 {
                    // anchor table is non-null
                    let anchor_id = result.offsets[idx].object;
                    let anchor = &graph.objects[&anchor_id];
                    let anchor = anchor.reparse::<rgpos::AnchorFormat1>().unwrap();

                    let exp_val = u16_to_i16((base * N_CLASSES) + original_mark_class);
                    assert_eq!(
                        anchor.x_coordinate(),
                        exp_val,
                        "base {base} mark {mark_class} {anchor_id:?}"
                    );
                    idx += 1;
                }
            }
        }
    }
}