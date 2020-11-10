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

use super::super::CpuidFeature;

#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
pub(crate) static CPUID: [&[CpuidFeature]; 176] = [
	&[CpuidFeature::INTEL8086],// INTEL8086
	&[CpuidFeature::INTEL8086_ONLY],// INTEL8086_ONLY
	&[CpuidFeature::INTEL186],// INTEL186
	&[CpuidFeature::INTEL286],// INTEL286
	&[CpuidFeature::INTEL286_ONLY],// INTEL286_ONLY
	&[CpuidFeature::INTEL386],// INTEL386
	&[CpuidFeature::INTEL386_ONLY],// INTEL386_ONLY
	&[CpuidFeature::INTEL386_A0_ONLY],// INTEL386_A0_ONLY
	&[CpuidFeature::INTEL486],// INTEL486
	&[CpuidFeature::INTEL486_A_ONLY],// INTEL486_A_ONLY
	&[CpuidFeature::UMOV],// UMOV
	&[CpuidFeature::IA64],// IA64
	&[CpuidFeature::X64],// X64
	&[CpuidFeature::ADX],// ADX
	&[CpuidFeature::AES],// AES
	&[CpuidFeature::AVX],// AVX
	&[CpuidFeature::AES, CpuidFeature::AVX],// AES_and_AVX
	&[CpuidFeature::AVX2],// AVX2
	&[CpuidFeature::AVX512_4FMAPS],// AVX512_4FMAPS
	&[CpuidFeature::AVX512_4VNNIW],// AVX512_4VNNIW
	&[CpuidFeature::AVX512_BITALG],// AVX512_BITALG
	&[CpuidFeature::AVX512_IFMA],// AVX512_IFMA
	&[CpuidFeature::AVX512_VBMI],// AVX512_VBMI
	&[CpuidFeature::AVX512_VBMI2],// AVX512_VBMI2
	&[CpuidFeature::AVX512_VNNI],// AVX512_VNNI
	&[CpuidFeature::AVX512_VPOPCNTDQ],// AVX512_VPOPCNTDQ
	&[CpuidFeature::AVX512BW],// AVX512BW
	&[CpuidFeature::AVX512CD],// AVX512CD
	&[CpuidFeature::AVX512DQ],// AVX512DQ
	&[CpuidFeature::AVX512ER],// AVX512ER
	&[CpuidFeature::AVX512F],// AVX512F
	&[CpuidFeature::AVX512F, CpuidFeature::AVX512_BF16],// AVX512F_and_AVX512_BF16
	&[CpuidFeature::AVX512F, CpuidFeature::AVX512_VP2INTERSECT],// AVX512F_and_AVX512_VP2INTERSECT
	&[CpuidFeature::AVX512PF],// AVX512PF
	&[CpuidFeature::AVX512VL, CpuidFeature::AVX512_BF16],// AVX512VL_and_AVX512_BF16
	&[CpuidFeature::AVX512VL, CpuidFeature::AVX512_BITALG],// AVX512VL_and_AVX512_BITALG
	&[CpuidFeature::AVX512VL, CpuidFeature::AVX512_IFMA],// AVX512VL_and_AVX512_IFMA
	&[CpuidFeature::AVX512VL, CpuidFeature::AVX512_VBMI],// AVX512VL_and_AVX512_VBMI
	&[CpuidFeature::AVX512VL, CpuidFeature::AVX512_VBMI2],// AVX512VL_and_AVX512_VBMI2
	&[CpuidFeature::AVX512VL, CpuidFeature::AVX512_VNNI],// AVX512VL_and_AVX512_VNNI
	&[CpuidFeature::AVX512VL, CpuidFeature::AVX512_VP2INTERSECT],// AVX512VL_and_AVX512_VP2INTERSECT
	&[CpuidFeature::AVX512VL, CpuidFeature::AVX512_VPOPCNTDQ],// AVX512VL_and_AVX512_VPOPCNTDQ
	&[CpuidFeature::AVX512VL, CpuidFeature::AVX512BW],// AVX512VL_and_AVX512BW
	&[CpuidFeature::AVX512VL, CpuidFeature::AVX512CD],// AVX512VL_and_AVX512CD
	&[CpuidFeature::AVX512VL, CpuidFeature::AVX512DQ],// AVX512VL_and_AVX512DQ
	&[CpuidFeature::AVX512VL, CpuidFeature::AVX512F],// AVX512VL_and_AVX512F
	&[CpuidFeature::BMI1],// BMI1
	&[CpuidFeature::BMI2],// BMI2
	&[CpuidFeature::CET_IBT],// CET_IBT
	&[CpuidFeature::CET_SS],// CET_SS
	&[CpuidFeature::CL1INVMB],// CL1INVMB
	&[CpuidFeature::CLDEMOTE],// CLDEMOTE
	&[CpuidFeature::CLFLUSHOPT],// CLFLUSHOPT
	&[CpuidFeature::CLFSH],// CLFSH
	&[CpuidFeature::CLWB],// CLWB
	&[CpuidFeature::CLZERO],// CLZERO
	&[CpuidFeature::CMOV],// CMOV
	&[CpuidFeature::CMPXCHG16B],// CMPXCHG16B
	&[CpuidFeature::CPUID],// CPUID
	&[CpuidFeature::CX8],// CX8
	&[CpuidFeature::D3NOW],// D3NOW
	&[CpuidFeature::D3NOWEXT],// D3NOWEXT
	&[CpuidFeature::OSS],// OSS
	&[CpuidFeature::ENQCMD],// ENQCMD
	&[CpuidFeature::F16C],// F16C
	&[CpuidFeature::FMA],// FMA
	&[CpuidFeature::FMA4],// FMA4
	&[CpuidFeature::FPU],// FPU
	&[CpuidFeature::FPU, CpuidFeature::CMOV],// FPU_and_CMOV
	&[CpuidFeature::FPU287],// FPU287
	&[CpuidFeature::FPU287XL_ONLY],// FPU287XL_ONLY
	&[CpuidFeature::FPU387],// FPU387
	&[CpuidFeature::FPU387SL_ONLY],// FPU387SL_ONLY
	&[CpuidFeature::FSGSBASE],// FSGSBASE
	&[CpuidFeature::FXSR],// FXSR
	&[CpuidFeature::CYRIX_D3NOW],// CYRIX_D3NOW
	&[CpuidFeature::GFNI],// GFNI
	&[CpuidFeature::AVX, CpuidFeature::GFNI],// AVX_and_GFNI
	&[CpuidFeature::AVX512F, CpuidFeature::GFNI],// AVX512F_and_GFNI
	&[CpuidFeature::AVX512VL, CpuidFeature::GFNI],// AVX512VL_and_GFNI
	&[CpuidFeature::HLE_or_RTM],// HLE_or_RTM
	&[CpuidFeature::INVPCID],// INVPCID
	&[CpuidFeature::LWP],// LWP
	&[CpuidFeature::LZCNT],// LZCNT
	&[CpuidFeature::MCOMMIT],// MCOMMIT
	&[CpuidFeature::MMX],// MMX
	&[CpuidFeature::MONITOR],// MONITOR
	&[CpuidFeature::MONITORX],// MONITORX
	&[CpuidFeature::MOVBE],// MOVBE
	&[CpuidFeature::MOVDIR64B],// MOVDIR64B
	&[CpuidFeature::MOVDIRI],// MOVDIRI
	&[CpuidFeature::MPX],// MPX
	&[CpuidFeature::MSR],// MSR
	&[CpuidFeature::MULTIBYTENOP],// MULTIBYTENOP
	&[CpuidFeature::PADLOCK_ACE],// PADLOCK_ACE
	&[CpuidFeature::PADLOCK_PHE],// PADLOCK_PHE
	&[CpuidFeature::PADLOCK_PMM],// PADLOCK_PMM
	&[CpuidFeature::PADLOCK_RNG],// PADLOCK_RNG
	&[CpuidFeature::PAUSE],// PAUSE
	&[CpuidFeature::PCLMULQDQ],// PCLMULQDQ
	&[CpuidFeature::PCLMULQDQ, CpuidFeature::AVX],// PCLMULQDQ_and_AVX
	&[CpuidFeature::PCOMMIT],// PCOMMIT
	&[CpuidFeature::PCONFIG],// PCONFIG
	&[CpuidFeature::PKU],// PKU
	&[CpuidFeature::POPCNT],// POPCNT
	&[CpuidFeature::PREFETCHW],// PREFETCHW
	&[CpuidFeature::PREFETCHWT1],// PREFETCHWT1
	&[CpuidFeature::PTWRITE],// PTWRITE
	&[CpuidFeature::RDPID],// RDPID
	&[CpuidFeature::RDPMC],// RDPMC
	&[CpuidFeature::RDPRU],// RDPRU
	&[CpuidFeature::RDRAND],// RDRAND
	&[CpuidFeature::RDSEED],// RDSEED
	&[CpuidFeature::RDTSCP],// RDTSCP
	&[CpuidFeature::RTM],// RTM
	&[CpuidFeature::SEP],// SEP
	&[CpuidFeature::SGX1],// SGX1
	&[CpuidFeature::SHA],// SHA
	&[CpuidFeature::SKINIT_or_SVM],// SKINIT_or_SVM
	&[CpuidFeature::SMAP],// SMAP
	&[CpuidFeature::SMX],// SMX
	&[CpuidFeature::SSE],// SSE
	&[CpuidFeature::SSE2],// SSE2
	&[CpuidFeature::SSE3],// SSE3
	&[CpuidFeature::FPU, CpuidFeature::SSE3],// FPU_and_SSE3
	&[CpuidFeature::SSE4_1],// SSE4_1
	&[CpuidFeature::SSE4_2],// SSE4_2
	&[CpuidFeature::SSE4A],// SSE4A
	&[CpuidFeature::SSSE3],// SSSE3
	&[CpuidFeature::SVM],// SVM
	&[CpuidFeature::SEV_ES],// SEV_ES
	&[CpuidFeature::SYSCALL],// SYSCALL
	&[CpuidFeature::TBM],// TBM
	&[CpuidFeature::TSC],// TSC
	&[CpuidFeature::VAES],// VAES
	&[CpuidFeature::AVX512F, CpuidFeature::VAES],// AVX512F_and_VAES
	&[CpuidFeature::AVX512VL, CpuidFeature::VAES],// AVX512VL_and_VAES
	&[CpuidFeature::VMX],// VMX
	&[CpuidFeature::VMX, CpuidFeature::INVEPT],// VMX_and_INVEPT
	&[CpuidFeature::VMX, CpuidFeature::INVVPID],// VMX_and_INVVPID
	&[CpuidFeature::VPCLMULQDQ],// VPCLMULQDQ
	&[CpuidFeature::AVX512F, CpuidFeature::VPCLMULQDQ],// AVX512F_and_VPCLMULQDQ
	&[CpuidFeature::AVX512VL, CpuidFeature::VPCLMULQDQ],// AVX512VL_and_VPCLMULQDQ
	&[CpuidFeature::WAITPKG],// WAITPKG
	&[CpuidFeature::WBNOINVD],// WBNOINVD
	&[CpuidFeature::XOP],// XOP
	&[CpuidFeature::XSAVE],// XSAVE
	&[CpuidFeature::XSAVEC],// XSAVEC
	&[CpuidFeature::XSAVEOPT],// XSAVEOPT
	&[CpuidFeature::XSAVES],// XSAVES
	&[CpuidFeature::SEV_SNP],// SEV_SNP
	&[CpuidFeature::SERIALIZE],// SERIALIZE
	&[CpuidFeature::TSXLDTRK],// TSXLDTRK
	&[CpuidFeature::INVLPGB],// INVLPGB
	&[CpuidFeature::AMX_BF16],// AMX_BF16
	&[CpuidFeature::AMX_TILE],// AMX_TILE
	&[CpuidFeature::AMX_INT8],// AMX_INT8
	&[CpuidFeature::CYRIX_FPU],// CYRIX_FPU
	&[CpuidFeature::CYRIX_SMM],// CYRIX_SMM
	&[CpuidFeature::CYRIX_SMINT],// CYRIX_SMINT
	&[CpuidFeature::CYRIX_SMINT_0F7E],// CYRIX_SMINT_0F7E
	&[CpuidFeature::CYRIX_SHR],// CYRIX_SHR
	&[CpuidFeature::CYRIX_DDI],// CYRIX_DDI
	&[CpuidFeature::CYRIX_EMMI],// CYRIX_EMMI
	&[CpuidFeature::CYRIX_DMI],// CYRIX_DMI
	&[CpuidFeature::CENTAUR_AIS],// CENTAUR_AIS
	&[CpuidFeature::MOV_TR],// MOV_TR
	&[CpuidFeature::SMM],// SMM
	&[CpuidFeature::TDX],// TDX
	&[CpuidFeature::KL],// KL
	&[CpuidFeature::AESKLE],// AESKLE
	&[CpuidFeature::AESKLE, CpuidFeature::WIDE_KL],// AESKLE_and_WIDE_KL
	&[CpuidFeature::UINTR],// UINTR
	&[CpuidFeature::HRESET],// HRESET
	&[CpuidFeature::AVX_VNNI],// AVX_VNNI
	&[CpuidFeature::PADLOCK_GMI],// PADLOCK_GMI
];
