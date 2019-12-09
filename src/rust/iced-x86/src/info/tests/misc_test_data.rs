/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use super::super::super::test_utils::from_str_conv::*;
use super::super::super::test_utils::section_file_reader::*;
use super::super::super::*;
use super::super::test_utils::*;
use std::collections::HashSet;

// GENERATOR-BEGIN: MiscSectionNames
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct MiscSectionNames;
impl MiscSectionNames {
	pub(crate) const JCC_SHORT: &'static str = "jcc-short";
	pub(crate) const JCC_NEAR: &'static str = "jcc-near";
	pub(crate) const JMP_SHORT: &'static str = "jmp-short";
	pub(crate) const JMP_NEAR: &'static str = "jmp-near";
	pub(crate) const JMP_FAR: &'static str = "jmp-far";
	pub(crate) const JMP_NEAR_INDIRECT: &'static str = "jmp-near-indirect";
	pub(crate) const JMP_FAR_INDIRECT: &'static str = "jmp-far-indirect";
	pub(crate) const CALL_NEAR: &'static str = "call-near";
	pub(crate) const CALL_FAR: &'static str = "call-far";
	pub(crate) const CALL_NEAR_INDIRECT: &'static str = "call-near-indirect";
	pub(crate) const CALL_FAR_INDIRECT: &'static str = "call-far-indirect";
	pub(crate) const JMP_INFO: &'static str = "jmp-info";
	pub(crate) const JCC_SHORT_INFO: &'static str = "jcc-short-info";
	pub(crate) const JCC_NEAR_INFO: &'static str = "jcc-near-info";
	pub(crate) const SETCC_INFO: &'static str = "setcc-info";
	pub(crate) const CMOVCC_INFO: &'static str = "cmovcc-info";
}
// GENERATOR-END: MiscSectionNames

struct MiscSectionNameIds {}
impl MiscSectionNameIds {
	const JCC_SHORT: u32 = 0;
	const JCC_NEAR: u32 = 1;
	const JMP_SHORT: u32 = 2;
	const JMP_NEAR: u32 = 3;
	const JMP_FAR: u32 = 4;
	const JMP_NEAR_INDIRECT: u32 = 5;
	const JMP_FAR_INDIRECT: u32 = 6;
	const CALL_NEAR: u32 = 7;
	const CALL_FAR: u32 = 8;
	const CALL_NEAR_INDIRECT: u32 = 9;
	const CALL_FAR_INDIRECT: u32 = 10;
	const JMP_INFO: u32 = 11;
	const JCC_SHORT_INFO: u32 = 12;
	const JCC_NEAR_INFO: u32 = 13;
	const SETCC_INFO: u32 = 14;
	const CMOVCC_INFO: u32 = 15;
}

pub(crate) struct MiscTestsData {
	pub(crate) jcc_short: HashSet<Code>,
	pub(crate) jmp_near: HashSet<Code>,
	pub(crate) jmp_far: HashSet<Code>,
	pub(crate) jmp_short: HashSet<Code>,
	pub(crate) jmp_near_indirect: HashSet<Code>,
	pub(crate) jmp_far_indirect: HashSet<Code>,
	pub(crate) jcc_near: HashSet<Code>,
	pub(crate) call_far: HashSet<Code>,
	pub(crate) call_near: HashSet<Code>,
	pub(crate) call_near_indirect: HashSet<Code>,
	pub(crate) call_far_indirect: HashSet<Code>,
	pub(crate) jmp_infos: Vec<(Code, Code)>,
	pub(crate) jcc_short_infos: Vec<(Code, Code, Code, ConditionCode)>,
	pub(crate) jcc_near_infos: Vec<(Code, Code, Code, ConditionCode)>,
	pub(crate) setcc_infos: Vec<(Code, Code, ConditionCode)>,
	pub(crate) cmovcc_infos: Vec<(Code, Code, ConditionCode)>,
}

lazy_static! {
	pub(super) static ref MISC_TESTS_DATA: MiscTestsData = {
		let mut data_reader = MiscTestsDataReader::new();
		data_reader.read();
		data_reader.data
	};
}

struct MiscTestsDataReader {
	data: MiscTestsData,
}

impl MiscTestsDataReader {
	fn new() -> Self {
		Self {
			data: MiscTestsData {
				jcc_short: HashSet::new(),
				jmp_near: HashSet::new(),
				jmp_far: HashSet::new(),
				jmp_short: HashSet::new(),
				jmp_near_indirect: HashSet::new(),
				jmp_far_indirect: HashSet::new(),
				jcc_near: HashSet::new(),
				call_far: HashSet::new(),
				call_near: HashSet::new(),
				call_near_indirect: HashSet::new(),
				call_far_indirect: HashSet::new(),
				jmp_infos: Vec::new(),
				jcc_short_infos: Vec::new(),
				jcc_near_infos: Vec::new(),
				setcc_infos: Vec::new(),
				cmovcc_infos: Vec::new(),
			},
		}
	}

