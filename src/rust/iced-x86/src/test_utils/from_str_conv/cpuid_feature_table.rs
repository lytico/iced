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

#![allow(unused_results)]

use super::super::super::CpuidFeature;
#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap;

lazy_static! {
	pub(super) static ref TO_CPUID_FEATURE_HASH: HashMap<&'static str, CpuidFeature> = {
		// GENERATOR-BEGIN: CpuidFeatureHash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(152);
		h.insert("INTEL8086", CpuidFeature::INTEL8086);
		h.insert("INTEL8086_ONLY", CpuidFeature::INTEL8086_ONLY);
		h.insert("INTEL186", CpuidFeature::INTEL186);
		h.insert("INTEL286", CpuidFeature::INTEL286);
		h.insert("INTEL286_ONLY", CpuidFeature::INTEL286_ONLY);
		h.insert("INTEL386", CpuidFeature::INTEL386);
		h.insert("INTEL386_ONLY", CpuidFeature::INTEL386_ONLY);
		h.insert("INTEL386_A0_ONLY", CpuidFeature::INTEL386_A0_ONLY);
		h.insert("INTEL486", CpuidFeature::INTEL486);
		h.insert("INTEL486_A_ONLY", CpuidFeature::INTEL486_A_ONLY);
		h.insert("UMOV", CpuidFeature::UMOV);
		h.insert("IA64", CpuidFeature::IA64);
		h.insert("X64", CpuidFeature::X64);
		h.insert("ADX", CpuidFeature::ADX);
		h.insert("AES", CpuidFeature::AES);
		h.insert("AVX", CpuidFeature::AVX);
		h.insert("AVX2", CpuidFeature::AVX2);
		h.insert("AVX512_4FMAPS", CpuidFeature::AVX512_4FMAPS);
		h.insert("AVX512_4VNNIW", CpuidFeature::AVX512_4VNNIW);
		h.insert("AVX512_BF16", CpuidFeature::AVX512_BF16);
		h.insert("AVX512_BITALG", CpuidFeature::AVX512_BITALG);
		h.insert("AVX512_IFMA", CpuidFeature::AVX512_IFMA);
		h.insert("AVX512_VBMI", CpuidFeature::AVX512_VBMI);
		h.insert("AVX512_VBMI2", CpuidFeature::AVX512_VBMI2);
		h.insert("AVX512_VNNI", CpuidFeature::AVX512_VNNI);
		h.insert("AVX512_VP2INTERSECT", CpuidFeature::AVX512_VP2INTERSECT);
		h.insert("AVX512_VPOPCNTDQ", CpuidFeature::AVX512_VPOPCNTDQ);
		h.insert("AVX512BW", CpuidFeature::AVX512BW);
		h.insert("AVX512CD", CpuidFeature::AVX512CD);
		h.insert("AVX512DQ", CpuidFeature::AVX512DQ);
		h.insert("AVX512ER", CpuidFeature::AVX512ER);
		h.insert("AVX512F", CpuidFeature::AVX512F);
		h.insert("AVX512PF", CpuidFeature::AVX512PF);
		h.insert("AVX512VL", CpuidFeature::AVX512VL);
		h.insert("BMI1", CpuidFeature::BMI1);
		h.insert("BMI2", CpuidFeature::BMI2);
		h.insert("CET_IBT", CpuidFeature::CET_IBT);
		h.insert("CET_SS", CpuidFeature::CET_SS);
		h.insert("CL1INVMB", CpuidFeature::CL1INVMB);
		h.insert("CLDEMOTE", CpuidFeature::CLDEMOTE);
		h.insert("CLFLUSHOPT", CpuidFeature::CLFLUSHOPT);
		h.insert("CLFSH", CpuidFeature::CLFSH);
		h.insert("CLWB", CpuidFeature::CLWB);
		h.insert("CLZERO", CpuidFeature::CLZERO);
		h.insert("CMOV", CpuidFeature::CMOV);
		h.insert("CMPXCHG16B", CpuidFeature::CMPXCHG16B);
		h.insert("CPUID", CpuidFeature::CPUID);
		h.insert("CX8", CpuidFeature::CX8);
		h.insert("D3NOW", CpuidFeature::D3NOW);
		h.insert("D3NOWEXT", CpuidFeature::D3NOWEXT);
		h.insert("OSS", CpuidFeature::OSS);
		h.insert("ENQCMD", CpuidFeature::ENQCMD);
		h.insert("F16C", CpuidFeature::F16C);
		h.insert("FMA", CpuidFeature::FMA);
		h.insert("FMA4", CpuidFeature::FMA4);
		h.insert("FPU", CpuidFeature::FPU);
		h.insert("FPU287", CpuidFeature::FPU287);
		h.insert("FPU287XL_ONLY", CpuidFeature::FPU287XL_ONLY);
		h.insert("FPU387", CpuidFeature::FPU387);
		h.insert("FPU387SL_ONLY", CpuidFeature::FPU387SL_ONLY);
		h.insert("FSGSBASE", CpuidFeature::FSGSBASE);
		h.insert("FXSR", CpuidFeature::FXSR);
		h.insert("CYRIX_D3NOW", CpuidFeature::CYRIX_D3NOW);
		h.insert("GFNI", CpuidFeature::GFNI);
		h.insert("HLE", CpuidFeature::HLE);
		h.insert("HLE_or_RTM", CpuidFeature::HLE_or_RTM);
		h.insert("INVEPT", CpuidFeature::INVEPT);
		h.insert("INVPCID", CpuidFeature::INVPCID);
		h.insert("INVVPID", CpuidFeature::INVVPID);
		h.insert("LWP", CpuidFeature::LWP);
		h.insert("LZCNT", CpuidFeature::LZCNT);
		h.insert("MCOMMIT", CpuidFeature::MCOMMIT);
		h.insert("MMX", CpuidFeature::MMX);
		h.insert("MONITOR", CpuidFeature::MONITOR);
		h.insert("MONITORX", CpuidFeature::MONITORX);
		h.insert("MOVBE", CpuidFeature::MOVBE);
		h.insert("MOVDIR64B", CpuidFeature::MOVDIR64B);
		h.insert("MOVDIRI", CpuidFeature::MOVDIRI);
		h.insert("MPX", CpuidFeature::MPX);
		h.insert("MSR", CpuidFeature::MSR);
		h.insert("MULTIBYTENOP", CpuidFeature::MULTIBYTENOP);
		h.insert("PADLOCK_ACE", CpuidFeature::PADLOCK_ACE);
		h.insert("PADLOCK_PHE", CpuidFeature::PADLOCK_PHE);
		h.insert("PADLOCK_PMM", CpuidFeature::PADLOCK_PMM);
		h.insert("PADLOCK_RNG", CpuidFeature::PADLOCK_RNG);
		h.insert("PAUSE", CpuidFeature::PAUSE);
		h.insert("PCLMULQDQ", CpuidFeature::PCLMULQDQ);
		h.insert("PCOMMIT", CpuidFeature::PCOMMIT);
		h.insert("PCONFIG", CpuidFeature::PCONFIG);
		h.insert("PKU", CpuidFeature::PKU);
		h.insert("POPCNT", CpuidFeature::POPCNT);
		h.insert("PREFETCHW", CpuidFeature::PREFETCHW);
		h.insert("PREFETCHWT1", CpuidFeature::PREFETCHWT1);
		h.insert("PTWRITE", CpuidFeature::PTWRITE);
		h.insert("RDPID", CpuidFeature::RDPID);
		h.insert("RDPMC", CpuidFeature::RDPMC);
		h.insert("RDPRU", CpuidFeature::RDPRU);
		h.insert("RDRAND", CpuidFeature::RDRAND);
		h.insert("RDSEED", CpuidFeature::RDSEED);
		h.insert("RDTSCP", CpuidFeature::RDTSCP);
		h.insert("RTM", CpuidFeature::RTM);
		h.insert("SEP", CpuidFeature::SEP);
		h.insert("SGX1", CpuidFeature::SGX1);
		h.insert("SHA", CpuidFeature::SHA);
		h.insert("SKINIT", CpuidFeature::SKINIT);
		h.insert("SKINIT_or_SVM", CpuidFeature::SKINIT_or_SVM);
		h.insert("SMAP", CpuidFeature::SMAP);
		h.insert("SMX", CpuidFeature::SMX);
		h.insert("SSE", CpuidFeature::SSE);
		h.insert("SSE2", CpuidFeature::SSE2);
		h.insert("SSE3", CpuidFeature::SSE3);
		h.insert("SSE4_1", CpuidFeature::SSE4_1);
		h.insert("SSE4_2", CpuidFeature::SSE4_2);
		h.insert("SSE4A", CpuidFeature::SSE4A);
		h.insert("SSSE3", CpuidFeature::SSSE3);
		h.insert("SVM", CpuidFeature::SVM);
		h.insert("SEV_ES", CpuidFeature::SEV_ES);
		h.insert("SYSCALL", CpuidFeature::SYSCALL);
		h.insert("TBM", CpuidFeature::TBM);
		h.insert("TSC", CpuidFeature::TSC);
		h.insert("VAES", CpuidFeature::VAES);
		h.insert("VMX", CpuidFeature::VMX);
		h.insert("VPCLMULQDQ", CpuidFeature::VPCLMULQDQ);
		h.insert("WAITPKG", CpuidFeature::WAITPKG);
		h.insert("WBNOINVD", CpuidFeature::WBNOINVD);
		h.insert("XOP", CpuidFeature::XOP);
		h.insert("XSAVE", CpuidFeature::XSAVE);
		h.insert("XSAVEC", CpuidFeature::XSAVEC);
		h.insert("XSAVEOPT", CpuidFeature::XSAVEOPT);
		h.insert("XSAVES", CpuidFeature::XSAVES);
		h.insert("SEV_SNP", CpuidFeature::SEV_SNP);
		h.insert("SERIALIZE", CpuidFeature::SERIALIZE);
		h.insert("TSXLDTRK", CpuidFeature::TSXLDTRK);
		h.insert("INVLPGB", CpuidFeature::INVLPGB);
		h.insert("AMX_BF16", CpuidFeature::AMX_BF16);
		h.insert("AMX_TILE", CpuidFeature::AMX_TILE);
		h.insert("AMX_INT8", CpuidFeature::AMX_INT8);
		h.insert("CYRIX_FPU", CpuidFeature::CYRIX_FPU);
		h.insert("CYRIX_SMM", CpuidFeature::CYRIX_SMM);
		h.insert("CYRIX_SMINT", CpuidFeature::CYRIX_SMINT);
		h.insert("CYRIX_SMINT_0F7E", CpuidFeature::CYRIX_SMINT_0F7E);
		h.insert("CYRIX_SHR", CpuidFeature::CYRIX_SHR);
		h.insert("CYRIX_DDI", CpuidFeature::CYRIX_DDI);
		h.insert("CYRIX_EMMI", CpuidFeature::CYRIX_EMMI);
		h.insert("CYRIX_DMI", CpuidFeature::CYRIX_DMI);
		h.insert("CENTAUR_AIS", CpuidFeature::CENTAUR_AIS);
		h.insert("MOV_TR", CpuidFeature::MOV_TR);
		h.insert("SMM", CpuidFeature::SMM);
		h.insert("TDX", CpuidFeature::TDX);
		h.insert("KL", CpuidFeature::KL);
		h.insert("AESKLE", CpuidFeature::AESKLE);
		h.insert("WIDE_KL", CpuidFeature::WIDE_KL);
		// GENERATOR-END: CpuidFeatureHash
		h
	};
}
