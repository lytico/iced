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

namespace Generator.Enums {
	static class RegisterEnum {
		const string documentation = "A register";

		static EnumValue[] GetValues() =>
			new EnumValue[] {
				new EnumValue("None"),
				new EnumValue("AL"),
				new EnumValue("CL"),
				new EnumValue("DL"),
				new EnumValue("BL"),
				new EnumValue("AH"),
				new EnumValue("CH"),
				new EnumValue("DH"),
				new EnumValue("BH"),
				new EnumValue("SPL"),
				new EnumValue("BPL"),
				new EnumValue("SIL"),
				new EnumValue("DIL"),
				new EnumValue("R8L"),
				new EnumValue("R9L"),
				new EnumValue("R10L"),
				new EnumValue("R11L"),
				new EnumValue("R12L"),
				new EnumValue("R13L"),
				new EnumValue("R14L"),
				new EnumValue("R15L"),
				new EnumValue("AX"),
				new EnumValue("CX"),
				new EnumValue("DX"),
				new EnumValue("BX"),
				new EnumValue("SP"),
				new EnumValue("BP"),
				new EnumValue("SI"),
				new EnumValue("DI"),
				new EnumValue("R8W"),
				new EnumValue("R9W"),
				new EnumValue("R10W"),
				new EnumValue("R11W"),
				new EnumValue("R12W"),
				new EnumValue("R13W"),
				new EnumValue("R14W"),
				new EnumValue("R15W"),
				new EnumValue("EAX"),
				new EnumValue("ECX"),
				new EnumValue("EDX"),
				new EnumValue("EBX"),
				new EnumValue("ESP"),
				new EnumValue("EBP"),
				new EnumValue("ESI"),
				new EnumValue("EDI"),
				new EnumValue("R8D"),
				new EnumValue("R9D"),
				new EnumValue("R10D"),
				new EnumValue("R11D"),
				new EnumValue("R12D"),
				new EnumValue("R13D"),
				new EnumValue("R14D"),
				new EnumValue("R15D"),
				new EnumValue("RAX"),
				new EnumValue("RCX"),
				new EnumValue("RDX"),
				new EnumValue("RBX"),
				new EnumValue("RSP"),
				new EnumValue("RBP"),
				new EnumValue("RSI"),
				new EnumValue("RDI"),
				new EnumValue("R8"),
				new EnumValue("R9"),
				new EnumValue("R10"),
				new EnumValue("R11"),
				new EnumValue("R12"),
				new EnumValue("R13"),
				new EnumValue("R14"),
				new EnumValue("R15"),
				new EnumValue("EIP"),
				new EnumValue("RIP"),
				new EnumValue("ES"),
				new EnumValue("CS"),
				new EnumValue("SS"),
				new EnumValue("DS"),
				new EnumValue("FS"),
				new EnumValue("GS"),
				new EnumValue("XMM0"),
				new EnumValue("XMM1"),
				new EnumValue("XMM2"),
				new EnumValue("XMM3"),
				new EnumValue("XMM4"),
				new EnumValue("XMM5"),
				new EnumValue("XMM6"),
				new EnumValue("XMM7"),
				new EnumValue("XMM8"),
				new EnumValue("XMM9"),
				new EnumValue("XMM10"),
				new EnumValue("XMM11"),
				new EnumValue("XMM12"),
				new EnumValue("XMM13"),
				new EnumValue("XMM14"),
				new EnumValue("XMM15"),
				new EnumValue("XMM16"),
				new EnumValue("XMM17"),
				new EnumValue("XMM18"),
				new EnumValue("XMM19"),
				new EnumValue("XMM20"),
				new EnumValue("XMM21"),
				new EnumValue("XMM22"),
				new EnumValue("XMM23"),
				new EnumValue("XMM24"),
				new EnumValue("XMM25"),
				new EnumValue("XMM26"),
				new EnumValue("XMM27"),
				new EnumValue("XMM28"),
				new EnumValue("XMM29"),
				new EnumValue("XMM30"),
				new EnumValue("XMM31"),
				new EnumValue("YMM0"),
				new EnumValue("YMM1"),
				new EnumValue("YMM2"),
				new EnumValue("YMM3"),
				new EnumValue("YMM4"),
				new EnumValue("YMM5"),
				new EnumValue("YMM6"),
				new EnumValue("YMM7"),
				new EnumValue("YMM8"),
				new EnumValue("YMM9"),
				new EnumValue("YMM10"),
				new EnumValue("YMM11"),
				new EnumValue("YMM12"),
				new EnumValue("YMM13"),
				new EnumValue("YMM14"),
				new EnumValue("YMM15"),
				new EnumValue("YMM16"),
				new EnumValue("YMM17"),
				new EnumValue("YMM18"),
				new EnumValue("YMM19"),
				new EnumValue("YMM20"),
				new EnumValue("YMM21"),
				new EnumValue("YMM22"),
				new EnumValue("YMM23"),
				new EnumValue("YMM24"),
				new EnumValue("YMM25"),
				new EnumValue("YMM26"),
				new EnumValue("YMM27"),
				new EnumValue("YMM28"),
				new EnumValue("YMM29"),
				new EnumValue("YMM30"),
				new EnumValue("YMM31"),
				new EnumValue("ZMM0"),
				new EnumValue("ZMM1"),
				new EnumValue("ZMM2"),
				new EnumValue("ZMM3"),
				new EnumValue("ZMM4"),
				new EnumValue("ZMM5"),
				new EnumValue("ZMM6"),
				new EnumValue("ZMM7"),
				new EnumValue("ZMM8"),
				new EnumValue("ZMM9"),
				new EnumValue("ZMM10"),
				new EnumValue("ZMM11"),
				new EnumValue("ZMM12"),
				new EnumValue("ZMM13"),
				new EnumValue("ZMM14"),
				new EnumValue("ZMM15"),
				new EnumValue("ZMM16"),
				new EnumValue("ZMM17"),
				new EnumValue("ZMM18"),
				new EnumValue("ZMM19"),
				new EnumValue("ZMM20"),
				new EnumValue("ZMM21"),
				new EnumValue("ZMM22"),
				new EnumValue("ZMM23"),
				new EnumValue("ZMM24"),
				new EnumValue("ZMM25"),
				new EnumValue("ZMM26"),
				new EnumValue("ZMM27"),
				new EnumValue("ZMM28"),
				new EnumValue("ZMM29"),
				new EnumValue("ZMM30"),
				new EnumValue("ZMM31"),
				new EnumValue("K0"),
				new EnumValue("K1"),
				new EnumValue("K2"),
				new EnumValue("K3"),
				new EnumValue("K4"),
				new EnumValue("K5"),
				new EnumValue("K6"),
				new EnumValue("K7"),
				new EnumValue("BND0"),
				new EnumValue("BND1"),
				new EnumValue("BND2"),
				new EnumValue("BND3"),
				new EnumValue("CR0"),
				new EnumValue("CR1"),
				new EnumValue("CR2"),
				new EnumValue("CR3"),
				new EnumValue("CR4"),
				new EnumValue("CR5"),
				new EnumValue("CR6"),
				new EnumValue("CR7"),
				new EnumValue("CR8"),
				new EnumValue("CR9"),
				new EnumValue("CR10"),
				new EnumValue("CR11"),
				new EnumValue("CR12"),
				new EnumValue("CR13"),
				new EnumValue("CR14"),
				new EnumValue("CR15"),
				new EnumValue("DR0"),
				new EnumValue("DR1"),
				new EnumValue("DR2"),
				new EnumValue("DR3"),
				new EnumValue("DR4"),
				new EnumValue("DR5"),
				new EnumValue("DR6"),
				new EnumValue("DR7"),
				new EnumValue("DR8"),
				new EnumValue("DR9"),
				new EnumValue("DR10"),
				new EnumValue("DR11"),
				new EnumValue("DR12"),
				new EnumValue("DR13"),
				new EnumValue("DR14"),
				new EnumValue("DR15"),
				new EnumValue("ST0"),
				new EnumValue("ST1"),
				new EnumValue("ST2"),
				new EnumValue("ST3"),
				new EnumValue("ST4"),
				new EnumValue("ST5"),
				new EnumValue("ST6"),
				new EnumValue("ST7"),
				new EnumValue("MM0"),
				new EnumValue("MM1"),
				new EnumValue("MM2"),
				new EnumValue("MM3"),
				new EnumValue("MM4"),
				new EnumValue("MM5"),
				new EnumValue("MM6"),
				new EnumValue("MM7"),
				new EnumValue("TR0"),
				new EnumValue("TR1"),
				new EnumValue("TR2"),
				new EnumValue("TR3"),
				new EnumValue("TR4"),
				new EnumValue("TR5"),
				new EnumValue("TR6"),
				new EnumValue("TR7"),
			};

		public static readonly EnumType Instance = new EnumType(EnumKind.Register, documentation, GetValues(), EnumTypeFlags.Public);
	}
}
