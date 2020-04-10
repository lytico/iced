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

use wasm_bindgen::prelude::*;

// GENERATOR-BEGIN: Enum
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Memory size options used by the formatters
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum MemorySizeOptions {
	/// Show memory size if the assembler requires it, else don't show anything
	Default = 0,
	/// Always show the memory size, even if the assembler doesn't need it
	Always = 1,
	/// Show memory size if a human can't figure out the size of the operand
	Minimum = 2,
	/// Never show memory size
	Never = 3,
}
// GENERATOR-END: Enum

#[allow(dead_code)]
pub(crate) fn memory_size_options_to_iced(value: MemorySizeOptions) -> iced_x86::MemorySizeOptions {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn iced_to_memory_size_options(value: iced_x86::MemorySizeOptions) -> MemorySizeOptions {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}
