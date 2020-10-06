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

// ⚠️This file was generated by GENERATOR!🦹‍♂️

#nullable enable

#if DECODER
using System;

namespace Iced.Intel {
	/// <summary>Decoder options</summary>
	[Flags]
	public enum DecoderOptions : uint {
		/// <summary>No option is enabled</summary>
		None = 0x00000000,
		/// <summary>Disable some checks for invalid encodings of instructions, eg. most instructions can&apos;t use a <c>LOCK</c> prefix so if one is found, they&apos;re decoded as <see cref="Code.INVALID"/> unless this option is enabled.</summary>
		NoInvalidCheck = 0x00000001,
		/// <summary>AMD decoder: allow 16-bit branch/ret instructions in 64-bit mode, no <c>o64 CALL/JMP FAR [mem], o64 LSS/LFS/LGS</c>, <c>UD0</c> has no modr/m byte. The AMD decoder can still decode Intel instructions.</summary>
		AMD = 0x00000002,
		/// <summary>AMD decoder: allow 16-bit branch/ret instructions in 64-bit mode, no <c>o64 CALL/JMP FAR [mem], o64 LSS/LFS/LGS</c>, <c>UD0</c> has no modr/m byte. The AMD decoder can still decode Intel instructions.</summary>
		[System.Obsolete("Use " + nameof(AMD) + " instead", true)]
		[System.ComponentModel.EditorBrowsable(System.ComponentModel.EditorBrowsableState.Never)]
		AmdBranches = 0x00000002,
		/// <summary>Decode opcodes <c>0F0D</c> and <c>0F18-0F1F</c> as reserved-nop instructions (eg. <see cref="Code.Reservednop_rm32_r32_0F1D"/>)</summary>
		ForceReservednop = 0x00000004,
		/// <summary>Decode <c>UMOV</c> instructions</summary>
		Umov = 0x00000008,
		/// <summary>Decode <c>XBTS</c>/<c>IBTS</c></summary>
		Xbts = 0x00000010,
		/// <summary>Decode <c>0FA6</c>/<c>0FA7</c> as <c>CMPXCHG</c></summary>
		Cmpxchg486A = 0x00000020,
		/// <summary>Decode some old removed FPU instructions (eg. <c>FRSTPM</c>)</summary>
		OldFpu = 0x00000040,
		/// <summary>Decode <c>PCOMMIT</c></summary>
		Pcommit = 0x00000080,
		/// <summary>Decode 286 <c>LOADALL</c> (<c>0F04</c> and <c>0F05</c>)</summary>
		Loadall286 = 0x00000100,
		/// <summary>Decode <c>LOADALL386</c></summary>
		Loadall386 = 0x00000200,
		/// <summary>Decode <c>CL1INVMB</c></summary>
		Cl1invmb = 0x00000400,
		/// <summary>Decode <c>MOV r32,tr</c> and <c>MOV tr,r32</c></summary>
		MovTr = 0x00000800,
		/// <summary>Decode <c>JMPE</c> instructions</summary>
		Jmpe = 0x00001000,
		/// <summary>Don&apos;t decode <c>PAUSE</c>, decode <c>NOP</c> instead</summary>
		NoPause = 0x00002000,
		/// <summary>Don&apos;t decode <c>WBNOINVD</c>, decode <c>WBINVD</c> instead</summary>
		NoWbnoinvd = 0x00004000,
		/// <summary>Don&apos;t decode <c>LOCK MOV CR0</c> as <c>MOV CR8</c> (AMD)</summary>
		NoLockMovCR = 0x00008000,
		/// <summary>Don&apos;t decode <c>LOCK MOV CR0</c> as <c>MOV CR8</c> (AMD)</summary>
		[System.Obsolete("Use " + nameof(NoLockMovCR) + " instead", true)]
		[System.ComponentModel.EditorBrowsable(System.ComponentModel.EditorBrowsableState.Never)]
		NoLockMovCR0 = 0x00008000,
		/// <summary>Don&apos;t decode <c>TZCNT</c>, decode <c>BSF</c> instead</summary>
		NoMPFX_0FBC = 0x00010000,
		/// <summary>Don&apos;t decode <c>LZCNT</c>, decode <c>BSR</c> instead</summary>
		NoMPFX_0FBD = 0x00020000,
		/// <summary>Don&apos;t decode <c>LAHF</c> and <c>SAHF</c> in 64-bit mode</summary>
		NoLahfSahf64 = 0x00040000,
		/// <summary>Decode <c>MPX</c> instructions</summary>
		MPX = 0x00080000,
		/// <summary>Decode most Cyrix instructions: <c>FPU</c>, <c>EMMI</c>, <c>SMM</c>, <c>DDI</c></summary>
		Cyrix = 0x00100000,
		/// <summary>Decode Cyrix <c>SMINT 0F7E</c> (Cyrix 6x86 or earlier)</summary>
		Cyrix_SMINT_0F7E = 0x00200000,
		/// <summary>Decode Cyrix <c>DMI</c> instructions (AMD Geode GX/LX)</summary>
		Cyrix_DMI = 0x00400000,
		/// <summary>Decode Centaur <c>ALTINST</c></summary>
		ALTINST = 0x00800000,
	}
}
#endif
