/*
 * File: bytes.rs
 * Project: knowitall
 * Created Date: Monday, December 7th 2020, 8:19:54 pm
 * Author: aspen
 * -----
 * Copyright (c) 2020 aspen
 *
 * This software is provided 'as-is', without any express or implied warranty. In
 * no event will the authors be held liable for any damages arising from the use of
 * this software.
 *
 * Permission is granted to anyone to use this software for any purpose, including
 * commercial applications, and to alter it and redistribute it freely, subject to
 * the following restrictions:
 *
 * 1.  The origin of this software must not be misrepresented; you must not claim
 *     that you wrote the original software. If you use this software in a product,
 *     an acknowledgment in the product documentation would be appreciated but is
 *     not required.
 *
 * 2.  Altered source versions must be plainly marked as such, and must not be
 *     misrepresented as being the original software.
 *
 * 3.  This notice may not be removed or altered from any source distribution.
 */

use crate::{
	patterns::BYTE_SIZE,
	provider::{InfoSegment, Provider, Tooltip},
};

pub struct ByteProvider;

impl Provider for ByteProvider {
	fn name(&self) -> &'static str {
		"Bytes"
	}

	fn parse_message(&self, src: &str) -> Vec<InfoSegment> {
		BYTE_SIZE
			.captures_iter(src)
			.filter_map(|capture| -> Option<InfoSegment> {
				let segment = capture.get(0)?;
				let value =
					lexical::parse(capture.name("size_value")?.as_str().replace(',', "")).ok()?;
				let prefix = match capture.name("size_prefix") {
					Some(x) => x.as_str().to_lowercase(),
					None => "".into(),
				};
				let is_si = capture.name("is_normal").is_none();
				let unit = capture.name("size_unit")?.as_str().to_lowercase();

				let mut value = match prefix.as_str() {
					"k" | "kilo" => {
						if is_si {
							bytesize::kb(value)
						} else {
							bytesize::kib(value)
						}
					}
					"m" | "mega" => {
						if is_si {
							bytesize::mb(value)
						} else {
							bytesize::mib(value)
						}
					}
					"g" | "giga" => {
						if is_si {
							bytesize::gb(value)
						} else {
							bytesize::gib(value)
						}
					}
					"t" | "tera" => {
						if is_si {
							bytesize::tb(value)
						} else {
							bytesize::tib(value)
						}
					}
					"p" | "peta" => {
						if is_si {
							bytesize::pb(value)
						} else {
							bytesize::pib(value)
						}
					}
					_ => value,
				};

				let bits = if unit.as_str() == "bit" {
					value /= 8;
					true
				} else {
					false
				};

				InfoSegment {
					start: segment.start(),
					end: segment.end(),
					info: Tooltip::Bytes {
						bytes: value,
						bits,
						si: is_si,
					},
				}
				.into()
			})
			.collect()
	}
}
