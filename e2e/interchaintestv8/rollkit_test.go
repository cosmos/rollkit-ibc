package main

import (
	"bytes"
	"compress/gzip"
	"context"
	"io"
	"os"
	"testing"
	"time"

	"github.com/stretchr/testify/suite"

	sdk "github.com/cosmos/cosmos-sdk/types"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	govv1 "github.com/cosmos/cosmos-sdk/x/gov/types/v1"

	ibcwasmtypes "github.com/cosmos/ibc-go/modules/light-clients/08-wasm/types"

	"github.com/strangelove-ventures/interchaintest/v8/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v8/ibc"
	"github.com/strangelove-ventures/interchaintest/v8/testutil"

	"github.com/cosmos/rollkit-ibc/e2e/interchaintest/v8/e2esuite"
)

// RollkitTestSuite is a suite of tests that wraps the TestSuite
// and can provide additional functionality
type RollkitTestSuite struct {
	e2esuite.TestSuite
}

// SetupSuite calls the underlying RollkitTestSuite's SetupSuite method
func (s *RollkitTestSuite) SetupSuite(ctx context.Context) {
	s.TestSuite.SetupSuite(ctx)
}

// TestWithRollkitTestSuite is the boilerplate code that allows the test suite to be run
func TestWithRollkitTestSuite(t *testing.T) {
	suite.Run(t, new(RollkitTestSuite))
}

// TestInitialize tests the initialization of the Rollkit client
func (s *RollkitTestSuite) TestInitialize() {
	ctx := context.Background()

	s.SetupSuite(ctx)

	// Store the contract code on ChainA via a governance proposal
	s.Run("StoreContractCode", func() {
		// Submit store contract code proposal on ChainA
		// The contract code should be built in the CI/CD pipeline first
		proposal := s.NewWasmClientProposal(ctx, s.ChainA, s.UserA, "../../artifacts/rollkit_ibc.wasm")
		_, err := s.BroadcastMessages(ctx, s.ChainA, s.UserA, 60_000_000, proposal)
		s.Require().NoError(err)

		// vote on the proposal
		err = s.ChainA.VoteOnProposalAllValidators(ctx, "1", cosmos.ProposalVoteYes)
		s.Require().NoError(err)

		// wait for the proposal to pass
		err = testutil.WaitForCondition(time.Second*30, time.Second*5, func() (bool, error) {
			resp, err := e2esuite.GRPCQuery[govv1.QueryProposalResponse](ctx, s.ChainA, &govv1.QueryProposalRequest{ProposalId: 1})
			if err != nil {
				return false, err
			}

			return resp.Proposal.Status == govv1.ProposalStatus_PROPOSAL_STATUS_PASSED, nil
		})
		s.Require().NoError(err)
	})
}

// NewWasmClientProposal submits a new wasm client governance proposal to the chain.
func (s *RollkitTestSuite) NewWasmClientProposal(ctx context.Context, chain *cosmos.CosmosChain, wallet ibc.Wallet, filePath string) *govv1.MsgSubmitProposal {
	file, err := os.Open(filePath)
	s.Require().NoError(err)

	content, err := io.ReadAll(file)
	s.Require().NoError(err)

	// compress the wasm file since it is too large to submit as a proposal
	var b bytes.Buffer
	gz := gzip.NewWriter(&b)
	_, err = gz.Write(content)
	s.Require().NoError(err)
	gz.Close()

	message := &ibcwasmtypes.MsgStoreCode{
		Signer:       s.GetModuleAddress(ctx, chain, govtypes.ModuleName),
		WasmByteCode: b.Bytes(),
	}

	govProposal, err := govv1.NewMsgSubmitProposal(
		[]sdk.Msg{message}, sdk.NewCoins(sdk.NewCoin(chain.Config().Denom, govv1.DefaultMinDepositTokens)),
		s.UserA.FormattedAddress(), "metadata:e2e", "title:e2e", "summary:e2e", false,
	)
	s.Require().NoError(err)

	// codeResp, err := query.GRPCQuery[wasmtypes.QueryCodeResponse](ctx, chain, &wasmtypes.QueryCodeRequest{Checksum: computedChecksum})
	// s.Require().NoError(err)

	return govProposal
}
