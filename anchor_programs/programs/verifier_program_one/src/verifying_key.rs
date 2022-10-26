use groth16_solana::groth16::Groth16Verifyingkey;

pub const VERIFYINGKEY: Groth16Verifyingkey =  Groth16Verifyingkey {
	nr_pubinputs: 18,

	vk_alpha_g1: [
		45,77,154,167,227,2,217,223,65,116,157,85,7,148,157,5,219,234,51,251,177,108,100,59,34,245,153,162,190,109,242,226,
		20,190,221,80,60,55,206,176,97,216,236,96,32,159,227,69,206,137,131,10,25,35,3,1,240,118,202,255,0,77,25,38,
	],

	vk_beta_g2: [
		9,103,3,47,203,247,118,209,175,201,133,248,136,119,241,130,211,132,128,166,83,242,222,202,169,121,76,188,59,243,6,12,
		14,24,120,71,173,76,121,131,116,208,214,115,43,245,1,132,125,214,139,192,224,113,36,30,2,19,188,127,193,61,183,171,
		48,76,251,209,224,138,112,74,153,245,232,71,217,63,140,60,170,253,222,196,107,122,13,55,157,166,154,77,17,35,70,167,
		23,57,193,177,164,87,168,199,49,49,35,210,77,47,145,146,248,150,183,198,62,234,5,169,213,127,6,84,122,208,206,200,
	],

	vk_gamme_g2: [
		25,142,147,147,146,13,72,58,114,96,191,183,49,251,93,37,241,170,73,51,53,169,231,18,151,228,133,183,174,243,18,194,
		24,0,222,239,18,31,30,118,66,106,0,102,94,92,68,121,103,67,34,212,247,94,218,221,70,222,189,92,217,146,246,237,
		9,6,137,208,88,95,240,117,236,158,153,173,105,12,51,149,188,75,49,51,112,179,142,243,85,172,218,220,209,34,151,91,
		18,200,94,165,219,140,109,235,74,171,113,128,141,203,64,143,227,209,231,105,12,67,211,123,76,230,204,1,102,250,125,170,
	],

	vk_delta_g2: [
		25,142,147,147,146,13,72,58,114,96,191,183,49,251,93,37,241,170,73,51,53,169,231,18,151,228,133,183,174,243,18,194,
		24,0,222,239,18,31,30,118,66,106,0,102,94,92,68,121,103,67,34,212,247,94,218,221,70,222,189,92,217,146,246,237,
		9,6,137,208,88,95,240,117,236,158,153,173,105,12,51,149,188,75,49,51,112,179,142,243,85,172,218,220,209,34,151,91,
		18,200,94,165,219,140,109,235,74,171,113,128,141,203,64,143,227,209,231,105,12,67,211,123,76,230,204,1,102,250,125,170,
	],

	vk_ic: &[
		[
			0,159,119,110,228,65,86,96,205,125,250,16,103,122,74,176,227,25,63,185,67,75,135,159,184,120,217,21,41,110,79,107,
			43,156,239,224,180,104,55,166,43,65,182,61,13,191,88,11,22,247,167,222,241,169,32,63,6,79,94,14,223,193,239,89,
		],
		[
			25,178,83,125,34,193,178,21,211,92,133,53,31,49,101,56,129,44,211,228,87,138,189,226,180,53,90,181,249,187,191,15,
			0,128,73,132,100,180,145,226,94,186,40,192,109,226,19,193,149,32,90,217,109,45,162,172,233,247,99,11,166,79,78,179,
		],
		[
			8,111,220,199,118,17,40,233,219,115,115,121,125,112,78,133,189,4,246,21,52,102,176,194,28,113,67,46,91,48,46,249,
			23,70,170,172,37,234,19,254,232,73,4,253,32,169,45,77,188,160,61,41,142,195,230,153,160,24,147,44,187,35,123,193,
		],
		[
			9,84,180,129,139,69,210,181,224,94,115,127,18,102,219,62,2,181,222,37,61,14,51,35,14,236,202,18,15,30,25,217,
			27,85,39,30,94,18,250,125,170,78,104,3,214,220,237,78,127,4,168,252,39,115,69,59,178,116,138,96,77,113,4,104,
		],
		[
			7,237,95,144,51,231,103,159,205,197,27,252,166,127,166,243,41,95,118,129,88,175,80,131,169,127,33,159,22,147,19,179,
			5,156,251,158,161,98,118,85,56,106,84,235,142,27,117,215,116,17,177,141,189,238,13,221,32,50,7,29,76,13,235,128,
		],
		[
			2,193,63,22,83,209,121,133,176,199,25,40,247,197,0,225,120,74,221,77,226,224,142,96,199,186,22,175,211,23,133,139,
			7,211,193,1,80,7,93,105,100,20,135,231,9,94,220,89,244,191,93,114,219,25,133,45,11,225,235,91,73,98,102,156,
		],
		[
			33,181,204,139,210,197,100,135,215,142,124,3,125,140,121,122,32,176,56,145,242,46,119,126,87,171,101,121,222,253,136,147,
			11,126,233,54,128,234,249,50,130,140,75,18,71,204,163,42,91,26,101,55,247,157,26,43,187,185,182,245,152,74,142,94,
		],
		[
			10,224,231,185,223,37,174,168,59,245,215,192,224,10,12,232,11,115,193,142,4,124,232,206,22,142,61,126,156,149,211,30,
			35,109,51,112,49,188,179,46,221,226,201,119,86,252,231,107,179,213,55,153,229,127,224,189,196,180,243,172,15,131,50,40,
		],
		[
			21,213,105,237,24,171,21,68,17,106,158,179,28,43,91,147,2,48,90,254,69,105,2,243,159,204,240,150,191,30,127,35,
			34,168,38,87,26,235,144,239,125,80,57,219,248,142,141,35,208,45,178,156,101,157,199,183,180,71,19,168,161,45,62,173,
		],
		[
			23,109,174,93,230,43,94,78,216,2,137,227,171,224,19,215,2,251,131,83,236,171,222,56,56,174,156,166,186,254,224,45,
			12,73,135,203,28,6,196,202,173,10,223,17,201,88,89,0,197,98,186,169,88,162,94,65,8,178,207,33,202,166,18,55,
		],
		[
			28,38,42,37,237,39,218,97,9,113,106,156,217,45,191,36,109,61,236,67,255,142,184,51,133,151,200,121,46,38,70,246,
			33,82,66,76,20,22,239,61,130,143,178,190,93,182,5,175,114,186,78,139,91,53,191,223,215,199,83,34,154,44,103,121,
		],
		[
			21,176,190,25,111,44,199,70,80,72,133,245,113,99,108,91,148,59,87,194,203,158,50,62,215,142,62,56,20,70,239,15,
			41,60,16,58,32,14,149,34,76,224,198,238,23,243,86,67,192,8,169,159,66,248,62,124,242,96,173,227,234,152,236,137,
		],
		[
			11,183,239,132,181,131,14,124,125,111,186,206,34,29,4,246,56,218,34,227,0,104,40,158,27,140,226,153,236,240,93,133,
			21,91,201,133,9,223,239,191,120,224,144,30,246,195,184,7,244,138,135,207,93,102,138,30,231,50,254,107,75,96,45,15,
		],
		[
			30,127,41,26,39,39,241,62,46,8,14,194,235,3,5,239,115,250,228,197,192,172,206,214,148,237,243,209,4,182,112,98,
			35,187,40,137,194,121,65,130,15,108,152,5,68,172,45,210,114,121,104,61,179,255,1,121,36,227,102,200,234,212,80,56,
		],
		[
			27,3,31,204,213,217,178,91,184,192,229,39,58,178,243,12,141,120,199,95,160,0,221,148,55,157,198,141,63,170,221,212,
			3,98,161,235,18,63,138,23,200,101,6,11,102,104,22,250,171,13,77,51,204,136,69,17,230,122,15,251,177,33,76,177,
		],
		[
			35,124,32,24,121,183,203,25,198,230,209,250,85,103,243,31,1,10,61,231,191,184,60,59,32,210,107,78,90,98,135,62,
			37,124,147,203,96,190,247,220,223,250,234,126,23,253,94,89,184,74,128,61,192,33,58,111,27,166,114,82,119,223,163,95,
		],
		[
			31,1,243,138,99,39,135,193,142,16,53,98,129,18,129,161,244,27,189,165,52,71,2,119,1,142,165,199,228,108,102,134,
			9,25,167,131,242,32,163,255,168,121,133,53,0,222,159,111,8,183,157,203,25,136,185,113,77,78,155,162,8,233,12,197,
		],
		[
			21,229,148,241,220,210,169,154,104,143,3,133,101,188,79,75,168,47,19,157,117,64,232,192,175,198,213,78,29,89,1,96,
			12,114,215,144,207,40,193,121,24,246,190,100,158,203,101,69,108,195,57,252,182,254,48,148,254,241,78,190,116,152,101,36,
		],
	]
};
