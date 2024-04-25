package chainconfig

import (
	interchaintest "github.com/strangelove-ventures/interchaintest/v8"
	"github.com/strangelove-ventures/interchaintest/v8/ibc"
)

var DefaultChainSpecs = []*interchaintest.ChainSpec{
	// -- 08-WASM SIMD --
	{
		ChainConfig: ibc.ChainConfig{
			Type:    "cosmos",
			Name:    "simd-1",
			ChainID: "simd-1",
			Images: []ibc.DockerImage{
				{
					Repository: "ghcr.io/cosmos/ibc-go-wasm-simd", // FOR LOCAL IMAGE USE: Docker Image Name
					Version:    "v8.0.0-e2e-upgrade",              // FOR LOCAL IMAGE USE: Docker Image Tag
					UidGid:     "1025:1025",
				},
			},
			Bin:            "simd",
			Bech32Prefix:   "cosmos",
			Denom:          "stake",
			GasPrices:      "0.00stake",
			GasAdjustment:  1.3,
			EncodingConfig: WasmEncodingConfig(),
			TrustingPeriod: "508h",
			NoHostMount:    false,
		},
	},
	// -- 08-WASM SIMD --
	{
		ChainConfig: ibc.ChainConfig{
			Type:    "cosmos",
			Name:    "simd-2",
			ChainID: "simd-2",
			Images: []ibc.DockerImage{
				{
					Repository: "ghcr.io/cosmos/ibc-go-wasm-simd", // FOR LOCAL IMAGE USE: Docker Image Name
					Version:    "v8.0.0-e2e-upgrade",              // FOR LOCAL IMAGE USE: Docker Image Tag
					UidGid:     "1025:1025",
				},
			},
			Bin:            "simd",
			Bech32Prefix:   "cosmos",
			Denom:          "stake",
			GasPrices:      "0.00stake",
			GasAdjustment:  1.3,
			EncodingConfig: WasmEncodingConfig(),
			TrustingPeriod: "508h",
			NoHostMount:    false,
		},
	},
}
