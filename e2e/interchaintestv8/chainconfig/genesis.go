package chainconfig

import (
	"fmt"
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	govv1 "github.com/cosmos/cosmos-sdk/x/gov/types/v1"

	"github.com/strangelove-ventures/interchaintest/v8/ibc"
)

func defaultModifyGenesis() func(ibc.ChainConfig, []byte) ([]byte, error) {
	return modifyGovV1AppState
}

// modifyGovV1AppState takes the existing gov app state and marshals it to a govv1 GenesisState.
func modifyGovV1AppState(chainConfig ibc.ChainConfig, govAppState []byte) ([]byte, error) {
	cdc := WasmEncodingConfig().Codec

	govGenesisState := &govv1.GenesisState{}
	if err := cdc.UnmarshalJSON(govAppState, govGenesisState); err != nil {
		return nil, fmt.Errorf("failed to unmarshal genesis bytes into gov genesis state: %w", err)
	}

	if govGenesisState.Params == nil {
		govGenesisState.Params = &govv1.Params{}
	}

	govGenesisState.Params.MinDeposit = sdk.NewCoins(sdk.NewCoin(chainConfig.Denom, govv1.DefaultMinDepositTokens))
	maxDep := time.Second * 10
	govGenesisState.Params.MaxDepositPeriod = &maxDep
	vp := time.Second * 30
	govGenesisState.Params.VotingPeriod = &vp

	// govGenBz := MustProtoMarshalJSON(govGenesisState)

	govGenBz, err := cdc.MarshalJSON(govGenesisState)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal gov genesis state: %w", err)
	}

	return govGenBz, nil
}
