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

// This file was generated by the Generator project
#nullable enable

#if !NO_DECODER
using System;

namespace Iced.Intel {
	/// <summary>Decoder options</summary>
	[Flags]
	public enum DecoderOptions : uint {
		/// <summary>No option is enabled</summary>
		None = 0x00000000,
		/// <summary>Disable some checks for invalid encodings of instructions, eg. most instructions can&apos;t use a <c>LOCK</c> prefix so if one is found, they&apos;re decoded as <see cref="Code.INVALID"/> unless this option is enabled.</summary>
		NoInvalidCheck = 0x00000001,
		/// <summary>AMD branch decoder: allow 16-bit branch/ret instructions in 64-bit mode</summary>
		AmdBranches = 0x00000002,
		/// <summary>Decode opcodes <c>0F0D</c> and <c>0F18-0F1F</c> as reserved-nop instructions (eg. <see cref="Code.ReservedNop_rm32_r32_0F1D"/>)</summary>
		ForceReservedNop = 0x00000004,
		/// <summary>Decode <c>umov</c> instructions (eg. <see cref="Code.Umov_r32_rm32"/>)</summary>
		Umov = 0x00000008,
		/// <summary>Decode <c>xbts</c>/<c>ibts</c></summary>
		Xbts = 0x00000010,
		/// <summary>Decode <c>0FA6</c>/<c>0FA7</c> as <c>cmpxchg</c></summary>
		Cmpxchg486A = 0x00000020,
		/// <summary>Decode some old removed FPU instructions (eg. <c>frstpm</c>)</summary>
		OldFpu = 0x00000040,
		/// <summary>Decode <see cref="Code.Pcommit"/></summary>
		Pcommit = 0x00000080,
		/// <summary>Decode 286 <c>loadall</c> ((#c:0F04)# and <c>0F05</c>)</summary>
		Loadall286 = 0x00000100,
		/// <summary>Decode <see cref="Code.Loadall386"/></summary>
		Loadall386 = 0x00000200,
		/// <summary>Decode <see cref="Code.Cl1invmb"/></summary>
		Cl1invmb = 0x00000400,
		/// <summary>Decode <see cref="Code.Mov_r32_tr"/> and <see cref="Code.Mov_tr_r32"/></summary>
		MovTr = 0x00000800,
		/// <summary>Decode <c>jmpe</c> instructions</summary>
		Jmpe = 0x00001000,
		/// <summary>Don&apos;t decode <see cref="Code.Pause"/>, decode <see cref="Code.Nopd"/>/etc instead</summary>
		NoPause = 0x00002000,
		/// <summary>Don&apos;t decode <see cref="Code.Wbnoinvd"/>, decode <see cref="Code.Wbinvd"/> instead</summary>
		NoWbnoinvd = 0x00004000,
		/// <summary>Don&apos;t decode <c>LOCK MOV CR0</c> as <c>MOV CR8</c> (AMD)</summary>
		NoLockMovCR0 = 0x00008000,
		/// <summary>Don&apos;t decode <see cref="Code.Tzcnt_r32_rm32"/>/etc, decode <see cref="Code.Bsf_r32_rm32"/>/etc instead</summary>
		NoMPFX_0FBC = 0x00010000,
		/// <summary>Don&apos;t decode <see cref="Code.Lzcnt_r32_rm32"/>/etc, decode <see cref="Code.Bsr_r32_rm32"/>/etc instead</summary>
		NoMPFX_0FBD = 0x00020000,
		/// <summary>Don&apos;t decode <see cref="Code.Lahf"/> and <see cref="Code.Sahf"/> in 64-bit mode</summary>
		NoLahfSahf64 = 0x00040000,
	}
}
#endif
