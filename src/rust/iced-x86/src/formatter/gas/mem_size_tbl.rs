// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use crate::formatter::fmt_consts::*;
use crate::formatter::gas::FormatterString;
use crate::iced_constants::IcedConstants;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::convert::TryInto;
use lazy_static::lazy_static;

// GENERATOR-BEGIN: BcstTo
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[rustfmt::skip]
static BCST_TO_DATA: [u8; 50] = [
	0x01,
	0x01,
	0x01,
	0x02,
	0x01,
	0x03,
	0x03,
	0x02,
	0x02,
	0x01,
	0x01,
	0x01,
	0x03,
	0x02,
	0x01,
	0x02,
	0x01,
	0x01,
	0x02,
	0x02,
	0x04,
	0x04,
	0x03,
	0x03,
	0x02,
	0x02,
	0x02,
	0x04,
	0x03,
	0x02,
	0x03,
	0x02,
	0x02,
	0x03,
	0x03,
	0x05,
	0x05,
	0x04,
	0x04,
	0x03,
	0x03,
	0x03,
	0x05,
	0x04,
	0x03,
	0x04,
	0x04,
	0x03,
	0x03,
	0x04,
];
// GENERATOR-END: BcstTo

lazy_static! {
	pub(super) static ref MEM_SIZE_TBL: Box<[&'static FormatterString; IcedConstants::MEMORY_SIZE_ENUM_COUNT]> = {
		let mut v = Vec::with_capacity(IcedConstants::MEMORY_SIZE_ENUM_COUNT);
		let c = &*FORMATTER_CONSTANTS;
		for _ in 0..IcedConstants::FIRST_BROADCAST_MEMORY_SIZE as usize {
			v.push(&c.empty);
		}
		for &d in BCST_TO_DATA.iter() {
			let bcst_to = match d {
				// GENERATOR-BEGIN: BroadcastToKindMatch
				// ⚠️This was generated by GENERATOR!🦹‍♂️
				0x00 => &c.empty,
				0x01 => &c.b1to2,
				0x02 => &c.b1to4,
				0x03 => &c.b1to8,
				0x04 => &c.b1to16,
				0x05 => &c.b1to32,
				// GENERATOR-END: BroadcastToKindMatch
				_ => unreachable!(),
			};
			v.push(bcst_to);
		}
		#[allow(clippy::unwrap_used)]
		v.into_boxed_slice().try_into().ok().unwrap()
	};
}