	fn read(&mut self) {
		let infos = &[
			(MiscSectionNames::JCC_SHORT, MiscSectionNameIds::JCC_SHORT),
			(MiscSectionNames::JMP_NEAR, MiscSectionNameIds::JMP_NEAR),
			(MiscSectionNames::JMP_FAR, MiscSectionNameIds::JMP_FAR),
			(MiscSectionNames::JMP_SHORT, MiscSectionNameIds::JMP_SHORT),
			(MiscSectionNames::JMP_NEAR_INDIRECT, MiscSectionNameIds::JMP_NEAR_INDIRECT),
			(MiscSectionNames::JMP_FAR_INDIRECT, MiscSectionNameIds::JMP_FAR_INDIRECT),
			(MiscSectionNames::JCC_NEAR, MiscSectionNameIds::JCC_NEAR),
			(MiscSectionNames::CALL_FAR, MiscSectionNameIds::CALL_FAR),
			(MiscSectionNames::CALL_NEAR, MiscSectionNameIds::CALL_NEAR),
			(MiscSectionNames::CALL_NEAR_INDIRECT, MiscSectionNameIds::CALL_NEAR_INDIRECT),
			(MiscSectionNames::CALL_FAR_INDIRECT, MiscSectionNameIds::CALL_FAR_INDIRECT),
			(MiscSectionNames::JMP_INFO, MiscSectionNameIds::JMP_INFO),
			(MiscSectionNames::JCC_SHORT_INFO, MiscSectionNameIds::JCC_SHORT_INFO),
			(MiscSectionNames::JCC_NEAR_INFO, MiscSectionNameIds::JCC_NEAR_INFO),
			(MiscSectionNames::SETCC_INFO, MiscSectionNameIds::SETCC_INFO),
			(MiscSectionNames::CMOVCC_INFO, MiscSectionNameIds::CMOVCC_INFO),
		];
		let mut reader = SectionFileReader::new(infos);
		let mut path = get_instr_info_unit_tests_dir();
		path.push("Misc.txt");
		reader.read(&path, self);
	}

	fn add_code(h: &mut HashSet<Code>, line: &str) -> Result<(), String> {
		let _ = h.insert(to_code(line)?);
		Ok(())
	}

	fn add_jmp_info(v: &mut Vec<(Code, Code)>, line: &str) -> Result<(), String> {
		const ELEMS: usize = 2;
		let elems: Vec<_> = line.split(',').collect();
		if elems.len() != ELEMS {
			return Err(format!("Expected {} elements, found {}", ELEMS, elems.len()));
		}
		v.push((to_code(elems[0])?, to_code(elems[1])?));
		Ok(())
	}

	fn add_jcc_info(v: &mut Vec<(Code, Code, Code, ConditionCode)>, line: &str) -> Result<(), String> {
		const ELEMS: usize = 4;
		let elems: Vec<_> = line.split(',').collect();
		if elems.len() != ELEMS {
			return Err(format!("Expected {} elements, found {}", ELEMS, elems.len()));
		}
		v.push((to_code(elems[0])?, to_code(elems[1])?, to_code(elems[2])?, to_condition_code(elems[3])?));
		Ok(())
	}

	fn add_instr_cc_info(v: &mut Vec<(Code, Code, ConditionCode)>, line: &str) -> Result<(), String> {
		const ELEMS: usize = 3;
		let elems: Vec<_> = line.split(',').collect();
		if elems.len() != ELEMS {
			return Err(format!("Expected {} elements, found {}", ELEMS, elems.len()));
		}
		v.push((to_code(elems[0])?, to_code(elems[1])?, to_condition_code(elems[2])?));
		Ok(())
	}
}

impl SectionFileLineHandler for MiscTestsDataReader {
	fn line(&mut self, id: u32, line: &str) -> Result<(), String> {
		match id {
			MiscSectionNameIds::JCC_SHORT => Self::add_code(&mut self.data.jcc_short, line),
			MiscSectionNameIds::JMP_NEAR => Self::add_code(&mut self.data.jmp_near, line),
			MiscSectionNameIds::JMP_FAR => Self::add_code(&mut self.data.jmp_far, line),
			MiscSectionNameIds::JMP_SHORT => Self::add_code(&mut self.data.jmp_short, line),
			MiscSectionNameIds::JMP_NEAR_INDIRECT => Self::add_code(&mut self.data.jmp_near_indirect, line),
			MiscSectionNameIds::JMP_FAR_INDIRECT => Self::add_code(&mut self.data.jmp_far_indirect, line),
			MiscSectionNameIds::JCC_NEAR => Self::add_code(&mut self.data.jcc_near, line),
			MiscSectionNameIds::CALL_FAR => Self::add_code(&mut self.data.call_far, line),
			MiscSectionNameIds::CALL_NEAR => Self::add_code(&mut self.data.call_near, line),
			MiscSectionNameIds::CALL_NEAR_INDIRECT => Self::add_code(&mut self.data.call_near_indirect, line),
			MiscSectionNameIds::CALL_FAR_INDIRECT => Self::add_code(&mut self.data.call_far_indirect, line),
			MiscSectionNameIds::JMP_INFO => Self::add_jmp_info(&mut self.data.jmp_infos, line),
			MiscSectionNameIds::JCC_SHORT_INFO => Self::add_jcc_info(&mut self.data.jcc_short_infos, line),
			MiscSectionNameIds::JCC_NEAR_INFO => Self::add_jcc_info(&mut self.data.jcc_near_infos, line),
			MiscSectionNameIds::SETCC_INFO => Self::add_instr_cc_info(&mut self.data.setcc_infos, line),
			MiscSectionNameIds::CMOVCC_INFO => Self::add_instr_cc_info(&mut self.data.cmovcc_infos, line),
			_ => panic!(),
		}
	}
}
